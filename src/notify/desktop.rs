//! Display ordinary desktop notifications using standard OS features.

use notify_rust;

use errors::*;
use super::{Notification, Notifier};

/// Display notifications on the local machine using its GUI
pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        notify_rust::Notification::new()
            .summary(&notification.title())
            .body(&notification.message())
            .show()?;
        Ok(())
    }
}
