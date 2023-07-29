pub mod ai;
pub mod err;
pub mod input;
pub mod output;
pub mod serde;
pub mod utils;
pub mod workflows;

pub trait Sample {
    fn sample() -> Self;
}
