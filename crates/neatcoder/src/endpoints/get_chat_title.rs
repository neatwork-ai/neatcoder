use anyhow::{anyhow, Result};
use js_sys::Function;

use crate::openai::{
    msg::{GptRole, OpenAIMsg},
    params::{OpenAIModels, OpenAIParams},
    request::chat_raw,
};

pub async fn get_chat_title(
    msg: &str,
    request_callback: &Function,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a copyrigther specialised in creating titles for texts",
        ),
    });

    let main_prompt = format!(
        "
Your task is to create a title for the following prompt:
\"\"\"{}\"\"\"

The title of the prompt is:",
        msg
    );

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let ai_params = OpenAIParams::empty(OpenAIModels::Gpt35Turbo).max_tokens(5);

    let chat =
        chat_raw(request_callback, &ai_params, &prompts, &[], &[]).await?;

    let answer = chat
        .choices
        .first()
        .ok_or_else(|| anyhow!("LLM Respose seems to be empty :("))?
        .message
        .content
        .clone();

    Ok(answer)
}
