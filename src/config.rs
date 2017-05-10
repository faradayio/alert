//! Configuration support.

use std::collections::HashMap;

/// A parsed version of our configuration file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// Configuration for each notifier.
    notifiers: HashMap<String, NotifierConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotifierConfig {
    enabled: bool,
    backend: String,
    options: HashMap<String, String>,
}
