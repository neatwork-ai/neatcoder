use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    models::{
        interfaces::{Interface, SchemaFile},
        jobs::Jobs,
    },
    utils::CodeStream,
};

#[derive(Debug, Serialize)]
pub enum WorkerResponse {
    Scaffold { scaffold: Value },
    BuildExecutionPlan { jobs: Jobs },
    CodeGen { stream: CodeStream },
    AddSchema { schema_name: String },
    AddInterface { interface_name: String },
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
}
