use std::any::Any;

use crate::{
    endpoints::{scaffold_project::ScaffoldParams, stream_code::CodeGenParams},
    utils::log,
    JsError,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskParams {
    pub(crate) task_type: TaskType,
    pub(crate) inner: TaskParamsInner,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub enum TaskType {
    ScaffoldProject,
    BuildExecutionPlan,
    CodeGen,
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskParamsInner {
    pub(crate) scaffold_project: Option<ScaffoldParams>,
    pub(crate) stream_code: Option<CodeGenParams>,
}

#[wasm_bindgen]
impl TaskParams {
    #[wasm_bindgen(constructor)]
    pub fn new(task_type: TaskType, inner: TaskParamsInner) -> TaskParams {
        Self { task_type, inner }
    }

    #[wasm_bindgen(getter)]
    pub fn inner(&self) -> TaskParamsInner {
        self.inner.clone()
    }

    #[wasm_bindgen(getter, js_name = scaffoldProject)]
    pub fn scaffold_project(&self) -> Option<ScaffoldParams> {
        match self.task_type {
            TaskType::ScaffoldProject => self.inner.scaffold_project.clone(),
            _ => None,
        }
    }

    #[wasm_bindgen(getter, js_name = streamCode)]
    pub fn stream_code(&self) -> Option<CodeGenParams> {
        log(&format!(
            "TASK TYPE SHOULD MATCH: TaskType::CodeGen => {:?}",
            self.task_type,
        ));

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

    #[wasm_bindgen(getter, js_name = taskType)]
    pub fn task_type(&self) -> TaskType {
        self.task_type
    }
}

#[wasm_bindgen]
impl TaskParamsInner {
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

    #[wasm_bindgen(getter, js_name = scaffoldProject)]
    pub fn scaffold_project(&self) -> Option<ScaffoldParams> {
        self.scaffold_project.clone()
    }

    #[wasm_bindgen(getter, js_name = streamCode)]
    pub fn stream_code(&self) -> Option<CodeGenParams> {
        self.stream_code.clone()
    }
}

impl TaskParams {
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
            TaskType::BuildExecutionPlan => Ok(TaskParams {
                task_type,
                inner: TaskParamsInner {
                    scaffold_project: None,
                    stream_code: None,
                },
            }),
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

    pub fn scaffold_project_(&self) -> Option<&ScaffoldParams> {
        match self.task_type {
            TaskType::ScaffoldProject => self.inner.scaffold_project.as_ref(),
            _ => None,
        }
    }

    pub fn stream_code_(&self) -> Option<&CodeGenParams> {
        match self.task_type {
            TaskType::CodeGen => self.inner.stream_code.as_ref(),
            _ => None,
        }
    }
}
