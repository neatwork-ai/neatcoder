use anyhow::Result;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};

use super::{commit::JobID, job::Job};
use crate::state::AppState;

pub struct JobQueue {
    jobs: HashMap<JobID, Job>,
    schedule: VecDeque<JobID>,
}

impl JobQueue {
    pub fn empty() -> Self {
        Self {
            jobs: HashMap::new(),
            schedule: VecDeque::new(),
        }
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

        for job_id in self.schedule.drain(..) {
            let job = self
                .jobs
                .remove(&job_id)
                .expect(&format!("Could not find job id in queue {:?}", job_id));

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

    pub async fn execute_next(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Result<Arc<String>> {
        let job_id = self.schedule.pop_front().unwrap();

        let job = self
            .jobs
            .remove(&job_id)
            .expect(&format!("Could not find job id in queue {:?}", job_id));

        // Execute the job and await the result
        let Job(job) = job; // destruct

        let result = job
            .call_box(client.clone(), ai_job.clone(), app_state.clone())
            .await?;

        Ok(result)
    }

    pub async fn execute_id(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Result<Arc<String>> {
        todo!()
    }
}
