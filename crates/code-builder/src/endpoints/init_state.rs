use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{
    jobs::job::Task, messages::inner::WorkerResponse, state::AppState, worker::JobFutures,
};
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};

// TODO: It seems odd that for simple write operations on the AppState we have
// to still refer the ai_client and the ai_job. We need to reconsider this
pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    new_state: AppState,
) -> Result<()> {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_init_state(c, j, state, new_state)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to execution queue: `InitState`");

    Ok(())
}

pub async fn run_init_state(
    _client: Arc<OpenAI>,
    _params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    new_state: AppState,
) -> Result<WorkerResponse> {
    init_state(app_state, new_state).await?;

    Ok(WorkerResponse::InitState)
}

pub async fn init_state(app_state: Arc<RwLock<AppState>>, new_state: AppState) -> Result<()> {
    let mut app_data = app_state.write().await;

    let AppState {
        specs,
        scaffold,
        interfaces,
        codebase,
        jobs,
    } = new_state;

    app_data.specs = specs;
    app_data.scaffold = scaffold;
    app_data.interfaces = interfaces;
    app_data.codebase = codebase;
    app_data.jobs = jobs;

    Ok(())
}
