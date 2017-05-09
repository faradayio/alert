//! Our `alert run` subcommand.

use clap::{App, ArgMatches, AppSettings, SubCommand};
use std::process;

use command::Command;
use errors::*;
use notify::{Notifier, Outcome};

/// Return a `clap::SubCommand` specifying our arguments.
pub fn subcommand_definition() -> App<'static, 'static> {
    // Build our subcommand.
    SubCommand::with_name("run")
        .about("Runs a command and notifies when it finishes")
        .setting(AppSettings::DisableVersion)
        // Essential: Don't require "--" before `CMD ARGS...`.
        .setting(AppSettings::TrailingVarArg)
        .args(&Command::clap_args())
}

pub fn run(_global_args: &ArgMatches,
           sub_args: &ArgMatches,
           notifier: &Notifier)
           -> Result<()> {
    let cmd = Command::from_arg_matches(sub_args)?;
    let status = process::Command::new(&cmd.cmd)
        .args(&cmd.args)
        .status()
        .chain_err(|| ErrorKind::CouldNotRun((&cmd).to_owned()))?;
    notifier
        .notify(Outcome::from_bool(status.success()), &cmd)?;
    if status.success() {
        Ok(())
    } else {
        Err(ErrorKind::CouldNotRun(cmd).into())
    }
}
