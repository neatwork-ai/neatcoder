use anyhow::{anyhow, Result};
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{
    job_worker::{self, JobFutures},
    messages::inner::ManagerRequest,
    state::AppState,
};

pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    job_id: Uuid,
) -> Result<()> {
    let mut app_data = app_state.write().await;

    let request = app_data.jobs.start_job_by_id(&job_id)?;
    drop(app_data);

    // This check is to avoid infinite recursion
    if let ManagerRequest::StartJob { job_uid: _ } = request {
        return Err(anyhow!("`StartJob` cannot start a task of its own type"));
    }

    job_worker::handle_request(request, job_futures, open_ai_client, params, app_state);

    Ok(())
}
