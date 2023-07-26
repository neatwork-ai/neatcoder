use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::{json, Value};

use super::{job::OpenAIJob, msg::OpenAIMsg, output::Body};

// Example of curl requests:

// Fill Mask Task:
// curl https://api-inference.huggingface.co/models/bert-base-uncased \
//         -X POST \
//         -d '{"inputs":"The answer to the universe is [MASK]."}' \
//         -H "Authorization: Bearer ${HF_API_TOKEN}"
// # [{"sequence":"the answer to the universe is no.","score":0.16963955760002136,"token":2053,"token_str":"no"},{"sequence":"the answer to the universe is nothing.","score":0.07344776391983032,"token":2498,"token_str":"nothing"},{"sequence":"the answer to the universe is yes.","score":0.05803241208195686,"token":2748,"token_str":"yes"},{"sequence":"the answer to the universe is unknown.","score":0.043957844376564026,"token":4242,"token_str":"unknown"},{"sequence":"the answer to the universe is simple.","score":0.04015745222568512,"token":3722,"token_str":"simple"}]

// Summarization Task
// curl https://api-inference.huggingface.co/models/deepset/roberta-base-squad2 \
//         -X POST \
//         -d '{"inputs":{"question":"What is my name?","context":"My name is Clara and I live in Berkeley."}}' \
//         -H "Authorization: Bearer ${HF_API_TOKEN}"
// # {"score":0.933128833770752,"start":11,"end":16,"answer":"Clara"}

// Q&A Task
// curl https://api-inference.huggingface.co/models/deepset/roberta-base-squad2 \
//         -X POST \
//         -d '{"inputs":{"question":"What is my name?","context":"My name is Clara and I live in Berkeley."}}' \
//         -H "Authorization: Bearer ${HF_API_TOKEN}"
// # {"score":0.933128833770752,"start":11,"end":16,"answer":"Clara"}

// Table Question Answering task
// curl https://api-inference.huggingface.co/models/google/tapas-base-finetuned-wtq \
//         -X POST \
//         -d '{"inputs":{"query":"How many stars does the transformers repository have?","table":{"Repository":["Transformers","Datasets","Tokenizers"],"Stars":["36542","4512","3934"],"Contributors":["651","77","34"],"Programming language":["Python","Python","Rust, Python and NodeJS"]}}}' \
//         -H "Authorization: Bearer ${HF_API_TOKEN}"
// # {"answer":"AVERAGE > 36542","coordinates":[[0,1]],"cells":["36542"],"aggregator":"AVERAGE"}

// Sentence Similarity Task
// curl https://api-inference.huggingface.co/models/sentence-transformers/all-MiniLM-L6-v2 \
//         -X POST \
//         -d '{"inputs":{"source_sentence": "That is a happy person", "sentences": ["That is a happy dog","That is a very happy person","Today is a sunny day"]}}' \
//         -H "Authorization: Bearer ${HF_API_TOKEN}"
// # [0.6945773363113403,0.9429150819778442,0.2568760812282562]

// Text Classification Task
// curl https://api-inference.huggingface.co/models/distilbert-base-uncased-finetuned-sst-2-english \
//         -X POST \
//         -d '{"inputs":"I like you. I love you"}' \
//         -H "Authorization: Bearer ${HF_API_TOKEN}"
// # [[{"label":"POSITIVE","score":0.9998738765716553},{"label":"NEGATIVE","score":0.0001261125144083053}]]

pub struct HuggingFace {
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
