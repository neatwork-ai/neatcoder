use anyhow::{Error, Result};
use futures::stream::FuturesUnordered;
use futures::Future;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::job::OpenAIJob;

use super::commit::{HashID, JobID};
use super::state::AppState;
use super::TaskTrait;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum JobState {
    Unintialized,
    InProgress,
    Done,
    Stopped,
}

#[derive(Serialize)]
pub struct Job {
    pub job_id: JobID,
    pub job_name: String,
    pub job_type: JobType,
    pub job_state: JobState,
    #[serde(skip_serializing)]
    pub task: Task,
}

// Need to implement this manually to skip the `task` closure which
// does not implement debug
impl fmt::Debug for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Job")
            .field("job_id", &self.job_id)
            .field("job_name", &self.job_name)
            .field("job_type", &self.job_type)
            .field("job_state", &self.job_type)
            // .field("task", &self.task)  // Intentionally skipping task
            .finish()
    }
}

// Marker enum
#[derive(Debug, Deserialize, Serialize)]
pub enum JobType {
    Scaffold,
    Ordering,
    CodeGen,
}

impl Job {
    pub fn new(job_name: String, job_type: JobType, task: Task) -> Self {
        let job_id = HashID::generate_random();

        Self {
            job_id,
            job_name,
            job_type,
            job_state: JobState::Unintialized,
            task: task,
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
        ai_job: Arc<OpenAIJob>,
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
