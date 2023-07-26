use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::commit::NodeID;

pub trait JobData: Debug {}

pub enum Job {
    Human,
    LLM(LLMJob),
    Program(ProgramJob),
}

pub struct LLMJob {
    pub data: Box<dyn JobData>,
}

pub struct ProgramJob {
    pub data: Box<dyn JobData>,
}

#[derive(Default)]
pub struct Jobs(pub HashMap<NodeID, Rc<Job>>);

// Deref coercion
impl Deref for Jobs {
    type Target = HashMap<NodeID, Rc<Job>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement DerefMut trait for your custom type
impl DerefMut for Jobs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
