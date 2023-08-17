use super::{commit::HashID, job_queue::JobQueue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobRequest {
    InitWork { prompt: String },
    AddSchema { schema: String },
    GetJobQueue,
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
    AddSchema {
        result: Option<String>,
        is_success: bool,
    },
    GetJobQueue {
        job_queue: JobQueue,
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
}
