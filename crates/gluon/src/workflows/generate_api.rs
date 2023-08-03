use crate::ai::openai::{
    client::OpenAI,
    job::OpenAIJob,
    msg::{GptRole, OpenAIMsg},
};
use anyhow::Result;

pub async fn gen_project_scaffold(
    client: &OpenAI,
    job: &OpenAIJob,
    api_description: String,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust based on the following project description:\n{}\n
The API should retrieve the relevant data from a MySQL database.

Based on the information provided write the project's folder structure, starting from `src`.

Answer in JSON format (Do not forget to start with ```json). For each file provide a brief description included in the json", api_description);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let resp = client.chat(job, &prompts, &[], &[]).await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    Ok(String::from(answer))
}
