use anyhow::Result;
use futures::{stream::FuturesUnordered, Future};
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};
use std::{pin::Pin, sync::Arc};
use tokio::sync::RwLock;

use crate::models::{
    job::{Job, JobType, Task},
    state::AppState,
};

use super::generate_api::{gen_project_scaffold, gen_work_schedule};

pub fn genesis(
    audit_trail: &mut FuturesUnordered<
        Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>>>>>,
    >,
    open_ai_client: Arc<OpenAI>,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
        gen_project_scaffold(c, j, state)
    };

    let job = Job::new(
        String::from("Scaffolding Project"),
        JobType::Scaffold,
        Task(Box::new(closure)),
    );

    audit_trail.push(job.task.call_box(open_ai_client, ai_job, app_state));

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
        gen_work_schedule(c, j, state)
    };

    let job = Job::new(
        String::from("Scheduling tasks"),
        JobType::Ordering,
        Task(Box::new(closure)),
    );

    audit_trail.push(job.task.call_box(open_ai_client, ai_job, app_state));
}
