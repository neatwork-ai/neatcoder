use crate::ai::openai::{
    client::OpenAI,
    input::{GptRole, Message},
};

use super::instruction::Instruction;

pub fn build_prompt_dyn<'a>(
    client: &'a OpenAI,
    instruction: &'a [Instruction],
    sys_msg: Option<&'a str>,
    user_msg: Option<&'a str>,
) -> &'a str {
    let sys_msg = Message {
        role: GptRole::System,
        content: String::from(sys_msg.unwrap_or("You are Prompt Generator specialised in receiving a list of instructions and outputing prompts that maximise the effectiveness of the prompt output.")),
    };

    let user_msg = user_msg.unwrap_or("Build a prompt which follows the instructions below: \n");

    // let resp = client.chat(&[sys_msg, user_msg], &[], &[]).await?;

    // resp.choices.first().unwrap().message.content.as_str()

    todo!()
}
