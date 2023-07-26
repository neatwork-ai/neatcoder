pub mod cli;
pub mod utils;

use dotenv::dotenv;
use gluon::ai::openai::{client::OpenAI, msg::GptRole};
use std::{
    env,
    io::{self, Write},
    rc::Rc,
    str::FromStr,
};

pub use crate::cli::{Cli, Commands};
use crate::utils::Options;

use anyhow::{anyhow, Result};
use async_openai::{types::CreateCompletionRequestArgs, Client};
use clap::Parser;
use dialoguer::Input;
use llm_store::chain::CausalChain;

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

            // TODO: Load this from DB
            let mut mgs = Messages::default();

            let mut chain = init_chain(&client, &mut mgs).await?;

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

                // match {
                //     Options::Talk => {
                //         let (seq, chain) = chat(client, chain, seq).await?;
                //     },
                //     Options::Retry => {},
                //     Options::Back => {},
                //     Options::Quit => {},
                // }

                // Input prompt
                todo!();
            }
        }
    }

    Ok(())
}

pub async fn init_chain(client: &OpenAI, msgs: &mut Messages) -> Result<CausalChain> {
    let user_msg = prompt_user();
    let llm_msg = prompt_llm(client, &[&user_msg]).await?;

    let user_msg: Rc<Msg> = Rc::new(user_msg.into());
    let llm_msg: Rc<Msg> = Rc::new(llm_msg.into());

    let mut chain = CausalChain::genesis(user_msg.clone());
    let llm_msg_id = chain.add_node(llm_msg.clone(), Some(chain.genesis_id))?;

    msgs.insert(chain.genesis_id, user_msg);
    msgs.insert(llm_msg_id, llm_msg);

    Ok(chain)
}

pub async fn chat(
    client: &OpenAI,
    msgs: &mut Messages,
    chain: &mut CausalChain,
    mut seq: Vec<&Message>,
) -> Result<()> {
    let user_msg = prompt_user();

    seq.push(&user_msg);
    let slice: &[&Message] = &seq;

    let llm_msg = prompt_llm(client, &seq).await?;

    let user_msg: Rc<Msg> = Rc::new(user_msg.into());
    let llm_msg: Rc<Msg> = Rc::new(llm_msg.into());

    let llm_msg_id = chain.add_node(llm_msg.clone(), Some(chain.genesis_id))?;

    msgs.insert(chain.genesis_id, user_msg);
    msgs.insert(llm_msg_id, llm_msg);

    Ok(())
}

pub fn prompt_user() -> Message {
    let prompt: String = Input::new()
        .with_prompt("\n Write your prompt")
        .interact()
        .unwrap();

    Message {
        role: GptRole::User,
        content: prompt,
    }
}

pub async fn prompt_llm(client: &OpenAI, seq: &[&Message]) -> Result<Message> {
    let client_resp = client.chat(&seq, &[], &[]).await?;
    let llm_resp = client_resp
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_str();

    println!("\n{}", llm_resp);

    Ok(Message {
        role: GptRole::Assistant,
        content: String::from(llm_resp),
    })
}
