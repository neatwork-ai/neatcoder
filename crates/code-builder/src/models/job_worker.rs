use anyhow::Error;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use gluon::ai::openai::{client::OpenAI, params::OpenAIParams};
use parser::parser::json::AsJson;
use std::{pin::Pin, sync::Arc};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, RwLock,
    },
    task::JoinHandle,
};

use super::{
    job::JobType,
    shutdown::ShutdownSignal,
    state::AppState,
    types::{JobRequest, JobResponse},
};
use crate::endpoints::{self};

pub type JobFutures = FuturesUnordered<
    Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + 'static + Send>>,
>;

#[derive(Debug)]
pub struct JobWorker {
    open_ai_client: Arc<OpenAI>,
    ai_job: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
    job_futures: JobFutures,
    rx_job: Receiver<JobRequest>,
    tx_result: Sender<JobResponse>, // TODO: Refactor this to hold a String, or a `Response` value
}

impl JobWorker {
    pub fn new(
        open_ai_client: Arc<OpenAI>,
        ai_job: Arc<OpenAIParams>,
        tx_result: Sender<JobResponse>,
        rx_job: Receiver<JobRequest>,
    ) -> Self {
        Self {
            rx_job,
            ai_job,
            job_futures: FuturesUnordered::new(),
            tx_result,
            open_ai_client,
            app_state: Arc::new(RwLock::new(AppState::empty())),
        }
    }

    pub fn spawn(
        open_ai_client: Arc<OpenAI>,
        ai_job: Arc<OpenAIParams>,
        rx_job: Receiver<JobRequest>,
        tx_result: Sender<JobResponse>,
        tcp_stream: Arc<Mutex<TcpStream>>,
        shutdown: ShutdownSignal, // TODO: Refactor to `AtomicBool`
    ) -> JoinHandle<Result<(), Error>> {
        tokio::spawn(async move {
            Self::new(open_ai_client, ai_job, tx_result, rx_job)
                .run(tcp_stream, shutdown)
                .await
        })
    }

    pub async fn run(
        &mut self,
        tcp_stream: Arc<Mutex<TcpStream>>,
        shutdown: ShutdownSignal,
    ) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(request) = self.rx_job.recv() => {
                    handle_request(request, &mut self.job_futures, self.open_ai_client.clone(), self.ai_job.clone(), self.app_state.clone())?;
                },
                Some(result) = self.job_futures.next() => {
                    if let Err(e) = result {
                        println!("TODO: handle errors with logging: {e}");
                        continue;
                    }
                    let inner = result.unwrap();
                    let (job_type, message) = inner.as_ref();
                    let response = match job_type {
                        JobType::Scaffold => {
                            endpoints::init_work::handle_scaffold_job().await?;
                            JobResponse::Scaffold
                        },
                        JobType::Ordering => {
                            let job_schedule = message.as_str().as_json()?;

                            endpoints::init_work::handle_schedule_job(
                                job_schedule.clone(),
                                self.open_ai_client.clone(),
                                &mut self.job_futures,
                                self.ai_job.clone(),
                                self.app_state.clone(),
                                tcp_stream.clone()
                            ).await?;

                            JobResponse::Ordering { schedule_json: job_schedule}
                        },
                        JobType::CodeGen => {
                            JobResponse::CodeGen { is_sucess: true}
                        },
                    };
                    let tx = self.tx_result.clone();
                    tx.send(response).await.expect("Failed to send response back");
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
pub fn handle_request(
    request: JobRequest,
    audit_trail: &mut FuturesUnordered<
        Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + Send + 'static>>,
    >,
    open_ai_client: Arc<OpenAI>,
    ai_job: Arc<OpenAIParams>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<(), Error> {
    match request {
        JobRequest::InitWork { prompt } => {
            let open_ai_client = open_ai_client.clone();
            let app_state = app_state.clone();
            endpoints::init_work::handle(open_ai_client, audit_trail, ai_job, app_state, prompt);
        }
        // JobRequest::AddModel { path, schema } => {
        //     let open_ai_client = open_ai_client.clone();
        //     let app_state = app_state.clone();
        //     endpoints::add_model::handle();
        // }
        JobRequest::CodeGen { filename } => {}
        _ => todo!(),
    }

    Ok(())
}
