use anyhow::Result;
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{
    job::{Job, JobType, Task},
    job_queue::JobQueue,
    state::AppState,
};

use super::generate_api::{gen_project_scaffold, gen_work_schedule};

pub fn genesis() -> Result<JobQueue> {
    let mut job_queue = JobQueue::empty();

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
        gen_project_scaffold(c, j, state)
    };

    let job = Job::new(
        String::from("Scaffolding Project"),
        JobType::Scaffold,
        Task(Box::new(closure)),
    );

    job_queue.push_back(job);

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
        gen_work_schedule(c, j, state)
    };

    let job = Job::new(
        String::from("Scheduling tasks"),
        JobType::Ordering,
        Task(Box::new(closure)),
    );

    job_queue.push_back(job);

    Ok(job_queue)
}
