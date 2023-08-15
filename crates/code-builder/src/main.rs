use code_builder::models::ClientCommand;
use serde_json;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let command: ClientCommand = serde_json::from_str(&line.unwrap()).unwrap();
        match command {
            ClientCommand::InitWork { prompt } => {
                // Handle ...
                todo!()
            }
            ClientCommand::AddSchema { schema } => {
                // Handle ...
                todo!()
            }
            ClientCommand::GetJobQueue => {
                // Handle ...
                todo!()
            }
            ClientCommand::StartJob { job_id } => {
                // Handle ...
                todo!()
            }
            ClientCommand::StopJob { job_id } => {
                // Handle ...
                todo!()
            }
            ClientCommand::RetryJob { job_id } => {
                // Handle ...
                todo!()
            } // _ => {}
        }
    }
}
