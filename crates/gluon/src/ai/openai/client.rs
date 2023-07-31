use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::{json, Value};

use super::{job::OpenAIJob, msg::OpenAIMsg, output::Body};

pub struct OpenAI {
    api_key: Option<String>,
}

impl OpenAI {
    pub fn empty() -> Self {
        Self { api_key: None }
    }

    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Some(api_key),
        }
    }
    // === Setter methods with chaining ===

    pub fn api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }

    // === OpenAI IO ===

    pub async fn chat(
        &self,
        job: &OpenAIJob,
        msgs: &[&OpenAIMsg],
        funcs: &[&String],
        stop_seq: &[String],
    ) -> Result<Body> {
        let client = Client::new();

        // fill in your own data as needed
        let req_body = self.request_body(job, msgs, funcs, stop_seq)?;

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.api_key.as_ref().expect("No API Keys provided")
                ),
            )
            .header("Content-Type", "application/json")
            .json(&req_body)
            .send()
            .await?;

        let status = res.status();

        if !status.is_success() {
            return Err(anyhow!("Failed with status: {}", status));
        }

        let body = res.text().await?;
        let api_response = serde_json::from_str(body.as_str())?;

        Ok(api_response)
    }

    fn request_body(
        &self,
        job: &OpenAIJob,
        msgs: &[&OpenAIMsg],
        // TODO: Add to OpenAIJob
        funcs: &[&String],
        // TODO: Add to OpenAIJob
        stop_seq: &[String],
    ) -> Result<Value> {
        let mut data = json!({
            "model": job.model.as_str(),
            "messages": msgs,
            // "stop": self.stop,
        });

        if let Some(temperature) = job.temperature {
            data["temperature"] = serde_json::to_value(temperature)?;
        }

        if let Some(top_p) = job.top_p {
            data["top_p"] = serde_json::to_value(top_p)?;
        }

        if !funcs.is_empty() {
            data["functions"] = serde_json::to_value(funcs)?;
        }

        if !stop_seq.is_empty() {
            if stop_seq.len() > 4 {
                return Err(anyhow!(
                    "Maximum limit of stop sequences reached. {} out of 4",
                    stop_seq.len()
                ));
            };

            data["stop"] = serde_json::to_value(stop_seq)?;
        }

        if let Some(user) = &job.user {
            data["user"] = serde_json::to_value(user)?;
        }

        if !job.logit_bias.is_empty() {
            data["logit_bias"] = serde_json::to_value(&job.logit_bias)?;
        }

        if let Some(frequency_penalty) = &job.frequency_penalty {
            data["frequency_penalty"] = serde_json::to_value(frequency_penalty)?;
        }

        if let Some(presence_penalty) = &job.presence_penalty {
            data["presence_penalty"] = serde_json::to_value(presence_penalty)?;
        }

        if let Some(max_tokens) = &job.max_tokens {
            data["max_tokens"] = serde_json::to_value(max_tokens)?;
        }

        if let Some(n) = &job.n {
            data["n"] = serde_json::to_value(n)?;
        }

        Ok(data)
    }
}
