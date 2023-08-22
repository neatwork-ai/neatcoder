use std::sync::Arc;
use tokio::sync::RwLock;

use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};

use crate::models::{job_worker::JobFutures, state::AppState};

pub async fn handle(
    _open_ai_client: Arc<OpenAI>,
    _job_futures: &mut JobFutures,
    _ai_job: Arc<OpenAIParams>,
    _app_state: Arc<RwLock<AppState>>,
) {
    todo!();
}
