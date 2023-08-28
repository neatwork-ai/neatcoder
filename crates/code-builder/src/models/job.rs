use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::params::OpenAIParams;

use super::messages::inner::{ManagerRequest, RequestType, WorkerResponse};
use super::state::AppState;
use super::TaskTrait;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub request: Option<ManagerRequest>,
    pub job_type: RequestType,
    pub status: JobStatus,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum JobStatus {
    Todo,
    InProgress,
    Done,
    Stopped,
}

// TODO: Reconsider if needed
// unsafe impl Send for ManagerRequest {}

impl Job {
    pub fn new_todo(name: &str, request: ManagerRequest) -> Self {
        let id = Uuid::new_v4();
        let job_type = RequestType::from(&request);

        Self {
            id,
            name: name.to_string(),
            request: Some(request),
            job_type,
            status: JobStatus::Todo,
        }
    }

    pub fn new_in_progress(name: &str, job_type: RequestType) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            name: name.to_string(),
            request: None,
            job_type,
            status: JobStatus::InProgress,
        }
    }

    pub fn start(&mut self) -> Result<ManagerRequest> {
        match self.status {
            JobStatus::Todo => {
                self.status = JobStatus::InProgress;
                let request = self.request.take().expect("No `ManagerRequest` available.");

                Ok(request)
            }
            _ => Err(anyhow!("Job has already been initialized")),
        }
    }

    pub fn complete(&mut self) -> Result<()> {
        self.status = JobStatus::Done;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        match self.status {
            JobStatus::InProgress => {
                self.status = JobStatus::Stopped;
                Ok(())
            }
            _ => Err(anyhow!("Job is not in progress so it cannot be stopped")),
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
        params: Arc<OpenAIParams>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Result<WorkerResponse> {
        let Self(job) = self; // destruct

        // Execute the job and await the result
        let result = job
            .call_box(client.clone(), params.clone(), app_state.clone())
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
