use anyhow::{Context, Result};
use dotenv::dotenv;
use parser::parser::json::AsJson;
use serde_json::Value;
use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels};

use code_builder::{
    get_sql_statements,
    models::{fs::Files, job::Job, state::AppState},
    workflows::{generate_api::gen_code, genesis::genesis},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut args: Vec<String> = env::args().collect();

    let job_uuid = args.pop().unwrap();
    let project = args.pop().unwrap();

    // === File System Operations ===
    let project_path = format!("examples/projects/{}", project);
    let project_path = Path::new(project_path.as_str());

    let jobs_path = project_path.join("jobs");

    let serial_number = fs::read_dir(jobs_path.clone())?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .count()
        + 1;

    let job_path = project_path
        .join("jobs")
        .join(serial_number.to_string().as_str());

    fs::create_dir_all(job_path.clone())?;

    // === LLM Agent Operations ===

    println!("Initializing OpenAI Client");
    let client = Arc::new(OpenAI::new(env::var("OPENAI_API_KEY")?));

    let ai_job = Arc::new(
        OpenAIJob::empty(OpenAIModels::Gpt35Turbo)
            .temperature(0.7)
            .top_p(0.9)?,
    );

    println!("Fetching SQL data model...");
    let sql_stmts = get_sql_statements(project_path)?;

    let data_model = sql_stmts
        .iter()
        .map(|s| s.raw.clone())
        .collect::<Vec<String>>();

    println!("Fetching API description...");
    let api_description = get_api_description(project_path.join("specs").as_path(), job_uuid)?;

    println!("Initializing APP State...");
    let app_state = Arc::new(Mutex::new(
        AppState::new(api_description).with_model(data_model)?,
    ));

    println!("Initializing Job Queue...");
    let mut job_queue = genesis()?;

    // Execute the jobs and handle the results
    println!("Building Project Scaffold");
    let scaffold: Arc<String> = job_queue
        .execute_next(client.clone(), ai_job.clone(), app_state.clone())
        .await?;

    println!("Building Task Dependency Map");
    let dep_graph: Arc<String> = job_queue
        .execute_next(client.clone(), ai_job.clone(), app_state.clone())
        .await?;

    // These operations are redundant as they have been done by the job handles
    let scaffold_json = scaffold.as_str().as_json()?;
    let job_schedule = dep_graph.as_str().as_json()?;

    let mut files = Files::from_schedule(job_schedule.clone())?;

    write_scaffold(scaffold_json.clone(), job_path.clone())?;
    write_graph(job_schedule, job_path.clone())?;

    // Add jobs to the job queue
    for file in files.iter() {
        let file_ = file.clone();

        let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
            gen_code(c, j, state, file_)
        };

        let job = Job::new(Box::new(closure));
        job_queue.push_back(job);
    }

    for (_job_id, job) in job_queue.drain() {
        let file = files.pop_front().unwrap();
        println!("Running job {:?}", file);
        let file_path = Path::new(&file);

        let code_string: Arc<String> = job
            .execute(client.clone(), ai_job.clone(), app_state.clone())
            .await?;

        println!("Finished running job {:?}", file);

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

        code_file.write_all(code_string.as_bytes())?;
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
