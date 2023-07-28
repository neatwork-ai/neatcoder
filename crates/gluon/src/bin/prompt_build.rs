// use anyhow::Result;
// use dotenv::dotenv;
// use gluon::{
//     ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels, msg::OpenAIMsg},
//     input::{
//         instruction::{Instruction, InstructionType},
//         prompt_builder::{build_prompt, build_prompt_dyn},
//     },
// };
// use std::env;

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv().ok();

//     let client = OpenAI::new(env::var("OPENAI_API_KEY")?);

//     let job = OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
//         .temperature(0.7)
//         .top_p(0.9)?;

//     let dyn_prompt = build_prompt_dyn(
//         &client,
//         &job,
//         &[
//             Instruction::new(InstructionType::Context, "You are an entrepreneur creating a company called Promptify, a marketplace for LLM prompts"),
//             Instruction::new(InstructionType::Purpose, "Persuade a Venture Capital firm in investing in your startup"),
//             Instruction::new(InstructionType::Audience, "3 VC analysts who report to the partner. This is your intro call"),
//             Instruction::new(InstructionType::Principle, "Focus on the long-term vision"),
//         ],
//         None,
//         None,
//     ).await?;

//     let static_prompt = build_prompt(&[
//         Instruction::new(InstructionType::Context, "You are an entrepreneur creating a company called Promptify, a marketplace for LLM prompts"),
//         Instruction::new(InstructionType::Purpose, "Persuade a Venture Capital firm in investing in your startup"),
//         Instruction::new(InstructionType::Audience, "3 VC analysts who report to the partner. This is your intro call"),
//         Instruction::new(InstructionType::Principle, "Focus on the long-term vision"),
//     ], None).await?;

//     println!("Prompt: {}", dyn_prompt);

//     let job = job.temperature(1.0).top_p(1.0)?;

//     let dyn_resp = client
//         .chat(
//             &job,
//             &[
//             &OpenAIMsg::system("You are an entrepreneur creating a company called Promptify, a marketplace for LLM prompts"),
//             &OpenAIMsg::user(dyn_prompt.as_str()),
//         ], &[], &[])
//         .await?;

//     let dyn_json = dyn_resp.choices.first().unwrap().message.content.as_str();

//     println!("DYNAMIC: {}", dyn_json);

//     let static_resp = client
//         .chat(
//             &job,
//             &[
//             &OpenAIMsg::system("You are an entrepreneur creating a company called Promptify, a marketplace for LLM prompts"),
//             &OpenAIMsg::user(dyn_prompt.as_str()),
//         ], &[], &[])
//         .await?;

//     let static_json = static_resp
//         .choices
//         .first()
//         .unwrap()
//         .message
//         .content
//         .as_str();

//     println!("STATIC: {}", static_json);

//     Ok(())
// }
