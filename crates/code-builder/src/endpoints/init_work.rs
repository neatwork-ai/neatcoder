use anyhow::Result;
use futures::{stream::FuturesUnordered, Future};
use parser::parser::json::AsJson;
use std::{pin::Pin, sync::Arc};
use tokio::sync::RwLock;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};

use crate::{
    models::{
        fs::Files,
        job::{Job, JobType, Task},
        state::AppState,
    },
    workflows::{generate_api::gen_code, genesis::genesis},
};

pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    audit_trail: &mut FuturesUnordered<
        Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>>>>>,
    >,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
    init_prompt: String,
) -> Result<()> {
    // Adding the inital prompt in the AppState
    {
        // RwLock gets unlocked once out of scope
        let mut state = app_state.write().await;
        state.specs = Some(init_prompt);
    }

    // Generates Job Queue with the two initial jobs:
    // 1. Build Project Scaffold
    // 2. Build Job Schedule
    genesis(audit_trail, open_ai_client, ai_job, app_state);

    Ok(())
}

pub async fn handle_scaffold_job() -> Result<()> {
    Ok(())
}

pub async fn handle_schedule_job(
    job_schedule: Arc<String>,
    open_ai_client: Arc<OpenAI>,
    audit_trail: &mut FuturesUnordered<
        Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>>>>>,
    >,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<()> {
    let job_schedule_json = job_schedule.as_str().as_json()?;
    let files = Files::from_schedule(job_schedule_json)?;

    // Add code writing jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();
        let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
            gen_code(c, j, state, file_)
        };

        let job = Job::new(
            String::from("TODO: This is a placeholder"),
            JobType::CodeGen,
            Task(Box::new(closure)),
        );

        audit_trail.push(job.task.call_box(
            open_ai_client.clone(),
            ai_job.clone(),
            app_state.clone(),
        ));
        // audit_trail.push();
    }
    Ok(())
}
