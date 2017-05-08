//! A command and its arguments.

use clap::{Arg, ArgMatches};
use shell_escape;
use std::borrow::Cow;
use std::fmt;

use errors::*;

/// A command and its arguments.  We use this as a wrapper for consistency.
#[derive(Clone, Debug)]
pub struct Command {
    /// The name of the program to run.
    pub cmd: String,
    /// The arguments to pass to the program.
    pub args: Vec<String>,
}

impl Command {
    /// Generate `clap::Arg` values which will match a command name and
    /// optional arguments.
    pub fn clap_args() -> Vec<Arg<'static, 'static>> {
        let command_arg = Arg::with_name("COMMAND")
            .help("The command to run")
            .required(true);
        let args_arg = Arg::with_name("ARGS")
            .help("Arguments to pass to the command")
            .multiple(true);
        vec![command_arg, args_arg]
    }

    /// Given a `clap::ArgMatches`, create a new `Command`.
    pub fn from_arg_matches(arg_matches: &ArgMatches) -> Result<Command> {
        let cmd: &str = arg_matches
            .value_of("COMMAND")
            .ok_or_else(|| -> Error {
                "No command specified".into()
            })?;
        let mut args: Vec<String> = vec![];
        if let Some(arg_iter) = arg_matches.values_of("ARGS") {
            for arg in arg_iter {
                args.push(arg.to_owned());
            }
        }
        Ok(Command {
            cmd: cmd.to_owned(),
            args: args,
        })
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cmd)?;
        for arg in &self.args {
            write!(f, " {}", shell_escape::escape(Cow::Borrowed(arg)))?;
        }
        Ok(())
    }
}

#[test]
fn commands_are_escaped_for_display() {
    let cmd = Command {
        cmd: "echo".to_owned(),
        args: vec!["Hello world".to_owned()],
    };
    assert_eq!(format!("{}", cmd), "echo \'Hello world\'");
}
