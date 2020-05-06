//! A command and its arguments.

use std::borrow::Cow;
use std::fmt;

use crate::errors::*;

/// A command and its arguments.  We use this as a wrapper for consistency.
#[derive(Clone, Debug)]
pub struct Command {
    /// The name of the program to run.
    pub cmd: String,
    /// The arguments to pass to the program.
    pub args: Vec<String>,
}

impl Command {
    /// Given a `clap::ArgMatches`, create a new `Command`.
    pub fn from_slice(args: &[String]) -> Result<Command> {
        if args.is_empty() {
            Err(Error::NoCommandSpecified)
        } else {
            Ok(Command {
                cmd: args[0].clone(),
                args: args[1..].to_owned(),
            })
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
