use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::RwLock;

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
    pub fn execute_all(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Result<()> {
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
                let future = task.call_box(client.clone(), ai_job.clone(), app_state.clone());
            } else {
                return Err(anyhow!("Invalid Job State for Job Id = {:?}", job_id));
            }
        }

        Ok(())
    }

    pub fn execute_next(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<RwLock<AppState>>,
    ) -> Result<()> {
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
            let future = task.call_box(client.clone(), ai_job.clone(), app_state.clone());
            println!(
                "New job future added to the audit_trait, with job_id = {:?}",
                job_id
            );
        } else {
            return Err(anyhow!("Invalid Job State for Job Id = {:?}", job_id));
        }
        Ok(())
    }

    pub fn execute_id(
        &mut self,
        client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
        app_state: Arc<RwLock<AppState>>,
        job_id: &JobID,
    ) -> Result<()> {
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
            let future = task.call_box(client.clone(), ai_job.clone(), app_state.clone());
            println!(
                "New job future added to the audit_trait, with job_id = {:?}",
                job_id
            );
        } else {
            return Err(anyhow!("Invalid Job State for Job Id = {:?}", job_id));
        }
        Ok(())
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
