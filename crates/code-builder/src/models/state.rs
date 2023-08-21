use anyhow::{anyhow, Result};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::jobs::Jobs;

#[derive(Debug)]
pub struct AppState {
    /// Initial prompt containing the specifications of the project
    pub specs: Option<String>,
    /// JSON String containing the File System Scaffold
    pub scaffold: Option<Arc<String>>,
    /// Vector of strings containing the interface config files (e.g. SQL DLLs, etc.)
    pub interfaces: Vec<String>,
    /// HashMap containing all the code files in the codebase
    /// Should be read as HashMap<FileName, Code String>
    pub codebase: Mutex<HashMap<String, String>>,
    // TODO: This should be refactored out, potentially logged or stored in a
    // Database, instead of being part of the AppState
    /// HashMap containing all the prompts to ChatGPT
    pub raw: Mutex<HashMap<String, String>>,
    /// Keeps track of all the jobs performed or to be performed by the worker
    pub jobs: Mutex<Jobs>,
}

impl AppState {
    pub fn new(specs: String) -> Self {
        Self {
            specs: Some(specs),
            scaffold: None,
            interfaces: Vec::new(),
            codebase: Mutex::new(HashMap::new()),
            raw: Mutex::new(HashMap::new()),
            jobs: Mutex::new(Jobs::empty()),
        }
    }

    pub fn empty() -> Self {
        Self {
            specs: None,
            scaffold: None,
            interfaces: Vec::new(),
            codebase: Mutex::new(HashMap::new()),
            raw: Mutex::new(HashMap::new()),
            jobs: Mutex::new(Jobs::empty()),
        }
    }

    pub fn with_interfaces(mut self, interfaces: Vec<String>) -> Result<Self> {
        if !self.interfaces.is_empty() {
            return Err(anyhow!("Data model already exists"));
        }

        self.interfaces = interfaces;

        Ok(self)
    }
}
