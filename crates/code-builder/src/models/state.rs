use anyhow::{anyhow, Result};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub specs: Option<String>,
    pub fs: Option<Arc<String>>,
    pub data_model: Option<Vec<String>>,
    pub files: Mutex<HashMap<String, String>>,
    pub raw: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new(specs: String) -> Self {
        Self {
            specs: Some(specs),
            fs: None,
            data_model: None,
            files: Mutex::new(HashMap::new()),
            raw: Mutex::new(HashMap::new()),
        }
    }

    pub fn empty() -> Self {
        Self {
            specs: None,
            fs: None,
            data_model: None,
            files: Mutex::new(HashMap::new()),
            raw: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_model(mut self, data_model: Vec<String>) -> Result<Self> {
        if self.data_model.is_some() {
            return Err(anyhow!("Data model already exists"));
        }

        self.data_model = Some(data_model);

        Ok(self)
    }
}
