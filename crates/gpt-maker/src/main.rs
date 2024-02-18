pub mod cli;
pub mod deref;
pub mod get_refs;
pub mod io;
pub mod preprocess;
pub mod process;
pub mod ref_map;
pub mod utils;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::{Cli, Commands};
use console::style;
use dialoguer::Input;
use dotenv::dotenv;
use io::{LocalRead, LocalWrite};
use openapiv3::OpenAPI;
use std::{collections::HashSet, env, fs, sync::Arc};
// use endpoints::*;
// use io::LocalWrite;
use oai::models::assistant::assistant::Tool;
use oai::models::assistant::CustomGPT;
use utils::{get_dialoguer_theme, multi_select};

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
        Commands::AddApi { api_specs } => {
            let openapi_path = config_path
                .parent()
                .expect("Could not find parent folder")
                .join("openapi")
                .join(format!("{}.json", api_specs));

            // Read the OpenAPI JSON file into a string
            let openapi_json_str = fs::read_to_string(openapi_path)?;

            // Parse the YAML string into an OpenAPI structure
            let mut openapi_spec: OpenAPI =
                serde_json::from_str(&openapi_json_str)?;

            let tag_strs: Vec<&str> = openapi_spec
                .tags
                .iter()
                .map(|tag| tag.name.as_str())
                .collect();

            println!("# of tags: {}", openapi_spec.tags.len());
            println!("# of Paths: {}", openapi_spec.paths.paths.len());

            let tags_selected = multi_select(
                &theme,
                "Choose which tags you want to include",
                &tag_strs,
            )?;

            let tags_selected_ref: Arc<HashSet<&str>> = Arc::new(
                tags_selected.iter().map(|tag| tag.as_str()).collect(),
            );

            // process_specs(&mut openapi_spec);

            // let open_ai_specs =
            //     split_specs(Arc::new(openapi_spec), tags_selected_ref.clone())?;
        }
    }

    // IO Write
    gpt.write(&config_path.as_path())?;

    Ok(())
}
