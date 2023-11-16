pub mod cli;
pub mod io;
pub mod utils;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::{Cli, Commands};
use console::style;
use dialoguer::Input;
use dotenv::dotenv;
use io::{LocalRead, LocalWrite};
use std::env;
// use endpoints::*;
// use io::LocalWrite;
use oai::models::assistant::assistant::Tool;
use oai::models::assistant::CustomGPT;
use utils::get_dialoguer_theme;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => {}
        Err(err) => {
            println!("\n{}", err,);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not set in .env file");

    let cli = Cli::parse();
    let theme = get_dialoguer_theme();

    // IO Read
    let config_path = io::get_maker_path();
    let mut gpt = match CustomGPT::read(&config_path) {
        Some(gpt) => gpt,
        None => CustomGPT::new(api_key),
    }?;

    // Endpoint routing
    match cli.command {
        Commands::InitAssistant { name } => {
            let (api_name, instruction): (String, String) = match name {
                Some(name) => (
                    name.clone(),
                    format!(
                        "You are a GPT specialised in calling the {} API",
                        name
                    ),
                ),
                None => {
                    let name: String = Input::with_theme(&theme)
                        .with_prompt("API Name:")
                        .interact()
                        .unwrap();

                    let instruction = Input::with_theme(&theme)
                        .with_prompt("GPT Instruction")
                        .default(format!(
                            "You are a GPT specialised in calling the {} API",
                            name
                        ))
                        .interact()
                        .unwrap();

                    (name, instruction)
                }
            };

            // Logic
            let assistant_id = gpt
                .create_assistant(
                    &api_name,
                    &instruction,
                    vec![Tool::CodeInterpreter],
                )
                .await?;

            println!(
                "{}",
                style(format!(
                    "An assistant has been created with the following ID: {}",
                    assistant_id
                ))
                .green()
                .bold()
                .on_bright()
            );
        }
        Commands::AddApi { root_dir: _ } => {}
    }

    // IO Write
    gpt.write(&config_path.as_path())?;

    Ok(())
}
