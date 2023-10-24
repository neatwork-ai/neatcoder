//! This module defines the structure and behavior of tasks.
//!
//! It provides a `Task` struct that represents an individual task,
//! with associated task parameters and status.

use super::task_params::{TaskParams, TaskType};
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents a task with associated parameters and status.
///
/// This struct is serializable and can be used with WebAssembly
/// through the provided `wasm_bindgen` annotations.
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: usize,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) task_params: TaskParams,
    pub status: TaskStatus,
}

/// Represents the possible statuses a task can have.
#[wasm_bindgen]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum TaskStatus {
    Todo,
    Done,
}

#[wasm_bindgen]
impl Task {
    /// Creates a new `Task` with the given parameters.
    ///
    /// By default, the task status is set to `Todo`.
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: usize,
        name: &str,
        description: &str,
        task_params: TaskParams,
    ) -> Self {
        Task {
            id,
            name: name.to_string(),
            description: description.to_string(),
            task_params,
            status: TaskStatus::Todo,
        }
    }

    /// Marks the task as complete by setting its status to `Done`.
    pub fn complete(&mut self) {
        self.status = TaskStatus::Done;
    }

    /// Returns the name of the task.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone().into()
    }

    /// Returns the description of the task.
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> JsString {
        self.description.clone().into()
    }

    /// Returns the task parameters.
    #[wasm_bindgen(getter, js_name = taskParams)]
    pub fn task_params(&self) -> TaskParams {
        self.task_params.clone()
    }

    /// Returns the type of the task from the associated task parameters.
    #[wasm_bindgen(js_name = taskType)]
    pub fn task_type(&self) -> TaskType {
        self.task_params.task_type
    }
}
