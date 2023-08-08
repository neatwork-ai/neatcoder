use anyhow::{anyhow, Context, Result};
use code_builder::genesis;
use code_builder::jobs::job::Job;
use code_builder::state::AppState;
use code_builder::workflows::generate_api::gen_code;
use code_builder::{fs::Files, get_sql_statements};
use dotenv::dotenv;
use futures::executor;
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels};
use parser::parser::{json::AsJson, rust::AsRust};
use serde_json::{from_value, Value};
use std::sync::{Arc, Mutex};
use std::{env, fs::File, path::Path};
use std::{fs, io::Write};
use std::{io::Read, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut args: Vec<String> = env::args().collect();

    let job_uuid = args.pop().unwrap();
    let project = args.pop().unwrap();

    let project_path = format!("examples/projects/{}", project);
    let project_path = Path::new(project_path.as_str());

    let client = Arc::new(OpenAI::new(env::var("OPENAI_API_KEY")?));

    let ai_job = Arc::new(
        OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
            .temperature(0.7)
            .top_p(0.9)?,
    );

    let sql_stmts = get_sql_statements(project_path)?;

    let data_model = sql_stmts
        .iter()
        .map(|s| s.raw.clone())
        .collect::<Vec<String>>();

    let api_description = get_api_description(project_path.join("specs").as_path(), job_uuid)?;

    let app_state = Arc::new(Mutex::new(AppState::new(api_description)));

    let mut job_queue = genesis()?;

    // Execute the jobs and handle the results
    let scaffold: Arc<String> =
        executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    let dep_graph: Arc<String> =
        executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;

    let scaffold_json = scaffold.as_str().as_json()?;
    let dep_graph_json = dep_graph.as_str().as_json()?;

    let mut files: Files = match from_value::<Files>(dep_graph_json["order"].clone()) {
        Ok(files) => files,
        Err(e) => {
            // Handle the error
            return Err(anyhow!(
                "Error converting dependecy graph to `Files` struct: {e}"
            ));
        }
    };

    let jobs_path = project_path.join("jobs");

    let serial_number = fs::read_dir(jobs_path.clone())?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .count()
        + 1;

    let job_path = project_path
        .join("jobs")
        .join(serial_number.to_string().as_str());

    fs::create_dir_all(job_path.clone())?;
    write_scaffold(scaffold_json.clone(), job_path.clone())?;
    write_graph(dep_graph_json, job_path.clone())?;

    // Add jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();

        let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
            gen_code(c, j, state, file_)
        };

        let job = Job::new(Box::new(closure));
        job_queue.push_back(job);
    }

    for job in job_queue.drain(..) {
        let code_string: Arc<String> =
            executor::block_on(job.execute(client.clone(), ai_job.clone(), app_state.clone()))?;

        println!("The CODEE! {:?}", code_string);

        let file = files.pop_front().unwrap();

        // write to file
        let file_path = Path::new(&file);

        if let Some(parent_path) = file_path.parent() {
            let log_path = job_path.join("logs/").join(parent_path);

            let parent_path = job_path.join("codebase/").join(parent_path);
            fs::create_dir_all(parent_path)?;
            fs::create_dir_all(log_path)?;
        }

        let file_path = job_path.join(format!("codebase/{}", file));
        let mut code_file = File::create(file_path.clone())
            .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

        let log_path = job_path.join(format!("logs/{}", file));
        let mut log_file = File::create(log_path.clone())
            .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;
        log_file.write_all(code_string.as_bytes())?;

        let code = code_string.as_str().strip_rust()?;
        code_file.write_all(code.raw.as_bytes())?;
    }

    Ok(())
}

fn get_api_description(path: &Path, filename: String) -> Result<String> {
    let file_path = path.join(format!("{}.txt", filename));

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_scaffold(scaffold_json: Value, project_path: PathBuf) -> Result<()> {
    let file_path = project_path.join("fs.json");

    let mut scaffold_file = File::create(file_path.clone())
        .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

    scaffold_file.write_all(serde_json::to_string_pretty(&scaffold_json)?.as_bytes())?;

    Ok(())
}

fn write_graph(dep_graph_json: Value, project_path: PathBuf) -> Result<()> {
    let file_path = project_path.join("dg.json");

    let mut graph_file = File::create(file_path.clone())
        .with_context(|| format!(r#"Could not create "{path}""#, path = file_path.display()))?;

    graph_file.write_all(serde_json::to_string_pretty(&dep_graph_json)?.as_bytes())?;

    Ok(())
}
