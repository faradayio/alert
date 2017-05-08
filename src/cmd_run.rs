
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::Command;

use errors::*;

/// Return a `clap::SubCommand` specifying our arguments.
pub fn subcommand_defintion() -> App<'static, 'static> {
    // Define our arguments, with all the bells and whistles we want.
    let command_arg = Arg::with_name("COMMAND")
        .help("The command to run")
        .required(true);
    let args_arg = Arg::with_name("ARGS")
        .help("Arguments to pass to the command")
        .multiple(true);

    // Build our subcommand.
    SubCommand::with_name("run")
        .about("Runs a command and notifies when it finishes")
        .arg(&command_arg)
        .arg(&args_arg)
}

pub fn run(_global_args: &ArgMatches, sub_args: &ArgMatches) -> Result<()> {
    let cmd: &str = sub_args.value_of("COMMAND")
        .expect("clap should have guaranteed COMMAND was present");
    let args: Vec<&str> = sub_args.values_of("ARGS")
        .map_or_else(|| vec![], |vals| vals.collect());

    let status = Command::new(cmd)
        .args(&args)
        .status()
        .chain_err(|| Error::external_command(cmd, &args))?;
    if !status.success() {
        return Err(Error::external_command(cmd, &args));
    }

    Ok(())
}
