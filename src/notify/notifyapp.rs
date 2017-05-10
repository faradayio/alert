//! Send notifications using the open source [Notify][] app.
//!
//! [Notify]: https://github.com/mashlol/notify

use reqwest;
use std::env;

use errors::*;
use super::{Notification, Notifier};

/// Notify the user of an event using the pushover.net service from
/// Superblock, LLC.
pub struct NotifyAppNotifier {
    /// Registration key.
    key: String,
}

impl NotifyAppNotifier {
    /// Create a new notifier and configure it automatically.
    pub fn new() -> Result<NotifyAppNotifier> {
        Ok(NotifyAppNotifier { key: env::var("NOTIFYAPP_KEY")? })
    }
}

impl Notifier for NotifyAppNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        let client = reqwest::Client::new()?;

        let base_url = "https://appnotify.herokuapp.com/notify";
        let url =
            reqwest::Url::parse_with_params(base_url,
                                            &[("to", &self.key),
                                              ("title", &notification.title()),
                                              ("text", &notification.message())])?;

        let response = client.get(url.as_str()).send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err("Could not send notification using Notify app".into())
        }
    }
}