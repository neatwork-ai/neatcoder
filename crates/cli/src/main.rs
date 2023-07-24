pub mod cli;

use dotenv::dotenv;
use std::env;

pub use crate::cli::{Cli, Commands};

use anyhow::Result;
use async_openai::{types::CreateCompletionRequestArgs, Client};
use clap::Parser;
use dialoguer::Input;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => {
            println!("Process ran successfully.");
        }
        Err(err) => {
            println!("\n{}", err);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::WritePrompt { output_dir: _ } => {
            dotenv().ok();

            let api_key = env::var("OPENAI_API_KEY")?;
            let client = Client::new().with_api_key(api_key);

            // Input prompt
            let prompt: String = Input::new()
                .with_prompt("Write your prompt")
                .interact()
                .unwrap();

            // Build request
            let request = CreateCompletionRequestArgs::default()
                .model("text-davinci-003")
                .prompt(prompt)
                .max_tokens(40_u16)
                .build()
                .unwrap();

            // Call API
            let response = client
                .completions() // Get the API "group" (completions, images, etc.) from the client
                .create(request) // Make the API call in that "group"
                .await
                .unwrap();

            println!("{}", response.choices.first().unwrap().text);
        }
    }

    Ok(())
}
