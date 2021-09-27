# Pieces

An argument parser built with control in mind.

## Parsing

The results you get are dependent on what order you parse in. If you want to say 
only parse positional arguemnts then only call `.parse_args()`. If you want to
parse flag arguments before posistional (for some odd reason) then call `.parse_flags()`
before calling `.parse_args()`. **The example belows parsing of positional arguments before flag arguments.**

## Example
```rs
use piecesv2;

let parser = &mut piecesv2::parser::Parser::build2(
    std::env::args(),
    vec![], // Commands, not currently implemented...
    vec![
        piecesv2::args::Arg::new("name"), // First positional argument
        piecesv2::args::Arg::new("age"), // Second positional argument

        piecesv2::args::Arg::new("email") // First flag argument
            .short("e")
            .long("email")
            .set(piecesv2::args::ArgSettings::HAS_VALUE), // Takes value

        piecesv2::args::Arg::new("phone-number") // Second flag argument
            .short("p")
            .long("phone-number")
            .set(piecesv2::args::ArgSettings::HAS_VALUE)   
            .set(piecesv2::args::ArgSettings::MULTIPLE), // Allows multiple of this flag
    ] // Args and Flags.
);

// Parses only positional arguments:
let results = parser.parse_args();

if !results.contains_key(&String::from("age")) {
    panic!("Age is a required argument that wasn't provided.");
}

// Parsers only flags:
let flags = parser.parse_flags();

if !flags.contains_key(&String::from("email")) {
    panic!("Email is a required flag that wasn't provided.");
}
```