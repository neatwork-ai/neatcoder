use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    models::interfaces::{Interface, SchemaFile},
    utils::CodeStream,
};

#[derive(Debug, Serialize)]
pub enum WorkerResponse {
    Scaffold { scaffold: Value },
    BuildExecutionPlan { execution_plan: Value },
    CodeGen { stream: CodeStream },
}
