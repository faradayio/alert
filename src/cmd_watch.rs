//! Our `alert run` subcommand.

use clap::AppSettings;
use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::process;
use std::thread;
use std::time;
use structopt::StructOpt;

use crate::command::Command;
use crate::errors::*;
use crate::notify::{Notification, Notifier, Outcome};

/// Options for `watch`.
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Runs a command repeatedly and watches for output",
    // Essential: Don't require "--" before `CMD...`.
    setting(AppSettings::TrailingVarArg)
)]
pub struct Opt {
    /// Report success when matching text appears.
    #[structopt(short = "s", long = "success", value_name = "SUCCESS_REGEX")]
    success: Option<Regex>,

    /// Report failure when matching text appears.
    #[structopt(short = "f", long = "failure", value_name = "FAILURE_REGEX")]
    failure: Option<Regex>,

    /// Give up if nothing happens after a wait.
    #[structopt(short = "t", long = "timeout", value_name = "SECONDS")]
    timeout: Option<u64>,

    /// Time to wait between runs.
    #[structopt(
        short = "n",
        long = "interval",
        value_name = "SECONDS",
        default_value = "2"
    )]
    interval: u64,

    /// The command to run, with any arguments.
    cmd: Vec<String>,
}

pub fn run(opt: &Opt, notifier: &dyn Notifier) -> Result<()> {
    let cmd = Command::from_slice(&opt.cmd)?;

    let start = time::SystemTime::now();
    let end = opt
        .timeout
        .map(|t_o| start + time::Duration::from_secs(t_o));

    loop {
        // Run our command once, and figure out what to do.
        match run_once(&cmd) {
            Err(err) => {
                eprintln!("{}", err);
            }
            Ok(output) => {
                // Print out the command's output. We end up undoing any
                // interleaving in the original output, but it would take a
                // fair bit more code to avoid this.
                io::stdout().write(&output.stdout).map_err(|source| {
                    Error::CouldNotWriteToStdio {
                        dest: "stdout",
                        source,
                    }
                })?;
                io::stderr().write(&output.stderr).map_err(|source| {
                    Error::CouldNotWriteToStdio {
                        dest: "stderr",
                        source,
                    }
                })?;

                // Convert stdout and stderr to a vector of lines so we
                // can easily search them both.  (There are other reasonable
                // ways to do this.)
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let mut all_lines: Vec<&str> = vec![];
                all_lines.extend(stdout.lines());
                all_lines.extend(stderr.lines());

                // Check for failure first so it takes priority.
                for line in &all_lines {
                    if let Some(re) = &opt.failure {
                        if re.is_match(line) {
                            let notification = Notification::new(Outcome::Failure)
                                .command(cmd.clone());
                            notifier.send(&notification)?;
                            return Err(Error::CommandFailedOrTimedOut {
                                status: None,
                            });
                        }
                    }
                }

                // Check for success.
                for line in &all_lines {
                    if let Some(re) = &opt.success {
                        if re.is_match(line) {
                            let notification = Notification::new(Outcome::Success)
                                .command(cmd.clone());
                            notifier.send(&notification)?;
                            return Ok(());
                        }
                    }
                }
            }
        }

        // Check our timeout.
        if let Some(end) = end {
            if time::SystemTime::now() >= end {
                let notification = Notification::new(Outcome::Timeout).command(cmd);
                notifier.send(&notification)?;
                return Err(Error::CommandFailedOrTimedOut { status: None });
            }
        }

        // Wait until it's time to run again.
        thread::sleep(time::Duration::from_secs(opt.interval));
    }
}

/// Run a command a single time, and return its output.
pub fn run_once(cmd: &Command) -> Result<process::Output> {
    let output = process::Command::new(&cmd.cmd)
        .args(&cmd.args)
        .output()
        .map_err(|source| Error::CouldNotRun {
            cmd: cmd.to_owned(),
            source,
        })?;
    Ok(output)
}
