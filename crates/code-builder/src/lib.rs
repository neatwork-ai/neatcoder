use anyhow::Result;
use futures::lock::Mutex;
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob};
use state::AppState;
use std::{
    fs::{read_dir, File},
    io::Read,
    path::Path,
    sync::Arc,
};
use workflows::generate_api::{gen_project_scaffold, gen_work_schedule};

use parser::parser::sql::{AsSql, SqlStatement};

pub mod crates;
pub mod fs;
pub mod jobs;
pub mod schema;
pub mod specs;
pub mod state;
pub mod utils;
pub mod workflows;

use jobs::{job::Job, job_queue::JobQueue};

pub fn genesis() -> Result<JobQueue> {
    let mut job_queue = JobQueue::empty();

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
        gen_project_scaffold(c, j, state)
    };

    let job = Job(Box::new(closure));
    job_queue.push_back(job);

    let closure = |c: Arc<OpenAI>, j: Arc<OpenAIJob>, state: Arc<Mutex<AppState>>| {
        gen_work_schedule(c, j, state)
    };

    let job = Job(Box::new(closure));
    job_queue.push_back(job);

    Ok(job_queue)
}

pub fn get_sql_statements(path: &Path) -> Result<Vec<SqlStatement>> {
    let mut sql_stmts = Vec::new();

    for entry in read_dir(Path::new(path))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let sql_stmt = contents.as_str().as_sql()?.as_stmt()?;

            sql_stmts.push(sql_stmt);
        }
    }

    Ok(sql_stmts)
}
