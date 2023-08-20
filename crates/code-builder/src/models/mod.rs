use anyhow::{Error, Result};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::job::OpenAIJob;

use self::commit::JobID;
use self::job::JobType;
use self::state::AppState;

pub mod commit;
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
    AddSchema { schema: String },
    GetJobQueue,
    StartJob { job_id: JobID },
    StopJob { job_id: JobID },
    RetryJob { job_id: JobID },
}

pub trait TaskTrait: Send + 'static {
    fn call_box(
        self: Box<Self>,
        client: Arc<OpenAI>,
        job: Arc<OpenAIJob>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + Send>>;
}

impl<F, Fut> TaskTrait for F
where
    F: FnOnce(Arc<OpenAI>, Arc<OpenAIJob>, Arc<RwLock<AppState>>) -> Fut + Send + 'static,
    Fut: Future<Output = Result<Arc<(JobType, String)>>> + Send + 'static,
{
    fn call_box(
        self: Box<Self>,
        client: Arc<OpenAI>,
        job: Arc<OpenAIJob>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + Send>> {
        Box::pin((*self)(client, job, app_state))
    }
}
