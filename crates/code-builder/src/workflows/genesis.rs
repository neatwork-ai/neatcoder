use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{job::Task, state::AppState, JobFuts};

use super::generate_api::{gen_project_scaffold, gen_work_schedule};

pub fn genesis(
    audit_trail: &mut JobFuts,
    open_ai_client: Arc<OpenAI>,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) {
    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
        gen_project_scaffold(c, j, state)
    };

    // TODO: Reconsider adding back Job and JobQueue
    // let job = Job::new(
    //     String::from("Scaffolding Project"),
    //     JobType::Scaffold,
    //     Task(Box::new(closure)),
    // );

    let task = Task(Box::new(closure));

    audit_trail.push(
        task.0
            .call_box(open_ai_client.clone(), ai_job.clone(), app_state.clone()),
    );

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
        gen_work_schedule(c, j, state)
    };

    // TODO: Reconsider adding back Job and JobQueue
    // let job = Job::new(
    //     String::from("Scheduling tasks"),
    //     JobType::Ordering,
    //     Task(Box::new(closure)),
    // );

    let task = Task(Box::new(closure));

    audit_trail.push(task.0.call_box(open_ai_client, ai_job, app_state));
}
