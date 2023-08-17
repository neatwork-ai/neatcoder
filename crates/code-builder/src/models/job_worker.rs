use std::{pin::Pin, sync::Arc};

use crate::endpoints::{self};

use super::{
    job::JobType,
    state::AppState,
    types::{JobRequest, JobResponse},
};
use anyhow::Error;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, RwLock,
    },
    task::JoinHandle,
};

pub struct JobWorker {
    open_ai_client: Arc<OpenAI>,
    app_state: Arc<RwLock<AppState>>,
    job_queue:
        FuturesUnordered<Pin<Box<dyn Future<Output = Result<Arc<(JobType, String)>, Error>>>>>,
    rx_job: Receiver<JobRequest>,
    tx_result: Sender<JobResponse>, // TODO: Refactor this to hold a String, or a `Response` value
}

impl JobWorker {
    pub fn new(
        rx_job: Receiver<JobRequest>,
        tx_result: Sender<JobResponse>,
        open_ai_client: Arc<OpenAI>,
    ) -> Self {
        Self {
            rx_job,
            job_queue: FuturesUnordered::new(),
            tx_result,
            open_ai_client,
            app_state: Arc::new(RwLock::new(AppState::empty())),
        }
    }

    pub async fn spawn(
        rx_job: Receiver<JobRequest>,
        tx_result: Sender<JobResponse>,
        open_ai_client: Arc<OpenAI>,
        shutdown: Arc<Mutex<bool>>, // TODO: Refactor to `AtomicBool`
    ) -> JoinHandle<Result<(), Error>> {
        let worker = Self::new(rx_job, tx_result, open_ai_client);
        tokio::spawn(worker.run(shutdown))
    }

    // TODO: make an appropriate use of the return type
    pub async fn handle_request(&mut self, request: JobRequest) -> Result<(), Error> {
        match request {
            JobRequest::InitWork { prompt } => {
                // Initialize an empty `AppState` instance
                let ai_job = Arc::new(
                    OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
                        .temperature(0.7)
                        .top_p(0.9)?,
                );
                let open_ai_client = self.open_ai_client.clone();
                let app_state = self.app_state.clone();
                let audit_trail = &mut self.job_queue;
                endpoints::init_work::handle(
                    open_ai_client,
                    audit_trail,
                    ai_job,
                    app_state,
                    prompt,
                )
                .await?;
            }
            _ => todo!(),
        }

        Ok(())
    }

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
                    let response = self.handle_request(request).await?;
                },
                result = self.job_queue.next() => {
                    self.tx_result.send(result).await?;
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
