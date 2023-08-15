use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::job::OpenAIJob;

use self::commit::JobID;
use self::state::AppState;

pub mod commit;
pub mod job;
pub mod job_queue;
pub mod state;

pub enum Request {
    InitWork(String),
    AddSchema(String),
    GetJobQueue,
    StartJob(JobID),
    StopJob(JobID),
    RetryJob(JobID),
}

pub trait JobTrait: Send + 'static {
    fn call_box(
        self: Box<Self>,
        client: Arc<OpenAI>,
        job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<String>>>>>;
}

impl<F, Fut> JobTrait for F
where
    F: FnOnce(Arc<OpenAI>, Arc<OpenAIJob>, Arc<Mutex<AppState>>) -> Fut + Send + 'static,
    Fut: Future<Output = Result<Arc<String>>> + 'static,
{
    fn call_box(
        self: Box<Self>,
        client: Arc<OpenAI>,
        job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<String>>>>> {
        Box::pin((*self)(client, job, app_state))
    }
}
