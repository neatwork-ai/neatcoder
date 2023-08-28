use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{
    code_stream::CodeStream,
    interfaces::{Interface, SchemaFile},
    jobs::jobs::Jobs,
    state::AppState,
};

#[derive(Debug, Serialize)]
pub enum WorkerResponse {
    InitState,
    Scaffold { scaffold: Value },
    BuildExecutionPlan { jobs: Jobs },
    CodeGen { stream: CodeStream },
    AddSchema { schema_name: String },
    RemoveSchema { schema_name: String },
    AddInterface { interface_name: String },
    RemoveInterface { interface_name: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ManagerRequest {
    InitState {
        state: AppState,
    },
    ScaffoldProject {
        prompt: String,
    },
    BuildExecutionPlan {},
    CodeGen {
        filename: String,
    },
    AddSchema {
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    },
    RemoveSchema {
        interface_name: String,
        schema_name: String,
    },
    AddInterface {
        interface: Interface,
    },
    RemoveInterface {
        interface_name: String,
    },
    StartJob {
        job_uid: Uuid,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RequestType {
    InitState,
    ScaffoldProject,
    BuildExecutionPlan,
    CodeGen,
    AddSchema,
    RemoveSchema,
    AddInterface,
    RemoveInterface,
    StartJob,
}

impl RequestType {
    pub fn from(manager_request: &ManagerRequest) -> Self {
        match manager_request {
            ManagerRequest::InitState { .. } => RequestType::InitState,
            ManagerRequest::ScaffoldProject { .. } => RequestType::ScaffoldProject,
            ManagerRequest::BuildExecutionPlan { .. } => RequestType::BuildExecutionPlan,
            ManagerRequest::CodeGen { .. } => RequestType::CodeGen,
            ManagerRequest::AddSchema { .. } => RequestType::AddSchema,
            ManagerRequest::RemoveSchema { .. } => RequestType::RemoveSchema,
            ManagerRequest::AddInterface { .. } => RequestType::AddInterface,
            ManagerRequest::RemoveInterface { .. } => RequestType::RemoveInterface,
            ManagerRequest::StartJob { .. } => RequestType::StartJob,
        }
    }
}
