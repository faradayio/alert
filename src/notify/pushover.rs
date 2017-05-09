//! Notifier implementation for pushover.net. This is a commercial service
//! with excellent support for mobile notifications on iOS, Android and the
//! desktop, with the option of aggressively annoying sounds.
//!
//! Pushover is a trademark of Superblock, LLC. This library is not associated
//! with Superblock, LLC in any way.

use reqwest;
use std::env;

use command::Command;
use errors::*;
use super::{Notifier, Outcome};

/// Notify the user of an event using the pushover.net service from
/// Superblock, LLC.
pub struct PushoverNotifier {
    /// Per-application notification token.
    token: String,
    /// Per-user key to specify who should be notified.
    user: String,
}

impl PushoverNotifier {
    /// Create a new notifier and configure it automatically.
    pub fn new() -> Result<PushoverNotifier> {
        Ok(PushoverNotifier {
               token: env::var("PUSHOVER_TOKEN")?,
               user: env::var("PUSHOVER_USER")?,
           })
    }
}

impl Notifier for PushoverNotifier {
    fn notify(&self, outcome: Outcome, cmd: &Command) -> Result<()> {
        let (title, sound) = match outcome {
            Outcome::Success => ("Command succeeded", "classical"),
            Outcome::Failure => ("Command failed", "tugboat"),
            Outcome::Timeout => ("Command timed out", "tugboat"),
        };

        let client = reqwest::Client::new()?;
        let params = [("token", &self.token[..]),
                      ("user", &self.user[..]),
                      ("title", title),
                      ("sound", sound),
                      ("message", &format!("{}", cmd)[..])];
        let response = client
            .post("https://api.pushover.net/1/messages.json")
            .form(&params)
            .send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err("could not send notification using pushover.net".into())
        }
    }
}
