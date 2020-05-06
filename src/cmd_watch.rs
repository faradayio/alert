//! Our `alert run` subcommand.

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::process;
use std::thread;
use std::time;

use crate::command::Command;
use crate::errors::*;
use crate::notify::{Notification, Notifier, Outcome};

/// Return a `clap::SubCommand` specifying our arguments.
pub fn subcommand_definition() -> App<'static, 'static> {
    let success_arg = Arg::with_name("success")
        .short("s")
        .long("success")
        .value_name("SUCCESS_REGEX")
        .help("Report success when matching text appears");
    let failure_arg = Arg::with_name("failure")
        .short("f")
        .long("failure")
        .value_name("FAILURE_REGEX")
        .help("Report failure when matching text appears");
    let timeout_arg = Arg::with_name("timeout")
        .short("t")
        .long("timeout")
        .value_name("SECONDS")
        .help("Give up if nothing happens after a wait");
    let interval_arg = Arg::with_name("interval")
        .short("n")
        .long("interval")
        .value_name("SECONDS")
        .default_value("2")
        .help("Time to wait between runs");

    // Build our subcommand.
    SubCommand::with_name("watch")
        .about("Runs a command repeatedly and watches for output")
        .setting(AppSettings::DisableVersion)
        .arg(success_arg)
        .arg(failure_arg)
        .arg(timeout_arg)
        .arg(interval_arg)
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

    // Parse our arguments.  We could do this in fewer lines of code by
    // heavily using obscure Rust tricks, but I prefer clarity here.
    let success_re: Option<Regex> = match sub_args.value_of("success") {
        Some(s) => Some(Regex::new(s)?),
        None => None,
    };
    let failure_re: Option<Regex> = match sub_args.value_of("failure") {
        Some(s) => Some(Regex::new(s)?),
        None => None,
    };
    let timeout: Option<u64> = match sub_args.value_of("timeout") {
        Some(v) => Some(v.parse()?),
        None => None,
    };
    let interval: u64 = match sub_args.value_of("interval") {
        Some(v) => v.parse()?,
        None => unreachable!("interval should have defaulted automatically"),
    };

    let start = time::SystemTime::now();
    let end = timeout.map(|t_o| start + time::Duration::from_secs(t_o));

    loop {
        // Run our command once, and figure out what to do.
        match run_once(&cmd) {
            Err(err) => {
                writeln!(&mut io::stderr(), "{}", err)
                    .chain_err(|| "Could not write to stderr")?;
            }
            Ok(output) => {
                // Print out the command's output. We end up undoing any
                // interleaving in the original output, but it would take a
                // fair bit more code to avoid this.
                io::stdout()
                    .write(&output.stdout)
                    .chain_err(|| "Could not write to stdout")?;
                io::stderr()
                    .write(&output.stderr)
                    .chain_err(|| "Could not write to stderr")?;

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
                    if let Some(ref re) = failure_re {
                        if re.is_match(line) {
                            let notification = Notification::new(Outcome::Failure)
                                .command(cmd.clone());
                            notifier.send(&notification)?;
                            return Err(
                                ErrorKind::CommandFailedOrTimedOut(None).into()
                            );
                        }
                    }
                }

                // Check for success.
                for line in &all_lines {
                    if let Some(ref re) = success_re {
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
                let notification =
                    Notification::new(Outcome::Timeout).command(cmd.clone());
                notifier.send(&notification)?;
                return Err(ErrorKind::CommandFailedOrTimedOut(None).into());
            }
        }

        // Wait until it's time to run again.
        thread::sleep(time::Duration::from_secs(interval));
    }
}

/// Run a command a single time, and return its output.
pub fn run_once(cmd: &Command) -> Result<process::Output> {
    let output = process::Command::new(&cmd.cmd)
        .args(&cmd.args)
        .output()
        .chain_err(|| ErrorKind::CouldNotRun(cmd.to_owned()))?;
    Ok(output)
}
