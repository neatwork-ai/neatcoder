use crate::utils::map_to_jsvalue;

use super::task::{Task, TaskType};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::collections::{HashMap, VecDeque};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskPool {
    pub counter: usize,
    pub(crate) todo: Todo,
    pub(crate) in_progress: InProgress,
    pub(crate) done: Done,
}

#[wasm_bindgen]
impl TaskPool {
    pub fn new(
        counter: usize,
        todo: Todo,
        in_progress: InProgress,
        done: Done,
    ) -> Self {
        Self {
            counter,
            todo,
            in_progress,
            done,
        }
    }

    pub fn empty() -> Self {
        Self {
            counter: 0,
            todo: Pipeline::empty(),
            in_progress: Pipeline::empty(),
            done: Pipeline::empty(),
        }
    }

    pub fn add_todo(&mut self, name: &str, task_type: TaskType) -> usize {
        self.counter += 1;
        let task_id = self.counter;

        self.todo
            .push_back(Task::new_todo(task_id, name, task_type));

        task_id
    }

    pub fn add_in_progress(
        &mut self,
        name: &str,
        task_type: TaskType,
    ) -> usize {
        self.counter += 1;
        let task_id = self.counter;

        self.in_progress
            .push_back(Task::new_in_progress(task_id, name, task_type));

        task_id
    }

    pub fn start_task_by_id(&mut self, task_id: usize) -> Result<(), JsValue> {
        let mut task = self
            .todo
            .remove(task_id)
            .expect("Could not find task in todo list");

        task.start()?;

        self.in_progress.push_back(task);

        Ok(())
    }

    pub fn start_task_by_order(&mut self) -> Result<(), JsValue> {
        let mut task = self
            .todo
            .pop_front()
            .expect("Could not find any task in the todo list");

        task.start()?;

        self.in_progress.push_back(task);

        Ok(())
    }

    pub fn finish_task_by_id(&mut self, task_id: usize) -> Result<(), JsValue> {
        let mut task = self
            .in_progress
            .remove(task_id)
            .expect("Could not find task in todo list");

        task.complete()?;

        self.done.push_back(task);

        Ok(())
    }

    pub fn finish_task_by_order(&mut self) -> Result<(), JsValue> {
        let mut task = self
            .in_progress
            .pop_front()
            .expect("Could not find any task in the todo list");

        task.complete()?;

        self.done.push_back(task);

        Ok(())
    }
}

pub type Todo = Pipeline;
pub type InProgress = Pipeline;
pub type Stopped = Pipeline;
pub type Done = Pipeline;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub(crate) tasks: HashMap<usize, Task>,
    pub(crate) order: VecDeque<usize>,
}

#[wasm_bindgen]
impl Pipeline {
    pub fn empty() -> Self {
        Self {
            tasks: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    #[wasm_bindgen(getter, js_name = tasks)]
    pub fn get_tasks(&self) -> JsValue {
        map_to_jsvalue::<usize, Task>(&self.tasks)
    }

    #[wasm_bindgen(getter, js_name = order)]
    pub fn get_order(&self) -> JsValue {
        let vec: Vec<usize> = self.order.clone().into();
        to_value(&vec).unwrap()
    }
}

impl Pipeline {
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
    tasks: HashMap<usize, Task>,
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
        let tasks = std::mem::replace(&mut self.tasks, HashMap::new());
        let order = std::mem::replace(&mut self.order, VecDeque::new());

        PipelineIterator { tasks, order }
    }
}
