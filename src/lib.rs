pub mod ai;
pub mod utils;
pub mod workflows;

pub trait Sample {
    fn sample() -> Self;
}
