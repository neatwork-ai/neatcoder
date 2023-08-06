use anyhow::{Context, Result};
use code_builder::get_sql_statements;
use dotenv::dotenv;
use gluon::{
    ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels},
    workflows::generate_api_specs::generate_api_specs,
};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut args: Vec<String> = env::args().collect();

    let project = args.pop().unwrap();
    let project_path = format!("examples/projects/{}/models", project);
    let project_path = Path::new(project_path.as_str());

    let client = OpenAI::new(env::var("OPENAI_API_KEY")?);

    let job = OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
        .temperature(0.7)
        .top_p(0.9)?;

    let sql_stmts = get_sql_statements(project_path)?;

    let data_model = sql_stmts
        .iter()
        .map(|s| s.raw.clone())
        .collect::<Vec<String>>();

    let api_idea = generate_api_specs(&client, &job, &data_model).await?;

    println!("API IDEA: {}", api_idea);

    // IO
    let project_path = Path::new("examples/projects/").join(project).join("specs/");
    fs::create_dir_all(project_path.clone())?;

    let uuid = Uuid::new_v4();

    let file_path = project_path.join(format!("{}.txt", uuid));
    let mut api_file = File::create(file_path.clone())
        .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

    api_file.write_all(api_idea.as_bytes())?;

    Ok(())
}
