use pieces::{args::Arg, parser::Parser};

fn main() {
  let args = vec![Arg::new("your-name")];

  let parser = &mut Parser::build2(std::env::args(), vec![], args.clone());

  let given_args = parser.parse_args(Some(&args));
  if let Some(name_arg) = given_args.get_key_value(&String::from("your-name")) {
    if let Some(name) = &name_arg.1.value {
      println!("Hello, {}", name);
    };
  }
}
