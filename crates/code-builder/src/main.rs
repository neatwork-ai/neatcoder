use anyhow::Result;
use serde_json;
use std::{env, sync::Arc};
use tokio::io::AsyncWriteExt;
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

    let client = Arc::new(OpenAI::new(env::var("OPENAI_API_KEY")?));

    let ai_job = Arc::new(
        OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
            .temperature(0.7)
            .top_p(0.9)?,
    );

    let app_state = Arc::new(Mutex::new(AppState::empty()));

    loop {
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let message_str = String::from_utf8_lossy(&buf[..n]);

        match serde_json::from_str::<ClientCommand>(&message_str) {
            Ok(command) => {
                match command {
                    ClientCommand::InitWork { prompt } => {
                        let job_queue = endpoints::init_work::handle(
                            client.clone(),
                            ai_job.clone(),
                            app_state.clone(),
                            prompt,
                        )
                        .await?;

                        // Serialize job_queue to JSON
                        let response = serde_json::to_string(&job_queue)?;

                        // Send the serialized job_queue to the client
                        socket.write_all(response.as_bytes()).await?;
                    }
                    ClientCommand::AddSchema { schema } => {
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
