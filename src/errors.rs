//! Custom error types using the `error-chain` crate.

#[cfg(feature = "notify-rust")]
use notify_rust;
use regex;
use reqwest;
use std::env;
use std::num;

use command::Command;

error_chain! {
    // Wrap errors provided by other libraries.
    foreign_links {
        Desktop(notify_rust::Error) #[cfg(feature = "notify-rust")];
        Env(env::VarError);
        ParseInt(num::ParseIntError);
        Regex(regex::Error);
        Reqwest(reqwest::Error);
        ReqwestUrl(reqwest::UrlError);
    }

    errors {
        /// An error occurred running an external command.
        CouldNotRun(cmd: Command) {
            description("Could not run external command")
            display("Could not run {}", cmd)
        }
    }
}
