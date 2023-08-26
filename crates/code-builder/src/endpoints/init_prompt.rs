use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};

use crate::{
    models::{
        fs::Files,
        job::{Job, JobType, Task},
        job_worker::JobFutures,
        state::AppState,
        types::JobRequest,
    },
    workflows::generate_api::{gen_code, gen_execution_plan, gen_project_scaffold},
};

pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    ai_job: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    init_prompt: String,
) {
    // Generates Job Queue with the two initial jobs:
    // 1. Build Project Scaffold
    // 2. Build Execution plan

    scaffold_project(
        open_ai_client.clone(),
        job_futures,
        ai_job.clone(),
        app_state.clone(),
        init_prompt,
    )
    .await;

    build_execution_plan(open_ai_client, job_futures, ai_job, app_state).await;
}

pub async fn scaffold_project(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    ai_job: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    init_prompt: String,
) {
    let mut app_data = app_state.write().await;

    // TODO: Return error if `specs` field already exists..
    app_data.specs = Some(init_prompt);
    println!("[INFO] Registered Project Specifications.");

    let job_name = "Scaffolding";

    app_data
        .jobs
        .add_todo(Job::new(job_name, JobType::Scaffold, None));

    println!("[INFO] Added task `{}` as TODO", job_name);

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        gen_project_scaffold(c, j, state)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), ai_job.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to the exeuction queue: `{}`", job_name);
}

pub async fn build_execution_plan(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    ai_job: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) {
    let mut app_data = app_state.write().await;

    let job_name = "Build Execution Plan";

    app_data
        .jobs
        .add_todo(Job::new(job_name, JobType::Ordering, None));

    println!("[INFO] Added task `{}` as TODO", job_name);

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
        gen_execution_plan(c, j, state)
    };

    let task = Task(Box::new(closure));

    job_futures.push(
        task.0
            .call_box(open_ai_client.clone(), ai_job.clone(), app_state.clone()),
    );

    println!("[INFO] Pushed task to exeuction queue: `{}`", job_name);
}

pub async fn orchestrate_code_gen(
    job_schedule: Value,
    app_state: Arc<RwLock<AppState>>,
) -> Result<()> {
    println!("[INFO] Orchestrating Job Queue");
    let files = Files::from_schedule(job_schedule)?;
    let mut app_data = app_state.write().await;

    // Add code writing jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();
        let code_job = JobRequest::CodeGen { filename: file_ };

        app_data.jobs.add_todo(Job::new(
            "TODO: This is a placeholder",
            JobType::CodeGen,
            Some(code_job),
        ));
    }
    Ok(())
}

// pub async fn orchestrate_code_gen(
//     job_schedule: Value,
//     open_ai_client: Arc<OpenAI>,
//     job_futures: &mut JobFutures,
//     ai_job: Arc<OpenAIParams>,
//     app_state: Arc<RwLock<AppState>>,
//     listener_address: String,
// ) -> Result<()> {
//     println!("[INFO] Orchestrating Job Queue");
//     let files = Files::from_schedule(job_schedule)?;
//     let mut app_data = app_state.write().await;

//     // Add code writing jobs to the job queue
//     for file in files.iter() {
//         let file_ = file.clone();
//         let listener_address = listener_address.clone();
//         let code_job = JobRequest::CodeGen { filename: file_ };

//         println!("[INFO] Added task `{:?}` as TODO", code_job);

//         let closure = move |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
//             gen_code(c, j, state, code_job, listener_address)
//         };

//         app_data
//             .jobs
//             .add_todo(Job::new("TODO: This is a placeholder", JobType::CodeGen));

//         let task = Task(Box::new(closure));

//         job_futures.push(task.0.call_box(
//             open_ai_client.clone(),
//             ai_job.clone(),
//             app_state.clone(),
//         ));
//     }
//     Ok(())
// }
