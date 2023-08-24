use anyhow::{anyhow, Result};
use std::{collections::HashMap, sync::Arc};

use super::{
    interfaces::{Interface, SchemaFile},
    jobs::Jobs,
};

#[derive(Debug)]
pub struct AppState {
    /// Initial prompt containing the specifications of the project
    pub specs: Option<String>,
    /// JSON String containing the File System Scaffold
    pub scaffold: Option<Arc<String>>,
    /// Vector of strings containing the interface config files (e.g. SQL DLLs, etc.)
    /// The HashMap represents HashMap<Interface Name, Interface>
    pub interfaces: HashMap<String, Interface>,
    /// HashMap containing all the code files in the codebase
    /// Should be read as HashMap<FileName, Code String>
    pub codebase: HashMap<String, String>,
    // TODO: This should be refactored out, potentially logged or stored in a
    // Database, instead of being part of the AppState
    /// HashMap containing all the prompts to ChatGPT
    pub raw: HashMap<String, String>,
    /// Keeps track of all the jobs performed or to be performed by the worker
    pub jobs: Jobs,
}

impl AppState {
    pub fn new(specs: String) -> Self {
        Self {
            specs: Some(specs),
            scaffold: None,
            interfaces: HashMap::new(),
            codebase: HashMap::new(),
            raw: HashMap::new(),
            jobs: Jobs::empty(),
        }
    }

    pub fn empty() -> Self {
        Self {
            specs: None,
            scaffold: None,
            interfaces: HashMap::new(),
            codebase: HashMap::new(),
            raw: HashMap::new(),
            jobs: Jobs::empty(),
        }
    }

    pub fn with_interfaces(mut self, interfaces: HashMap<String, Interface>) -> Result<Self> {
        if !self.interfaces.is_empty() {
            return Err(anyhow!("Data model already exists"));
        }

        self.interfaces = interfaces;

        Ok(self)
    }

    pub fn add_schema(
        &mut self,
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    ) -> Result<()> {
        let check = !self.interfaces.contains_key(&interface_name);

        if check {
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
}
