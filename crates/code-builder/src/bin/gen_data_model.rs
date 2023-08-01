use anyhow::{Context, Result};
use code_builder::schema::DataModel;
use dotenv::dotenv;
use gluon::{
    ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels},
    workflows::generate_db_schema::generate_db_schema,
};
use heck::ToSnakeCase;
use std::{env, fs::File, path::Path};
use std::{fs, io::Write};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let client = OpenAI::new(env::var("OPENAI_API_KEY")?);

    let job = OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
        .temperature(0.7)
        .top_p(0.9)?;

    let (mut company_name, sql_str) = generate_db_schema(&client, &job).await?;

    company_name.retain(|c| c.is_alphabetic());
    let company_name = company_name.to_snake_case();

    let data_model = DataModel::from_sql(&sql_str)?;

    // IO
    let project_path = Path::new("examples/models/").join(company_name);
    fs::create_dir_all(project_path.clone())?;

    for dll in data_model.iter() {
        let name = dll.name().to_snake_case();
        let file_path = &project_path.join(format!("{}.sql", name));
        let mut dll_file = File::create(file_path)
            .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

        dll_file.write_all(dll.raw.as_bytes())?;
    }

    Ok(())
}
