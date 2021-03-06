//! Send notifications using the open source [Notify][] app.
//!
//! [Notify]: https://github.com/mashlol/notify

use log::debug;

use super::{Notification, Notifier};
use crate::errors::*;
use crate::util::env_var;

/// Notify the user of an event using the open source Notify app.
pub struct NotifyAppNotifier {
    /// Registration key.
    key: String,
}

impl NotifyAppNotifier {
    /// Create a new notifier and configure it automatically.
    pub fn new() -> Result<NotifyAppNotifier> {
        Ok(NotifyAppNotifier {
            key: env_var("NOTIFYAPP_KEY")?,
        })
    }
}

impl Notifier for NotifyAppNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        let client = reqwest::blocking::Client::new();

        let base_url = "https://appnotify.herokuapp.com/notify";
        let url = reqwest::Url::parse_with_params(
            base_url,
            &[
                ("to", &self.key),
                ("title", &notification.title()),
                ("text", &notification.message()),
            ],
        )?;

        debug!("Sending notification via Notify app");
        let response = client.get(url.as_str()).send()?;
        debug!("Notify response: {:?}", response.status());
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::CouldNotSendNotification {
                service: "Notify app".to_owned(),
            })
        }
    }
}
