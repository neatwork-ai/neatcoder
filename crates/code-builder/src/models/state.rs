use anyhow::{anyhow, Result};
use js_sys::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::{
    endpoints::{
        execution_plan::build_execution_plan,
        scaffold_project::{scaffold_project, ScaffoldProject},
        stream_code::{stream_code, CodeGen},
    },
    openai::{client::OpenAI, params::OpenAIParams},
    utils::{jsvalue_to_map, map_to_jsvalue},
};

use super::{
    interfaces::{Interface, SchemaFile},
    task_pool::TaskPool,
};

// NOTE: We will need to perform the following improvements to the data model:
//
// 1. The extension itself will be interactive, and will not rely solely on an
// initial prompt but rather a sequence of prompts, or even a tree of prompts.
// there are different models we can use to model this. We can think of modeling
// as a chat app like Slack, in which each message can have a Thread or we can
// generalise it further to something more intricate.
//
// 2. This struct is storing static application data such as `scaffold`, `codebase`.
// we will need to find a way to make the application state dynamic such that it reflects the
// current state of the codebase at any given time. We should also consider if
// have the field `codebase` makes sense here, because we can also access the codebase
// via the Language Server on the client side.
//
/// Acts as a shared application data (i.e. shared state). It contains
/// information related to the initial prompt, the scaffold of the project, its
/// interfaces, and current jobs in the TODO pipeline among others (see `Jobs`).
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppState {
    #[serde(skip_serializing, skip_deserializing)]
    listeners: Vec<js_sys::Function>,
    /// Initial prompt containing the specifications of the project
    pub(crate) specs: Option<String>,
    /// JSON String containing the File System Scaffold
    /// Example:
    /// ```json
    /// {
    ///     "src": {
    ///       "config.rs": "Module for handling configuration variables",
    ///       "db.rs": "Module for establishing and managing database connections",
    ///       "handlers": {
    ///         "company.rs": "Module for handling company-related API endpoints",
    ///         "customer.rs": "Module for handling customer-related API endpoints",
    ///         "order.rs": "Module for handling order-related API endpoints",
    ///         "product.rs": "Module for handling product-related API endpoints"
    ///       },
    ///       "main.rs": "Main entry point of the API server",
    ///       "models": {
    ///         "company.rs": "Module defining the Company struct and its database operations",
    ///         "customer.rs": "Module defining the Customer struct and its database operations",
    ///         "order.rs": "Module defining the Order struct and its database operations",
    ///         "product.rs": "Module defining the Product struct and its database operations"
    ///       },
    ///       "routes.rs": "Module for defining API routes and their corresponding handlers",
    ///       "utils.rs": "Module for utility functions and helper methods"
    ///     }
    ///   }
    /// ```
    pub(crate) scaffold: Option<String>,
    /// Vector of strings containing the interface config files (e.g. SQL DLLs, etc.)
    /// The HashMap represents HashMap<Interface Name, Interface>
    pub(crate) interfaces: HashMap<String, Interface>,
    pub(crate) task_pool: TaskPool,
}

#[wasm_bindgen]
impl AppState {
    #[wasm_bindgen(constructor)]
    pub fn new(specs: String, task_pool: TaskPool) -> Self {
        Self {
            listeners: Vec::new(),
            specs: Some(specs),
            scaffold: None,
            interfaces: HashMap::new(),
            task_pool,
        }
    }

    pub fn empty() -> Self {
        Self {
            listeners: Vec::new(),
            specs: None,
            scaffold: None,
            interfaces: HashMap::new(),
            task_pool: TaskPool::empty(),
        }
    }

    pub fn subscribe(&mut self, callback: &js_sys::Function) {
        self.listeners.push(callback.clone());
    }

    #[wasm_bindgen(getter, js_name = specs)]
    pub fn get_specs(&self) -> JsValue {
        match &self.specs {
            Some(s) => JsValue::from_str(s),
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(getter, js_name = scaffold)]
    pub fn get_scaffold(&self) -> JsValue {
        match &self.scaffold {
            Some(s) => JsValue::from_str(s),
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(getter, js_name = interfaces)]
    pub fn get_interfaces(&self) -> JsValue {
        map_to_jsvalue::<String, Interface>(&self.interfaces)
    }

    #[wasm_bindgen(setter = setInterface)]
    pub fn set_interfaces(
        &mut self,
        interfaces: JsValue,
    ) -> Result<(), JsValue> {
        if !self.interfaces.is_empty() {
            return Err(anyhow!("Data model already exists"))
                .map_err(|e| Error::new(&e.to_string()).into());
        }

        let interfaces = jsvalue_to_map::<Interface>(&interfaces);
        self.interfaces = interfaces;

        Ok(())
    }

    #[wasm_bindgen(js_name = addSchema)]
    pub fn add_schema(
        &mut self,
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    ) -> Result<(), JsValue> {
        self.add_schema_(interface_name, schema_name, schema)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = removeSchema)]
    pub fn remove_schema(
        &mut self,
        interface_name: &str,
        schema_name: &str,
    ) -> Result<(), JsValue> {
        self.remove_schema_(interface_name, schema_name)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = addInterface)]
    pub fn add_interface(
        &mut self,
        interface: Interface,
    ) -> Result<(), JsValue> {
        self.add_interface_(interface)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = removeInterface)]
    pub fn remove_interface(
        &mut self,
        interface_name: &str,
    ) -> Result<(), JsValue> {
        self.remove_interface_(interface_name)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = scaffoldProject)]
    pub async fn scaffold_project(
        &mut self,
        client: &OpenAI,
        ai_params: &OpenAIParams,
        client_params: ScaffoldProject,
    ) -> Result<(), JsValue> {
        let scaffold_json =
            scaffold_project(client, ai_params, client_params, self)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.scaffold = Some(scaffold_json.to_string());

        Ok(())
    }

    #[wasm_bindgen(js_name = buildExecutionPlan)]
    pub async fn build_execution_plan(
        &mut self,
        client: &OpenAI,
        ai_params: &OpenAIParams,
    ) -> Result<(), JsValue> {
        build_execution_plan(client, ai_params, self)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(())
    }

    #[wasm_bindgen(js_name = streamCode)]
    pub async fn stream_code(
        &mut self,
        client: &OpenAI,
        ai_params: &OpenAIParams,
        client_params: CodeGen,
    ) -> Result<(), JsValue> {
        stream_code(client, ai_params, client_params, self)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(())
    }
}

impl AppState {
    fn add_schema_(
        &mut self,
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    ) -> Result<()> {
        if !self.interfaces.contains_key(&interface_name) {
            // TODO: We need proper error escallation and communication with the client
            eprintln!("[ERROR] The interface does not exist. Please create an interface first.");

            return Err(anyhow!("Interface does not exist"));
        }

        // Safe to unwrap due to previous check
        let interface = self.interfaces.get_mut(&interface_name).unwrap();

        // Replaces the existing interface if any
        interface.insert_schema(schema_name, schema);

        Ok(())
    }

    pub fn remove_schema_(
        &mut self,
        interface_name: &str,
        schema_name: &str,
    ) -> Result<()> {
        if !self.interfaces.contains_key(interface_name) {
            // TODO: We need proper error escallation and communication with the client
            eprintln!("[ERROR] The interface does not exist.");

            return Err(anyhow!("Interface does not exist"));
        }

        // Safe to unwrap due to previous check
        let interface = self.interfaces.get_mut(interface_name).unwrap();

        // Replaces the existing interface if any
        interface.remove_schema(schema_name);

        Ok(())
    }

    pub fn add_interface_(&mut self, interface: Interface) -> Result<()> {
        let interface_name = interface.get_name();

        if self.interfaces.contains_key(&interface_name) {
            // TODO: We need proper error escallation and communication with the client
            eprintln!("[ERROR] The interface already exists. Skipping.");

            return Err(anyhow!("Interface already exists"));
        }

        self.interfaces
            .insert(interface_name.to_string(), interface);

        Ok(())
    }

    pub fn remove_interface_(&mut self, interface_name: &str) -> Result<()> {
        if !self.interfaces.contains_key(interface_name) {
            // TODO: We need proper error escallation and communication with the client
            eprintln!("[ERROR] The interface does not exist. Skipping.");

            return Err(anyhow!("Interface does not exist"));
        }

        self.interfaces.remove(interface_name);

        Ok(())
    }
}

impl AppState {
    fn trigger_callbacks(&self) {
        // Notify listeners of event A
        for callback in &self.listeners {
            let this = JsValue::NULL;
            let _ = callback.call0(&this);
        }
    }
}
