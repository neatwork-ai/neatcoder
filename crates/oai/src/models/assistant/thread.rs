use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::http::post_api;

use super::message::{Message, Messages};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Thread {
    pub inner: ThreadInner,
    pub messages: Vec<Messages>,
    pub msg_ids: HashMap<MessageID, MessageIndex>,
}

pub type MessageID = String;
pub type MessageIndex = u32;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ThreadInner {
    pub id: String,
    pub object: String,
    pub created_at: u32, // Use chrono::DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

impl Thread {
    pub async fn create_thread(
        client: &Client,
        headers: &HeaderMap,
    ) -> Result<Thread> {
        let payload = json!({});

        let response_body =
            post_api(client, headers, "threads", &payload).await?;
        let inner: ThreadInner = serde_json::from_value(response_body)?;

        println!("Thread: {:?}", inner);

        Ok(Thread {
            inner,
            ..Default::default()
        })
    }

    pub async fn add_message_to_thread(
        &self,
        client: &Client,
        headers: &HeaderMap,
        content: String,
    ) -> Result<Message> {
        let payload = json!({
            "role": "user",
            "content": content,
        });

        let response_body = post_api(
            client,
            headers,
            &format!("threads/{}/messages", self.inner.id),
            &payload,
        )
        .await?;

        let message: Message = serde_json::from_value(response_body)?;

        println!("Message: {:?}", message);

        Ok(message)
    }
}
