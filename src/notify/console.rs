//! A mostly-useless notifier which prints to standard error.  This largely
//! defeats the purpose of this tool, but it's handy for integration testing
//! the CLI.

use colored::*;
use std::io;
use std::io::prelude::*;

use command::Command;
use errors::*;
use super::{Notification, Notifier, Outcome};

/// Notify the user of an event using the console.
pub struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        let label = format!("{}:", notification.title());
        let color_label = match notification.outcome() {
            Outcome::Success => label.green().bold(),
            Outcome::Failure | Outcome::Timeout => label.red().bold(),
        };
        writeln!(&mut io::stderr(),
                 "{} {}",
                 color_label,
                 notification.message().bold())
                .chain_err(|| -> Error { "Could not write to stderr".into() })
    }
}
