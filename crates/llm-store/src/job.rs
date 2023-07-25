use std::fmt::Debug;

pub trait JobData: Debug {}

pub struct LLMJob {
    pub data: Box<dyn JobData>,
}

pub struct ProgramJob {
    pub data: Box<dyn JobData>,
}
