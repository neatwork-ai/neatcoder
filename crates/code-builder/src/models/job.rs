use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::params::OpenAIParams;

use super::state::AppState;
use super::types::JobRequest;
use super::TaskTrait;

#[derive(Debug, Serialize)]
pub struct Job {
    pub job_id: Uuid,
    pub job_name: String,
    // Deprecate in favour of JobRequest
    pub job_type: JobType,
    pub job_state: JobState,
    pub job_payload: Option<JobRequest>,
}

// Marker enum
#[derive(Debug, Deserialize, Serialize)]
pub enum JobType {
    Scaffold,
    Ordering,
    CodeGen,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum JobState {
    Unintialized,
    InProgress,
    Done,
    Stopped,
}

unsafe impl Send for JobType {}

impl Job {
    pub fn new(job_name: &str, job_type: JobType, job_payload: Option<JobRequest>) -> Self {
        let job_id = Uuid::new_v4();

        Self {
            job_id,
            job_name: job_name.to_string(),
            job_type,
            job_state: JobState::Unintialized,
            job_payload,
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
    ) -> Result<Arc<(JobType, String)>> {
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
