//! Custom error types using the `error-chain` crate.

error_chain! {
    errors {
        /// An error occurred running an external command.
        ExternalCommand(name: String, args: Vec<String>) {
            description("Error running external command")
            display("Error running {:?} with arguments {:?}", name, args)
        }
    }
}

impl Error {
    /// Create an `Error::ExternalCommand` from anything we can convert
    /// into `String`s, using some magic with the `Into` trait.
    pub fn external_command<S1, I, S2>(cmd: S1, args: I) -> Error
    where
        S1: Into<String>,
        I: IntoIterator<Item=S2>,
        S2: AsRef<str>
    {
        let cmd = cmd.into();
        let args = args.into_iter().map(|a| a.as_ref().to_owned()).collect();
        ErrorKind::ExternalCommand(cmd, args).into()
    }
}
