use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum OpenAIModels {
    Gpt432k,
    Gpt4,
    Gpt35Turbo,
    Gpt35Turbo16k,
}
