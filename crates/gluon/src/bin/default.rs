use anyhow::Result;
use dotenv::dotenv;
use gluon::ai::openai::{
    client::{OpenAI, OpenAIModels},
    input::{GptRole, Message},
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let model = OpenAI::new(OpenAIModels::Gpt35Turbo)
        .api_key(env::var("OPENAI_API_KEY")?)
        .temperature(0.7)
        .top_p(0.9)?;

    let sys_msg = Message {
        role: GptRole::System,
        content: String::from("You are a Rust Engineer with 1000 years of experience. You completely outpace any human programmer.")
    };

    let user_msg = Message {
        role: GptRole::User,
        content: String::from("Write an AGI."),
    };

    let resp = model.chat(&[&sys_msg, &user_msg], &[], &[]).await?;

    println!("{:?}", resp);

    Ok(())
}
