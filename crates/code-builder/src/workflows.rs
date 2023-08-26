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

pub async fn scaffold_project(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<Value> {
    let mut state = app_state.write().await;

    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    let specs = state
        .specs
        .as_ref()
        .ok_or(anyhow!("AppState missing `specs` field"))?;

    if state.scaffold.is_some() {
        return Err(anyhow!("Scaffold already exists. Skipping..."));
    }

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust based on the following project description:\n{}\n
The API should retrieve the relevant data from a MySQL database.

Based on the information provided write the project's folder structure, starting from `src`.

Answer in JSON format (Do not forget to start with ```json). For each file provide a brief description included in the json", specs);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (_, scaffold_json) = write_json(client, params, &prompts).await?;

    state.scaffold = Some(scaffold_json.to_string());

    Ok(scaffold_json)
}

pub async fn build_execution_plan(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<Value> {
    let state = app_state.read().await;

    let mut prompts = Vec::new();

    if state.interfaces.is_empty() {
        println!("[INFO] No Interfaces detected. Proceeding...");
    }

    let api_description = &state.specs.as_ref().unwrap();

    if state.scaffold.is_none() {
        return Err(anyhow!("No folder scaffold config available.."));
    }

    let project_scaffold = state.scaffold.as_ref().unwrap();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for (_, interface) in state.interfaces.iter() {
        // Attaches context to the message sequence
        interface.add_context(&mut prompts)?;
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: api_description.to_string(),
    });

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust.
You are assigned to build the API based on the project folder structure. Your current task is to order the files in accordance to the order of work that best fits the file dependencies.
The project scaffold is the following:\n{}\n

Answer in JSON format. Define the order by adding the file names to an ordered list (START WITH THE DELIMITER '```json').
Use the following schema:

```json
{{'order': [...]}}
```
", project_scaffold);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (answer, tasks) = write_json(client, params, &prompts).await?;

    println!("[DEBUG] LLM: {}", answer);

    Ok(tasks)
}

pub async fn gen_code(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<CodeStream> {
    // TODO: add task to DONE..
    println!("[INFO] Running `CodeGen` Job: {}", filename);

    let state = app_state.read().await;
    let mut prompts = Vec::new();

    let api_description = state.specs.as_ref().unwrap();

    if state.scaffold.is_none() {
        return Err(anyhow!("No folder scaffold config available.."));
    }

    let project_scaffold = state.scaffold.as_ref().unwrap();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for (_, interface) in state.interfaces.iter() {
        // Attaches context to the message sequence
        interface.add_context(&mut prompts)?;
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: String::from(api_description),
    });

    for file in state.codebase.keys() {
        let code = state.codebase.get(file).unwrap();

        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: code.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        // Needs to be optimized
        content: project_scaffold.to_string(),
    });

    let main_prompt = format!(
        "
        You are a Rust engineer tasked with creating an API in Rust.
        You are assigned to build the API based on the project folder structure
        Your current task is to write the module `{}.rs
        ",
        filename
    );

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let stream = CodeStream::new(filename, client, params, prompts);

    Ok(stream)
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
