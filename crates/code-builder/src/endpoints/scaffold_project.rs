use crate::prelude::*;
use crate::{models::state::AppState, utils::write_json};
use anyhow::{anyhow, Result};
use gluon::ai::openai::msg::{GptRole, OpenAIMsg};
use serde_json::Value;

pub async fn handle(
    conf: &Conf,
    dot_neat: &mut AppState,
    init_prompt: String,
) -> Result<Value> {
    if dot_neat.specs.is_some() || dot_neat.scaffold.is_some() {
        // TBD: what if the user wishes to re-scaffold the project?
        // They can set the .neat to empty and re-run the command
        return Err(anyhow!("Project already scaffolded"));
    }

    info!("Scaffolding project with specs: {init_prompt}");

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust based on the following project description:\n{init_prompt}\n
The API should retrieve the relevant data from a MySQL database.

Based on the information provided write the project's folder structure, starting from `src`.

Answer in JSON format (Do not forget to start with ```json). For each file provide a brief description included in the json");

    // we store the spec in case the OpenAI request fails - we will be able to
    // offer retry
    dot_neat.specs = Some(init_prompt);

    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (_, scaffold_json) = write_json(conf, &prompts).await?;

    dot_neat.scaffold = Some(scaffold_json.clone());

    Ok(scaffold_json)
}
