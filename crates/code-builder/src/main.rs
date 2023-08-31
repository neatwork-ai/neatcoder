mod prelude;

use anyhow::{anyhow, Result};
use bincode;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use serde_json;
use std::{
    env,
    io::{self, Cursor},
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use gluon::ai::openai::{
    client::OpenAI, model::OpenAIModels, params::OpenAIParams,
};

use code_builder::models::{
    messages::{
        inner::{ManagerRequest, WorkerResponse},
        outer::{ClientMsg, ServerMsg},
    },
    shutdown::ShutdownSignal,
    worker::JobWorker,
};

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder().format_timestamp_millis().init();

    info!("Reading OpenAI API key");
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        anyhow!("OPENAI_API_KEY environment variable not found")
    })?;

    let open_ai_client = Arc::new(OpenAI::new(api_key));

    let ai_job = Arc::new(
        OpenAIParams::empty(OpenAIModels::Gpt35Turbo)
            .temperature(0.7)
            .top_p(0.9)?,
    );

    let listener_address = "127.0.0.1:1895"; // TODO: conf
    let listener = TcpListener::bind(listener_address).await?;
    info!("TPC server listening on {:?}", listener.local_addr());

    let (mut socket, _socket_addr) = listener.accept().await?;
    debug!("Client bound to TCP Socket");

    // TODO: Need to control the maximum buffer size
    let mut buf = vec![0u8; 1024];

    // Channels for sending and receiving the jobs
    let (tx_request, rx_request) = tokio::sync::mpsc::channel(100);

    // Channels for sending and receiving the results
    let (tx_response, mut rx_response) = tokio::sync::mpsc::channel(100);

    let shutdown = ShutdownSignal::new();

    let _join_handle = JobWorker::spawn(
        open_ai_client,
        ai_job,
        rx_request,
        tx_response,
        shutdown,
    );

    // Defines the buffer length for the message delimiter
    // In TCP, data can be streamed continuously without clear message boundaries
    // Therefore at the beginning of each message the client will send a delimiter
    // followed by a message length prefix
    const DELIMITER: &[u8] = b"MSG:"; // Define a suitable delimiter
    let mut length_buf = [0u8; 4]; // 4-byte buffer for length prefix

    'tpc: loop {
        tokio::select! {
            _ = socket.readable() => {
                // TODO: we only need last `DELIMITER.len()` bytes
                let mut temp_buf = Vec::new();

                // Search for the delimiter
                while temp_buf.ends_with(DELIMITER) == false {
                    let mut byte = [0u8; 1];

                    match socket.try_read(&mut byte) {
                        // socket.readable was a false positive
                        Ok(0) => continue 'tpc,
                        // again, socket.readable was a false positive
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue 'tpc;
                        }
                        Err(e) => {
                            error!("Got an error while searching for delimiter: {e}");
                            break 'tpc;
                        },
                        Ok(_) => {
                            // remember the byte
                            temp_buf.extend_from_slice(&byte);
                        },
                    }
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
                debug!("Json: {}", message_str);

                match serde_json::from_str::<ClientMsg>(&message_str) {
                    Ok(command) => {
                        match command {
                            ClientMsg::InitState { state } => {
                                tx_request.send(ManagerRequest::InitState { state }).await?;
                            }
                            ClientMsg::InitPrompt { prompt } => {
                                tx_request.send(ManagerRequest::ScaffoldProject { prompt }).await?;

                                // TODO: Technically the execution should be sequential and
                                // not asynchronous
                                tx_request.send(ManagerRequest::BuildExecutionPlan {}).await?;
                            }
                            ClientMsg::AddSchema { interface_name, schema_name, schema } => {
                                tx_request.send(ManagerRequest::AddSchema { interface_name, schema_name, schema}).await?;
                            }
                            ClientMsg::RemoveSchema { interface_name, schema_name } => {
                                tx_request.send(ManagerRequest::RemoveSchema { interface_name, schema_name }).await?;
                            }
                            ClientMsg::AddInterface { interface } => {
                                tx_request.send(ManagerRequest::AddInterface { interface }).await?;
                            }
                            ClientMsg::RemoveInterface { interface_name } => {
                                tx_request.send(ManagerRequest::RemoveInterface { interface_name }).await?;
                            }
                            ClientMsg::AddSourceFile { filename, file } => {
                                tx_request.send(ManagerRequest::AddSourceFile { filename, file }).await?;
                            }
                            ClientMsg::RemoveSourceFile { filename } => {
                                tx_request.send(ManagerRequest::RemoveSourceFile { filename }).await?;
                            }
                            ClientMsg::UpdateScaffold { scaffold } => {
                                tx_request.send(ManagerRequest::UpdateScaffold { scaffold }).await?;
                            }
                            ClientMsg::StartJob { .. } => {
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
            message = rx_response.recv() => {
                debug!("Received a new job response: {:?}", message);
                if let Some(msg) = message {
                    match msg {
                        WorkerResponse::InitState => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::Scaffold { scaffold: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::BuildExecutionPlan { jobs } => {
                            println!("[INFO] Sending `UpdateJobQueue` to client");
                            let server_msg = ServerMsg::UpdateJobQueue { jobs };
                            tcp_write(&mut socket, server_msg).await?;

                            println!("[INFO] Sent `UpdateJobQueue` to client");

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
                        WorkerResponse::UpdateScaffold => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::AddSourceFile { filename: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::RemoveSourceFile { filename: _ } => {
                            // TODO: Consider adding acknowledge command
                        }
                        WorkerResponse::CodeGen { stream } => {

                            let begin_msg = ServerMsg::BeginStream { filename: stream.filename.clone() };
                            tcp_write(&mut socket, begin_msg).await?;

                            stream.stream_rust(&mut socket).await?;

                            let end_msg = ServerMsg::EndStream {};
                            tcp_write(&mut socket, end_msg).await?;
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

pub async fn tcp_write(socket: &mut TcpStream, msg: ServerMsg) -> Result<()> {
    socket.writable().await?;

    // Define the delimiter
    const DELIMITER: &[u8] = b"MSG:";

    // Serialize the message using serde
    let msg_string = serde_json::to_string(&msg)?;
    let msg_buffer = msg_string.as_bytes().to_vec();

    // Calculate the length and convert it to a buffer of 4 bytes
    let mut length_buffer = [0u8; 4];
    BigEndian::write_u32(&mut length_buffer, msg_buffer.len() as u32);

    // Combine DELIMITER, length_buffer, and message_buffer
    let mut complete_buffer = Vec::new();
    complete_buffer.extend_from_slice(DELIMITER);
    complete_buffer.extend_from_slice(&length_buffer);
    complete_buffer.extend_from_slice(&msg_buffer);

    // Write the complete buffer to the socket
    socket.write_all(&complete_buffer).await?;

    Ok(())
}
