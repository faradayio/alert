// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

use clap::{App, AppSettings};
use log::debug;
use std::process;

mod cmd_run;
mod cmd_watch;
mod command;
mod config;
mod errors;
mod notify;

use crate::errors::*;
use crate::notify::choose_notifier;

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
    let app = App::new("alert")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Eric Kidd")
        .about("Runs processes and notifies you about what happened")
        .subcommand(cmd_run::subcommand_definition())
        .subcommand(cmd_watch::subcommand_definition())
        .setting(AppSettings::SubcommandRequiredElseHelp);
    let matches = app.get_matches();
    debug!("Arguments: {:#?}", &matches);

    // Create our notifier _now_ before running any multi-hour subcommands, so
    // that it has a chance to make sure it's configured correctly while the
    // user is still watching.
    let notifier = choose_notifier()?;

    // Run a subcommand.
    let result = match matches.subcommand() {
        ("run", Some(sub_args)) => cmd_run::run(&matches, sub_args, notifier.as_ref()),
        ("watch", Some(sub_args)) => {
            cmd_watch::run(&matches, sub_args, notifier.as_ref())
        }
        (_, _) => unreachable!("unimplemented subcommand"),
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
