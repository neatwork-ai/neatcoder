use std::fmt::Debug;

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
