// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Enable clippy if we were asked to do so.
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate notify_rust;
extern crate regex;
extern crate reqwest;

use clap::{App, AppSettings};

mod cmd_run;
mod errors;
mod notify;

use errors::*;
use notify::choose_notifier;

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init().expect("could not initialize log subsystem");

    // Parse our command-line arguments.
    let app = App::new("alert")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Eric Kidd")
        .about("Runs processes and notifies you about what happened")
        .subcommand(cmd_run::subcommand_defintion())
        .setting(AppSettings::SubcommandRequiredElseHelp);
    let matches = app.get_matches();
    debug!("Arguments: {:#?}", &matches);

    // Create our notifier _now_ before running any multi-hour subcommands, so
    // that it has a chance to make sure it's configured correctly while the
    // user is still watching.
    let notifier = choose_notifier()?;

    // Run a subcommand.
    let result = match matches.subcommand() {
        ("run", Some(sub_args)) => cmd_run::run(&matches, sub_args),
        (_, _) => unreachable!("unimplemented subcommand"),
    };

    // Notify the user about what happened.
    match result {
        Ok(()) => notifier.notify_success()?,
        Err(ref err) => notifier.notify_failure(err)?,
    }

    result
}
