use anyhow::Result;
use dotenv::dotenv;
use gluon::{
    ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels},
    workflows::generative_tree::generate_tree,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let client = OpenAI::new(env::var("OPENAI_API_KEY")?);

    let job = OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
        .temperature(0.7)
        .top_p(0.9)?;

    generate_tree(&client, &job).await?;

    Ok(())
}
