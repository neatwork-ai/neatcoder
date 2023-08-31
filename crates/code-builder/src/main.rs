mod conf;
mod prelude;
mod tcp_socket;

use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

use gluon::ai::openai::{
    client::OpenAI, model::OpenAIModels, params::OpenAIParams,
};

use code_builder::models::{
    messages::{
        inner::{ManagerRequest, WorkerResponse},
        outer::{ClientMsg, ServerMsg},
    },
    worker::JobWorker,
};

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder().format_timestamp_millis().init();

    info!("Building configuration");
    let conf = conf::Conf::from_env()?;

    let open_ai_client = Arc::new(OpenAI::new(conf.openai_api_key.clone()));

    let ai_job = Arc::new(
        OpenAIParams::empty(OpenAIModels::Gpt35Turbo)
            .temperature(conf.llm_temperature)
            .top_p(conf.llm_top_p)?,
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

    let _join_handle =
        JobWorker::spawn(open_ai_client, ai_job, rx_request, tx_response);

    // Defines the buffer length for the message delimiter
    // In TCP, data can be streamed continuously without clear message
    // boundaries Therefore at the beginning of each message the client will
    // send a delimiter followed by a message length prefix
    let mut length_buf = [0u8; 4]; // 4-byte buffer for length prefix

    loop {
        tokio::select! {
            _ = socket.readable() => {
                let body = match tcp_socket::try_read(&mut socket, &mut buf, &mut length_buf).await {
                    Ok(Some(body)) => body,
                    Ok(None) => continue,
                    Err(e) => {
                        error!("Unrecoverable error while reading from TCP socket: {e}");
                        break
                    }
                };

                debug!("Incoming TPC: {body:?}");
                route_request(&tx_request, body).await?; // TODO: handle error
            },
            next_tcp_response = rx_response.recv() => {
                let Some(resp) = next_tcp_response else {
                    error!("TCP Server recv channel closed");
                    // TBD: restart the worker or exit and restart the binary?
                    break
                };

                debug!("Received a new job response: {resp:?}");
                route_response(&mut socket, resp).await?; // TODO: handle error
            }
        }
    }

    Ok(())
}

async fn route_request(
    tx_request: &tokio::sync::mpsc::Sender<ManagerRequest>,
    body: ClientMsg,
) -> Result<()> {
    match body {
        ClientMsg::InitState { state } => {
            tx_request.send(ManagerRequest::InitState { state }).await?;
        }
        ClientMsg::InitPrompt { prompt } => {
            tx_request
                .send(ManagerRequest::ScaffoldProject { prompt })
                .await?;

            // TODO: Technically the execution should be sequential and
            // not asynchronous
            tx_request
                .send(ManagerRequest::BuildExecutionPlan {})
                .await?;
        }
        ClientMsg::AddSchema {
            interface_name,
            schema_name,
            schema,
        } => {
            tx_request
                .send(ManagerRequest::AddSchema {
                    interface_name,
                    schema_name,
                    schema,
                })
                .await?;
        }
        ClientMsg::RemoveSchema {
            interface_name,
            schema_name,
        } => {
            tx_request
                .send(ManagerRequest::RemoveSchema {
                    interface_name,
                    schema_name,
                })
                .await?;
        }
        ClientMsg::AddInterface { interface } => {
            tx_request
                .send(ManagerRequest::AddInterface { interface })
                .await?;
        }
        ClientMsg::RemoveInterface { interface_name } => {
            tx_request
                .send(ManagerRequest::RemoveInterface { interface_name })
                .await?;
        }
        ClientMsg::AddSourceFile { filename, file } => {
            tx_request
                .send(ManagerRequest::AddSourceFile { filename, file })
                .await?;
        }
        ClientMsg::RemoveSourceFile { filename } => {
            tx_request
                .send(ManagerRequest::RemoveSourceFile { filename })
                .await?;
        }
        ClientMsg::UpdateScaffold { scaffold } => {
            tx_request
                .send(ManagerRequest::UpdateScaffold { scaffold })
                .await?;
        }
        ClientMsg::StartJob { .. } => {
            // Handle ...
            todo!()
        }
    };

    Ok(())
}

async fn route_response(
    socket: &mut TcpStream,
    resp: WorkerResponse,
) -> Result<()> {
    match resp {
        WorkerResponse::InitState => {
            // TODO: Consider adding acknowledge command
        }
        WorkerResponse::Scaffold { scaffold: _ } => {
            // TODO: Consider adding acknowledge command
        }
        WorkerResponse::BuildExecutionPlan { jobs } => {
            println!("[INFO] Sending `UpdateJobQueue` to client");
            let server_msg = ServerMsg::UpdateJobQueue { jobs };
            tcp_socket::write(socket, server_msg).await?;

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
            let begin_msg = ServerMsg::BeginStream {
                filename: stream.filename.clone(),
            };
            tcp_socket::write(socket, begin_msg).await?;

            stream.stream_rust(socket).await?;

            let end_msg = ServerMsg::EndStream {};
            tcp_socket::write(socket, end_msg).await?;
        }
    };

    Ok(())
}
