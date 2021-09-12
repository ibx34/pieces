#![deny(missing_docs)]
//! Pieces is a command line argument parser with user control in mind.

/// FancyArgs is just a better way of using [env::args](std::env::args).
#[derive(Debug)]
pub struct FancyArgs {
    /// The actual args in a vec string format.
	pub inner: Vec<String>,
}

impl FancyArgs {
    /// Manually gets [env::args](std::env::args) and removes the first
    /// item, if no args are passed it will be zero. If you don't have custom
    /// args, from like a string, use this instead of [load](load).
	pub fn grab() -> FancyArgs {

		let mut args = std::env::args();

		args.next();

		FancyArgs {
			inner: args.into_iter().collect::<Vec<String>>(),
		}
	}

    /// Returns FancyArgs from the provided args.
	pub fn load(args: std::env::Args) -> FancyArgs {

		FancyArgs {
			inner: args.into_iter().collect::<Vec<String>>(),
		}
	}
}

/// The parser for parsing commands, flags, and arguments.
#[derive(Debug)]

pub struct Parser {
	pub(crate) raw_args: FancyArgs,

    /// Args from the main app
	pub args: Vec<args::Arg>,
}

impl Parser {
	// TODO after the app is created, add an argument here and to the struct
	// for getting commands and such.

    /// Builds the parser so parsing can begin.
	pub fn build(raw_args: FancyArgs, args: Option<Vec<args::Arg>>) -> Parser {

		Parser {
			raw_args,
			args: match args {
				Some(a) => a,
				None => vec![],
			},
		}
	}
}

/// Everything argument related
pub mod args {

    /// The struct for Arg's. Creates a new Argument for commands and the main app
    /// without a long/short name it will be considered a positional argument.
    /// If you do set a long/short name people can use `--<arg>`/`-<arg>`.
	#[derive(Debug, Clone)]
	pub struct Arg {
        /// Name of the argument
		pub name: String,
        /// Long name of the argument
		pub long: Option<String>,
        /// Short name of the argument
		pub short: Option<String>,
        /// Help for the argument. Appears when users run help.
		pub help: Option<String>,
        /// Whether or not the argument is required. If it is required it will
        /// be checked when parsing is underway.
		pub required: bool,
	}

	impl Arg {
        /// Creates a new argument with the provided name.
		pub fn new(name: String) -> Arg {

			Arg {
				name,
				long: None,
				short: None,
				help: None,
				required: false,
			}
		}

        /// Sets the argument's long name. So it can be used like `--<name>`
		pub fn long(mut self, long: String) -> Self {

			self.long = Some(long);

			self
		}

        /// Sets the argument's short name. So it ca be used like `-<name>`
		pub fn short(mut self, short: String) -> Self {

			self.short = Some(short);

			self
		}

        /// Sets the argument's help. Appears when running help on the argument's 
        /// command / running help for the main command.
		pub fn help(mut self, help: String) -> Self {

			self.help = Some(help);

			self
		}

        /// Sets if the argument is required.
		pub fn required(mut self, required: bool) -> Self {

			self.required = required;

			self
		}
	}

    /// Checks the provided vec of args for duplicates. If any of the arguments's
    /// names and long/short names match the function will return true, the 
    /// current argument, the argument the current argument matched on.
	pub fn check_args<'a>(
		args: &'a Vec<Arg>,
	) -> (bool, Option<&'a Arg>, Option<&'a Arg>) {

		let mut args = args.iter();

		while let Some(arg) = args.next() {

			match args.find(|a| {

				a.name == arg.name
					|| a.short == arg.short && a.short.is_some()
					|| a.long == arg.long && a.long.is_some()
			}) {
				Some(ark) => return (true, Some(&arg), Some(ark)),
				None => continue,
			}
		}

		return (false, None, None);
	}
}
