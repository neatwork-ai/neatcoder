use anyhow::Result;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::Mutex;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};

use super::job::Job;
use crate::state::AppState;

pub struct JobQueue(VecDeque<Job>);

impl JobQueue {
    pub fn empty() -> Self {
        Self(VecDeque::new())
    }
}

impl AsRef<VecDeque<Job>> for JobQueue {
    fn as_ref(&self) -> &VecDeque<Job> {
        &self.0
    }
}

impl Deref for JobQueue {
    type Target = VecDeque<Job>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JobQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JobQueue {
    pub async fn execute_jobs(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Result<Vec<Arc<String>>> {
        let mut results = Vec::new();

        for job in self.drain(..) {
            let Job(job) = job; // destruct

            // Execute the job and await the result
            let result = job
                .call_box(client.clone(), ai_job.clone(), app_state.clone())
                .await?;

            // TODO: These ARCs might be hard to manage along with mutexes, the
            // safest is to instead drop them after each iteration instead accumulating
            // them and return the result
            results.push(result);
        }

        Ok(results)
    }

    pub async fn execute(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Result<Arc<String>> {
        let job = self.pop_front().unwrap();

        // Execute the job and await the result
        let Job(job) = job; // destruct

        let result = job
            .call_box(client.clone(), ai_job.clone(), app_state.clone())
            .await?;

        Ok(result)
    }
}
