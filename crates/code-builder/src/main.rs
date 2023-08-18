use anyhow::Result;
use code_builder::models::job_worker::{self, JobWorker};
use code_builder::models::types::{JobRequest, JobResponse};
use serde_json;
use std::{env, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::join;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::{io::AsyncReadExt, net::TcpListener};

use code_builder::{
    endpoints,
    models::{state::AppState, ClientCommand},
};
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?; // Binding to localhost on port 7878

    println!("Listening on {:?}", listener.local_addr());

    let (mut socket, _socket_addr) = listener.accept().await?;
    let mut buf = vec![0u8; 1024];

    let open_ai_client = Arc::new(OpenAI::new(env::var("OPENAI_API_KEY")?));

    let ai_job = Arc::new(
        OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
            .temperature(0.7)
            .top_p(0.9)?,
    );

    let app_state = Arc::new(Mutex::new(AppState::empty()));

    let (tx_result, mut rx_result) = tokio::sync::mpsc::channel(100);
    let (tx_job, rx_job) = tokio::sync::mpsc::channel(100);

    let mut worker = JobWorker::new(open_ai_client, ai_job, tx_result, rx_job);
    let shutdown = Arc::new(Mutex::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    // then spawns a new thread
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen to SIGINT");
        *shutdown_clone.lock().await = true;
    });

    let join_handle = tokio::spawn(async move { worker.run(shutdown).await });

    loop {
        let n = socket.read(&mut buf).await?;
        let socked_read_fut = socket.read(&mut buf);
        let rx_result_fut = rx_result.recv();

        let join_handle = join!(socked_read_fut, rx_result_fut);

        tokio::select! {
            buf = socket.read(&mut buf) => {
                if let Err(e) = buf {
                    println!("TODO: add proper logging");
                }
                let n = buf.unwrap();

            }
        }
        if n == 0 {
            break;
        }

        let message_str = String::from_utf8_lossy(&buf[..n]);

        match serde_json::from_str::<ClientCommand>(&message_str) {
            Ok(command) => {
                match command {
                    ClientCommand::InitWork { prompt } => {
                        tx_job.send(JobRequest::InitWork { prompt }).await?;
                    }
                    ClientCommand::AddSchema { schema } => {
                        // Handle ...
                        todo!()
                    }
                    ClientCommand::GetJobQueue => {
                        // Handle ...
                        todo!()
                    }
                    ClientCommand::StartJob { job_id } => {
                        // Handle ...
                        todo!()
                    }
                    ClientCommand::StopJob { job_id } => {
                        // Handle ...
                        todo!()
                    }
                    ClientCommand::RetryJob { job_id } => {
                        // Handle ...
                        todo!()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse command: {}", e);
            }
        }
    }

    Ok(())
}
