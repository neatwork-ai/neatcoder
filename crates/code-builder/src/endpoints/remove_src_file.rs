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
    filename: String,
) -> Result<()> {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        run_remove_src_file(c, j, state, filename)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), params.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to execution queue: `RemoveSourceFile`");

    Ok(())
}

pub async fn run_remove_src_file(
    _client: Arc<OpenAI>,
    _params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<WorkerResponse> {
    remove_src_file(app_state, &filename).await?;

    Ok(WorkerResponse::RemoveSourceFile { filename })
}

pub async fn remove_src_file(app_state: Arc<RwLock<AppState>>, filename: &str) -> Result<()> {
    let mut app_data = app_state.write().await;

    app_data.codebase.remove(filename);

    Ok(())
}
