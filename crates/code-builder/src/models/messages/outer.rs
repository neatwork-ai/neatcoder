use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    interfaces::{Interface, SchemaFile},
    jobs::jobs::Jobs,
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientMsg {
    InitState {
        state: AppState,
    },
    InitPrompt {
        prompt: String,
    },
    AddInterface {
        interface: Interface,
    },
    RemoveInterface {
        #[serde(rename = "interfaceName")]
        interface_name: String,
    },
    AddSchema {
        #[serde(rename = "interfaceName")]
        interface_name: String,
        #[serde(rename = "schemaName")]
        schema_name: String,
        #[serde(rename = "schema")]
        schema: SchemaFile,
    },
    RemoveSchema {
        #[serde(rename = "interfaceName")]
        interface_name: String,
        #[serde(rename = "schemaName")]
        schema_name: String,
    },
    UpdateScaffold {
        scaffold: String,
    },
    AddSourceFile {
        filename: String,
        file: String,
    },
    RemoveSourceFile {
        filename: String,
    },
    StartJob {
        #[serde(rename = "jobId")]
        job_id: Uuid,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerMsg {
    UpdateJobQueue { jobs: Jobs },
    CreateFile { filename: String },
    BeginStream { filename: String },
    EndStream {},
}
