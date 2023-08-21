use super::job::Job;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct JobQueue {
    jobs: HashMap<Uuid, Job>,
    schedule: VecDeque<Uuid>,
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
    jobs: HashMap<Uuid, Job>,
    schedule: VecDeque<Uuid>,
}

impl Iterator for JobQueueIterator {
    type Item = (Uuid, Job);

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
