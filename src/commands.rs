use std::collections::HashMap;

use multimap::MultiMap;

use crate::args;

#[derive(Debug, PartialEq, Eq)]
pub struct Commands {
  pub command: String,
  pub present_args: HashMap<String, args::Output>,
  pub present_flags: MultiMap<String, args::Output>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command<'a> {
  pub name: &'a str,
  pub args: Vec<args::Arg<'a>>,
}

impl<'a> Command<'a> {
  pub fn new(name: &'a str, args: Vec<args::Arg<'a>>) -> Command<'a> { Command { name, args } }
}
