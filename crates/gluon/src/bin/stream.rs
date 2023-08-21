use anyhow::Result;
use dotenv::dotenv;
use futures::StreamExt;
use gluon::ai::openai::{
    client::OpenAI,
    model::OpenAIModels,
    msg::{GptRole, OpenAIMsg},
    params::OpenAIParams,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let client = OpenAI::new(env::var("OPENAI_API_KEY")?);

    let job = OpenAIParams::empty(OpenAIModels::Gpt35Turbo)
        .temperature(0.7)
        .top_p(0.9)?;

    let sys_msg = OpenAIMsg {
        role: GptRole::System,
        content: String::from("You are a Rust Engineer with 1000 years of experience. You completely outpace any human programmer.")
    };

    let user_msg = OpenAIMsg {
        role: GptRole::User,
        content: String::from("Write an AGI."),
    };

    let mut stream = client
        .chat_stream(&job, &[&sys_msg, &user_msg], &[], &[])
        .await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                let token = std::str::from_utf8(&bytes).unwrap();
                println!("{}", token);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    Ok(())
}
