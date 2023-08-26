use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ManagerRequest {
    ScaffoldProject { prompt: String },
    BuildExecutionPlan {},
    CodeGen { filename: String },
}
