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