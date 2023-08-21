use anyhow::{Error, Result};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::params::OpenAIParams;

use self::job::JobType;
use self::state::AppState;

pub mod fs;
pub mod job;
pub mod job_queue;
pub mod job_worker;
pub mod schema;
pub mod shutdown;
pub mod state;
pub mod types;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientCommand {
    InitWork { prompt: String },
    AddModel { path: String, schema: String },
    StartJob { job_id: Uuid },
    StopJob { job_id: Uuid },
    RetryJob { job_id: Uuid },
}

pub trait TaskTrait: Send + 'static {
    fn call_box(
        self: Box<Self>,
        client: Arc<OpenAI>,
        job: Arc<OpenAIParams>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + Send>>;
}

impl<F, Fut> TaskTrait for F
where
    F: FnOnce(Arc<OpenAI>, Arc<OpenAIParams>, Arc<RwLock<AppState>>) -> Fut + Send + 'static,
    Fut: Future<Output = Result<Arc<(JobType, String)>>> + Send + 'static,
{
    fn call_box(
        self: Box<Self>,
        client: Arc<OpenAI>,
        job: Arc<OpenAIParams>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + Send>> {
        Box::pin((*self)(client, job, app_state))
    }
}
