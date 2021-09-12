#[test]
fn basic_command() {
	use pieces::commands::Command;

	let correct_out = Command {
		name: String::from("set"),
		help: None,
		args: vec![],
	};

	assert_eq!(Command::new(String::from("set")), correct_out);
}

#[test]
fn command_with_args() {
	use pieces::{args::{Arg, ArgSettings},
	             commands::Command};

	let correct_out = Command {
		name: String::from("set"),
		help: None,
		args: vec![Arg::new(String::from("home"))
			.long(String::from("home"))
			.short(String::from("h"))
			.help(String::from("Sets your home!"))
			.setting(ArgSettings::REQUIRED)],
	};

	assert_eq!(
		Command::new(String::from("set")).arg(
			Arg::new(String::from("home"))
				.long(String::from("home"))
				.short(String::from("h"))
				.help(String::from("Sets your home!"))
				.setting(ArgSettings::REQUIRED)
		),
		correct_out
	);
}
