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
    println!("THE STR IS: {}", rust_str);

    let syntax_tree = syn::parse_file(rust_str)?;

    Ok(Rust {
        raw: rust_str.to_string(),
        ast: syntax_tree,
    })
}

#[test]
fn test_parse_2() -> Result<(), GluonError> {
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
