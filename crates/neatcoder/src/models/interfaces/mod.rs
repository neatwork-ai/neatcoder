use self::{apis::Api, dbs::Database, storage::Storage};
use crate::{openai::msg::OpenAIMsg, utils::map_to_jsvalue};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod apis;
pub mod dbs;
pub mod storage;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(typescript_type = "Record<string, string>")]
//     pub type ISchemas;
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub struct Interface {
    pub(crate) interface_type: InterfaceType,
    pub(crate) inner: InterfaceInner,
}

#[wasm_bindgen]
impl Interface {
    #[wasm_bindgen(js_name = newDb)]
    pub fn new_db(db: Database) -> Self {
        Self {
            interface_type: InterfaceType::Database,
            inner: InterfaceInner::new_db(db),
        }
    }
    #[wasm_bindgen(js_name = newApi)]
    pub fn new_api(api: Api) -> Self {
        Self {
            interface_type: InterfaceType::Api,
            inner: InterfaceInner::new_api(api),
        }
    }
    #[wasm_bindgen(js_name = newStorage)]
    pub fn new_storage(storage: Storage) -> Self {
        Self {
            interface_type: InterfaceType::Storage,
            inner: InterfaceInner::new_storage(storage),
        }
    }

    #[wasm_bindgen(getter, js_name = interface)]
    pub fn get_interface(&self) -> JsValue {
        match &self.interface_type {
            InterfaceType::Database => self.inner.get_database(),
            InterfaceType::Storage => self.inner.get_storage(),
            InterfaceType::Api => self.inner.get_api(),
        }
    }

    #[wasm_bindgen(getter, js_name = interfaceType)]
    pub fn get_interface_type(&self) -> InterfaceType {
        self.interface_type
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[wasm_bindgen]
#[serde(rename_all = "camelCase")]
pub struct InterfaceInner {
    pub(crate) database: Option<Database>,
    pub(crate) storage: Option<Storage>,
    pub(crate) api: Option<Api>,
}

#[wasm_bindgen]
impl InterfaceInner {
    #[wasm_bindgen(js_name = newDb)]
    pub fn new_db(db: Database) -> Self {
        Self {
            database: Some(db),
            storage: None,
            api: None,
        }
    }

    #[wasm_bindgen(js_name = newApi)]
    pub fn new_api(api: Api) -> Self {
        Self {
            database: None,
            storage: None,
            api: Some(api),
        }
    }

    #[wasm_bindgen(js_name = newStorage)]
    pub fn new_storage(storage: Storage) -> Self {
        Self {
            database: None,
            storage: Some(storage),
            api: None,
        }
    }

    #[wasm_bindgen(getter, js_name = database)]
    pub fn get_database(&self) -> JsValue {
        match &self.database {
            Some(database) => database.clone().into(),
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(getter, js_name = storage)]
    pub fn get_storage(&self) -> JsValue {
        match &self.storage {
            Some(storage) => storage.clone().into(),
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(getter, js_name = api)]
    pub fn get_api(&self) -> JsValue {
        match &self.api {
            Some(api) => api.clone().into(),
            None => JsValue::NULL,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
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

#[wasm_bindgen]
impl Interface {
    #[wasm_bindgen(getter, js_name = name)]
    pub fn get_name(&self) -> String {
        match self.interface_type {
            InterfaceType::Database => {
                self.inner.database.as_ref().unwrap().name.clone()
            }
            InterfaceType::Storage => {
                self.inner.storage.as_ref().unwrap().name.clone()
            }
            InterfaceType::Api => self.inner.api.as_ref().unwrap().name.clone(),
        }
    }

    #[wasm_bindgen(getter, js_name = schemas)]
    pub fn get_schemas(&mut self) -> JsValue {
        let schemas = match self.interface_type {
            InterfaceType::Database => {
                &self.inner.database.as_mut().unwrap().schemas
            }
            InterfaceType::Storage => {
                &self.inner.storage.as_mut().unwrap().schemas
            }
            InterfaceType::Api => &self.inner.api.as_mut().unwrap().schemas,
        };

        map_to_jsvalue(schemas)
    }

    #[wasm_bindgen(js_name = insertSchema)]
    pub fn insert_schema(&mut self, schema_name: String, schema: String) {
        let schemas = self.schemas_mut();
        schemas.insert(schema_name, schema);
    }

    #[wasm_bindgen(js_name = removeSchema)]
    pub fn remove_schema(&mut self, schema_name: &str) {
        let schemas = self.schemas_mut();
        schemas.remove(schema_name);
    }
}

impl Interface {
    fn schemas_mut(&mut self) -> &mut BTreeMap<String, SchemaFile> {
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
}
