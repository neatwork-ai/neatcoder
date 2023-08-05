use anyhow::{anyhow, Context, Result};
use code_builder::{fs::Files, get_sql_statements};
use dotenv::dotenv;
use gluon::{
    ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels},
    serde::json::AsJson,
    workflows::generate_api::{gen_code, gen_project_scaffold, gen_work_schedule},
};
use serde_json::from_value;
use std::io::Read;
use std::{env, fs::File, path::Path};
use std::{fs, io::Write};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut args: Vec<String> = env::args().collect();

    let description_filename = args.pop().unwrap();
    let project = args.pop().unwrap();

    let project_path = format!("examples/projects/{}", project);
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

    let api_description =
        get_api_description(project_path.join("specs").as_path(), description_filename)?;

    let scaffold = gen_project_scaffold(&client, &job, &api_description).await?;
    let dep_graph =
        gen_work_schedule(&client, &job, &api_description, &data_model, &scaffold).await?;

    let scaffold_json = scaffold.as_str().strip_json()?;
    let dep_graph_json = dep_graph.as_str().strip_json()?;

    let files: Files = match from_value::<Files>(dep_graph_json["order"].clone()) {
        Ok(files) => files,
        Err(e) => {
            // Handle the error
            return Err(anyhow!(
                "Error converting dependecy graph to `Files` struct: {e}"
            ));
        }
    };

    let mut prior_code = vec![];
    println!("b");

    for file in files.0.iter().rev() {
        let code = gen_code(
            &client,
            &job,
            api_description.clone(),
            &data_model,
            scaffold_json.to_string(),
            &prior_code, // prior_code
            file,
        )
        .await?;

        // write to file
        let file_path = Path::new(file);

        if let Some(parent_path) = file_path.parent() {
            let parent_path = project_path.join("codebase/").join(parent_path);
            fs::create_dir_all(parent_path)?;
        }

        let file_path = project_path.join(format!("codebase/{}", file));

        let mut code_file = File::create(file_path.clone())
            .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;
        code_file.write_all(code.as_bytes())?;

        // push
        prior_code.push(code);
    }
    println!("c");

    // println!("Project Dependency graph: {:?}", dep_graph_json);

    // IO
    let project_path = Path::new("examples/projects/").join(project);
    fs::create_dir_all(project_path.clone())?;
    println!("d");

    let serial_number = fs::read_dir(project_path.clone())?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .count()
        + 1;

    println!("e");
    let file_path = project_path.join(format!("fs/{}.json", serial_number));
    fs::create_dir_all(file_path.clone())?;
    let mut scaffold_file = File::create(file_path.clone())
        .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

    let file_path = project_path.join(format!("dg/{}.json", serial_number));
    fs::create_dir_all(file_path.clone())?;
    let mut graph_file = File::create(file_path.clone())
        .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

    scaffold_file.write_all(serde_json::to_string_pretty(&scaffold_json)?.as_bytes())?;
    graph_file.write_all(serde_json::to_string_pretty(&dep_graph_json)?.as_bytes())?;

    Ok(())
}

fn get_api_description(path: &Path, filename: String) -> Result<String> {
    let file_path = path.join(format!("{}.txt", filename));

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
