//! Custom error types using the `error-chain` crate.

use std::env;
use std::io;
use std::num;
use std::process::ExitStatus;
use thiserror::Error;

use crate::command::Command;

/// Create a custom `Result` type which defaults `Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error that occurs while running `alert`.
///
/// TODO: Support backtraces when they stabilize in `std`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    #[cfg(feature = "notify-rust")]
    Desktop(#[from] notify_rust::Error),

    #[error(transparent)]
    ParseInt(#[from] num::ParseIntError),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    /// The user was already notified of a failure, so we don't need to
    /// print a visible message, but we do need to preserve our exit
    /// status.
    #[error("Command failed or timed out with status {:?}", .status)]
    CommandFailedOrTimedOut { status: Option<ExitStatus> },

    /// An error occurred running an external command.
    #[error("Could not run {} ({})", .cmd, .source)]
    CouldNotRun { cmd: Command, source: io::Error },

    /// The notify app failed for some unspecified reason.
    #[error("Could not send notification using {}", .service)]
    CouldNotSendNotification { service: String },

    /// We could not write to either stdout or stderr.
    #[error("Could not write to {} ({})", .dest, .source)]
    CouldNotWriteToStdio {
        /// Either `"stdout"` or `"stderr"`.
        dest: &'static str,
        source: io::Error,
    },

    /// We could not read an environment variable.
    #[error("Could not read environment variable {} ({})", .name, .source)]
    Env { name: String, source: env::VarError },

    /// No command to run was specified.
    #[error("No command to run was specified")]
    NoCommandSpecified,

    /// The user requested an unknown notification backend.
    #[error("Unknown notifier: {}", .name)]
    UnknownNotifier { name: String },
}
