//! Custom error types using the `error-chain` crate.

use reqwest;
use std::env;

use command::Command;

error_chain! {
    // Wrap errors provided by other libraries.
    foreign_links {
        Env(env::VarError);
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
