use self::{dbs::Database, storage::Datastore};
use anyhow::Result;
use gluon::ai::openai::msg::OpenAIMsg;

pub mod apis;
pub mod dbs;
pub mod storage;

#[derive(Debug)]
pub enum Interface {
    Database(Database),
    Storage(Datastore),
    // Api(Api) TODO
}

pub trait AsContext {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()>;
}

impl AsContext for Interface {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()> {
        match self {
            Interface::Database(db) => db.add_context(msg_sequence),
            Interface::Storage(ds) => ds.add_context(msg_sequence),
        }
    }
}
