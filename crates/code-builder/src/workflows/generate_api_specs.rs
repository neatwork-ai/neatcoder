use anyhow::Result;
use gluon::ai::openai::{
    client::OpenAI,
    msg::{GptRole, OpenAIMsg},
    params::OpenAIParams,
};

pub async fn generate_api_specs(
    client: &OpenAI,
    job: &OpenAIParams,
    data_model: &Vec<String>,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for model in data_model.iter() {
        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: model.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: String::from(
            "Based on the data model described above, create an idea for an API service.",
        ),
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let answer = client.chat(job, &prompts, &[], &[]).await?;

    Ok(String::from(answer))
}
