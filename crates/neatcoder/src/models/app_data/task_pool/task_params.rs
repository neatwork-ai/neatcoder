//! This module defines the structure and behavior of task parameters.
//!
//! It provides a `TaskParams` struct that represents parameters of a task,
//! including the task type and associated inner parameters.

use crate::endpoints::{
    scaffold_project::ScaffoldParams, stream_code::CodeGenParams,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::any::Any;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasmer::{JsError, log};

/// Represents parameters for a task.
///
/// These parameters include the task type and associated inner parameters.
#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskParams {
    pub(crate) task_type: TaskType,
    pub(crate) inner: TaskParamsInner,
}

/// Specifies the type of task.
///
/// This enum helps to categorize tasks into different types.
#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub enum TaskType {
    ScaffoldProject,
    CodeGen,
}

/// Holds the actual parameters for the task based on its type.
///
/// Depending on the task type, one of the fields will be populated.
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskParamsInner {
    pub(crate) scaffold_project: Option<ScaffoldParams>,
    pub(crate) stream_code: Option<CodeGenParams>,
}

#[wasm_bindgen]
impl TaskParams {
    /// Creates a new `TaskParams` with the given task type and inner parameters.
    #[wasm_bindgen(constructor)]
    pub fn new(task_type: TaskType, inner: TaskParamsInner) -> TaskParams {
        Self { task_type, inner }
    }

    /// Returns the inner parameters of the task.
    #[wasm_bindgen(getter)]
    pub fn inner(&self) -> TaskParamsInner {
        self.inner.clone()
    }

    /// Retrieves the scaffold project parameters if the task type is `ScaffoldProject`.
    #[wasm_bindgen(getter, js_name = scaffoldProject)]
    pub fn scaffold_project(&self) -> Option<ScaffoldParams> {
        match self.task_type {
            TaskType::ScaffoldProject => self.inner.scaffold_project.clone(),
            _ => None,
        }
    }

    /// Retrieves the stream code parameters if the task type is `CodeGen`.
    #[wasm_bindgen(getter, js_name = streamCode)]
    pub fn stream_code(&self) -> Option<CodeGenParams> {
        match self.task_type {
            TaskType::CodeGen => {
                log(&format!(
                    "RETURNING: {:?}",
                    self.inner.stream_code.clone(),
                ));

                self.inner.stream_code.clone()
            }
            _ => None,
        }
    }

    /// Returns the type of the task.
    #[wasm_bindgen(getter, js_name = taskType)]
    pub fn task_type(&self) -> TaskType {
        self.task_type
    }
}

#[wasm_bindgen]
impl TaskParamsInner {
    /// Creates a new `TaskParamsInner` with the given scaffold project and stream code parameters.
    ///
    /// This constructor ensures that only one set of parameters is provided.
    ///
    /// # Errors
    ///
    /// Returns an error if both `scaffold_project` and `stream_code` parameters are provided.
    #[wasm_bindgen(constructor)]
    pub fn new(
        scaffold_project: Option<ScaffoldParams>,
        stream_code: Option<CodeGenParams>,
    ) -> Result<TaskParamsInner, JsValue> {
        if scaffold_project.is_some() && stream_code.is_some() {
            return Err(anyhow!("Cannot accept multiple parameter types."))
                .map_err(|e| JsError::from_str(&e.to_string()));
        }
        Ok(Self {
            scaffold_project,
            stream_code,
        })
    }

    /// Returns the scaffold project parameters if they exist.
    #[wasm_bindgen(getter, js_name = scaffoldProject)]
    pub fn scaffold_project(&self) -> Option<ScaffoldParams> {
        self.scaffold_project.clone()
    }

    /// Returns the stream code parameters if they exist.
    #[wasm_bindgen(getter, js_name = streamCode)]
    pub fn stream_code(&self) -> Option<CodeGenParams> {
        self.stream_code.clone()
    }
}

impl TaskParams {
    /// Creates a new `TaskParams` instance using dynamic type checking.
    ///
    /// This function tries to downcast the provided inner parameters based on the task type.
    ///
    /// # Errors
    ///
    /// Returns an error if the downcast fails for the provided task type.
    pub fn new_(task_type: TaskType, inner: Box<dyn Any>) -> Result<Self> {
        match task_type {
            TaskType::ScaffoldProject => {
                if let Some(scaffold) = inner.downcast_ref::<ScaffoldParams>() {
                    Ok(TaskParams {
                        task_type,
                        inner: TaskParamsInner {
                            scaffold_project: Some(scaffold.clone()),
                            stream_code: None,
                        },
                    })
                } else {
                    Err(anyhow!("Failed to downcast to ScaffoldProject"))
                }
            }
            TaskType::CodeGen => {
                if let Some(code_gen) = inner.downcast_ref::<CodeGenParams>() {
                    Ok(TaskParams {
                        task_type,
                        inner: TaskParamsInner {
                            scaffold_project: None,
                            stream_code: Some(code_gen.clone()),
                        },
                    })
                } else {
                    Err(anyhow!("Failed to downcast to CodeGen"))
                }
            }
        }
    }

    /// Retrieves a reference to the scaffold project parameters if the task type is `ScaffoldProject`.
    pub fn scaffold_project_(&self) -> Option<&ScaffoldParams> {
        match self.task_type {
            TaskType::ScaffoldProject => self.inner.scaffold_project.as_ref(),
            _ => None,
        }
    }

    /// Retrieves a reference to the stream code parameters if the task type is `CodeGen`.
    pub fn stream_code_(&self) -> Option<&CodeGenParams> {
        match self.task_type {
            TaskType::CodeGen => self.inner.stream_code.as_ref(),
            _ => None,
        }
    }
}
