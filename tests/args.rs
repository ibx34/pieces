#[test]
fn basic_arg() {
    use pieces::args::{Arg,ArgSettings};

    let correct_out = Arg { 
        name: String::from("Foo"),
        help: Some(String::from("Bar")),
        long: None,
        short: None,
        settings: ArgSettings::empty()
    };
    assert_eq!(Arg::new(String::from("Foo")).help(String::from("Bar")), correct_out);
}