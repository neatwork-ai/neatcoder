use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{
    code_stream::CodeStream,
    interfaces::{Interface, SchemaFile},
    jobs::jobs::Jobs,
};

#[derive(Debug, Serialize)]
pub enum WorkerResponse {
    Scaffold { scaffold: Value },
    BuildExecutionPlan { jobs: Jobs },
    CodeGen { stream: CodeStream },
    AddSchema { schema_name: String },
    AddInterface { interface_name: String },
    RemoveInterface { interface_name: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ManagerRequest {
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
    ScaffoldProject,
    BuildExecutionPlan,
    CodeGen,
    AddSchema,
    AddInterface,
    RemoveInterface,
    StartJob,
}

impl RequestType {
    pub fn from(manager_request: &ManagerRequest) -> Self {
        match manager_request {
            ManagerRequest::ScaffoldProject { .. } => RequestType::ScaffoldProject,
            ManagerRequest::BuildExecutionPlan { .. } => RequestType::BuildExecutionPlan,
            ManagerRequest::CodeGen { .. } => RequestType::CodeGen,
            ManagerRequest::AddSchema { .. } => RequestType::AddSchema,
            ManagerRequest::AddInterface { .. } => RequestType::AddInterface,
            ManagerRequest::RemoveInterface { .. } => RequestType::RemoveInterface,
            ManagerRequest::StartJob { .. } => RequestType::StartJob,
        }
    }
}
