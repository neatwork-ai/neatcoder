use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;
use std::{collections::HashMap, env, fmt};

use self::{
    assistant::{Assistant, Tool},
    message::{Message, Messages},
    run::Run,
    thread::Thread,
};
use serde::Serializer;

pub mod assistant;
pub mod file;
pub mod message;
pub mod run;
pub mod thread;

#[derive(Debug, Default)]
pub struct CustomGPT {
    pub(crate) client: Client,
    pub(crate) headers: HeaderMap,
    pub assistants: HashMap<AssistantID, Assistant>,
    pub threads: HashMap<ThreadID, Thread>,
}

pub type AssistantID = String;
pub type ThreadID = String;
pub type FileID = String;

impl CustomGPT {
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");

        let client = Client::new();
        let headers = get_auth_headers(&api_key)?;

        Ok(Self {
            client,
            headers,
            ..Default::default()
        })
    }

    pub async fn create_assistant(
        &mut self,
        name: &str,
        instructions: &str,
        tools: Vec<Tool>,
    ) -> Result<AssistantID> {
        let assistant = Assistant::create_assistant(
            &self.client,
            &self.headers,
            name.to_string(),
            instructions.to_string(),
            tools,
            OpenAIModels::Gpt41106Preview,
        )
        .await?;

        let assistant_id = assistant.id.clone();

        self.assistants.insert(assistant_id.clone(), assistant);

        Ok(assistant_id)
    }

    pub async fn create_thread(&mut self) -> Result<ThreadID> {
        let thread = Thread::create_thread(&self.client, &self.headers).await?;
        let thread_id = thread.inner.id.clone();

        self.threads.insert(thread_id.clone(), thread);

        Ok(thread_id)
    }

    pub async fn add_message_to_thread(
        &self,
        thread_id: &ThreadID,
        content: String,
    ) -> Result<Message> {
        let thread = self
            .threads
            .get(thread_id)
            .ok_or_else(|| anyhow!(format!("No thread found for ID: {:?}", thread_id)))?;

        let message =
            Thread::add_message_to_thread(&thread, &self.client, &self.headers, content).await?;

        Ok(message)
    }

    pub async fn run(&self, thread_id: &ThreadID, assistant_id: &AssistantID) -> Result<Run> {
        let run = Run::run(&self.client, &self.headers, thread_id, assistant_id).await?;

        Ok(run)
    }

    pub async fn get_messages(&self, thread_id: &ThreadID) -> Result<Messages> {
        let messages = Messages::get_messages(&self.client, &self.headers, thread_id).await?;

        Ok(messages)
    }
}

// Updated get_auth_headers function to accept the API key as a parameter
fn get_auth_headers(api_key: &str) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let auth_value = format!("Bearer {}", api_key);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);
    headers.insert("OpenAI-Beta", HeaderValue::from_static("assistants=v1"));
    Ok(headers)
}

#[derive(Debug, Clone, Copy)]
pub enum OpenAIModels {
    Gpt432k,
    Gpt4,
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo1106,
    Gpt41106Preview,
}

impl Default for OpenAIModels {
    fn default() -> Self {
        OpenAIModels::Gpt35Turbo16k
    }
}

impl OpenAIModels {
    pub fn new(model: String) -> Self {
        let model = match model.as_str() {
            "gpt-4-32k" => OpenAIModels::Gpt432k,
            "gpt-4" => OpenAIModels::Gpt4,
            "gpt-3.5-turbo" => OpenAIModels::Gpt35Turbo,
            "gpt-3.5-turbo-16k" => OpenAIModels::Gpt35Turbo16k,
            "gpt-3.5-turbo-1106" => OpenAIModels::Gpt35Turbo1106,
            "gpt-4-1106-preview" => OpenAIModels::Gpt41106Preview,
            _ => panic!("Invalid model {}", model),
        };

        model
    }

    pub fn as_string(&self) -> String {
        match self {
            OpenAIModels::Gpt432k => String::from("gpt-4-32k"),
            OpenAIModels::Gpt4 => String::from("gpt-4"),
            OpenAIModels::Gpt35Turbo => String::from("gpt-3.5-turbo"),
            OpenAIModels::Gpt35Turbo16k => String::from("gpt-3.5-turbo-16k"),
            OpenAIModels::Gpt35Turbo1106 => String::from("gpt-3.5-turbo-1106"),
            OpenAIModels::Gpt41106Preview => String::from("gpt-4-1106-preview"),
        }
    }
}

impl Serialize for OpenAIModels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let model_str = self.as_string();
        serializer.serialize_str(&model_str)
    }
}

impl<'de> Deserialize<'de> for OpenAIModels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OpenAIModelsVisitor;

        impl<'de> Visitor<'de> for OpenAIModelsVisitor {
            type Value = OpenAIModels;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an OpenAI model")
            }

            fn visit_str<E>(self, value: &str) -> Result<OpenAIModels, E>
            where
                E: de::Error,
            {
                match value {
                    "gpt-4-32k" => Ok(OpenAIModels::Gpt432k),
                    "gpt-4" => Ok(OpenAIModels::Gpt4),
                    "gpt-3.5-turbo" => Ok(OpenAIModels::Gpt35Turbo),
                    "gpt-3.5-turbo-16k" => Ok(OpenAIModels::Gpt35Turbo16k),
                    "gpt-3.5-turbo-1106" => Ok(OpenAIModels::Gpt35Turbo1106),
                    "gpt-4-1106-preview" => Ok(OpenAIModels::Gpt41106Preview),
                    _ => Err(E::custom(format!("unexpected OpenAI model: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(OpenAIModelsVisitor)
    }
}

pub fn get_data(response_body: &Value) -> Result<&Value> {
    let data = response_body.get("data").ok_or_else(|| {
        anyhow!(format!(
            "Unable to find field \"data\" in response_body: {}",
            response_body
        ))
    })?;

    Ok(data)
}
