// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

use clap::{App, AppSettings};
use log::debug;
use std::process;
use structopt::StructOpt;

mod cmd_run;
mod cmd_watch;
mod command;
mod config;
mod errors;
mod notify;

use crate::errors::*;
use crate::notify::choose_notifier;

/// Our command-line options.
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs processes and notifies you about what happened")]
enum Opt {
    /// Runs a command and notifies when it finishes.
    Run {
        #[structopt(flatten)]
        run_opt: cmd_run::Opt,
    },

    /// Runs a command repeatedly and watches for output.
    Watch {
        #[structopt(flatten)]
        watch_opt: cmd_watch::Opt,
    },
}

fn main() {
    if let Err(err) = run() {
        // Our only "wrapper" errors print out the original error, too, so don't
        // worry about printing our original errors.
        eprintln!("ERROR: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    env_logger::init();

    // Parse our command-line arguments.
    let opt = Opt::from_args();
    debug!("Arguments: {:#?}", opt);

    // Create our notifier _now_ before running any multi-hour subcommands, so
    // that it has a chance to make sure it's configured correctly while the
    // user is still watching.
    let notifier = choose_notifier()?;

    // Run a subcommand.
    let result = match &opt {
        Opt::Run { run_opt } => cmd_run::run(run_opt, notifier.as_ref()),
        Opt::Watch { watch_opt } => cmd_watch::run(watch_opt, notifier.as_ref()),
    };

    // Handle `CommandFailedOrTimedOut` specially, and pass everything else
    // through to `quick_main!`.
    if let Err(ref err) = result {
        if let Error::CommandFailedOrTimedOut { status } = err {
            // We've already notified the user of this failure, but we still
            // need to exit with the right status code.
            //
            // TODO: We might be able to use Unix-specific Rust APIs to
            // preserve the exit status when the child process is terminated
            // by a signal, too.
            let code = status.and_then(|s| s.code()).unwrap_or(1);
            debug!("Exiting with code {}", code);
            process::exit(code);
        }
    }
    result
}
