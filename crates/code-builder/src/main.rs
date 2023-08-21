use anyhow::Result;
use bincode;
use code_builder::models::job_worker::JobWorker;
use code_builder::models::shutdown::ShutdownSignal;
use code_builder::models::types::JobRequest;
use serde_json;
use std::{env, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpListener};

use code_builder::models::ClientCommand;
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

    let (tx_result, mut rx_result) = tokio::sync::mpsc::channel(100);
    let (tx_job, rx_job) = tokio::sync::mpsc::channel(100);

    let shutdown = ShutdownSignal::new();

    let _join_handle = JobWorker::spawn(open_ai_client, ai_job, rx_job, tx_result, shutdown);
    loop {
        tokio::select! {
            result = socket.read(&mut buf) => {
                let n = if let Err(e) = result {
                    println!("TODO: add proper logging {e}");
                    continue;
                } else {
                    result.unwrap()
                };

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
                            ClientCommand::AddModel { path, schema } => {
                                tx_job.send(JobRequest::AddModel { path, schema }).await?;
                            }
                            ClientCommand::StartJob { .. } => {
                                // Handle ...
                                todo!()
                            }
                            ClientCommand::StopJob { .. } => {
                                // Handle ...
                                todo!()
                            }
                            ClientCommand::RetryJob { .. } => {
                                // Handle ...
                                todo!()
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse command: {}", e);
                    }
                }
            },
            message = rx_result.recv() => {
                println!("Received a new job response: {:?}", message);
                if let Some(msg) = message {
                    socket.writable().await?;
                    let buffer: Vec<u8> = bincode::serialize(&msg)?;
                    match socket.try_write(&buffer) {
                        Ok(n) => {
                            println!("Write new content to buffer, with length: {n}");
                            continue
                        },
                        Err(e) => println!("Failed to write message to buffer, with error: {e}"),
                    }
                }
            }
        }
    }

    Ok(())
}
