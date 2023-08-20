use anyhow::{anyhow, Result};
use gluon::ai::openai::{
    client::OpenAI,
    job::OpenAIJob,
    msg::{GptRole, OpenAIMsg},
};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    models::{job::JobType, state::AppState},
    utils::{write_json, write_rust},
};

pub async fn gen_project_scaffold(
    client: Arc<OpenAI>,
    job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<Arc<(JobType, String)>> {
    let state = app_state.write().await;

    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    let specs = state.specs.as_ref().unwrap();

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

    let (_, scaffold_json) = write_json(client, job, &prompts).await?;

    let fs = Arc::new((JobType::Scaffold, scaffold_json.to_string()));

    Ok(fs)
}

pub async fn gen_work_schedule(
    client: Arc<OpenAI>,
    job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<Arc<(JobType, String)>> {
    let state = app_state.read().await;

    let mut prompts = Vec::new();

    if state.data_model.is_none() {
        // TODO: Consider relaxing this and instead gracefully handle the task without the data model
        return Err(anyhow!("No data model available.."));
    }

    let data_model = state.data_model.as_ref().unwrap();
    let api_description = &state.specs.as_ref().unwrap();

    if state.fs.is_none() {
        return Err(anyhow!("No folder scaffold config available.."));
    }

    let project_scaffold = state.fs.as_ref().unwrap();

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

    let (answer, tasks) = write_json(client, job, &prompts).await?;

    println!("{}", answer);

    let dg = Arc::new((JobType::Ordering, tasks.to_string()));

    Ok(dg)
}

pub async fn gen_code(
    client: Arc<OpenAI>,
    job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<Arc<(JobType, String)>> {
    let state = app_state.read().await;
    let mut prompts = Vec::new();

    let data_model = state.data_model.as_ref().unwrap();
    let api_description = state.specs.as_ref().unwrap();

    if state.fs.is_none() {
        return Err(anyhow!("No folder scaffold config available.."));
    }

    let project_scaffold = state.fs.as_ref().unwrap();
    let mut files = state.files.lock().await;

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
        content: String::from(api_description),
    });

    for file in files.keys() {
        let code = files.get(file).unwrap();

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
    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let (answer, code_raw) = {
        let (answer, code) = write_rust(client, job, &prompts).await?;
        (answer.to_string(), code.raw.clone())
    };

    // Update state
    let mut raw = state.raw.lock().await;
    raw.insert(filename.to_string(), answer.to_string());
    files.insert(filename.to_string(), code_raw.clone());

    // TODO: Optimize
    Ok(Arc::new((JobType::CodeGen, code_raw)))
}
