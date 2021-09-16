#![deny(missing_docs)]
#![allow(soft_unstable)]
#![allow(clippy::iter_next_loop)]
//! Pieces is a command line argument parser with user control in mind.

/// FancyArgs is just a better way of using [env::args](std::env::args).
#[derive(Debug, PartialEq, Clone)]
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
	use std::hash::Hash;
	use std::ops::Range;

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
	pub struct ParserCmdResult<'a> {
		/// ...
		pub command: Option<&'a commands::Command>,
		/// ...
		pub command_and_args: Option<Range<usize>>,
	}

	/// ...
	#[derive(Debug, PartialEq)]
	pub struct ParserCmdArgResult {
		/// ...
		pub name: String,
		/// ...
		/// May be none if the arg doesnt take value (mainly flags)
		pub value: Option<String>,
	}

	/// ...
	#[derive(Debug, PartialEq)]
	pub struct ParserResult<'a> {
		/// ...
		pub commands: HashMap<String, ParserCmdResult<'a>>,
		/// ...
		pub present_args: HashMap<String, &'a args::Arg>,
	}

	impl<'a> ParserResult<'a> {
		/// ...
		pub fn new() -> ParserResult<'a> {
			ParserResult {
				commands: HashMap::new(),
				present_args: HashMap::new(),
			}
		}

		/// ...
		// pub fn value_of<'b>(
		// 	&'b self,
		// 	key: String,
		// ) -> Result<&'b commands::Command, std::io::Error> {
		// 	match self.present_commands.get(&key) {
		// 		Some(command) => Ok(command),
		// 		None => Err(std::io::Error::new(
		// 			std::io::ErrorKind::NotFound,
		// 			"Command not present.",
		// 		)),
		// 	}
		// }

		/// ...
		pub fn is_present(self, key: String) -> bool {
			self.commands.get(&key).is_some()
		}
	}

	impl<'a> Default for ParserResult<'a> {
		fn default() -> ParserResult<'a> {
			ParserResult {
				commands: HashMap::new(),
				present_args: HashMap::new(),
			}
		}
	}

	/// The parser for parsing commands, flags, and arguments.
	#[derive(Debug, PartialEq)]

	pub struct Parser {
		/// ...
		pub raw_args: FancyArgs,

		/// Args from the main app
		pub args: HashMap<String, args::Arg>,

		/// Commands
		pub commands: HashMap<String, commands::Command>, /* Vec<commands::
		                                                   * Command>, */

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
					Some(a) => {
						let mut args = HashMap::new();

						for arg in a {
							args.insert(arg.name.to_string(), arg);
						}

						args
					}
					None => HashMap::new(),
				},
				commands: match commands {
					Some(a) => {
						let mut cmds = HashMap::new();

						for cmd in a {
							cmds.insert(cmd.name.to_string(), cmd);
						}

						cmds
					}
					None => HashMap::new(),
				},
				settings: ParserSettings::empty(),
			}
		}

		/// Parses the commands added to the parser in [to_parse]
		/// This ONLY parses out top-level commands, not args or sub-commands.
		pub fn parse_commands<'b, T>(
			&'b self,
			to_parse: &mut T,
		) -> ParserResult<'b>
		where
			T: Iterator<Item = &'b String>, {
			let mut results = ParserResult::new();

			while let Some(cmd) = to_parse.next() {
				let command = match self.commands.get(cmd) {
					Some(cmd) => cmd,
					None => continue,
				};

				let start_posistion =
					match self.raw_args.inner.iter().position(|e| e == cmd) {
						Some(pos) => pos,
						None => continue,
					};

				let mut end_position = start_posistion + 1;

				for next_cmd in to_parse.next() {
					let _command = self.commands.get(cmd);

					if _command.is_some() {
						let pos = match self
							.raw_args
							.inner
							.iter()
							.position(|e| e == next_cmd)
						{
							Some(pos) => pos,
							None => continue,
						};

						end_position = pos;
						break;
					}
				}

				let currnet_command = ParserCmdResult {
					command: Some(command),
					command_and_args: Some(start_posistion..end_position),
				};
				println!("{:?}", currnet_command);
				results
					.commands
					.insert(command.name.to_string(), currnet_command);
			}

			results
		}

		/// ...
		pub fn setting(mut self, setting: ParserSettings) -> Self {
			self.settings |= setting;
			self
		}

		/// ...
		pub fn check_flag<'b>(
			&self,
			string: &'b str,
		) -> (bool, Option<&'b str>) {
			match (string.starts_with('-'), string.starts_with("--")) {
				(true, true) => (true, string.strip_prefix("--")),
				(true, false) => (true, string.strip_prefix('-')),
				(false, true) => (true, string.strip_prefix("--")),
				(false, false) => (false, None),
			}
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
			/// Whether or not the argument takes a value
			const TAKES_VALUE = 1 << 3;
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
			self.settings |= setting;
			self
		}
	}

	/// ...
	pub fn check_req_args<'a>(
		args: &'a [Arg],
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
					&& raw_args.contains(arg.short.as_ref().unwrap())
				|| arg.long.is_some()
					&& raw_args.contains(arg.long.as_ref().unwrap())
			{
				continue;
			}

			return (true, Some(arg));
		}

		(false, None)
	}
}
