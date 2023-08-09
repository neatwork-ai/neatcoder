use anyhow::{anyhow, Context, Result};
use code_builder::genesis;
use code_builder::jobs::job::Job;
use code_builder::jobs::job_queue::JobQueue;
use code_builder::state::AppState;
use code_builder::workflows::generate_api::{gen_code, gen_project_scaffold};
use code_builder::{fs::Files, get_sql_statements};
use dotenv::dotenv;
use futures::executor;
use futures::lock::Mutex;
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, model::OpenAIModels};
use parser::parser::json::AsJson;
use serde_json::{from_value, Value};
use std::sync::Arc;
use std::{env, fs::File, path::Path};
use std::{fs, io::Write};
use std::{io::Read, path::PathBuf};

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

    // LLM Agent Operations

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
    let mut job_queue = JobQueue::empty();

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
        gen_project_scaffold(c, j, state)
    };

    let job_1 = Job::new(Box::new(closure));
    let job_2 = Job::new(Box::new(closure));
    let job_3 = Job::new(Box::new(closure));
    let job_4 = Job::new(Box::new(closure));
    let job_5 = Job::new(Box::new(closure));
    let job_6 = Job::new(Box::new(closure));
    let job_7 = Job::new(Box::new(closure));
    let job_8 = Job::new(Box::new(closure));
    let job_9 = Job::new(Box::new(closure));
    let job_10 = Job::new(Box::new(closure));
    job_queue.push_back(job_1);
    job_queue.push_back(job_2);
    job_queue.push_back(job_3);
    job_queue.push_back(job_4);
    job_queue.push_back(job_5);
    job_queue.push_back(job_6);
    job_queue.push_back(job_7);
    job_queue.push_back(job_8);
    job_queue.push_back(job_9);
    job_queue.push_back(job_10);

    // Execute the jobs and handle the results
    println!("Building Project Scaffold");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("1 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("2 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("3 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("4 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("5 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("6 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("7 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("8 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("9 done");
    executor::block_on(job_queue.execute(client.clone(), ai_job.clone(), app_state.clone()))?;
    println!("10 done");

    Ok(())
}

fn get_api_description(path: &Path, filename: String) -> Result<String> {
    let file_path = path.join(format!("{}.txt", filename));

    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
