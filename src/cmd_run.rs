//! Our `alert run` subcommand.

use clap::AppSettings;
use std::process;
use structopt::StructOpt;

use crate::command::Command;
use crate::errors::*;
use crate::notify::{Notification, Notifier, Outcome};

/// Options for `run`.
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Runs a command and notifies when it finishes",
    // Essential: Don't require "--" before `CMD...`.
    setting(AppSettings::TrailingVarArg)
)]
pub struct Opt {
    /// The command to run, with any arguments.
    cmd: Vec<String>,
}

pub fn run(opt: &Opt, notifier: &dyn Notifier) -> Result<()> {
    let cmd = Command::from_slice(&opt.cmd)?;
    let status = process::Command::new(&cmd.cmd)
        .args(&cmd.args)
        .status()
        .map_err(|source| Error::CouldNotRun {
            cmd: (&cmd).to_owned(),
            source,
        })?;
    let notification =
        Notification::new(Outcome::from_bool(status.success())).command(cmd);
    notifier.send(&notification)?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailedOrTimedOut {
            status: Some(status),
        })
    }
}
