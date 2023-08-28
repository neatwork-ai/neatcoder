use anyhow::Result;
use futures::StreamExt;
use gluon::ai::openai::{client::OpenAI, msg::OpenAIMsg, params::OpenAIParams};
use serde::Serialize;
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug, Serialize)]
pub struct CodeStream {
    pub filename: String,
    #[serde(skip_serializing)]
    client: Arc<OpenAI>,
    #[serde(skip_serializing)]
    params: Arc<OpenAIParams>,
    prompts: Vec<OpenAIMsg>,
}

impl CodeStream {
    pub fn new(
        filename: String,
        client: Arc<OpenAI>,
        params: Arc<OpenAIParams>,
        prompts: Vec<OpenAIMsg>,
    ) -> Self {
        Self {
            filename,
            client,
            params,
            prompts,
        }
    }

    pub async fn stream_rust(&self, socket: &mut TcpStream) -> Result<()> {
        println!("[INFO] Initiating Stream");

        let prompts = self.prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

        let mut chat_stream = self
            .client
            .chat_stream(&self.params, &prompts, &[], &[])
            .await?;

        let mut start_delimiter = false;
        while let Some(item) = chat_stream.next().await {
            match item {
                Ok(bytes) => {
                    let token =
                        std::str::from_utf8(&bytes).expect("Failed to generate utf8 from bytes");
                    if !start_delimiter && ["```rust", "```"].contains(&token) {
                        start_delimiter = true;
                        continue;
                    } else if !start_delimiter {
                        continue;
                    } else {
                        if token == "```" {
                            break;
                        }
                        socket.writable().await?;
                        if let Err(e) = socket.write_all(bytes.as_ref()).await {
                            eprintln!("Failed to write bytes to tcp stream, with error: {e}");
                        }
                    }
                }
                Err(e) => eprintln!("Failed to receive token, with error: {e}"),
            }
        }
        Ok(())
    }
}
