//! Utility functions.

use std::env;

use crate::errors::*;

/// Look up an environment variable.
pub fn env_var(name: &str) -> Result<String> {
    env::var(name).map_err(|source| Error::Env {
        name: name.to_owned(),
        source,
    })
}
