use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::Mutex;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};

use super::{
    commit::JobID,
    job::{Job, JobState, Task},
    state::AppState,
};

#[derive(Debug, Serialize)]
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
    pub async fn execute_all(
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

            let Job {
                job_id,
                job_name,
                job_type,
                job_state,
                task,
            } = job; // destruct

            let Task(task) = task;

            // Execute the job and await the result, only if the job has not been initialized yet
            if job_state == JobState::Unintialized {
                let result = task
                    .call_box(client.clone(), ai_job.clone(), app_state.clone())
                    .await?;

                // TODO: These ARCs might be hard to manage along with mutexes, the
                // safest is to instead drop them after each iteration instead accumulating
                // them and return the result
                results.push(result);
            }
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
        let Job {
            job_id,
            job_name,
            job_type,
            job_state,
            task,
        } = job; // destruct

        let Task(task) = task;

        if job_state == JobState::Unintialized {
            let result = task
                .call_box(client.clone(), ai_job.clone(), app_state.clone())
                .await?;
            Ok(result)
        } else {
            return Err(anyhow!("Invalid Job State for Job Id = {:?}", job_id));
        }
    }

    pub async fn execute_id(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<Mutex<AppState>>,
        job_id: &JobID,
    ) -> Result<Arc<String>> {
        let job = self
            .jobs
            .remove(&job_id)
            .expect(&format!("Could not find job id in queue {:?}", job_id));

        // Execute the job and await the result
        let Job {
            job_id,
            job_name,
            job_type,
            job_state,
            task,
        } = job; // destruct

        let Task(task) = task;

        if job_state == JobState::Unintialized {
            let result: Arc<String> = task
                .call_box(client.clone(), ai_job.clone(), app_state.clone())
                .await?;

            Ok(result)
        } else {
            return Err(anyhow!("Invalid Job State for Job Id = {:?}", job_id));
        }
    }
}

impl JobQueue {
    pub fn push_front(&mut self, job: Job) {
        let job_id = job.job_id;

        self.jobs.insert(job_id, job);
        self.schedule.push_front(job_id);
    }

    pub fn push_back(&mut self, job: Job) {
        let job_id = job.job_id;

        self.jobs.insert(job.job_id, job);
        self.schedule.push_back(job_id);
    }

    pub fn pop_front(&mut self) -> Option<Job> {
        let job_id = self.schedule.pop_front();

        if let Some(job_id) = job_id {
            self.jobs.remove(&job_id)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<Job> {
        let job_id = self.schedule.pop_back();

        if let Some(job_id) = job_id {
            self.jobs.remove(&job_id)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&Job> {
        let job_id = self.schedule.front();

        if let Some(job_id) = job_id {
            self.jobs.get(job_id)
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&Job> {
        let job_id = self.schedule.back();

        if let Some(job_id) = job_id {
            self.jobs.get(job_id)
        } else {
            None
        }
    }
}

pub struct JobQueueIterator {
    jobs: HashMap<JobID, Job>,
    schedule: VecDeque<JobID>,
}

impl Iterator for JobQueueIterator {
    type Item = (JobID, Job);

    fn next(&mut self) -> Option<Self::Item> {
        let job_id = self.schedule.pop_front()?;
        let job = self.jobs.remove(&job_id)?;
        Some((job_id, job))
    }
}

impl JobQueue {
    pub fn drain(&mut self) -> JobQueueIterator {
        let jobs = std::mem::replace(&mut self.jobs, HashMap::new());
        let schedule = std::mem::replace(&mut self.schedule, VecDeque::new());

        JobQueueIterator { jobs, schedule }
    }
}
