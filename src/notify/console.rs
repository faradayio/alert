//! A mostly-useless notifier which prints to standard error.  This largely
//! defeats the purpose of this tool, but it's handy for integration testing
//! the CLI.

use std::io;
use std::io::prelude::*;

use command::Command;
use errors::*;
use super::{Notifier, Outcome};

/// Notify the user of an event using the console.
pub struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn notify(&self, outcome: Outcome, cmd: &Command) -> Result<()> {
        let label = match outcome {
            Outcome::Success => "SUCCESS",
            Outcome::Failure => "FAILURE",
        };
        writeln!(&mut io::stderr(), "NOTIFICATION: {}: {}", label, cmd)
            .chain_err(|| -> Error { "Could not write to stderr".into() })
    }
}
