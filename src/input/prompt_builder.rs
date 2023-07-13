use super::instruction::Instruction;
use crate::ai::openai::{
    client::OpenAI,
    input::{GptRole, Message},
};
use anyhow::Result;

/// Dynamically builds the prompt using an LLM
///
/// The core idea is to allow for the pipelining of instructions that do
/// not necessarily have context of each other. Therefore, we pre-prompt an LLM to \
/// help contextualize the prompt.
///
/// An example:
///
/// User Message: "
/// Based only on the following input, generate a writing prompt:
/// - Context: You are an entrepreneur creating a company called Promptify, a marketplace for LLM prompts;
/// - Purpose: Persuade a Venture Capital firm in investing in your startup;
/// - Audience: 3 VC analysts who report to the partner. This is your intro call;
/// - Principle: Focus on the long-term vision
///
/// Do not include any additional background information or assumptions. Be as strict as possible, stick 100% to the input."
///
/// Prompt: Writing Prompt:
/// "Imagine you are an entrepreneur pitching your startup, Promptify, to a
/// Venture Capital firm. Craft a persuasive argument that highlights the
/// long-term vision of your company and its potential impact on the marketplace
/// for LLM prompts. Your audience consists of three VC analysts who report to
/// the partner. Write a compelling introduction for your introductory call,
/// emphasizing the unique value proposition of Promptify and why investing
/// in your startup is a lucrative opportunity.
pub async fn build_prompt_dyn(
    client: &OpenAI,
    instructions: &[Instruction],
    sys_msg: Option<&str>,
    user_msg: Option<&str>,
) -> Result<String> {
    // TODO: Things to consider, this prompt works best with low `temperature` and `top_p`
    // Also, how can we lambda-fy the whole Client api?
    let sys_msg = Message {
        role: GptRole::System,
        content: String::from(sys_msg.unwrap_or("ChatGPT, your role in this interaction is to serve as a writing prompt generator. Generate prompts based on the context provided.")),
    };

    let mut user_msg = String::from(
        user_msg.unwrap_or("Based only on the following input, generate a writing prompt:\n"),
    );

    for instruction in instructions.iter() {
        user_msg.push_str(&instruction.to_string());
    }

    user_msg.push_str("Do not include any additional background information or assumptions. Be as strict as possible, stick 100% to the input.");

    println!("User Message: {:?}", user_msg);

    let user_msg = Message {
        role: GptRole::User,
        content: user_msg,
    };

    let resp = client.chat(&[sys_msg, user_msg], &[], &[]).await?;
    let prompt = resp.choices.first().unwrap().message.content.as_str();

    Ok(prompt.to_string())
}

pub async fn build_prompt(instructions: &[Instruction], user_msg: Option<&str>) -> Result<String> {
    let mut user_msg =
        String::from(user_msg.unwrap_or("Consider the following items in your response:\n"));

    for instruction in instructions.iter() {
        user_msg.push_str(&instruction.to_string());
    }

    Ok(user_msg)
}
