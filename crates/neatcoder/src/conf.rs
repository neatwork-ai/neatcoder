use serde::Deserialize;

use crate::prelude::*;

/// Loads configuration from env variables.
#[derive(Deserialize, Debug, Clone)]
pub struct Conf {
    pub openai_api_key: String,
    #[serde(default = "defaults::llm_temperature")]
    pub llm_temperature: f64,
    #[serde(default = "defaults::llm_top_p")]
    pub llm_top_p: f64,
}

mod defaults {
    pub fn llm_temperature() -> f64 {
        0.7
    }

    pub fn llm_top_p() -> f64 {
        0.9
    }
}

impl Conf {
    pub fn from_env() -> Result<Self> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}