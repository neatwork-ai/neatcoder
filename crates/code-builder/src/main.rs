use anyhow::{anyhow, Result};
use bincode;
use dotenv::dotenv;
use serde_json;
use std::{env, sync::Arc};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use gluon::ai::openai::{client::OpenAI, model::OpenAIModels, params::OpenAIParams};

use code_builder::models::{
    messages::{
        inner::{ManagerRequest, WorkerResponse},
        outer::{ClientMsg, ServerMsg},
    },
    shutdown::ShutdownSignal,
    worker::JobWorker,
};

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
    // TODO: Need to control the maximum buffer size
    let mut buf = vec![0u8; 1024];

    println!("Client binded to TCP Socket");

    // Channels for sending and receiving the results
    let (tx_result, mut rx_result) = tokio::sync::mpsc::channel(100);

    // Channels for sending and receiving the jobs
    let (tx_job, rx_job) = tokio::sync::mpsc::channel(100);

    let shutdown = ShutdownSignal::new();

    let _join_handle = JobWorker::spawn(open_ai_client, ai_job, rx_job, tx_result, shutdown);

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


                match serde_json::from_str::<ClientMsg>(&message_str) {
                    Ok(command) => {
                        match command {
                            ClientMsg::InitPrompt { prompt } => {
                                tx_job.send(ManagerRequest::ScaffoldProject { prompt }).await?;

                                // TODO: Technically the exectution should be sequential and
                                // not asynchronous
                                tx_job.send(ManagerRequest::BuildExecutionPlan {}).await?;
                            }
                            ClientMsg::AddSchema { interface_name, schema_name, schema } => {
                                tx_job.send(ManagerRequest::AddSchema { interface_name, schema_name, schema}).await?;
                            }
                            ClientMsg::RemoveSchema { interface_name, schema_name } => {
                                tx_job.send(ManagerRequest::RemoveSchema { interface_name, schema_name }).await?;
                            }
                            ClientMsg::AddInterface { interface } => {
                                tx_job.send(ManagerRequest::AddInterface { interface }).await?;
                            }
                            ClientMsg::RemoveInterface { interface_name } => {
                                tx_job.send(ManagerRequest::RemoveInterface { interface_name }).await?;
                            }
                            ClientMsg::StartJob { .. } => {
                                // Handle ...
                                todo!()
                            }
                            ClientMsg::StopJob { .. } => {
                                // Handle ...
                                todo!()
                            }
                            ClientMsg::RetryJob { .. } => {
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
                    match msg {
                        WorkerResponse::Scaffold { scaffold: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::BuildExecutionPlan { jobs } => {
                            let server_msg = ServerMsg::UpdateJobQueue { jobs };
                            tcp_write(&socket, server_msg).await?;

                        }
                        WorkerResponse::AddSchema { schema_name: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::RemoveSchema { schema_name: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::AddInterface { interface_name: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::RemoveInterface { interface_name: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::CodeGen { stream } => {

                            let begin_msg = ServerMsg::BeginStream { filename: stream.filename.clone() };
                            tcp_write(&socket, begin_msg).await?;

                            stream.stream_rust(&mut socket).await?;

                            let end_msg = ServerMsg::EndStream {};
                            tcp_write(&socket, end_msg).await?;
                        }
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

pub async fn tcp_write(socket: &TcpStream, msg: ServerMsg) -> Result<()> {
    socket.writable().await?;
    let buffer: Vec<u8> = bincode::serialize(&msg)?;
    match socket.try_write(&buffer) {
        Ok(n) => println!("Write new content to buffer, with length: {n}"),
        Err(e) => println!("Failed to write message to buffer, with error: {e}"),
    }

    Ok(())
}
