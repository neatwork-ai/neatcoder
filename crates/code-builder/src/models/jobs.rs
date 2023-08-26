use super::job::Job;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Jobs {
    todo: Todo,
    done: Done,
}

impl Jobs {
    pub fn empty() -> Self {
        Self {
            todo: Pipeline::empty(),
            done: Pipeline::empty(),
        }
    }

    pub fn add_todo(&mut self, job: Job) {
        self.todo.push_back(job);
    }

    pub fn add_done(&mut self, job: Job) {
        self.done.push_back(job);
    }

    pub fn mark_done_by_id(&mut self, job_id: &Uuid) {
        let job = self
            .todo
            .remove(job_id)
            .expect("Could not find job in todo list");

        self.done.push_back(job)
    }

    pub fn mark_done_by_order(&mut self) {
        let job = self
            .todo
            .pop_front()
            .expect("Could not find any job in the todo list");

        self.done.push_back(job);
    }
}

pub type Todo = Pipeline;
pub type Done = Pipeline;

#[derive(Debug, Serialize, Clone)]
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
        let job_id = job.job_id;

        self.jobs.insert(job_id, job);
        self.order.push_front(job_id);
    }

    pub fn push_back(&mut self, job: Job) {
        let job_id = job.job_id;

        self.jobs.insert(job.job_id, job);
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
