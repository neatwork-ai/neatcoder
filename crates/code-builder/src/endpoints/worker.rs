use anyhow::Result;
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    models::{
        fs::Files,
        job::Job,
        messages::{manager::ManagerRequest, worker::WorkerResponse},
        state::AppState,
    },
    workflows,
};

pub async fn gen_project_scaffold(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<WorkerResponse> {
    println!("[INFO] Running `Scaffolding` Job...");

    let scaffold = workflows::scaffold_project(client, params, app_state).await?;

    println!("[INFO] Completed `Scaffolding` Job");

    Ok(WorkerResponse::Scaffold { scaffold })
}

pub async fn gen_execution_plan(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<WorkerResponse> {
    println!("[INFO] Running `Planning Execution` Job...");

    let execution_plan = workflows::scaffold_project(client, params, app_state.clone()).await?;

    let files = Files::from_schedule(&execution_plan)?;
    let mut app_data = app_state.write().await;

    // Add code writing jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();

        app_data.jobs.add_todo(Job::new(
            "TODO: This is a placeholder",
            Some(ManagerRequest::CodeGen { filename: file_ }),
        ));
    }

    println!("[INFO] Completed `Planning Execution` Job...");

    Ok(WorkerResponse::BuildExecutionPlan { execution_plan })
}

pub async fn gen_code(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<WorkerResponse> {
    println!("[INFO] Running `CodeGen` Job: {}", filename);

    let stream = workflows::gen_code(client, params, app_state, filename).await?;

    println!("[INFO] Completed `CodeGen` Job...");

    Ok(WorkerResponse::CodeGen { stream })
}
