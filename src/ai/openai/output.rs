use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub headers: Headers,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Headers {
    pub date: String,
    pub content_type: String,
    pub content_length: String,
    pub connection: String,
    pub access_control_allow_origin: String,
    pub cache_control: String,
    pub openai_model: String,
    pub openai_organization: String,
    pub openai_processing_ms: String,
    pub openai_version: String,
    pub strict_transport_security: String,
    pub x_ratelimit_limit_requests: String,
    pub x_ratelimit_limit_tokens: String,
    pub x_ratelimit_remaining_requests: String,
    pub x_ratelimit_remaining_tokens: String,
    pub x_ratelimit_reset_requests: String,
    pub x_ratelimit_reset_tokens: String,
    pub x_request_id: String,
    pub cf_cache_status: String,
    pub server: String,
    pub cf_ray: String,
    pub alt_svc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}
