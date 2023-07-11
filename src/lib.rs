pub mod ai;
pub mod input;
pub mod output;
pub mod utils;
pub mod workflows;

pub trait Sample {
    fn sample() -> Self;
}
