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
        self.as_format(deserialize_rust)
    }

    fn strip_rust(&self) -> Result<Rust, GluonError> {
        self.strip_format(deserialize_rust, "rust")
    }

    fn strip_rusts(&self) -> Result<Vec<Rust>, GluonError> {
        self.strip_formats(deserialize_rust, "rust")
    }
}

#[derive(Debug)]
pub struct Rust {
    pub raw: String,
    pub ast: File,
}

fn deserialize_rust(rust_str: &str) -> Result<Rust, GluonError> {
    let syntax_tree = syn::parse_file(rust_str)?;

    Ok(Rust {
        raw: rust_str.to_string(),
        ast: syntax_tree,
    })
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

    println!("{:?}", actual.ast.items);

    // assert_eq!(actual, expected);

    Ok(())
}
