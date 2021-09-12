#[test]
fn basic_arg() {
	use pieces::args::{Arg, ArgSettings};

	let correct_out = Arg {
		name: String::from("Foo"),
		help: Some(String::from("Bar")),
		long: None,
		short: None,
		settings: ArgSettings::empty(),
	};
	assert_eq!(
		Arg::new(String::from("Foo")).help(String::from("Bar")),
		correct_out
	);
}

#[test]
fn long_arg() {
	use pieces::args::{Arg, ArgSettings};

	let correct_out = Arg {
		name: String::from("Foo"),
		help: Some(String::from("Bar")),
		long: Some(String::from("Snack")),
		short: None,
		settings: ArgSettings::empty(),
	};
	assert_eq!(
		Arg::new(String::from("Foo"))
			.help(String::from("Bar"))
			.long(String::from("Snack")),
		correct_out
	);
}

#[test]
fn short_arg() {
	use pieces::args::{Arg, ArgSettings};

	let correct_out = Arg {
		name: String::from("Foo"),
		help: Some(String::from("Bar")),
		long: Some(String::from("Snack")),
		short: Some(String::from("Bar")),
		settings: ArgSettings::empty(),
	};
	assert_eq!(
		Arg::new(String::from("Foo"))
			.help(String::from("Bar"))
			.long(String::from("Snack"))
			.short(String::from("Bar")),
		correct_out
	);
}
