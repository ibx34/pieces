#![deny(missing_docs)]
//! Pieces is a command line argument parser with user control in mind.

/// FancyArgs is just a better way of using [env::args](std::env::args).
#[derive(Debug, PartialEq)]
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

/// ...
pub mod parse {

	use std::collections::HashMap;

	use bitflags::bitflags;

	use crate::args;
	use crate::commands;
	use crate::FancyArgs;

	bitflags! {
		/// ...
		pub struct ParserSettings: u32 {
			/// ...
			const CHECK_ARG_UNIQUENESS = 1;
			/// ...
			const CHECK_ARG_NAMES = 1 << 2;
			/// ...
			const CHECK_COMMAND_UNIQUENESS = 1 << 3;
			/// ...
			const CHECK_COMMAND_NAMES = 1 << 4;
			/// ...
			const CHECK_COMMAND_ARG_UNIQUENESS = 1 << 5;
			/// ...
			const CHECK_COMMAND_ARG_NAMES = 1 << 6;
		}
	}

	/// ...
	#[derive(Debug, PartialEq)]
	pub struct ParserResult<'a> {
		/// ...
		pub present_commands: HashMap<String, &'a commands::Command>,
		/// ...
		pub present_args: HashMap<String, args::Arg>,
	}

	impl<'a> ParserResult<'a> {
		/// ...
		pub fn new() -> ParserResult<'a> {
			ParserResult {
				present_commands: HashMap::new(),
				present_args: HashMap::new(),
			}
		}

		/// ...
		pub fn value_of<'b>(
			&'b self,
			key: String,
		) -> Result<&'b commands::Command, std::io::Error> {
			match self.present_commands.get(&key) {
				Some(command) => Ok(command),
				None => Err(std::io::Error::new(
					std::io::ErrorKind::NotFound,
					"Command not present.",
				)),
			}
		}

		/// ...
		pub fn is_present(
			self,
			key: String,
		) -> bool{
			match self.present_commands.get(&key) {
				Some(_) => true,
				None => false,
			}
		}
	}

	/// The parser for parsing commands, flags, and arguments.
	#[derive(Debug, PartialEq)]

	pub struct Parser {
		/// ...
		pub raw_args: FancyArgs,

		/// Args from the main app
		pub args: Vec<args::Arg>,

		/// Commands
		pub commands: Vec<commands::Command>,

		/// Settings, set with ParserSettings:
		pub settings: ParserSettings,
	}

	impl Parser {
		// TODO after the app is created, add an argument here and to the struct
		// for getting commands and such.

		/// Builds the parser so parsing can begin
		pub fn build(
			raw_args: FancyArgs,
			args: Option<Vec<args::Arg>>,
			commands: Option<Vec<commands::Command>>,
		) -> Parser {
			Parser {
				raw_args,
				args: match args {
					Some(a) => a,
					None => vec![],
				},
				commands: match commands {
					Some(a) => a,
					None => vec![],
				},
				settings: ParserSettings::empty(),
			}
		}

		/// ...
		pub fn parse<'a>(&'a self) -> ParserResult<'a> {
			let mut results = ParserResult::new();
			let mut iter = self.raw_args.inner.iter();

			while let Some(raw_arg) = iter.next() {
				if let Some(command) =
					self.commands.iter().find(|c| &c.name == raw_arg)
				{
					match results
						.present_commands
						.insert(raw_arg.to_string(), command)
					{
						Some(_) => continue,
						None => continue,
					}
				}
			}

			results
		}

		/// ...
		pub fn setting(mut self, setting: ParserSettings) -> Self {
			self.settings = self.settings | setting;
			self
		}

		/// ...
		pub fn check_uniqueness<'a, T: PartialEq>(
			&'a self,
			items: &'a Vec<T>,
		) -> (bool, Option<&'a T>) {
			let mut iter = items.iter();

			while let Some(item) = iter.next() {
				let katch =
					items.iter().filter(|i| i == &item).collect::<Vec<&T>>();

				if katch.len() > 1 {
					return (true, Some(item));
				}
			}

			(false, None)
		}

		/// ...
		pub fn check_command_names<'a>(
			&'a self,
		) -> (
			bool,
			Option<&'a commands::Command>,
			Option<&'a commands::Command>,
		) {
			commands::check_cmds(&self.commands)
		}

		/// ...
		pub fn check_arg_names<'a>(
			&'a self,
		) -> (bool, Option<&'a args::Arg>, Option<&'a args::Arg>) {
			args::check_args(&self.args)
		}
	}
}

/// Everything command related
pub mod commands {
	use crate::args::Arg;

	/// ...
	#[derive(Debug, Clone, PartialEq, Eq)]
	pub struct Command {
		/// Name of the argument
		pub name: String,
		/// Help for the command
		pub help: Option<String>,
		/// Command Arguments
		pub args: Vec<Arg>,
	}

	impl Command {
		/// Creates a new command with the provided name.
		pub fn new(name: String) -> Command {
			Command {
				name,
				help: None,
				args: vec![],
			}
		}

		/// Sets the command's help. Appears when running help on the
		/// command / running help for the main command.
		pub fn help(mut self, help: String) -> Self {
			self.help = Some(help);

			self
		}

		/// ...
		pub fn arg(mut self, arg: Arg) -> Self {
			self.args.push(arg);
			self
		}

		/// ...
		pub fn args(mut self, args: Vec<Arg>) -> Self {
			self.args = args;
			self
		}
	}

	/// ...
	pub fn check_cmds<'a>(
		commands: &'a Vec<Command>,
	) -> (bool, Option<&'a Command>, Option<&'a Command>) {
		let mut commands = commands.iter();

		while let Some(command) = commands.next() {
			match commands.find(|cmd| cmd.name == command.name) {
				Some(cmd) => {
					return (true, Some(&command), Some(cmd));
				}
				None => continue,
			}
		}

		return (false, None, None);
	}
}

/// Everything argument related
pub mod args {

	use bitflags::bitflags;

	use crate::FancyArgs;

	bitflags! {
		/// Argument Settings
		pub struct ArgSettings: u32 {
			/// Whether or not more than one argument should be allowed
			const MULTIPLE = 1;
			/// Whether or not the argument should be required
			const REQUIRED = 1 << 2;
		}
	}

	/// The struct for Arg's. Creates a new Argument for commands and the main
	/// app without a long/short name it will be considered a positional
	/// argument. If you do set a long/short name people can use
	/// `--<arg>`/`-<arg>`.
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Arg {
		/// Name of the argument
		pub name: String,
		/// Long name of the argument
		pub long: Option<String>,
		/// Short name of the argument
		pub short: Option<String>,
		/// Help for the argument. Appears when users run help.
		pub help: Option<String>,
		/// Argument Settings
		pub settings: ArgSettings,
	}

	impl Arg {
		/// Creates a new argument with the provided name.
		pub fn new(name: String) -> Arg {
			Arg {
				name,
				long: None,
				short: None,
				help: None,
				settings: ArgSettings::empty(),
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

		/// Sets the argument's help. Appears when running help on the
		/// argument's command / running help for the main command.
		pub fn help(mut self, help: String) -> Self {
			self.help = Some(help);

			self
		}

		/// Set a setting
		pub fn setting(mut self, setting: ArgSettings) -> Self {
			self.settings = self.settings | setting;
			self
		}
	}

	/// Checks the provided vec of args for duplicates. If any of the
	/// arguments's names and long/short names match the function will return
	/// true, the current argument, the argument the current argument matched
	/// on.
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
				Some(ark) => {
					if arg.settings.contains(ArgSettings::MULTIPLE) {
						continue;
					}

					return (true, Some(&arg), Some(ark));
				}
				None => continue,
			}
		}

		return (false, None, None);
	}

	/// ...
	pub fn check_req_args<'a>(
		args: &'a Vec<Arg>,
		raw_args: &FancyArgs,
	) -> (bool, Option<&'a Arg>) {
		let raw_args = raw_args
			.inner
			.iter()
			.map(|ra| {
				ra.split("--")
					.collect::<String>()
					.split('-')
					.collect::<String>()
			})
			.collect::<Vec<String>>();
		let args = args
			.iter()
			.filter(|e| e.settings.contains(ArgSettings::REQUIRED))
			.collect::<Vec<&Arg>>();

		for arg in args {
			if raw_args.contains(&arg.name)
				|| arg.short.is_some()
					&& raw_args.contains(&arg.short.as_ref().unwrap())
				|| arg.long.is_some()
					&& raw_args.contains(&arg.long.as_ref().unwrap())
			{
				continue;
			}

			return (true, Some(arg));
		}

		(false, None)
	}
}
