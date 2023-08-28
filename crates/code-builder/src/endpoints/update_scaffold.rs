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
    scaffold: String,
) -> Result<()> {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_update_scaffold(c, j, state, scaffold)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to exeuction queue: `UpdateScaffold`");

    Ok(())
}

pub async fn run_update_scaffold(
    _client: Arc<OpenAI>,
    _params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    scaffold: String,
) -> Result<WorkerResponse> {
    update_scaffold(app_state, scaffold.clone()).await?;

    Ok(WorkerResponse::UpdateScaffold)
}

pub async fn update_scaffold(app_state: Arc<RwLock<AppState>>, scaffold: String) -> Result<()> {
    let mut app_data = app_state.write().await;

    app_data.scaffold = Some(scaffold);

    Ok(())
}
