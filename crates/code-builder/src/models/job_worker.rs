use anyhow::Error;
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

use super::{
    messages::{manager::ManagerRequest, server::ServerMsg, worker::WorkerResponse},
    shutdown::ShutdownSignal,
    state::AppState,
};
use crate::endpoints::manager as manager_endpoints;

// TODO: Potentially link `JobFutures` with `Jobs` via Uuid.
pub type JobFutures =
    FuturesUnordered<Pin<Box<dyn Future<Output = Result<WorkerResponse, Error>> + 'static + Send>>>;

#[derive(Debug)]
pub struct JobWorker {
    open_ai_client: Arc<OpenAI>,
    ai_params: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    job_futures: JobFutures,
    rx_request: Receiver<ManagerRequest>,
    tx_response: Sender<WorkerResponse>, // TODO: Refactor this to hold a String, or a `Response` value
    listener_address: String,
}

impl JobWorker {
    pub fn new(
        open_ai_client: Arc<OpenAI>,
        ai_params: Arc<OpenAIParams>,
        rx_request: Receiver<ManagerRequest>,
        tx_response: Sender<WorkerResponse>,
        listener_address: String,
    ) -> Self {
        Self {
            rx_request,
            ai_params,
            job_futures: FuturesUnordered::new(),
            tx_response,
            open_ai_client,
            app_state: Arc::new(RwLock::new(AppState::empty())),
            listener_address,
        }
    }

    pub fn spawn(
        open_ai_client: Arc<OpenAI>,
        ai_params: Arc<OpenAIParams>,
        rx_request: Receiver<ManagerRequest>,
        tx_response: Sender<WorkerResponse>,
        listener_address: String,
        shutdown: ShutdownSignal, // TODO: Refactor to `AtomicBool`
    ) -> JoinHandle<Result<(), Error>> {
        tokio::spawn(async move {
            Self::new(
                open_ai_client,
                ai_params,
                rx_request,
                tx_response,
                listener_address,
            )
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
                    handle_response(result, self.tx_response.clone(), self.app_state.clone());
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

// TODO: make an appropriate use of the return type
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
        ManagerRequest::ScaffoldProject { prompt } => {
            manager_endpoints::init_prompt::scaffold_project(
                open_ai_client.clone(),
                job_futures,
                ai_params,
                app_state.clone(),
                prompt,
            )
            .await;
        }
        ManagerRequest::BuildExecutionPlan {} => {
            manager_endpoints::init_prompt::build_execution_plan(
                open_ai_client.clone(),
                job_futures,
                ai_params,
                app_state.clone(),
            )
            .await;
        }
        // TODO
        // ManagerRequest::CodeGen { filename: String } => {

        // }
        // TODO: Reconsider
        // ManagerRequest::AddSchema {
        //     interface_name,
        //     schema_name,
        //     schema,
        // } => {
        //     let app_state = app_state.clone();
        //     endpoints::add_schema::handle(app_state, interface_name, schema_name, schema).await?;
        // }
        // ManagerRequest::AddInterface { interface } => {
        //     let app_state = app_state.clone();
        //     endpoints::add_interface::handle(app_state, interface).await?;
        // }
        _ => todo!(),
    }

    Ok(())
}

pub async fn handle_response(
    result: Result<WorkerResponse, Error>,
    tx_response: Sender<WorkerResponse>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<(), Error> {
    let worker_response = result.unwrap();
    tx_response
        .send(worker_response)
        .await
        .expect("Failed to send response back");

    Ok(())
}
