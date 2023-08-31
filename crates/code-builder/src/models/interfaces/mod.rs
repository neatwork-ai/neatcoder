use self::{apis::Api, dbs::Database, storage::Datastore};
use crate::openai::msg::OpenAIMsg;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod apis;
pub mod dbs;
pub mod storage;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Interface {
    pub(crate) interface_type: InterfaceType,
    pub(crate) inner: InterfaceInner,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[wasm_bindgen]
pub struct InterfaceInner {
    pub(crate) database: Option<Database>,
    pub(crate) storage: Option<Datastore>,
    pub(crate) api: Option<Api>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[wasm_bindgen]
pub enum InterfaceType {
    /// `Database` variant refers to Database storage solutions or to more
    /// classic Data Warehousing solutions such as Snowflake and the likes.
    /// The core difference between `Database` and `Storage` variants is that
    /// whilst both are storage solutions, the `Database` variant encapsulates
    /// storage under a Management system that typically guarantees ACID
    /// transactions as well as CAP Theorem guarantees. Usually these solutions
    /// provide a declarative framework for accessing and managing data.
    Database,
    /// `Storage` variant refers to more raw storage solutions that usually provide
    /// a direct interface to a file or object-store system. This leads to a decoupling
    /// of the storage system and the file types themselves. For example, using
    /// storage services like AWS S3 we ould build a data-lake that utilizes
    /// `parquet` files or `ndjson` files.
    Storage,
    /// `Api` variant refers to interfaces of executables themselves or
    /// execution environments, and therefore it groups RPC APIs, WebSockets,
    /// library interfaces, IDLs, etc.
    Api,
    // TODO: Add Infrastructure-As-Code (IAC)
}

/// Type-alias for any file that provides information about an interface.
/// There are no constraints to the  files themselves (i.e. extension types),
/// and this type is only here for improved readability.
pub type SchemaFile = String;

/// Trait that injects the context of interfaces onto the LLM Message Sequence.
pub trait AsContext {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()>;
}

impl AsContext for Interface {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()> {
        match self.interface_type {
            InterfaceType::Database => self
                .inner
                .database
                .as_ref()
                .unwrap()
                .add_context(msg_sequence),
            InterfaceType::Storage => self
                .inner
                .database
                .as_ref()
                .unwrap()
                .add_context(msg_sequence),
            InterfaceType::Api => self
                .inner
                .database
                .as_ref()
                .unwrap()
                .add_context(msg_sequence),
        }
    }
}

impl Interface {
    pub fn name(&self) -> String {
        match self.interface_type {
            InterfaceType::Database => {
                self.inner.database.as_ref().unwrap().name()
            }
            InterfaceType::Storage => {
                self.inner.storage.as_ref().unwrap().name()
            }
            InterfaceType::Api => self.inner.api.as_ref().unwrap().name(),
        }
    }

    pub fn schemas_mut(&mut self) -> &mut HashMap<String, SchemaFile> {
        match self.interface_type {
            InterfaceType::Database => {
                &mut self.inner.database.as_mut().unwrap().schemas
            }
            InterfaceType::Storage => {
                &mut self.inner.storage.as_mut().unwrap().schemas
            }
            InterfaceType::Api => &mut self.inner.api.as_mut().unwrap().schemas,
        }
    }

    pub fn insert_schema(&mut self, schema_name: String, schema: String) {
        let schemas = self.schemas_mut();
        schemas.insert(schema_name, schema);
    }

    pub fn remove_schema(&mut self, schema_name: &str) {
        let schemas = self.schemas_mut();
        schemas.remove(schema_name);
    }
}
