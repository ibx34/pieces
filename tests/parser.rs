#[test]
fn basic_parser() {
	use pieces::{parse::{Parser, ParserSettings},
	             FancyArgs};

	let correct_out = Parser {
		raw_args: FancyArgs::grab(),
		args: vec![],
		commands: vec![],
		settings: ParserSettings::empty(),
	};

	assert_eq!(Parser::build(FancyArgs::grab(), None, None), correct_out);
}

#[test]
fn parser_command_name_comparison() {
	use pieces::{commands::Command, parse::Parser, FancyArgs};

	let commands = vec![
		Command::new(String::from("set")),
		Command::new(String::from("set")),
	];

	let parser = Parser::build(FancyArgs::grab(), None, Some(commands));
	assert_eq!(
		parser.check_command_names(),
		(
			true,
			Some(&Command {
				name: String::from("set"),
				args: vec![],
				help: None
			}),
			Some(&Command {
				name: String::from("set"),
				args: vec![],
				help: None
			})
		)
	);
}

#[test]
fn parser_command_uniqueness() {
	use pieces::{commands::Command, parse::Parser, FancyArgs};

	let commands = vec![
		Command::new(String::from("set")),
		Command::new(String::from("set")),
	];

	let parser = Parser::build(FancyArgs::grab(), None, Some(commands));
	assert_eq!(
		parser.check_uniqueness(&parser.commands),
		(
			true,
			Some(&Command {
				name: String::from("set"),
				args: vec![],
				help: None
			})
		)
	);
}

#[test]
fn parser_command_uniqueness_1() {
	use pieces::{commands::Command, parse::Parser, FancyArgs};

	let commands = vec![
		Command::new(String::from("set")),
		Command::new(String::from("set_home")),
	];

	let parser = Parser::build(FancyArgs::grab(), None, Some(commands));
	assert_eq!(parser.check_uniqueness(&parser.commands), (false, None));
}

#[test]
fn parser_arg_uniqueness() {
	use pieces::{args::{Arg, ArgSettings},
	             parse::Parser,
	             FancyArgs};

	let args = vec![
		Arg::new(String::from("Foo"))
			.long(String::from("Bar"))
			.short(String::from("F")),
		Arg::new(String::from("Foo"))
			.long(String::from("Bar"))
			.short(String::from("F")),
	];

	let parser = Parser::build(FancyArgs::grab(), Some(args), None);
	assert_eq!(
		parser.check_uniqueness(&parser.args),
		(
			true,
			Some(&Arg {
				name: String::from("Foo"),
				short: Some(String::from("F")),
				long: Some(String::from("Bar")),
				settings: ArgSettings::empty(),
				help: None
			})
		)
	);
}

#[test]
fn parser_arg_uniqueness_1() {
	use pieces::{args::Arg, parse::Parser, FancyArgs};

	let args = vec![
		Arg::new(String::from("Foo"))
			.long(String::from("Bar"))
			.short(String::from("F")),
		Arg::new(String::from("Bar"))
			.long(String::from("Foo"))
			.short(String::from("B")),
	];

	let parser = Parser::build(FancyArgs::grab(), Some(args), None);
	assert_eq!(parser.check_uniqueness(&parser.args), (false, None));
}
