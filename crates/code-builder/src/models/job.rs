use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::params::OpenAIParams;

use super::messages::inner::{ManagerRequest, WorkerResponse};
use super::state::AppState;
use super::TaskTrait;

#[derive(Debug, Serialize, Clone)]
pub struct Job {
    pub job_id: Uuid,
    pub job_name: String,
    pub request: Option<ManagerRequest>,
    pub job_state: JobState,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum JobState {
    Unintialized,
    InProgress,
    Done,
    Stopped,
}

// TODO: Reconsider if needed
// unsafe impl Send for ManagerRequest {}

impl Job {
    pub fn new(job_name: &str, request: Option<ManagerRequest>) -> Self {
        let job_id = Uuid::new_v4();

        Self {
            job_id,
            job_name: job_name.to_string(),
            request,
            job_state: JobState::Unintialized,
        }
    }
}

pub struct Task(pub(crate) Box<dyn TaskTrait>);

impl Task {
    pub fn new(closure: Box<dyn TaskTrait>) -> Self {
        Self(closure)
    }

    pub async fn execute(
        self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIParams>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Result<WorkerResponse> {
        let Self(job) = self; // destruct

        // Execute the job and await the result
        let result = job
            .call_box(client.clone(), ai_job.clone(), app_state.clone())
            .await?;

        Ok(result)
    }
}

impl AsRef<Box<dyn TaskTrait>> for Task {
    fn as_ref(&self) -> &Box<dyn TaskTrait> {
        &self.0
    }
}

impl Deref for Task {
    type Target = Box<dyn TaskTrait>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Task {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
