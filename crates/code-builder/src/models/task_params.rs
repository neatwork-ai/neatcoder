use std::any::Any;

use crate::endpoints::{
    scaffold_project::ScaffoldProject, stream_code::CodeGen,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskParams {
    pub task_type: TaskType,
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
pub struct TaskParamsInner {
    scaffold_project: Option<ScaffoldProject>,
    stream_code: Option<CodeGen>,
}

#[wasm_bindgen]
impl TaskParams {
    #[wasm_bindgen(getter, js_name = scaffoldProject)]
    pub fn get_scaffold_project(&self) -> JsValue {
        match &self.inner.scaffold_project {
            Some(scaffold_project) => scaffold_project.clone().into(),
            None => JsValue::NULL,
        }
    }

    #[wasm_bindgen(getter, js_name = streamCode)]
    pub fn get_stream_code(&self) -> JsValue {
        match &self.inner.stream_code {
            Some(stream_code) => stream_code.clone().into(),
            None => JsValue::NULL,
        }
    }
}

#[wasm_bindgen]
impl TaskParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        task_type: TaskType,
        inner: JsValue,
    ) -> Result<TaskParams, JsValue> {
        match task_type {
            TaskType::ScaffoldProject => Ok(TaskParams {
                task_type,
                inner: TaskParamsInner {
                    scaffold_project: Some(
                        serde_wasm_bindgen::from_value(inner)
                            .expect("Failed to cast to type `ScaffoldProject`"),
                    ),
                    stream_code: None,
                },
            }),
            TaskType::BuildExecutionPlan => Ok(TaskParams {
                task_type,
                inner: TaskParamsInner {
                    scaffold_project: None,
                    stream_code: None,
                },
            }),
            TaskType::CodeGen => Ok(TaskParams {
                task_type,
                inner: TaskParamsInner {
                    scaffold_project: None,
                    stream_code: Some(
                        serde_wasm_bindgen::from_value(inner)
                            .expect("Failed to cast to type `ScaffoldProject`"),
                    ),
                },
            }),
        }
    }
}

impl TaskParams {
    pub fn new_(task_type: TaskType, inner: Box<dyn Any>) -> Result<Self> {
        match task_type {
            TaskType::ScaffoldProject => {
                if let Some(scaffold) = inner.downcast_ref::<ScaffoldProject>()
                {
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
                if let Some(code_gen) = inner.downcast_ref::<CodeGen>() {
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

    pub fn scaffold_project(self) -> Result<ScaffoldProject> {
        match self.task_type {
            TaskType::ScaffoldProject => match self.inner.scaffold_project {
                Some(scaffold_project) => Ok(scaffold_project),
                None => return Err(anyhow!("No scaffold project field")),
            },
            _ => {
                return Err(anyhow!(
                    "No scaffold project field. This error should not occur."
                ))
            }
        }
    }

    pub fn stream_code(self) -> Result<CodeGen> {
        match self.task_type {
            TaskType::CodeGen => match self.inner.stream_code {
                Some(stream_code) => Ok(stream_code),
                None => return Err(anyhow!("No stream code field")),
            },
            _ => {
                return Err(anyhow!(
                    "No stream code field. This error should not occur."
                ))
            }
        }
    }
}
