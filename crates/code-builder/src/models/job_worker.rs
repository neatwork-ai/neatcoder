use std::{net::Shutdown, sync::Arc};

use crate::endpoints::{self};

use super::{job::Job, job_queue::JobQueue, state::AppState, types::JobRequest};
use anyhow::Error;
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex, RwLock,
};

pub struct JobWorker {
    rx_job: Receiver<JobRequest>,
    job_queue: Arc<RwLock<JobQueue>>,
    tx_result: Sender<Arc<Mutex<String>>>,
    open_ai_client: Arc<OpenAI>,
}

impl JobWorker {
    pub fn new(
        rx_job: Receiver<JobRequest>,
        job_queue: Arc<RwLock<JobQueue>>,
        tx_result: Sender<Arc<Mutex<String>>>,
        open_ai_client: Arc<OpenAI>,
    ) -> Self {
        Self {
            rx_job,
            job_queue,
            tx_result,
            open_ai_client,
        }
    }

    pub async fn handle_request(&self, request: JobRequest) -> Result<(), Error> {
        match request {
            JobRequest::InitWork { prompt } => {
                // Get a write lock for the internal job queue
                let job_queue = self.job_queue.clone();
                let job_queue_rw_guard = job_queue.write().await;
                // Initialize an empty `AppState` instance
                let app_state = Arc::new(RwLock::new(AppState::empty()));
                let ai_job = Arc::new(
                    OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
                        .temperature(0.7)
                        .top_p(0.9)?,
                );
                let open_ai_client = self.open_ai_client.clone();
                endpoints::init_work::handle(
                    open_ai_client,
                    ai_job,
                    job_queue_rw_guard,
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
        !// How to generate a shutdown signal, by the spawner:
        !//
        !// let shutdown = Arc::new(Mutex::new(false));
        !// let shutdown_clone = Arc::clone(&shutdown);
        !//
        !// // then spawns a new thread
        !// tokio::spawn(async move {
        !//     signal::ctrl_c().await.expect("Failed to listen to SIGINT");
        !//     shutdown_clone.lock().await = true;
        !// })

        loop {
            tokio::select! {
                Some(request) = self.rx_job.recv() => {
                    let response = self.handle_request(request).await?;
                    // TODO: needs to send response back to client
                    self.tx_result.send(Arc::new(Mutex::new(response)));
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
