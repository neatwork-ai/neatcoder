use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{interfaces::InterfaceFile, state::AppState};

pub async fn handle(
    app_state: Arc<RwLock<AppState>>,
    interface_name: String,
    interface_file: InterfaceFile,
) {
    let mut app_data = app_state.write().await;

    app_data.add_interface_file(interface_name, interface_file);
}

// pub async fn scaffold_project(
//     open_ai_client: Arc<OpenAI>,
//     job_futures: &mut JobFutures,
//     ai_job: Arc<OpenAIParams>,
//     app_state: Arc<RwLock<AppState>>,
//     init_prompt: String,
// ) {
//     let mut app_data = app_state.write().await;

//     // TODO: Return error if `specs` field already exists..
//     app_data.specs = Some(init_prompt);

//     app_data
//         .jobs
//         .add_todo(Job::new("Scaffolding Project", JobType::Scaffold));

//     let closure = |c: Arc<OpenAI>, j: Arc<OpenAIParams>, state: Arc<RwLock<AppState>>| {
//         gen_project_scaffold(c, j, state)
//     };

//     let task = Task(Box::new(closure));

//     job_futures.push(
//         task.0
//             .call_box(open_ai_client.clone(), ai_job.clone(), app_state.clone()),
//     );
// }
