use anyhow::{anyhow, Result};
use futures::StreamExt;
use gluon::ai::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use parser::parser::{
    json::AsJson,
    rust::{AsRust, Rust},
};
use serde_json::Value;
use std::sync::Arc;

pub async fn write_rust(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<(String, Rust)> {
    let mut retries = 3;

    loop {
        let answer = client.chat(params.clone(), prompts, &[], &[]).await?;

        match answer.as_str().strip_rust() {
            Ok(result) => {
                break Ok((answer, result));
            }
            Err(e) => {
                println!("Error while parsing rust code: \n{}", e);
                retries -= 1;

                if retries <= 0 {
                    return Err(anyhow!("Failed to parse rust code."));
                }

                println!("Retrying...");
            }
        }
    }
}

pub async fn write_json(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<(String, Value)> {
    let mut retries = 3;

    loop {
        // write_dammit(client.clone(), params.clone(), &prompts).await?;

        println!("[INFO] Prompting the LLM...");
        let answer = client.chat(params.clone(), prompts, &[], &[]).await?;

        match answer.as_str().strip_json() {
            Ok(result) => {
                println!("[INFO] Received LLM answer...");
                break Ok((answer, result));
            }
            Err(e) => {
                println!("Failed to parse json: \n{}", e);
                retries -= 1;

                if retries <= 0 {
                    return Err(anyhow!("Failed to parse json."));
                }

                println!("Retrying...");
            }
        }
    }
}

async fn write_dammit(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<()> {
    println!("[INFO] Initiating Stream");

    let mut chat_stream =
        client.chat_stream(&params, &prompts, &[], &[]).await?;

    let mut start_delimiter = false;
    while let Some(item) = chat_stream.next().await {
        match item {
            Ok(bytes) => {
                let token = std::str::from_utf8(&bytes)
                    .expect("Failed to generate utf8 from bytes");
                if !start_delimiter && ["```json", "```"].contains(&token) {
                    start_delimiter = true;
                    continue;
                } else if !start_delimiter {
                    println!("{:?}", token);
                    continue;
                } else {
                    if token == "```" {
                        break;
                    }
                }
            }
            Err(e) => eprintln!("Failed to receive token, with error: {e}"),
        }
    }
    Ok(())
}
