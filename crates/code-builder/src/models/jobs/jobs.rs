use crate::models::messages::inner::{ManagerRequest, RequestType};

use super::job::Job;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jobs {
    todo: Todo,
    in_progress: InProgress,
    stopped: Stopped,
    done: Done,
}

impl Jobs {
    pub fn empty() -> Self {
        Self {
            todo: Pipeline::empty(),
            in_progress: Pipeline::empty(),
            stopped: Pipeline::empty(),
            done: Pipeline::empty(),
        }
    }

    pub fn new_todo(&mut self, job_name: &str, request: ManagerRequest) -> Uuid {
        let job = Job::new_todo(job_name, request);
        let job_id = job.id;

        self.todo.push_back(job);

        job_id
    }

    pub fn new_in_progress(&mut self, job_name: &str, request: RequestType) -> Uuid {
        let job = Job::new_in_progress(job_name, request);
        let job_id = job.id;

        self.in_progress.push_back(job);

        job_id
    }

    pub fn start_job_by_id(&mut self, job_id: &Uuid) -> Result<ManagerRequest> {
        let mut job = self
            .todo
            .remove(job_id)
            .expect("Could not find job in todo list");

        let manager_request = job.start()?;

        self.in_progress.push_back(job);

        Ok(manager_request)
    }

    pub fn start_job_by_order(&mut self) -> Result<ManagerRequest> {
        let mut job = self
            .todo
            .pop_front()
            .expect("Could not find any job in the todo list");

        let manager_request = job.start()?;

        self.in_progress.push_back(job);

        Ok(manager_request)
    }

    pub fn stop_job_by_id(&mut self, job_id: &Uuid) -> Result<()> {
        let mut job = self
            .in_progress
            .remove(job_id)
            .expect("Could not find job in todo list");

        job.stop()?;

        self.stopped.push_back(job);

        Ok(())
    }

    pub fn stop_job_by_order(&mut self) -> Result<()> {
        let mut job = self
            .in_progress
            .pop_front()
            .expect("Could not find any job in the todo list");

        job.stop()?;

        self.stopped.push_back(job);

        Ok(())
    }

    pub fn finish_job_by_id(&mut self, job_id: &Uuid) -> Result<()> {
        let mut job = self
            .in_progress
            .remove(job_id)
            .expect("Could not find job in todo list");

        job.complete()?;

        self.done.push_back(job);

        Ok(())
    }

    pub fn finish_job_by_order(&mut self) -> Result<()> {
        let mut job = self
            .in_progress
            .pop_front()
            .expect("Could not find any job in the todo list");

        job.complete()?;

        self.done.push_back(job);

        Ok(())
    }
}

pub type Todo = Pipeline;
pub type InProgress = Pipeline;
pub type Stopped = Pipeline;
pub type Done = Pipeline;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    jobs: HashMap<Uuid, Job>,
    order: VecDeque<Uuid>,
}

impl Pipeline {
    pub fn empty() -> Self {
        Self {
            jobs: HashMap::new(),
            order: VecDeque::new(),
        }
    }
}

impl Pipeline {
    pub fn push_front(&mut self, job: Job) {
        let job_id = job.id;

        self.jobs.insert(job_id, job);
        self.order.push_front(job_id);
    }

    pub fn push_back(&mut self, job: Job) {
        let job_id = job.id;

        self.jobs.insert(job.id, job);
        self.order.push_back(job_id);
    }

    pub fn pop_front(&mut self) -> Option<Job> {
        let job_id = self.order.pop_front();

        if let Some(job_id) = job_id {
            self.jobs.remove(&job_id)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<Job> {
        let job_id = self.order.pop_back();

        if let Some(job_id) = job_id {
            self.jobs.remove(&job_id)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&Job> {
        let job_id = self.order.front();

        if let Some(job_id) = job_id {
            self.jobs.get(job_id)
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&Job> {
        let job_id = self.order.back();

        if let Some(job_id) = job_id {
            self.jobs.get(job_id)
        } else {
            None
        }
    }

    pub fn remove(&mut self, job_id: &Uuid) -> Option<Job> {
        let job = self.jobs.remove(job_id);

        if job.is_some() {
            self.order.retain(|&id| id != *job_id);
        }

        job
    }
}

pub struct PipelineIterator {
    jobs: HashMap<Uuid, Job>,
    order: VecDeque<Uuid>,
}

impl Iterator for PipelineIterator {
    type Item = (Uuid, Job);

    fn next(&mut self) -> Option<Self::Item> {
        let job_id = self.order.pop_front()?;
        let job = self.jobs.remove(&job_id)?;
        Some((job_id, job))
    }
}

impl Pipeline {
    pub fn drain(&mut self) -> PipelineIterator {
        let jobs = std::mem::replace(&mut self.jobs, HashMap::new());
        let order = std::mem::replace(&mut self.order, VecDeque::new());

        PipelineIterator { jobs, order }
    }
}
