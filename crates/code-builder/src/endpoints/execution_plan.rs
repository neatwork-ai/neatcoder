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
    models::{
        fs::Files,
        interfaces::AsContext,
        job::Task,
        job_worker::JobFutures,
        messages::inner::{ManagerRequest, RequestType, WorkerResponse},
        state::AppState,
    },
    utils::write_json,
};

pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) {
    let mut app_data = app_state.write().await;
    let job_name = "Build Execution Plan";

    app_data
        .jobs
        .new_in_progress(job_name, RequestType::BuildExecutionPlan);

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_build_execution_plan(c, j, state)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to exeuction queue: `{}`", job_name);
}

pub async fn run_build_execution_plan(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<WorkerResponse> {
    println!("[INFO] Running `Planning Execution` Job...");

    let execution_plan = build_execution_plan(client, params, app_state.clone()).await?;

    let files = Files::from_schedule(&execution_plan)?;
    let mut app_data = app_state.write().await;

    // Add code writing jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();

        app_data.jobs.new_todo(
            "TODO: This is a placeholder",
            ManagerRequest::CodeGen { filename: file_ },
        );
    }

    app_data.jobs.finish_job_by_order()?;

    println!("[INFO] Completed `Planning Execution` Job...");

    Ok(WorkerResponse::BuildExecutionPlan {
        jobs: app_data.jobs.clone(),
    })
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
