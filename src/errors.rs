//! Custom error types using the `error-chain` crate.

#[cfg(feature = "notify-rust")]
use notify_rust;
use regex;
use reqwest;
use std::env;
use std::num;
use std::process::ExitStatus;

use crate::command::Command;

error_chain! {
    // Wrap errors provided by other libraries.
    foreign_links {
        Desktop(notify_rust::Error) #[cfg(feature = "notify-rust")];
        Env(env::VarError);
        ParseInt(num::ParseIntError);
        Regex(regex::Error);
        Reqwest(reqwest::Error);
        Url(url::ParseError);
    }

    errors {
        /// The user was already notified of a failure, so we don't need to
        /// print a visible message, but we do need to preserve our exit
        /// status.
        CommandFailedOrTimedOut(status: Option<ExitStatus>) {
            description("Command failed or timed out")
            display("Command failed or timed out with status {:?}", &status)
        }

        /// An error occurred running an external command.
        CouldNotRun(cmd: Command) {
            description("Could not run external command")
            display("Could not run {}", cmd)
        }
    }
}
