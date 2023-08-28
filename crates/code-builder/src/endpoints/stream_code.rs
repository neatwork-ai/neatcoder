use anyhow::{anyhow, Result};
use gluon::ai::openai::{
    client::OpenAI,
    msg::{GptRole, OpenAIMsg},
    params::OpenAIParams,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{
    code_stream::CodeStream, interfaces::AsContext, messages::inner::WorkerResponse,
    state::AppState, worker::JobFutures,
};

pub async fn handle(
    open_ai_client: Arc<OpenAI>,
    job_futures: &mut JobFutures,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<()> {
    todo!();
}

pub async fn run_stream_code(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<WorkerResponse> {
    println!("[INFO] Running `CodeGen` Job: {}", filename);

    let stream = stream_code(client, params, app_state, filename).await?;

    println!("[INFO] Completed `CodeGen` Job...");

    Ok(WorkerResponse::CodeGen { stream })
}

pub async fn stream_code(
    client: Arc<OpenAI>,
    params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    filename: String,
) -> Result<CodeStream> {
    // TODO: add task to DONE..
    println!("[INFO] Running `CodeGen` Job: {}", filename);

    let state = app_state.read().await;
    let mut prompts = Vec::new();

    let api_description = state.specs.as_ref().unwrap();

    if state.scaffold.is_none() {
        return Err(anyhow!("No folder scaffold config available.."));
    }

    let project_scaffold = state.scaffold.as_ref().unwrap();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for (_, interface) in state.interfaces.iter() {
        // Attaches context to the message sequence
        interface.add_context(&mut prompts)?;
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: String::from(api_description),
    });

    for file in state.codebase.keys() {
        let code = state.codebase.get(file).unwrap();

        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: code.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        // Needs to be optimized
        content: project_scaffold.to_string(),
    });

    let main_prompt = format!(
        "
        You are a Rust engineer tasked with creating an API in Rust.
        You are assigned to build the API based on the project folder structure
        Your current task is to write the module `{}.rs
        ",
        filename
    );

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let stream = CodeStream::new(filename, client, params, prompts);

    Ok(stream)
}
