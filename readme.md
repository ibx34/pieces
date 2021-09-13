# Pieces
A command line argumnt parser. (owo?)

## Quick Start (Not much now)
```rust
use pieces::{parse::Parser,commands::Command,FancyArgs};
let parser = Parser::build(FancyArgs::grab(), None, Some(vec![
    Command::new(String::from("owo"))   
]));

let present = parser.parse();
if present.is_present(String::from("owo")) {
    println!("owo");
} else {
    println!("Please provide the owo command :)")
}
```