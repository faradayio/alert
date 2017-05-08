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

mod errors;
mod cmd_run;

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init().expect("could not initialize log subsystem");

    let app = App::new("alert")
        .version("FOO")
        .author("Eric Kidd")
        .about("Runs processes and notifies you about what happened")
        .subcommand(cmd_run::subcommand_defintion())
        .setting(AppSettings::SubcommandRequiredElseHelp);
    let matches = app.get_matches();
    debug!("Arguments: {:#?}", &matches);

    match matches.subcommand() {
        ("run", Some(sub_args)) => cmd_run::run(&matches, sub_args)?,
        (_, _) => unreachable!("unimplemented subcommand"),
    }

    Ok(())
}
