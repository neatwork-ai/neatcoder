use super::{commit::HashID, job_queue::JobQueue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobRequest {
    InitWork { prompt: String },
    AddModel { path: String, schema: String },
    AddJob { job_id: HashID },
    StopJob { job_id: HashID },
    RetryJob { job_id: HashID },
}

#[derive(Debug, Serialize)]
pub enum JobResponse {
    InitWork {
        result: Option<String>,
        is_success: bool,
    },
    AddModel {
        result: Option<String>,
        is_success: bool,
    },
    AddJob {
        job_id: HashID,
        is_success: bool,
    },
    StopJobId {
        job_id: HashID,
        is_success: bool,
    },
    RetryJob {
        job_id: HashID,
        is_success: bool,
    },
    CodeGen,
    Scaffold,
    Ordering {
        schedule_json: Value,
    },
}
