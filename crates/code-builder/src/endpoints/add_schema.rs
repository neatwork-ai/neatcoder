use anyhow::Result;
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{
    interfaces::SchemaFile, jobs::job::Task, messages::inner::WorkerResponse, state::AppState,
    worker::JobFutures,
};

// TODO: It seems odd that for simple write operations on the AppState we have
// to still refer the ai_client and the ai_job. We need to reconsider this
pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    interface_name: String,
    schema_name: String,
    schema: SchemaFile,
) -> Result<()> {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_add_schema(c, j, state, interface_name, schema_name, schema)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to exeuction queue: `AddSchema`");

    Ok(())
}

pub async fn run_add_schema(
    _client: Arc<OpenAI>,
    _params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    interface_name: String,
    schema_name: String,
    schema: SchemaFile,
) -> Result<WorkerResponse> {
    println!("[INFO] Resolved Task: `AddSchema`");
    add_schema(app_state, interface_name, schema_name.clone(), schema).await?;
    println!("[INFO] Resolved Task: `AddSchema`");

    Ok(WorkerResponse::AddSchema { schema_name })
}

pub async fn add_schema(
    app_state: Arc<RwLock<AppState>>,
    interface_name: String,
    schema_name: String,
    schema: SchemaFile,
) -> Result<()> {
    let mut app_data = app_state.write().await;

    app_data.add_schema(interface_name, schema_name, schema)?;

    Ok(())
}
