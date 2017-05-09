//! Custom error types using the `error-chain` crate.

use regex;
use reqwest;
use std::env;
use std::num;

use command::Command;

error_chain! {
    // Wrap errors provided by other libraries.
    foreign_links {
        Env(env::VarError);
        ParseInt(num::ParseIntError);
        Regex(regex::Error);
        Reqwest(reqwest::Error);
    }

    errors {
        /// An error occurred running an external command.
        CouldNotRun(cmd: Command) {
            description("Could not run external command")
            display("Could not run {}", cmd)
        }
    }
}
