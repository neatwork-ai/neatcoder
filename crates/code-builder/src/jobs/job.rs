use anyhow::Result;
use futures::lock::Mutex;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use gluon::ai::openai::client::OpenAI;
use gluon::ai::openai::job::OpenAIJob;

use super::JobTrait;
use crate::state::AppState;

pub struct Job(pub(crate) Box<dyn JobTrait>);

impl Job {
    pub fn new(closure: Box<dyn JobTrait>) -> Self {
        Self(closure)
    }

    pub async fn execute(
        self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Result<Arc<String>> {
        let Self(job) = self; // destruct

        // Execute the job and await the result
        let result = job
            .call_box(client.clone(), ai_job.clone(), app_state.clone())
            .await?;

        Ok(result)
    }
}

impl AsRef<Box<dyn JobTrait>> for Job {
    fn as_ref(&self) -> &Box<dyn JobTrait> {
        &self.0
    }
}

impl Deref for Job {
    type Target = Box<dyn JobTrait>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Job {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
