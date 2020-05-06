//! Display ordinary desktop notifications using standard OS features.

use super::{Notification, Notifier};
use crate::errors::*;

/// Display notifications on the local machine using its GUI
pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        debug!("Sending notification to desktop");
        notify_rust::Notification::new()
            .summary(&notification.title())
            .body(&notification.message())
            .show()?;
        debug!("Sent notification to desktop");
        Ok(())
    }
}
