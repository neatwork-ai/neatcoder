use anyhow::Result;
use dotenv::dotenv;
use gluon::{
    ai::openai::client::{OpenAI, OpenAIModels},
    workflows::generative_tree::generate_tree,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let client = OpenAI::new(OpenAIModels::Gpt35Turbo)
        .api_key(env::var("OPENAI_API_KEY")?)
        .temperature(0.7)
        .top_p(0.9)?;

    generate_tree(&client).await?;

    Ok(())
}
