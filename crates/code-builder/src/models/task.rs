use anyhow::{anyhow, Result};
use js_sys::Error;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub(crate) name: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub enum TaskType {
    ScaffoldProject,
    BuildExecutionPlan,
    CodeGen,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[wasm_bindgen]
impl Task {
    pub fn new_todo(id: usize, name: &str, task_type: TaskType) -> Self {
        Task {
            id,
            name: name.to_string(),
            task_type,
            status: TaskStatus::Todo,
        }
    }

    pub fn new_in_progress(id: usize, name: &str, task_type: TaskType) -> Self {
        Self {
            id,
            name: name.to_string(),
            task_type,
            status: TaskStatus::InProgress,
        }
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        match self.status {
            TaskStatus::Todo => {
                self.status = TaskStatus::InProgress;

                Ok(())
            }
            _ => Err(anyhow!("Task has already been initialized"))
                .map_err(|e| Error::new(&e.to_string()).into()),
        }
    }

    pub fn complete(&mut self) -> Result<(), JsValue> {
        self.status = TaskStatus::Done;

        Ok(())
    }

    #[wasm_bindgen(getter, js_name = name)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
