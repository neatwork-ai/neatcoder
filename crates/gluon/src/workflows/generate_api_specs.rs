use crate::ai::openai::{
    client::OpenAI,
    job::OpenAIJob,
    msg::{GptRole, OpenAIMsg},
};
use anyhow::Result;

pub async fn generate_api_specs(
    client: &OpenAI,
    job: &OpenAIJob,
    mut data_model: Vec<String>,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for model in data_model.drain(..) {
        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: model,
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: String::from(
            "Based on the data model described above, create an idea for an API service.",
        ),
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let resp = client.chat(job, &prompts, &[], &[]).await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    Ok(String::from(answer))
}
