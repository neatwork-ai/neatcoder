use self::{apis::Api, dbs::Database, storage::Datastore};
use anyhow::Result;
use gluon::ai::openai::msg::OpenAIMsg;
use serde::{Deserialize, Serialize};

pub mod apis;
pub mod dbs;
pub mod storage;

/// Enum-Struct documenting a type of interface. Currently we acceept three
/// types of interfaces, `Database`, `Storage` and `Api`.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Interface {
    /// `Database` variant refers to Database storage solutions or to more
    /// classic Data Warehousing solutions such as Snowflake and the likes.
    /// The core difference between `Database` and `Storage` variants is that
    /// whilst both are storage solutions, the `Database` variant encapsulates
    /// storage under a Management system that typically guarantees ACID
    /// transactions as well as CAP Theorem guarantees. Usually these solutions
    /// provide a declarative framework for accessing and managing data.
    #[serde(rename = "database")]
    Database(Database),
    /// `Storage` variant refers to more raw storage solutions that usually provide
    /// a direct interface to a file or object-store system. This leads to a decoupling
    /// of the storage system and the file types themselves. For example, using
    /// storage services like AWS S3 we ould build a data-lake that utilizes
    /// `parquet` files or `ndjson` files.
    #[serde(rename = "storage")]
    Storage(Datastore),
    /// `Api` variant refers to interfaces of executables themselves or
    /// execution environments, and therefore it groups RPC APIs, WebSockets,
    /// library interfaces, IDLs, etc.
    #[serde(rename = "api")]
    Api(Api),
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
        match self {
            Interface::Database(db) => db.add_context(msg_sequence),
            Interface::Storage(ds) => ds.add_context(msg_sequence),
            Interface::Api(api) => api.add_context(msg_sequence),
        }
    }
}

impl Interface {
    pub fn name(&self) -> &str {
        match self {
            Interface::Database(db) => &db.name,
            Interface::Storage(ds) => &ds.name,
            Interface::Api(api) => &api.name,
        }
    }

    pub fn insert_schema(&mut self, schema_name: String, schema: String) {
        let schemas = match self {
            Interface::Database(db) => &mut db.schemas,
            Interface::Storage(ds) => &mut ds.schemas,
            Interface::Api(api) => &mut api.schemas,
        };

        schemas.insert(schema_name, schema);
    }
}
