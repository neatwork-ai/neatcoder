use std::str::FromStr;

pub enum Options {
    // T - Talk
    Talk,
    // R - Talk
    Retry,
    // B - Go Back
    Back,
    // Q - Quit
    Quit,
}

impl FromStr for Options {
    type Err = ();

    fn from_str(input: &str) -> Result<Options, Self::Err> {
        match input {
            "\n" => Ok(Options::Talk),
            "t" => Ok(Options::Talk),
            "r" => Ok(Options::Retry),
            "b" => Ok(Options::Back),
            "q" => Ok(Options::Quit),
            _ => Err(()),
        }
    }
}
