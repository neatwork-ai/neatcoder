use std::{pin::Pin, sync::Arc};

use crate::endpoints::{self};

use super::{
    job::JobType,
    state::AppState,
    types::{JobRequest, JobResponse},
};
use anyhow::Error;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};
use parser::parser::json::AsJson;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex, RwLock,
};

pub type JobFutures = FuturesUnordered<
    Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>> + 'static + Send>>,
>;

pub struct JobWorker {
    open_ai_client: Arc<OpenAI>,
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
    job_futures: JobFutures,
    rx_job: Receiver<JobRequest>,
    tx_result: Sender<JobResponse>, // TODO: Refactor this to hold a String, or a `Response` value
}

impl JobWorker {
    pub fn new(
        open_ai_client: Arc<OpenAI>,
        ai_job: Arc<OpenAIJob>,
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

    // pub fn spawn(
    //     &self,
    //     open_ai_client: Arc<OpenAI>,
    //     ai_job: Arc<OpenAIJob>,
    //     rx_job: Receiver<JobRequest>,
    //     tx_result: Sender<JobResponse>,
    //     shutdown: Arc<Mutex<bool>>, // TODO: Refactor to `AtomicBool`
    // ) -> JoinHandle<Result<(), Error>> {
    //     tokio::spawn(self.run(shutdown))
    // }

    pub async fn run(&mut self, shutdown: Arc<Mutex<bool>>) -> Result<(), Error> {
        // How to generate a shutdown signal, by the spawner:
        //
        // let shutdown = Arc::new(Mutex::new(false));
        // let shutdown_clone = Arc::clone(&shutdown);
        //
        // // then spawns a new thread
        // tokio::spawn(async move {
        //     signal::ctrl_c().await.expect("Failed to listen to SIGINT");
        //     shutdown_clone.lock().await = true;
        // });

        loop {
            tokio::select! {
                Some(request) = self.rx_job.recv() => {
                    handle_request(request, &mut self.job_futures, self.open_ai_client.clone(), self.ai_job.clone(), self.app_state.clone())?;
                },
                Some(result) = self.job_futures.next() => {
                    if let Err(e) = result {
                        println!("TODO: handle errors with logging");
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
                            endpoints::init_work::handle_schedule_job(job_schedule.clone(), self.open_ai_client.clone(), &mut self.job_futures, self.ai_job.clone(), self.app_state.clone()).await?;
                            JobResponse::Ordering { schedule_json: job_schedule}
                        },
                        JobType::CodeGen => {
                            JobResponse::CodeGen
                        },
                    };
                    let tx = self.tx_result.clone();
                    tx.send(response).await.expect("Failed to send response back");
                },
                shutdown_value = shutdown.lock() => {
                    if *shutdown_value {
                        break;
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
    ai_job: Arc<OpenAIJob>,
    app_state: Arc<RwLock<AppState>>,
) -> Result<(), Error> {
    match request {
        JobRequest::InitWork { prompt } => {
            let open_ai_client = open_ai_client.clone();
            let app_state = app_state.clone();
            endpoints::init_work::handle(open_ai_client, audit_trail, ai_job, app_state, prompt);
        }
        _ => todo!(),
    }

    Ok(())
}
