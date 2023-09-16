use js_sys::JsString;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::models::task_params::TaskType;

use super::task_params::TaskParams;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: usize,
    pub(crate) name: String,
    pub(crate) task_params: TaskParams,
    pub status: TaskStatus,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum TaskStatus {
    Todo,
    Done,
}

#[wasm_bindgen]
impl Task {
    #[wasm_bindgen(constructor)]
    pub fn new(id: usize, name: &str, task_params: TaskParams) -> Self {
        Task {
            id,
            name: name.to_string(),
            task_params,
            status: TaskStatus::Todo,
        }
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Done;
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone().into()
    }

    #[wasm_bindgen(getter, js_name = taskParams)]
    pub fn task_params(&self) -> TaskParams {
        self.task_params.clone()
    }

    #[wasm_bindgen(js_name = taskType)]
    pub fn task_type(&self) -> TaskType {
        self.task_params.task_type
    }
}
