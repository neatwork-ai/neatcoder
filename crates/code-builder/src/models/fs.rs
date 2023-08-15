use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Files(pub VecDeque<String>);

impl AsRef<VecDeque<String>> for Files {
    fn as_ref(&self) -> &VecDeque<String> {
        &self.0
    }
}

impl Deref for Files {
    type Target = VecDeque<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Files {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Files {
    pub fn from_schedule(job_schedule: Value) -> Result<Self> {
        let mut files: Files = match from_value::<Files>(job_schedule["order"].clone()) {
            Ok(files) => files,
            Err(e) => {
                // Handle the error
                return Err(anyhow!(
                    "Error converting dependecy graph to `Files` struct: {e}"
                ));
            }
        };

        // Filter out files that are not rust files
        files.retain(|file| {
            if file.ends_with(".rs") {
                true
            } else {
                println!("Filtered out: {}", file);
                false
            }
        });

        Ok(files)
    }
}
