use bitflags::bitflags;

#[derive(Debug, PartialEq, Eq)]
pub struct Output {
  pub arg: String,
  pub value: Option<String>,
}

bitflags! {
    pub struct ArgSettings: u32 {
        const MULTIPLE = 1;
        const HAS_VALUE = 1 << 2;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arg<'a> {
  pub name: &'a str,
  pub short: Option<&'a str>,
  pub long: Option<&'a str>,
  pub settings: ArgSettings,
}

impl<'a> Arg<'a> {
  pub fn new(name: &'a str) -> Arg<'a> {
    Arg { name,
          short: None,
          long: None,
          settings: ArgSettings::empty() }
  }

  pub fn short(mut self, short: &'a str) -> Self {
    self.short = Some(short);
    self
  }

  pub fn long(mut self, long: &'a str) -> Self {
    self.long = Some(long);
    self
  }

  pub fn set(mut self, setting: ArgSettings) -> Self {
    self.settings.set(setting, true);
    self
  }
}
