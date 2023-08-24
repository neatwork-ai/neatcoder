use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::interfaces::SchemaFile;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobRequest {
    InitPrompt {
        prompt: String,
    },
    AddSchema {
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    },
    AddJob {
        job_id: Uuid,
    },
    StopJob {
        job_id: Uuid,
    },
    RetryJob {
        job_id: Uuid,
    },
    CodeGen {
        filename: String,
    },
}

#[derive(Debug, Serialize)]
pub enum JobResponse {
    InitWork {
        result: Option<String>,
        is_success: bool,
    },
    AddInterface {
        result: Option<String>,
        is_success: bool,
    },
    AddJob {
        job_id: Uuid,
        is_success: bool,
    },
    StopJobId {
        job_id: Uuid,
        is_success: bool,
    },
    RetryJob {
        job_id: Uuid,
        is_success: bool,
    },
    CodeGen {
        filename: String,
        is_sucess: bool,
    },
    Scaffold,
    Ordering {
        schedule_json: Value,
    },
}
