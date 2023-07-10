pub mod ai;
pub mod utils;
pub mod workflows;

use anyhow::Result;

pub trait Sample {
    fn sample() -> Self;

    fn sample_json() -> Result<String>;
}
