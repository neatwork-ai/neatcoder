use anyhow::{anyhow, Result};
use gluon::ai::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use parser::parser::{
    json::AsJson,
    rust::{AsRust, Rust},
};
use serde_json::Value;
use std::sync::Arc;

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
