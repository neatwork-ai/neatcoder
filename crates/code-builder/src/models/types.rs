use std::hash::Hash;

use super::{commit::HashID, job_queue::JobQueue};

pub enum JobRequest {
    InitWork { prompt: String },
    AddSchema { schema: String },
    GetJobQueue,
    AddJobId { job_id: HashID },
    StopJob { job_id: HashID },
    RetryJob { job_id: HashID },
}

pub enum Response {
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
    AddJobId {
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
