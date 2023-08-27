use anyhow::{anyhow, Result};
use gluon::ai::openai::{
    client::OpenAI,
    msg::{GptRole, OpenAIMsg},
    params::OpenAIParams,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    models::{
        job::{Job, Task},
        job_worker::JobFutures,
        messages::inner::{RequestType, WorkerResponse},
        state::AppState,
    },
    utils::write_json,
};

pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    init_prompt: String,
) {
    let mut app_data = app_state.write().await;

    // TODO: Return error if `specs` field already exists..
    app_data.specs = Some(init_prompt);
    println!("[INFO] Registered Project Specifications.");
    let job_name = "Scaffolding";

    app_data
        .jobs
        .new_in_progress(job_name, RequestType::ScaffoldProject);

    println!("[INFO] Added task `{}` as TODO", job_name);

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_scaffold_project(c, j, state)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to the exeuction queue: `{}`", job_name);
}

pub async fn run_scaffold_project(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<WorkerResponse> {
    println!("[INFO] Running `Scaffolding` Job...");

    let scaffold = scaffold_project(client, params, app_state.clone()).await?;

    let mut app_data = app_state.write().await;
    app_data.jobs.finish_job_by_order();

    println!("[INFO] Completed `Scaffolding` Job");

    Ok(WorkerResponse::Scaffold { scaffold })
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
