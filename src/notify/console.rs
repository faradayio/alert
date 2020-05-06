//! A mostly-useless notifier which prints to standard error.  This largely
//! defeats the purpose of this tool, but it's handy for integration testing
//! the CLI.

use colored::*;

use super::{Notification, Notifier, Outcome};
use crate::errors::*;

/// Notify the user of an event using the console.
pub struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        let label = format!("{}:", notification.title());
        let color_label = match notification.outcome() {
            Outcome::Success => label.green().bold(),
            Outcome::Failure | Outcome::Timeout => label.red().bold(),
        };
        eprintln!("{} {}", color_label, notification.message().bold());
        Ok(())
    }
}
