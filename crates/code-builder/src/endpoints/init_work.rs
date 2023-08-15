use anyhow::Result;
use parser::parser::json::AsJson;
use std::sync::Arc;
use tokio::sync::Mutex;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};

use crate::{
    models::{fs::Files, job::Job, job_queue::JobQueue, state::AppState},
    workflows::{generate_api::gen_code, genesis::genesis},
};

pub async fn handle(
    client: Arc<OpenAI>,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<Mutex<AppState>>,
) -> Result<JobQueue> {
    // Generates Job Queue with the two initial jobs:
    // 1. Build Project Scaffold
    // 2. Build Job Schedule
    let mut job_queue = genesis()?;

    // Execute the jobs and handle the results
    println!("Building Project Scaffold");
    let _scaffold: Arc<String> = job_queue
        .execute_next(client.clone(), ai_job.clone(), app_state.clone())
        .await?;

    println!("Building Job Schedule");
    let job_schedule: Arc<String> = job_queue
        .execute_next(client.clone(), ai_job.clone(), app_state.clone())
        .await?;

    let job_schedule_json = job_schedule.as_str().as_json()?;

    let files = Files::from_schedule(job_schedule_json)?;

    // Add code writing jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();

        let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
            gen_code(c, j, state, file_)
        };

        let job = Job::new(Box::new(closure));
        job_queue.push_back(job);
    }

    Ok(job_queue)
}
