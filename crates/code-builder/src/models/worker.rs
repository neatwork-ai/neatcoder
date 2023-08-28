use anyhow::Error;
use async_recursion::async_recursion;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};
use std::{pin::Pin, sync::Arc};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};

use crate::endpoints;

use super::{
    messages::inner::{ManagerRequest, WorkerResponse},
    shutdown::ShutdownSignal,
    state::AppState,
};

/// Type alias for a collection of futures representing jobs.
pub type JobFutures =
    FuturesUnordered<Pin<Box<dyn Future<Output = Result<WorkerResponse, Error>> + 'static + Send>>>;

/// Definition of the JobWorker struct, responsible for managing and executing jobs.
/// When the server gets spawned it will spawn a `JobWorker` threads which will be
/// listening to requests from the main thread which in turn will be prompted by
/// the client messages.
///
/// The JobWorker listens to incoming requests in order to produce futures to add
/// to the `job_futures` as well as resolves the futures on the fly as they're added
/// to the `job_futures` set.
#[derive(Debug)]
pub struct JobWorker {
    /// OpenAI Client
    // TODO: Eventually generalise to other LLMs, in particular HuggingFace compatible LLMs
    open_ai_client: Arc<OpenAI>,
    /// Parameters for the LLM
    ai_params: Arc<OpenAIParams>,
    /// Shared Application State
    app_state: Arc<RwLock<AppState>>,
    /// Collection of active job futures. Whenever a future gets added to this set
    /// it will be picked up the worker thread and resolved.
    job_futures: JobFutures,
    // Channel receiver for incoming requests from the main thread.
    rx_request: Receiver<ManagerRequest>,
    // Channel sender for the outgoin responsed to the main thread.
    tx_response: Sender<WorkerResponse>,
}

impl JobWorker {
    pub fn new(
        open_ai_client: Arc<OpenAI>,
        ai_params: Arc<OpenAIParams>,
        rx_request: Receiver<ManagerRequest>,
        tx_response: Sender<WorkerResponse>,
    ) -> Self {
        Self {
            rx_request,
            ai_params,
            job_futures: FuturesUnordered::new(),
            tx_response,
            open_ai_client,
            app_state: Arc::new(RwLock::new(AppState::empty())),
        }
    }

    pub fn spawn(
        open_ai_client: Arc<OpenAI>,
        ai_params: Arc<OpenAIParams>,
        rx_request: Receiver<ManagerRequest>,
        tx_response: Sender<WorkerResponse>,
        shutdown: ShutdownSignal, // TODO: Refactor to `AtomicBool`
    ) -> JoinHandle<Result<(), Error>> {
        tokio::spawn(async move {
            Self::new(open_ai_client, ai_params, rx_request, tx_response)
                .run(shutdown)
                .await
        })
    }

    pub async fn run(&mut self, shutdown: ShutdownSignal) -> Result<(), Error> {
        loop {
            tokio::select! {
                // Handles requests from the client, reads/writes to `AppState`
                // accordingly and creates Job Futures if necessary.
                Some(request) = self.rx_request.recv() => {
                    handle_request(request, &mut self.job_futures, self.open_ai_client.clone(), self.ai_params.clone(), self.app_state.clone()).await?;
                },
                Some(result) = self.job_futures.next() => {
                    if let Err(e) = result {
                        println!("TODO: handle errors with logging: {e}");
                        continue;
                    }
                    handle_response(result, self.tx_response.clone()).await?;
                },
                shutdown_handle = shutdown.wait_for_signal().await => {
                    if let Ok(signal) = shutdown_handle {
                        if *signal.lock().await {
                            break;
                        }
                    } else if let Err(e) = shutdown_handle {
                        println!("Failed to get signal, with error: {e}")
                    }
                }
            }
        }

        Ok(())
    }
}

#[async_recursion]
pub async fn handle_request(
    request: ManagerRequest,
    job_futures: &mut FuturesUnordered<
        Pin<Box<dyn Future<Output = Result<WorkerResponse, Error>> + Send + 'static>>,
    >,
    open_ai_client: Arc<OpenAI>,
    ai_params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<(), Error> {
    match request {
        ManagerRequest::InitState { state } => {
            endpoints::init_state::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                state,
            )
            .await?;
        }
        ManagerRequest::ScaffoldProject { prompt } => {
            endpoints::scaffold_project::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                prompt,
            )
            .await;
        }
        ManagerRequest::BuildExecutionPlan {} => {
            endpoints::execution_plan::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
            )
            .await;
        }
        ManagerRequest::AddSchema {
            interface_name,
            schema_name,
            schema,
        } => {
            endpoints::add_schema::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                interface_name,
                schema_name,
                schema,
            )
            .await?;
        }
        ManagerRequest::RemoveSchema {
            interface_name,
            schema_name,
        } => {
            endpoints::remove_schema::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                interface_name,
                schema_name,
            )
            .await?;
        }
        ManagerRequest::AddInterface { interface } => {
            endpoints::add_interface::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                interface,
            )
            .await?;
        }
        ManagerRequest::RemoveInterface { interface_name } => {
            endpoints::remove_interface::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                interface_name,
            )
            .await?;
        }
        ManagerRequest::StartJob { job_uid } => {
            endpoints::start_job::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                job_uid,
            )
            .await?;
        }
        ManagerRequest::CodeGen { filename } => {
            endpoints::stream_code::handle(
                open_ai_client.clone(),
                job_futures,
                ai_params.clone(),
                app_state.clone(),
                filename,
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn handle_response(
    result: Result<WorkerResponse, Error>,
    tx_response: Sender<WorkerResponse>,
) -> Result<(), Error> {
    let worker_response = result.unwrap();
    tx_response
        .send(worker_response)
        .await
        .expect("Failed to send response back");

    Ok(())
}
