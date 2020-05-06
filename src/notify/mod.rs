//! Various tools for notifying the user.

use std::env;

mod console;
#[cfg(feature = "notify-rust")]
mod desktop;
mod notifyapp;
mod pushover;

use crate::command::Command;
use crate::errors::*;

/// A notification we want to send to the user.
#[derive(Clone, Debug)]
pub struct Notification {
    outcome: Outcome,
    command: Option<Command>,
}

impl Notification {
    /// Create a new notification.
    pub fn new(outcome: Outcome) -> Notification {
        Notification {
            outcome,
            command: None,
        }
    }

    /// Specify the command for a newly-created notification, using
    /// the builder pattern.
    pub fn command(mut self, command: Command) -> Notification {
        self.command = Some(command);
        self
    }

    /// Get the outcome associated with this notification, in case a
    /// particular notifier wishes to further customize message options
    /// like sounds or colors.
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    /// Generate a title for this notification.
    pub fn title(&self) -> String {
        match self.outcome {
            Outcome::Success => "Command succeeded".to_owned(),
            Outcome::Failure => "Command failed".to_owned(),
            Outcome::Timeout => "Command timed out".to_owned(),
        }
    }

    /// Generate a message body for this notification.
    pub fn message(&self) -> String {
        let mut lines = vec![];
        if let Some(ref command) = self.command {
            lines.push(format!("{}", command));
        }
        lines.join("\n")
    }
}

/// What happened to the process we were running?
#[derive(Clone, Copy, Debug)]
pub enum Outcome {
    /// The process succeeded.
    Success,
    /// The process failed.
    Failure,
    /// We timed out waiting for something to happen.
    Timeout,
}

impl Outcome {
    /// Create an `Outcome` from a boolean value indicating whether our
    /// process succeeded.
    pub fn from_bool(success: bool) -> Outcome {
        if success {
            Outcome::Success
        } else {
            Outcome::Failure
        }
    }
}

/// Interface for notifying the user.
pub trait Notifier {
    /// Let the user know that their process succeed.
    fn send(&self, notification: &Notification) -> Result<()>;
}

/// Choose an appropriate notifier backend to use.  We return a `Box`
/// containing an object that implements the trait `Notifier`, which is about
/// as close as Rust gets to object-oriented programming.
pub fn choose_notifier() -> Result<Box<dyn Notifier>> {
    let name = env::var("ALERT_NOTIFIER").unwrap_or_else(|_| "pushover".to_owned());
    match &name[..] {
        "console" => Ok(Box::new(console::ConsoleNotifier)),
        #[cfg(feature = "notify-rust")]
        "desktop" => Ok(Box::new(desktop::DesktopNotifier)),
        "notifyapp" => Ok(Box::new(notifyapp::NotifyAppNotifier::new()?)),
        "pushover" => Ok(Box::new(pushover::PushoverNotifier::new()?)),
        _ => Err(Error::UnknownNotifier { name }),
    }
}
