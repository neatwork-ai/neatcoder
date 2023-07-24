pub mod cli;
pub mod utils;

use dotenv::dotenv;
use gluon::ai::openai::{
    client::{OpenAI, OpenAIModels},
    input::{GptRole, Message},
};
use std::{
    env,
    io::{self, Write},
    str::FromStr,
};

pub use crate::cli::{Cli, Commands};
use crate::utils::Options;

use anyhow::{anyhow, Result};
use async_openai::{types::CreateCompletionRequestArgs, Client};
use clap::Parser;
use dialoguer::Input;
use llm_store::msg_node::CausalChain;

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
        Commands::WriteSequence {} => {
            dotenv().ok();

            let client = OpenAI::new(OpenAIModels::Gpt35Turbo)
                .api_key(env::var("OPENAI_API_KEY")?)
                .temperature(0.7)
                .top_p(0.9)?;

            let (mut seq, mut chain) = init_chain(&client).await?;

            loop {
                println!("\nOptions:");
                println!("[ENTER] - Talk");
                println!("R - Retry");
                println!("B - Go Back");
                println!("Q - Quit");

                io::stdout().flush().unwrap();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                let choice = choice.trim().to_ascii_lowercase();
                Options::from_str(&choice).map_err(|_err| anyhow!("Error parsing options"))?;
                println!("Choice: {:?}", choice);

                match {
                    Options::Talk => {
                        let (seq, chain) = chat(client, chain, seq).await?;
                    },
                    Options::Retry => {},
                    Options::Back => {},
                    Options::Quit => {},
                }
                
                // Input prompt
                let prompt: String = Input::new()
                    .with_prompt("\n Write your prompt")
                    .interact()
                    .unwrap();

                if &prompt == "quit" {
                    break;
                }

                let user_msg = Message {
                    role: GptRole::User,
                    content: prompt,
                };

                seq.push(user_msg);

                let client_resp = client.chat(&seq, &[], &[]).await?;
                let llm_resp = client_resp
                    .choices
                    .first()
                    .unwrap()
                    .message
                    .content
                    .as_str();

                println!("\n{}", llm_resp);

                let llm_msg = Message {
                    role: GptRole::Assistant,
                    content: String::from(llm_resp),
                };

                seq.push(llm_msg);
            }
        }
    }

    Ok(())
}

pub async fn init_chain(client: &OpenAI) -> Result<(Vec<Message>, CausalChain)> {
    // let mut chain = chain.unwrap_or_else(|| CausalChain::genesis(GptRole::User, vec![], prompt));
    chat(client, None, vec![]).await
}

pub async fn chat(
    client: &OpenAI,
    chain: &mut CausalChain,
    seq: &mut Vec<Message>,
) -> Result<()> {
    let prompt: String = Input::new()
        .with_prompt("\n Write your prompt")
        .interact()
        .unwrap();

    let user_msg = Message {
        role: GptRole::User,
        content: prompt.clone(),
    };

    chain.add_node(
        user_msg.role.clone(),
        // TODO: avoid cloning
        seq.clone(),
        None,
        user_msg.content.clone(),
    )?;

    seq.push(user_msg);

    let client_resp = client.chat(&seq, &[], &[]).await?;
    let llm_resp = client_resp
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_str();

    println!("\n{}", llm_resp);

    let llm_msg = Message {
        role: GptRole::Assistant,
        content: String::from(llm_resp),
    };

    chain.add_node(
        llm_msg.role.clone(),
        vec![chain.genesis_id],
        Some(chain.genesis_id),
        llm_msg.content.clone(),
    )?;

    seq.push(llm_msg);

    Ok((seq, chain))
}
