use anyhow::{anyhow, Result};
use std::collections::HashMap;

use super::{
    interfaces::{Interface, SchemaFile},
    jobs::Jobs,
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
#[derive(Debug)]
pub struct AppState {
    /// Initial prompt containing the specifications of the project
    pub specs: Option<String>,
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
    pub scaffold: Option<String>,
    /// Vector of strings containing the interface config files (e.g. SQL DLLs, etc.)
    /// The HashMap represents HashMap<Interface Name, Interface>
    pub interfaces: HashMap<String, Interface>,
    /// HashMap containing all the code files in the codebase
    /// Should be read as HashMap<FileName, Code String>
    // TODO: This is static and does not reflect codebase changes...
    pub codebase: HashMap<String, String>,
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
            jobs: Jobs::empty(),
        }
    }

    pub fn empty() -> Self {
        Self {
            specs: None,
            scaffold: None,
            interfaces: HashMap::new(),
            codebase: HashMap::new(),
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

    pub fn add_interface(&mut self, interface: Interface) -> Result<()> {
        let interface_name = interface.name();

        if self.interfaces.contains_key(interface_name) {
            // TODO: We need proper error escallation and communication with the client
            eprintln!("[ERROR] The interface already exists. Skipping.");

            return Err(anyhow!("Interface already exists"));
        }

        self.interfaces
            .insert(interface_name.to_string(), interface);

        Ok(())
    }

    pub fn remove_interface(&mut self, interface_name: &str) -> Result<()> {
        if !self.interfaces.contains_key(interface_name) {
            // TODO: We need proper error escallation and communication with the client
            eprintln!("[ERROR] The interface does not exist. Skipping.");

            return Err(anyhow!("Interface does not exist"));
        }

        self.interfaces.remove(interface_name);

        Ok(())
    }
}
