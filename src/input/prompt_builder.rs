use super::instruction::Instruction;
use crate::ai::openai::{
    client::OpenAI,
    input::{GptRole, Message},
};
use anyhow::Result;

pub async fn build_prompt_dyn(
    client: &OpenAI,
    instructions: &[Instruction],
    sys_msg: Option<&str>,
    user_msg: Option<&str>,
) -> Result<String> {
    let sys_msg = Message {
        role: GptRole::System,
        content: String::from(sys_msg.unwrap_or("You are Prompt Generator specialised in receiving a list of instructions and outputing prompts that maximise the effectiveness of the prompt output.")),
    };

    let mut user_msg =
        String::from(user_msg.unwrap_or("Build a prompt which follows the instructions below: \n"));

    for instruction in instructions.iter() {
        user_msg.push_str(&instruction.to_string());
    }

    let user_msg = Message {
        role: GptRole::User,
        content: user_msg,
    };

    let resp = client.chat(&[sys_msg, user_msg], &[], &[]).await?;

    let prompt = resp.choices.first().unwrap().message.content.as_str();

    Ok(prompt.to_string())
}
