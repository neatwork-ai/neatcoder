use std::ops::{Deref, DerefMut};
use syn::File;

use super::AsFormat;
use crate::err::GluonError;

pub trait AsRust: AsFormat {
    fn as_rust(&self) -> Result<Rust, GluonError>;
    fn strip_rust(&self) -> Result<Rust, GluonError>;
    fn strip_rusts(&self) -> Result<Vec<Rust>, GluonError>;
}

impl<'a> AsRust for &'a str {
    fn as_rust(&self) -> Result<Rust, GluonError> {
        let deserializer = |s: &str| deserialize_rust(s);

        self.as_format(deserializer)
    }

    fn strip_rust(&self) -> Result<Rust, GluonError> {
        let deserializer = |s: &str| deserialize_rust(s);

        self.strip_format(deserializer, "rust")
    }

    fn strip_rusts(&self) -> Result<Vec<Rust>, GluonError> {
        let deserializer = |s: &str| deserialize_rust(s);

        self.strip_formats(deserializer, "rust")
    }
}

#[derive(Debug)]
pub struct Rust(File);

impl AsRef<File> for Rust {
    fn as_ref(&self) -> &File {
        &self.0
    }
}

impl Deref for Rust {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rust {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deserialize_rust(rust_str: &str) -> Result<Rust, GluonError> {
    let syntax_tree = syn::parse_file(rust_str)?;

    Ok(Rust(syntax_tree))
}

#[test]
fn test_parse() -> Result<(), GluonError> {
    let code_str = "
use structopt::StructOpt;

// Define the CLI structure
#[derive(Debug, StructOpt)]
#[structopt(name = \"example\", about = \"An example CLI application in Rust\")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    #[structopt(short = \"s\", long = \"speed\", default_value = \"42\")]
    speed: f64,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = \"v\", long = \"verbose\", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let opt = Opt::from_args();
    println!(\"{{:#?}}\", opt);
}";

    let rust_string = code_str.to_string();

    let obj_str = rust_string.as_str();

    let prompt = format!(
        "Sure! Here is an example of a cli app:\n```rust\n{}\n```",
        obj_str
    );

    let actual = prompt.as_str().strip_rust()?;

    println!("{:?}", actual.items);

    // assert_eq!(actual, expected);

    Ok(())
}
