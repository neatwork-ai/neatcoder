//! This module provides the structure and behavior of the task pool.
//!
//! The task pool manages tasks and their parameters in both "to-do" and "done" states.

pub mod task;
pub mod task_params;
use self::{task::Task, task_params::TaskParams};
use crate::typescript::{IOrder, ITasks};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use wasm_bindgen::prelude::wasm_bindgen;
use wasmer::{JsError, WasmType};

/// Represents a pool of tasks.
///
/// This struct manages tasks in two separate pipelines: "to-do" and "done".
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskPool {
    pub counter: usize,
    pub(crate) todo: Todo,
    pub(crate) done: Done,
}

#[wasm_bindgen]
impl TaskPool {
    /// Creates a new `TaskPool` with the given counter, "to-do", and "done" pipelines.
    #[wasm_bindgen(constructor)]
    pub fn new(counter: usize, todo: Todo, done: Done) -> Self {
        Self {
            counter,
            todo,
            done,
        }
    }

    /// Creates a new empty `TaskPool`.
    pub fn empty() -> Self {
        Self {
            counter: 0,
            todo: Pipeline::empty(),
            done: Pipeline::empty(),
        }
    }

    /// Adds a task to the "to-do" pipeline.
    ///
    /// Returns the ID of the newly added task.
    pub fn add_todo(
        &mut self,
        name: &str,
        description: &str,
        task_params: TaskParams,
    ) -> usize {
        self.counter += 1;
        let task_id = self.counter;

        self.todo
            .push_back(Task::new(task_id, name, description, task_params));

        task_id
    }

    /// Removes and returns a task from the "to-do" pipeline by its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the task ID is not found in the "to-do" pipeline.
    pub fn pop_todo(&mut self, task_id: usize) -> Result<Task, JsError> {
        self.todo.remove(task_id).ok_or_else(|| {
            JsError::from_str(&format!(
                "Failed to retrieve to-do task by id: {:?}",
                task_id
            ))
        })
    }

    /// Removes and returns a task from the "done" pipeline by its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the task ID is not found in the "done" pipeline.
    pub fn pop_done(&mut self, task_id: usize) -> Result<Task, JsError> {
        self.done.remove(task_id).ok_or_else(|| {
            JsError::from_str(&format!(
                "Failed to retrieve done task by id: {:?}",
                task_id
            ))
        })
    }

    /// Adds a task to the "done" pipeline.
    pub fn add_done(&mut self, task: Task) {
        self.done.push_back(task);
    }

    /// Adds a task back to the "to-do" pipeline.
    pub fn add_back_todo(&mut self, task: Task) {
        self.todo.push_back(task);
    }

    /// Marks a task as complete by its ID and moves it to the "done" pipeline.
    ///
    /// # Errors
    ///
    /// Returns error if the task ID is not found in the "to-do" pipeline.
    pub fn finish_task_by_id(&mut self, task_id: usize) -> Result<(), JsError> {
        let mut task = self.todo.remove(task_id).ok_or_else(|| {
            JsError::from_str(&format!(
                "Failed to retrieve task id: {:?}",
                task_id
            ))
        })?;

        task.complete();

        self.done.push_back(task);

        Ok(())
    }

    /// Marks the first task in the "to-do" pipeline as complete and moves it to the "done" pipeline.
    ///
    /// # Errors
    ///
    /// Return error if there are no tasks in the "to-do" pipeline.
    pub fn finish_task_by_order(&mut self) -> Result<(), JsError> {
        let mut task = self.todo.pop_front().ok_or_else(|| {
            JsError::from_str("Could not find any task in the todo list")
        })?;

        task.complete();

        self.done.push_back(task);

        Ok(())
    }
}

pub type Todo = Pipeline;
pub type Done = Pipeline;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    pub(crate) tasks: BTreeMap<usize, Task>,
    pub(crate) order: VecDeque<usize>,
}

#[wasm_bindgen]
impl Pipeline {
    #[wasm_bindgen(constructor)]
    pub fn new(tasks: ITasks, order: IOrder) -> Result<Pipeline, JsError> {
        let task_bm = BTreeMap::from_extern(tasks)?;
        let order_bm = VecDeque::from_extern(order)?;

        Ok(Self {
            tasks: task_bm,
            order: order_bm,
        })
    }

    pub fn empty() -> Self {
        Self {
            tasks: BTreeMap::new(),
            order: VecDeque::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn tasks(&self) -> Result<ITasks, JsError> {
        BTreeMap::to_extern(self.tasks.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn order(&self) -> Result<IOrder, JsError> {
        VecDeque::to_extern(self.order.clone())
    }
}

impl Pipeline {
    pub fn new_(tasks: BTreeMap<usize, Task>, order: VecDeque<usize>) -> Self {
        Self { tasks, order }
    }

    pub fn push_front(&mut self, task: Task) {
        let task_id = task.id;

        self.tasks.insert(task_id, task);
        self.order.push_front(task_id);
    }

    pub fn push_back(&mut self, task: Task) {
        let task_id = task.id;

        self.tasks.insert(task.id, task);
        self.order.push_back(task_id);
    }

    pub fn pop_front(&mut self) -> Option<Task> {
        let task_id = self.order.pop_front();

        if let Some(task_id) = task_id {
            self.tasks.remove(&task_id)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<Task> {
        let task_id = self.order.pop_back();

        if let Some(task_id) = task_id {
            self.tasks.remove(&task_id)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&Task> {
        let task_id = self.order.front();

        if let Some(task_id) = task_id {
            self.tasks.get(task_id)
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&Task> {
        let task_id = self.order.back();

        if let Some(task_id) = task_id {
            self.tasks.get(task_id)
        } else {
            None
        }
    }

    pub fn remove(&mut self, task_id: usize) -> Option<Task> {
        let task = self.tasks.remove(&task_id);

        if task.is_some() {
            self.order.retain(|&id| id != task_id);
        }

        task
    }
}

pub struct PipelineIterator {
    tasks: BTreeMap<usize, Task>,
    order: VecDeque<usize>,
}

impl Iterator for PipelineIterator {
    type Item = (usize, Task);

    fn next(&mut self) -> Option<Self::Item> {
        let task_id = self.order.pop_front()?;
        let task = self.tasks.remove(&task_id)?;
        Some((task_id, task))
    }
}

impl Pipeline {
    pub fn drain(&mut self) -> PipelineIterator {
        let tasks = std::mem::replace(&mut self.tasks, BTreeMap::new());
        let order = std::mem::replace(&mut self.order, VecDeque::new());

        PipelineIterator { tasks, order }
    }
}
