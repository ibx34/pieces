use std::{collections::HashMap, fmt::Debug};

use multimap::MultiMap;

use crate::{
  args::{Arg, ArgSettings, Output},
  commands::{Command, Commands},
};

#[derive(Debug)]
pub struct Parser<'a, A>
  where A: Iterator<Item = String> {
  pub(crate) env_args: A,
  pub commands: Vec<Command<'a>>,
  pub args: Vec<Arg<'a>>,
}

impl<'a, A> Parser<'a, A> where A: Iterator<Item = String> {
  pub fn build(env_args: A, commands: Vec<Command<'a>>, args: Vec<Arg<'a>>) -> Parser<'a, A> { Parser { env_args, commands, args } }

  pub fn build2(mut env_args: A, commands: Vec<Command<'a>>, args: Vec<Arg<'a>>) -> Parser<'a, A> {
    env_args.next();

    Parser { env_args, commands, args }
  }

  #[must_use = "Why even call me if you're not going to use my output?"]
  pub fn parse_cmds(&mut self) -> HashMap<String, Commands> {
    let mut results: HashMap<String, Commands> = HashMap::new();

    let commands = self.commands.clone();
    let cmds = commands.iter();

    while let Some(raw_arg) = self.env_args.next() {
      if let Some(cmd) = cmds.clone().find(|c| c.name == &raw_arg) {
        let mut cmd_entry = Commands { command: raw_arg,
                                       present_args: HashMap::new(),
                                       present_flags: MultiMap::new() };

        let args = self.parse_args(Some(&cmd.args));
        cmd_entry.present_args = args;

        let flags = self.parse_flags(Some(&cmd.args));
        cmd_entry.present_flags = flags;

        results.insert(cmd.name.to_string(), cmd_entry);
      }
    }

    results
  }

  #[must_use = "Why even call me if you're not going to use my output?"]
  pub fn parse_args(&mut self, args: Option<&Vec<Arg<'a>>>) -> HashMap<String, Output> {
    let mut results: HashMap<String, Output> = HashMap::new();
    let mut args = args.unwrap_or(&self.args).iter();

    while let Some(arg) = args.next() {
      if arg.long.is_some() || arg.short.is_some() {
        continue;
      }

      while let Some(raw_arg) = self.env_args.next() {
        results.insert(arg.name.to_string(), Output { arg: arg.name.to_string(),
                                                      value: Some(raw_arg) });

        break;
      }
    }

    results
  }

  #[must_use = "Why even call me if you're not going to use my output?"]
  pub fn parse_flags(&mut self, args: Option<&Vec<Arg<'a>>>) -> MultiMap<String, Output> {
    let mut results: MultiMap<String, Output> = MultiMap::new();

    let aa = args.unwrap_or(&self.args);
    let args = aa.iter().filter(|arg| arg.long.is_some() || arg.short.is_some());

    while let Some(raw_arg) = self.env_args.next() {
      let is_flag = match (raw_arg.starts_with('-'), raw_arg.starts_with("--")) {
        (true, true) => (true, raw_arg.strip_prefix("--")),
        (true, false) => (true, raw_arg.strip_prefix('-')),
        (false, true) => (true, raw_arg.strip_prefix("--")),
        (false, false) => (false, None),
      };

      if !is_flag.0 || is_flag.1.is_none() {
        continue;
      }

      let stripped_flag = is_flag.1.unwrap();
      let matched_arg = args.clone()
                            .find(|arg| arg.short.is_some() && stripped_flag == arg.short.unwrap() || arg.long.is_some() && stripped_flag == arg.long.unwrap());

      if let Some(arg) = matched_arg {
        if !arg.settings.contains(ArgSettings::MULTIPLE) && results.contains_key(arg.name) {
          continue;
        }

        let value = if arg.settings.contains(ArgSettings::HAS_VALUE) { self.env_args.next() } else { None };

        results.insert(arg.name.to_string(), Output { arg: arg.name.to_string(), value });
      }
    }

    results
  }
}

impl<'a, T> AsRef<Parser<'a, T>> for Parser<'a, T> where T: Iterator<Item = String> {
  fn as_ref(&self) -> &Self { &self }
}
