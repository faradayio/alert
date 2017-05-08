//! Various tools for notifying the user.

use std::env;

mod console;
mod pushover;

use errors::*;

/// Interface for notifying the user.
pub trait Notifier {
    /// Let the user know that their process succeed.
    fn notify_success(&self) -> Result<()>;
    /// Let the user know that their process failed.
    fn notify_failure(&self, err: &Error) -> Result<()>;
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
