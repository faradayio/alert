//! A mostly-useless notifier which prints to standard error.  This largely
//! defeats the purpose of this tool, but it's handy for integration testing
//! the CLI.

use std::io;
use std::io::prelude::*;

use errors::*;
use super::Notifier;

/// Notify the user of an event using the pushover.net service from
/// Superblock, LLC.
pub struct ConsoleNotifier;

impl ConsoleNotifier {
    /// Post a message to the API.
    fn send_message(&self, message: &str) -> Result<()> {
        writeln!(&mut io::stderr(), "NOTIFICATION: {}", message)
            .chain_err(|| -> Error { "Could not write to stderr".into() })
    }
}

impl Notifier for ConsoleNotifier {
    /// Let the user know that their process succeed.
    fn notify_success(&self) -> Result<()> {
        self.send_message("SUCCESS")
    }

    /// Let the user know that their process failed.
    fn notify_failure(&self, err: &Error) -> Result<()> {
        self.send_message("FAILURE")
    }
}
