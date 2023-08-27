use anyhow::{anyhow, Result};
use gluon::ai::openai::{
    client::OpenAI,
    msg::{GptRole, OpenAIMsg},
    params::OpenAIParams,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    models::{interfaces::AsContext, state::AppState},
    utils::{write_json, CodeStream},
};

pub async fn generate_api_specs(
    client: &OpenAI,
    job: &OpenAIParams,
    data_model: &Vec<String>,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for model in data_model.iter() {
        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: model.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: String::from(
            "Based on the data model described above, create an idea for an API service.",
        ),
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let answer = client.chat(job, &prompts, &[], &[]).await?;

    Ok(String::from(answer))
}

pub async fn generate_db_schema(client: &OpenAI, job: &OpenAIParams) -> Result<(String, String)> {
    let sys_msg = OpenAIMsg {
        role: GptRole::System,
        content: String::from("You are a entrepreneur with product management background."),
    };

    let user_msg = OpenAIMsg {
        role: GptRole::User,
        content: String::from("Generate a random idea for a company. The first word in your response should be the company name."),
    };

    let answer = client.chat(job, &[&sys_msg, &user_msg], &[], &[]).await?;

    let company_name = get_first_word(&answer);

    let sys_msg_2 = OpenAIMsg {
        role: GptRole::System,
        content: String::from("You are a data engineer hired to work on this project."),
    };

    let user_msg_2 = OpenAIMsg {
        role: GptRole::User,
        content: String::from(
            "Based on the above project description, produce a database schema in form of SQL DDL.",
        ),
    };

    let answer = client
        .chat(
            job,
            &[&sys_msg, &user_msg, &sys_msg_2, &user_msg_2],
            &[],
            &[],
        )
        .await?;

    Ok((String::from(company_name), String::from(answer)))
}

fn get_first_word(input: &str) -> &str {
    let mut words = input.split_whitespace();
    words.next().unwrap_or("")
}
