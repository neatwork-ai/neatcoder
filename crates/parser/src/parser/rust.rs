use syn::File;

use super::AsFormat;
use crate::err::ParseError;

/// Trait providing methods for working with Rust code.
pub trait AsRust: AsFormat {
    /// Converts the object to a Rust syntax tree.
    fn as_rust(&self) -> Result<Rust, ParseError>;

    /// Strips the Rust formatting, expecting encapsulation as in OpenAI's format, and returns the Rust syntax tree.
    fn strip_rust(&self) -> Result<Rust, ParseError>;

    /// Strips multiple Rust code blocks, assuming the same encapsulation as `strip_rust`.
    fn strip_rusts(&self) -> Result<Vec<Rust>, ParseError>;
}

impl<'a> AsRust for &'a str {
    /// Implementation for converting a string slice to a Rust syntax tree.
    fn as_rust(&self) -> Result<Rust, ParseError> {
        self.as_format(deserialize_rust)
    }

    /// Implementation for stripping Rust code from a string slice, assuming encapsulation like OpenAI.
    fn strip_rust(&self) -> Result<Rust, ParseError> {
        self.strip_format(deserialize_rust, "rust")
    }

    /// Implementation for stripping multiple Rust code blocks from a string slice.
    fn strip_rusts(&self) -> Result<Vec<Rust>, ParseError> {
        self.strip_formats(deserialize_rust, "rust")
    }
}

/// Struct representing a Rust code block with both raw text and parsed AST.
#[derive(Debug)]
pub struct Rust {
    /// Raw text of the Rust code
    pub raw: String,
    /// Abstract syntax tree (AST) representation
    pub ast: File,
}

/// Function to deserialize a Rust code string into a `Rust` struct.
///
/// # Arguments
/// * `rust_str` - The Rust code string to be deserialized.
///
/// # Returns
/// * A `Result` containing a `Rust` struct if successful, or a `ParseError` if an error occurred.
fn deserialize_rust(rust_str: &str) -> Result<Rust, ParseError> {
    let syntax_tree = syn::parse_file(rust_str)?;

    Ok(Rust {
        raw: rust_str.to_string(),
        ast: syntax_tree,
    })
}

#[test]
fn test_parse_2() -> Result<(), ParseError> {
    let code_str = "use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
    
mod config;
mod controllers;
mod db;
mod errors;
mod filters;
mod handlers;
mod middlewares;
mod models;
mod pagination;
mod utils;
    
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Load the environment variables
    let database_url = env::var(\"DATABASE_URL\").expect(\"DATABASE_URL not set\");

    // Create a new database connection pool
    let pool = db::create_pool(&database_url);

    // Create an instance of the application state
    let app_state = web::Data::new(config::AppState {
        db_pool: pool.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middlewares::AuthMiddleware)
            .service(
                web::scope(\"/api\")
                    .configure(handlers::customer::init_routes)
                    .configure(handlers::company::init_routes)
                    .configure(handlers::product::init_routes)
                    .configure(handlers::purchase::init_routes),
            )
    })
    .bind(\"127.0.0.1:8080\")?
    .run()
    .await
}";

    let rust_string = code_str.to_string();

    let obj_str = rust_string.as_str();

    let prompt = format!(
        "Sure! Here's an example implementation of the `main.rs` module for your e-commerce management system API:\n```rust\n{}\n```",
        obj_str
    );

    let actual = prompt.as_str().strip_rust()?;

    println!("{:?}", actual.ast.items);

    // assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn test_parse() -> Result<(), ParseError> {
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
