use anyhow::Result;
use dotenv::dotenv;
use gluon::{
    ai::openai::client::{OpenAI, OpenAIModels},
    input::{
        instruction::{Instruction, InstructionType},
        prompt_builder::build_prompt_dyn,
    },
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let client = OpenAI::new(OpenAIModels::Gpt35Turbo)
        .api_key(env::var("OPENAI_API_KEY")?)
        .temperature(0.0)
        .top_p(0.0)?;

    let prompt = build_prompt_dyn(
        &client,
        &[
            Instruction::new(InstructionType::Context, "You are an entrepreneur creating a company called Promptify, a marketplace for LLM prompts"),
            Instruction::new(InstructionType::Purpose, "Persuade a Venture Capital firm in investing in your startup"),
            Instruction::new(InstructionType::Audience, "3 VC analysts who report to the partner. This is your intro call"),
            Instruction::new(InstructionType::Principle, "Focus on the long-term vision"),
        ],
        None,
        None,
    ).await?;

    println!("Prompt: {}", prompt);

    Ok(())
}
