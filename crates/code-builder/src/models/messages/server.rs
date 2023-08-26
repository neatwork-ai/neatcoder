use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{
    interfaces::{Interface, SchemaFile},
    jobs::Jobs,
};

pub enum ServerMsg {
    UpdateJobQueue { jobs: Jobs },
    CreateFile { filename: String },
    BeginStream { filename: String },
    StreamToken { token: String },
    EndStream {},
}
