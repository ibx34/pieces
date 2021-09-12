use bitflags::bitflags;

// bitflags! {
//     pub struct Parser: u32 {}
// }

#[derive(Debug)]
pub struct FancyArgs {
    pub inner: Vec<String>
}

impl FancyArgs {
    
    pub fn grab() -> FancyArgs {
        let mut args = std::env::args();
        args.next();

        FancyArgs {
            inner: args.into_iter().collect::<Vec<String>>()
        }
    }

    pub fn load(args: std::env::Args) -> FancyArgs {
        FancyArgs {
            inner: args.into_iter().collect::<Vec<String>>()
        }            
    }
}


#[derive(Debug)]
pub struct Parser {
    pub(crate) raw_args: FancyArgs,
    pub args: Vec<args::Arg>
}

impl Parser {
    // TODO after the app is created, add an argument here and to the struct
    // for getting commands and such.
    pub fn build(raw_args: FancyArgs, args: Option<Vec<args::Arg>>) -> Parser {
        Parser { raw_args, 
            args: match args {
                Some(a) => a,
                None => vec![]
            }
        }
    }
}

pub mod args {

    #[derive(Debug,Clone)]
    pub struct Arg {
        pub name: String,
        pub long: Option<String>,
        pub short: Option<String>,
        pub help: Option<String>,
        pub required: bool,
    }    

    impl Arg {
        pub fn new(name: String) -> Arg {
            Arg { name, long: None, short: None, help: None, required: false }
        }

        pub fn long(mut self, long: String) -> Self {
            self.long = Some(long);
            self
        }

        pub fn short(mut self, short: String) -> Self {
            self.short = Some(short);
            self
        }

        pub fn help(mut self, help: String) -> Self {
            self.help = Some(help);
            self
        }

        pub fn required(mut self, required: bool) -> Self {
            self.required = required;
            self
        }
    }

    pub fn check_args<'a>(args: &'a Vec<Arg>) -> (bool,Option<&'a Arg>,Option<&'a Arg>) {
        let mut args = args.iter();

        while let Some(arg) = args.next() {
            match args.find(|a| {
                a.name == arg.name
                || a.short.is_some() && arg.short.is_some() && a.short.as_ref().unwrap() == arg.short.as_ref().unwrap()
                || a.long.is_some() && arg.long.is_some() && a.long.as_ref().unwrap() == arg.long.as_ref().unwrap()
            }) {
                Some(ark) => {
                    return (true,Some(&arg),Some(ark))
                }
                None => continue,
            }
        }

        return (false,None,None);
    }

}