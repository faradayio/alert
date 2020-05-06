//! Our `alert run` subcommand.

use clap::{App, AppSettings, ArgMatches, SubCommand};
use std::process;

use crate::command::Command;
use crate::errors::*;
use crate::notify::{Notification, Notifier, Outcome};

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

pub fn run(
    _global_args: &ArgMatches,
    sub_args: &ArgMatches,
    notifier: &dyn Notifier,
) -> Result<()> {
    let cmd = Command::from_arg_matches(sub_args)?;
    let status = process::Command::new(&cmd.cmd)
        .args(&cmd.args)
        .status()
        .chain_err(|| ErrorKind::CouldNotRun((&cmd).to_owned()))?;
    let notification = Notification::new(Outcome::from_bool(status.success()))
        .command(cmd.to_owned());
    notifier.send(&notification)?;
    if status.success() {
        Ok(())
    } else {
        Err(ErrorKind::CommandFailedOrTimedOut(Some(status)).into())
    }
}
