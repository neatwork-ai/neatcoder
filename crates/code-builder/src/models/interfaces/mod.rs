use self::{apis::Api, dbs::Database, storage::Datastore};
use anyhow::Result;
use gluon::ai::openai::msg::OpenAIMsg;
use serde::{Deserialize, Serialize};

pub mod apis;
pub mod dbs;
pub mod storage;

// TODO
// [DEBUG MSG] {"addInterface":{"interface":{"database":{"name":"DUCKO","dbType":{"name":"DUCKO","dbType":"DuckDb"}}}}}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Interface {
    #[serde(rename = "database")]
    Database(Database),
    #[serde(rename = "storage")]
    Storage(Datastore),
    #[serde(rename = "api")]
    Api(Api),
}

pub type SchemaFile = String;

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
