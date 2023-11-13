use anyhow::{anyhow, Result};
use js_sys::Function;
use oai::models::{
    chat::{
        params::wasm::ChatParamsWasm as ChatParams, request::wasm::chat_raw,
    },
    message::wasm::GptMessageWasm as GptMessage,
    role::Role as GptRole,
    Models as AiModels,
};

pub async fn get_chat_title(
    msg: &str,
    request_callback: &Function,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(GptMessage::new(GptRole::System, String::from(
        "
- Context: Briefly describe the key topics or themes of the chat.
- Title Specifications: The title should be concise, and not exceed 6 words. It should reflect the tone of the chat (e.g., professional, casual, informative, provocative, etc.).
- Output: Provide a title that encapsulates the main focus of the chat.
        ",
    )));

    let main_prompt = format!(
        "
Your task is to create a title for the following prompt:
\"\"\"{}\"\"\"

The title of the prompt is:",
        msg
    );

    prompts.push(GptMessage::new(GptRole::User, main_prompt));

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&GptMessage>>();

    let ai_params = ChatParams::empty(AiModels::Gpt35Turbo).max_tokens(15);

    let chat =
        chat_raw(request_callback, &ai_params, &prompts, &[], &[]).await?;

    let mut answer = chat
        .choices
        .first()
        .ok_or_else(|| anyhow!("LLM Respose seems to be empty :("))?
        .message
        .content
        .clone();

    answer = clean_title(answer);

    Ok(answer)
}

fn clean_title(answer: String) -> String {
    answer.replace("\"", "")
}
