use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};

use crate::{
    models::{
        fs::Files,
        job::{Job, JobType, Task},
        state::AppState,
        JobFuts,
    },
    workflows::{generate_api::gen_code, genesis::genesis},
};

pub fn handle(
    open_ai_client: Arc<OpenAI>,
    audit_trail: &mut JobFuts,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
    _init_prompt: String,
) {
    // Generates Job Queue with the two initial jobs:
    // 1. Build Project Scaffold
    // 2. Build Job Schedule
    genesis(audit_trail, open_ai_client, ai_job, app_state);
}

pub async fn handle_scaffold_job() -> Result<()> {
    Ok(())
}

pub async fn handle_schedule_job(
    job_schedule: Value,
    open_ai_client: Arc<OpenAI>,
    audit_trail: &mut JobFuts,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<()> {
    let files = Files::from_schedule(job_schedule)?;

    // Add code writing jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();
        let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
            gen_code(c, j, state, file_)
        };

        // let closure = Box::new(
        //     |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<RwLock<AppState>>| {
        //         gen_code(c, j, state, file_.clone())
        //     },
        // );

        let job = Job::new(
            String::from("TODO: This is a placeholder"),
            JobType::CodeGen,
            Task(Box::new(closure)),
        );

        audit_trail.push(job.task.0.call_box(
            open_ai_client.clone(),
            ai_job.clone(),
            app_state.clone(),
        ));
    }
    Ok(())
}
