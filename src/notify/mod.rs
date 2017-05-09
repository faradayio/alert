//! Various tools for notifying the user.

use std::env;

mod console;
mod pushover;

use command::Command;
use errors::*;

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
    fn notify(&self, outcome: Outcome, cmd: &Command) -> Result<()>;
}

/// Choose an appropriate notifier backend to use.  We return a `Box`
/// containing an object that implements the trait `Notifier`, which is about
/// as close as Rust gets to object-oriented programming.
pub fn choose_notifier() -> Result<Box<Notifier>> {
    let name = env::var("ALERT_NOTIFIER").unwrap_or_else(|_| "pushover".to_owned());
    match &name[..] {
        "console" => Ok(Box::new(console::ConsoleNotifier)),
        "pushover" => Ok(Box::new(pushover::PushoverNotifier::new()?)),
        _ => Err(format!("Unknown notifier: {}", name).into()),
    }
}
