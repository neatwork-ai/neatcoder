use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::{http::get_api, models::assistant::get_data};

use super::ThreadID;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub created_at: i64,
    pub id: String,
    pub object: String,
    pub thread_id: String,
    pub role: String,
    pub content: Vec<ContentBlock>,
    pub file_ids: Vec<String>, // TODO: Figure out what best type to use --> [file.id]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String, // e.g., "text"
    pub text: Option<TextBlock>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextBlock {
    pub value: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)] // This allows for the proper variant to be inferred during deserialization
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitation(FileCitation),

    #[serde(rename = "file_path")]
    FilePath(FilePath),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileCitation {
    pub file_id: String,
    pub quote: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilePath {
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages(Vec<Message>);

impl Deref for Messages {
    type Target = [Message];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Messages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Vec<Message>> for Messages {
    fn as_ref(&self) -> &Vec<Message> {
        &self.0
    }
}

impl AsRef<[Message]> for Messages {
    fn as_ref(&self) -> &[Message] {
        &self.0
    }
}

impl Messages {
    pub async fn get_messages(
        client: &Client,
        headers: &HeaderMap,
        thread_id: &ThreadID,
    ) -> Result<Messages> {
        let response_body = get_api(
            client,
            headers,
            &format!("threads/{}/messages", thread_id),
            None,
        )
        .await?;

        let msgs = get_data(&response_body)?;

        let messages: Messages = serde_json::from_value(msgs.clone())?;

        println!("List of Messages: {:?}", messages);

        Ok(messages)
    }
}
