use anyhow::{Context, Result};
use dotenv::dotenv;
use gluon::{
    ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels},
    serde::{
        yaml::AsYaml,
    },
    workflows::generate_api::gen_project_scaffold,
};
use std::io::Read;
use std::{env, fs::File, path::Path};
use std::{fs, io::Write};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut args: Vec<String> = env::args().collect();

    let project = args.pop().unwrap();
    let description_filename = args.pop().unwrap();

    let project_path = format!("examples/models/{}", project);
    let project_path = Path::new(project_path.as_str());

    let client = OpenAI::new(env::var("OPENAI_API_KEY")?);

    let job = OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
        .temperature(0.7)
        .top_p(0.9)?;

    let api_description = get_api_description(project_path, description_filename)?;

    let scaffold = gen_project_scaffold(&client, &job, api_description).await?;

    let scaffold_yaml = scaffold.as_str().strip_yaml()?;

    println!("Project Scaffold: {:?}", scaffold_yaml);

    // IO
    let project_path = Path::new("examples/projects/").join(project);
    fs::create_dir_all(project_path.clone())?;

    let serial_number = fs::read_dir(project_path.clone())?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .count()
        + 1;

    let file_path = project_path.join(format!("{}.yaml", serial_number));
    let mut scaffold_file = File::create(file_path.clone())
        .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

    scaffold_file.write_all(serde_yaml::to_string(&scaffold_yaml)?.as_bytes())?;

    Ok(())
}

fn get_api_description(path: &Path, filename: String) -> Result<String> {
    let file_path = path.join(format!("{}.txt", filename));

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}