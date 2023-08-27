pub mod add_interface;
pub mod add_schema;
pub mod execution_plan;
pub mod scaffold_project;
pub mod stream_code;

// TODO
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
