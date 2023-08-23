use anyhow::{anyhow, Result};
use futures::StreamExt;
use gluon::ai::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use parser::parser::{
    json::AsJson,
    rust::{AsRust, Rust},
};
use serde_json::Value;
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpListener};

pub async fn stream_rust(
    client: Arc<OpenAI>,
    job: Arc<OpenAIParams>,
    prompts: &[&OpenAIMsg],
    listener_address: &str,
) -> Result<()> {
    let listener = TcpListener::bind(listener_address).await?;
    let (mut tcp_stream, _) = listener.accept().await?;

    let mut chat_stream = client.chat_stream(&job, prompts, &[], &[]).await?;

    let mut start_delimiter = false;
    while let Some(item) = chat_stream.next().await {
        match item {
            Ok(bytes) => {
                let token =
                    std::str::from_utf8(&bytes).expect("Failed to generate utf8 from bytes");
                if !start_delimiter && ["```rust", "```"].contains(&token) {
                    start_delimiter = true;
                    continue;
                } else if !start_delimiter {
                    continue;
                } else {
                    if token == "```" {
                        break;
                    }
                    tcp_stream.writable().await?;
                    if let Err(e) = tcp_stream.write_all(bytes.as_ref()).await {
                        eprintln!("Failed to write bytes to tcp stream, with error: {e}");
                    }
                }
            }
            Err(e) => eprintln!("Failed to receive token, with error: {e}"),
        }
    }
    Ok(())
}

pub async fn write_rust(
    client: Arc<OpenAI>,
    job: Arc<OpenAIParams>,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<(String, Rust)> {
    let mut retries = 3;

    loop {
        let answer = client.chat(job.clone(), prompts, &[], &[]).await?;

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
    job: Arc<OpenAIParams>,
    prompts: &Vec<&OpenAIMsg>,
) -> Result<(String, Value)> {
    let mut retries = 3;

    loop {
        let answer = client.chat(job.clone(), prompts, &[], &[]).await?;

        match answer.as_str().strip_json() {
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
