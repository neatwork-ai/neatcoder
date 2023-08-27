use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    interfaces::{Interface, SchemaFile},
    jobs::Jobs,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientMsg {
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
    StartJob {
        #[serde(rename = "jobId")]
        job_id: Uuid,
    },
    StopJob {
        #[serde(rename = "jobId")]
        job_id: Uuid,
    },
    RetryJob {
        #[serde(rename = "jobId")]
        job_id: Uuid,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerMsg {
    // Server Side Acknowledge Message
    InitPromptAck {
        success: bool,
    },
    AddSchemaAck {
        #[serde(rename = "schemaName")]
        schema_name: String,
        success: bool,
    },
    AddInterfaceAck {
        #[serde(rename = "interfaceName")]
        interface_name: String,
        success: bool,
    },

    // Server Side Commands
    UpdateJobQueue {
        jobs: Jobs,
    },
    CreateFile {
        filename: String,
    },
    BeginStream {
        filename: String,
    },
    StreamToken {
        token: String,
    },
    EndStream {},
}
