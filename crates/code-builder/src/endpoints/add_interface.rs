use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{
    interfaces::Interface, jobs::job::Task, messages::inner::WorkerResponse, state::AppState,
    worker::JobFutures,
};
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};

// TODO: It seems odd that for simple write operations on the AppState we have
// to still refer the ai_client and the ai_job. We need to reconsider this
pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    interface: Interface,
) -> Result<()> {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_add_interface(c, j, state, interface)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to execution queue: `AddInterface`");

    Ok(())
}

pub async fn run_add_interface(
    _client: Arc<OpenAI>,
    _params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    interface: Interface,
) -> Result<WorkerResponse> {
    println!("[INFO] Resolving Task: `AddInterface`");
    let interface_name = interface.name().to_string();
    add_interface(app_state, interface).await?;
    println!("[INFO] Resolved Task: `AddInterface`");

    Ok(WorkerResponse::AddInterface { interface_name })
}

pub async fn add_interface(app_state: Arc<RwLock<AppState>>, interface: Interface) -> Result<()> {
    let mut app_data = app_state.write().await;

    app_data.add_interface(interface)?;

    Ok(())
}
