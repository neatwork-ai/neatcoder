use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct AppState {
    pub specs: String,
    pub fs: Option<Arc<String>>,
    pub data_model: Option<Vec<String>>,
    pub files: Mutex<HashMap<String, String>>,
    pub raw: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new(specs: String) -> Self {
        Self {
            specs,
            fs: None,
            data_model: None,
            files: Mutex::new(HashMap::new()),
            raw: Mutex::new(HashMap::new()),
        }
    }
}
