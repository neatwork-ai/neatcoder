use anyhow::{anyhow, Result};
use bincode;
use code_builder::models::job_worker::JobWorker;
use code_builder::models::shutdown::ShutdownSignal;
use code_builder::models::types::JobRequest;
use dotenv::dotenv;
use serde_json;
use std::{env, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpListener};

use code_builder::models::ClientCommand;
use gluon::ai::openai::{client::OpenAI, model::OpenAIModels, params::OpenAIParams};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| anyhow!("OPENAI_API_KEY environment variable not found"))?;

    let open_ai_client = Arc::new(OpenAI::new(api_key));

    let ai_job = Arc::new(
        OpenAIParams::empty(OpenAIModels::Gpt35Turbo)
            .temperature(0.7)
            .top_p(0.9)?,
    );

    let listener_address = "127.0.0.1:1895";
    let listener = TcpListener::bind(listener_address).await?; // Binding to localhost on port 7878
    println!("Listening on {:?}", listener.local_addr());

    let (mut socket, _socket_addr) = listener.accept().await?;
    let mut buf = vec![0u8; 1024];

    println!("Client binded to TCP Socket");

    let (tx_result, mut rx_result) = tokio::sync::mpsc::channel(100);
    let (tx_job, rx_job) = tokio::sync::mpsc::channel(100);

    let shutdown = ShutdownSignal::new();

    let _join_handle = JobWorker::spawn(
        open_ai_client,
        ai_job,
        rx_job,
        tx_result,
        String::from(listener_address),
        shutdown,
    );
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

                println!("[DEBUG] {}", message_str);

                match serde_json::from_str::<ClientCommand>(&message_str) {
                    Ok(command) => {
                        match command {
                            ClientCommand::InitPrompt { prompt } => {
                                tx_job.send(JobRequest::InitPrompt { prompt }).await?;
                            }
                            ClientCommand::AddInterfaceFile { interface, schema } => {
                                tx_job.send(JobRequest::AddInterfaceFile { interface, schema }).await?;
                            }
                            ClientCommand::RemoveInterface { path, schema } => {
                                // Handle ...
                                todo!()
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
                        eprintln!("Unknown command received: {}", e);
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
                else {
                    println!("Channel closed. Shutting down...");
                break;
                }
            }
        }
    }

    Ok(())
}
