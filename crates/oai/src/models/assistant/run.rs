use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use serde_json::json;
use std::{collections::HashMap, fmt, time::Duration};
use tokio::time::sleep;

use crate::http::{get_api, post_api};

use super::{assistant::Tool, get_data, AssistantID, OpenAIModels, ThreadID};

#[derive(Deserialize, Debug)]
pub struct Run {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub assistant_id: String,
    pub thread_id: String,
    pub status: RunStatus,
    pub started_at: Option<u32>,
    pub expires_at: Option<u32>,
    pub cancelled_at: Option<u32>,
    pub failed_at: Option<u32>,
    pub completed_at: Option<u32>,
    pub last_error: Option<RunErr>,
    pub model: OpenAIModels,
    pub instructions: Option<String>,
    pub tools: Vec<Tool>,
    pub file_ids: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Run {
    pub async fn create_run(
        client: &Client,
        headers: &HeaderMap,
        thread_id: &ThreadID,
        assistant_id: &AssistantID,
    ) -> Result<Run> {
        let payload = json!({
            "assistant_id": assistant_id,
        });

        let response_body = post_api(
            client,
            headers,
            &format!("threads/{}/runs", thread_id),
            &payload,
        )
        .await?;

        let run: Run = serde_json::from_value(response_body)?;

        println!("Run: {:?}", run);

        Ok(run)
    }
}

impl Run {
    pub async fn run(
        client: &Client,
        headers: &HeaderMap,
        thread_id: &ThreadID,
        assistant_id: &AssistantID,
    ) -> Result<Run> {
        let mut run =
            Run::create_run(client, headers, thread_id, assistant_id).await?;

        // Poll the run status until it is completed or failed
        while !run.is_completed() && !run.is_failed() {
            sleep(Duration::from_millis(500)).await;

            println!("Checking in if Run is completed");
            run = run.get_run(client, headers, &run.id).await?;
        }

        println!("Run has been completed.");

        Ok(run)
    }

    async fn get_run(
        &self,
        client: &Client,
        headers: &HeaderMap,
        run_id: &str,
    ) -> Result<Run> {
        let response_body = get_api(
            client,
            headers,
            &format!("threads/{}/runs/{}", self.thread_id, run_id),
            None,
        )
        .await?;

        serde_json::from_value(response_body).map_err(Into::into)
    }

    /// curl https://api.openai.com/v1/threads/thread_nrrMnuRNH45pTs00DEymPqol/runs \
    /// -H 'Authorization: Bearer <API_KEY>' \
    /// -H 'Content-Type: application/json' \
    /// -H 'OpenAI-Beta: assistants=v1'
    pub async fn list_runs(
        client: &Client,
        headers: &HeaderMap,
        thread_id: &ThreadID,
    ) -> Result<Vec<Run>> {
        let response_body = get_api(
            client,
            headers,
            &format!("threads/{}/runs", thread_id),
            None,
        )
        .await?;

        let runs = get_data(&response_body)?;

        serde_json::from_value(runs.clone()).map_err(Into::into)
    }
}

impl Run {
    fn is_completed(&self) -> bool {
        self.status == RunStatus::Completed
    }

    fn is_failed(&self) -> bool {
        self.status == RunStatus::Failed
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunStatus {
    Queued,
    InProgress,
    RequiresAction,
    Cancelling,
    Failed,
    Completed,
    Expired,
}

impl<'de> Deserialize<'de> for RunStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunStatusVisitor;

        impl<'de> Visitor<'de> for RunStatusVisitor {
            type Value = RunStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an RunStatus")
            }

            fn visit_str<E>(self, value: &str) -> Result<RunStatus, E>
            where
                E: de::Error,
            {
                match value {
                    "queued" => Ok(RunStatus::Queued),
                    "in_progress" => Ok(RunStatus::InProgress),
                    "requires_action" => Ok(RunStatus::RequiresAction),
                    "cancelling" => Ok(RunStatus::Cancelling),
                    "failed" => Ok(RunStatus::Failed),
                    "completed" => Ok(RunStatus::Completed),
                    "expired" => Ok(RunStatus::Expired),
                    _ => Err(E::custom(format!(
                        "unexpected RunStatus value: {}",
                        value
                    ))),
                }
            }
        }

        deserializer.deserialize_str(RunStatusVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct RunErr {
    pub code: RunErrStatus,
    pub message: String,
}

#[derive(Debug)]
pub enum RunErrStatus {
    ServerError,
    RateLimitExceeded,
}

impl<'de> Deserialize<'de> for RunErrStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RunErrStatusVisitor;

        impl<'de> Visitor<'de> for RunErrStatusVisitor {
            type Value = RunErrStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing an RunErr status")
            }

            fn visit_str<E>(self, value: &str) -> Result<RunErrStatus, E>
            where
                E: de::Error,
            {
                match value {
                    "server_error" => Ok(RunErrStatus::ServerError),
                    "rate_limit_exceeded" => {
                        Ok(RunErrStatus::RateLimitExceeded)
                    }
                    _ => Err(E::custom(format!(
                        "unexpected RunErr status: {}",
                        value
                    ))),
                }
            }
        }

        deserializer.deserialize_str(RunErrStatusVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() -> Result<()> {
        let json_data = json!({
            "assistant_id": "asst_QZG3jl4N01Enuh8KacwSjmSQ",
            "cancelled_at": null,
            "completed_at": null,
            "created_at": 1699460132,
            "expires_at": 1699460732,
            "failed_at": null,
            "file_ids": [],
            "id": "run_yQgBbVzMz8nMFaSiIR9pjyx3",
            "instructions": "You are a personal math tutor. Write and run code to answer math questions.",
            "last_error": null,
            "metadata": {},
            "model": "gpt-4-1106-preview",
            "object": "thread.run",
            "started_at": null,
            "status": "queued",
            "thread_id": "thread_D6wrSRmIE1k1MyEe4S7fdFHs",
            "tools": [{"type": "code_interpreter"}]
        })
        .to_string();

        let run: Run = serde_json::from_str(&json_data)?;

        assert_eq!(run.id, "run_yQgBbVzMz8nMFaSiIR9pjyx3");
        assert_eq!(run.assistant_id, "asst_QZG3jl4N01Enuh8KacwSjmSQ");
        assert!(matches!(run.status, RunStatus::Queued));
        assert_eq!(run.tools.len(), 1);
        assert!(matches!(run.tools[0], Tool::CodeInterpreter));

        Ok(())
    }
}
