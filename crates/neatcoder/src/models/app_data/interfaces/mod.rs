pub mod apis;
pub mod dbs;
pub mod storage;

use self::{apis::Api, dbs::Database, storage::Storage};
use crate::typescript::ISchemas;
use anyhow::{anyhow, Result};
use oai::models::message::wasm::GptMessageWasm as GptMessage;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasmer::JsError;

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

    #[wasm_bindgen(getter)]
    pub fn interface(&self) -> Result<JsValue, JsError> {
        match &self.interface_type {
            InterfaceType::Database => {
                let inner = self.inner.database().ok_or_else(|| {
                    JsError::from_str(
                        "Failed to retrieve inner Database interface",
                    )
                })?;
                Ok(JsValue::from_str(
                    &serde_json::to_string(&inner)
                        .map_err(|e| JsError::from_str(&e.to_string()))?,
                ))
            }
            InterfaceType::Storage => {
                let inner = self.inner.storage().ok_or_else(|| {
                    JsError::from_str(
                        "Failed to retrieve inner Storage interface",
                    )
                })?;
                Ok(JsValue::from_str(
                    &serde_json::to_string(&inner)
                        .map_err(|e| JsError::from_str(&e.to_string()))?,
                ))
            }
            InterfaceType::Api => {
                let inner = self.inner.api().ok_or_else(|| {
                    JsError::from_str("Failed to retrieve inner Api interface")
                })?;
                Ok(JsValue::from_str(
                    &serde_json::to_string(&inner)
                        .map_err(|e| JsError::from_str(&e.to_string()))?,
                ))
            }
        }
    }

    #[wasm_bindgen(getter, js_name = interfaceType)]
    pub fn interface_type(&self) -> InterfaceType {
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

    #[wasm_bindgen(getter)]
    pub fn database(&self) -> Option<Database> {
        self.database.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn storage(&self) -> Option<Storage> {
        self.storage.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn api(&self) -> Option<Api> {
        self.api.clone()
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
    fn add_context(&self, msg_sequence: &mut Vec<GptMessage>) -> Result<()>;
}

impl AsContext for Interface {
    fn add_context(&self, msg_sequence: &mut Vec<GptMessage>) -> Result<()> {
        match self.interface_type {
            InterfaceType::Database => self
                .inner
                .database
                .as_ref()
                .ok_or_else(|| anyhow!("Unable to retrieve inner Database :("))?
                .add_context(msg_sequence),
            InterfaceType::Storage => self
                .inner
                .database
                .as_ref()
                .ok_or_else(|| anyhow!("Unable to retrieve inner Storage :("))?
                .add_context(msg_sequence),
            InterfaceType::Api => self
                .inner
                .database
                .as_ref()
                .ok_or_else(|| anyhow!("Unable to retrieve inner Api :("))?
                .add_context(msg_sequence),
        }
    }
}

#[wasm_bindgen]
impl Interface {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        match self.interface_type {
            InterfaceType::Database => self
                .inner
                .database
                .as_ref()
                .expect("Unable to retrieve inner Database interface")
                .name
                .clone(),
            InterfaceType::Storage => self
                .inner
                .storage
                .as_ref()
                .expect("Unable to retrieve inner Storage interface")
                .name
                .clone(),
            InterfaceType::Api => self
                .inner
                .api
                .as_ref()
                .expect("Unable to retrieve inner Api interface")
                .name
                .clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn itype(&self) -> String {
        match self.interface_type {
            InterfaceType::Database => self
                .inner
                .database
                .as_ref()
                .expect("Unable to retrieve inner Database interface")
                .db_type
                .to_string(),
            InterfaceType::Storage => self
                .inner
                .storage
                .as_ref()
                .expect("Unable to retrieve inner Storage interface")
                .storage_type
                .to_string(),
            InterfaceType::Api => self
                .inner
                .api
                .as_ref()
                .expect("Unable to retrieve inner Api interface")
                .api_type
                .to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn schemas(&self) -> Result<ISchemas, JsError> {
        let schemas: Result<ISchemas, JsError> = match self.interface_type {
            InterfaceType::Database => self
                .inner
                .database
                .as_ref()
                .expect("Unable to retrieve inner Database interface")
                .schemas(),
            InterfaceType::Storage => self
                .inner
                .storage
                .as_ref()
                .expect("Unable to retrieve inner Storage interface")
                .schemas(),
            InterfaceType::Api => self
                .inner
                .api
                .as_ref()
                .expect("Unable to retrieve inner Api interface")
                .schemas(),
        };

        schemas
    }

    #[wasm_bindgen(js_name = insertSchema)]
    pub fn insert_schema(
        &mut self,
        schema_name: String,
        schema: String,
    ) -> Result<(), JsError> {
        let schemas = self.schemas_mut()?;
        schemas.insert(schema_name, schema);
        Ok(())
    }

    #[wasm_bindgen(js_name = removeSchema)]
    pub fn remove_schema(&mut self, schema_name: &str) -> Result<(), JsError> {
        let schemas = self.schemas_mut()?;
        schemas.remove(schema_name);
        Ok(())
    }
}

impl Interface {
    fn schemas_mut(
        &mut self,
    ) -> Result<&mut BTreeMap<String, SchemaFile>, JsError> {
        match self.interface_type {
            InterfaceType::Database => Ok(&mut self
                .inner
                .database
                .as_mut()
                .ok_or_else(|| {
                    JsError::from_str(
                        "Failed to retrieve inner Database interface",
                    )
                })?
                .schemas),
            InterfaceType::Storage => Ok(&mut self
                .inner
                .storage
                .as_mut()
                .ok_or_else(|| {
                    JsError::from_str(
                        "Failed to retrieve inner Storage interface",
                    )
                })?
                .schemas),
            InterfaceType::Api => Ok(&mut self
                .inner
                .api
                .as_mut()
                .ok_or_else(|| {
                    JsError::from_str("Failed to retrieve inner Api interface")
                })?
                .schemas),
        }
    }
}
