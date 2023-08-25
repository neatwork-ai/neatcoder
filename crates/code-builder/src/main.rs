use anyhow::{anyhow, Result};
use bincode;
use code_builder::models::job_worker::JobWorker;
use code_builder::models::shutdown::ShutdownSignal;
use code_builder::models::types::JobRequest;
use dotenv::dotenv;
use serde_json;
use std::{env, sync::Arc};
use tokio::{io::AsyncReadExt, net::TcpListener};

use code_builder::models::messages::client::ClientCommand;
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

    // TODO: Need to control the maximum buffer size
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

    // Defines the buffer length for the message delimiter
    // In TCP, data can be streamed continuously without clear message boundaries
    // Therefore at the beginning of each message the client will send a delimiter
    // followed by a message length prefix
    const DELIMITER: &[u8] = b"MSG:"; // Define a suitable delimiter
    let mut length_buf = [0u8; 4]; // 4-byte buffer for length prefix

    loop {
        tokio::select! {
            _ = socket.readable() => {
                let mut temp_buf = Vec::new();

                // Search for the delimiter
                while temp_buf.ends_with(DELIMITER) == false {
                    let mut byte = [0u8; 1];
                    if socket.read_exact(&mut byte).await.is_err() || byte[0] == 0 {
                        break;
                    }
                    temp_buf.extend_from_slice(&byte);
                }

                // Once delimiter is found, read the LENGTH prefix
                socket.read_exact(&mut length_buf).await?;
                let message_length = u32::from_be_bytes(length_buf);

                // Ensure our buffer is big enough to hold the incoming message
                if buf.len() < message_length as usize {
                    buf.resize(message_length as usize, 0);
                }

                // Read the actual message
                let n = socket.read_exact(&mut buf[0..message_length as usize]).await?;
                if n == 0 {
                    break;
                }

                let message_str = String::from_utf8_lossy(&buf[..n]);
                println!("[DEBUG MSG] {}", message_str);


                match serde_json::from_str::<ClientCommand>(&message_str) {
                    Ok(command) => {
                        match command {
                            ClientCommand::InitPrompt { prompt } => {
                                tx_job.send(JobRequest::InitPrompt { prompt }).await?;
                            }
                            ClientCommand::AddSchema { interface_name, schema_name, schema } => {
                                tx_job.send(JobRequest::AddSchema { interface_name, schema_name, schema }).await?;
                            }
                            ClientCommand::AddInterface { interface } => {
                                // Handle ...
                                println!("[DEBUG MSG] WE'RE HERE");
                                tx_job.send(JobRequest::AddInterface { interface }).await?;
                            }
                            ClientCommand::RemoveInterface { interface_name } => {
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
