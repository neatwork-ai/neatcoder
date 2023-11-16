use anyhow::{anyhow, Result};
use oai::models::assistant::CustomGPT;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

impl LocalRead for CustomGPT {}
impl LocalWrite for CustomGPT {}

pub fn get_maker_path() -> PathBuf {
    let mut filepath = dirs::home_dir().unwrap();
    filepath.push(format!(".gpt-maker/gpts.json"));

    filepath
}

pub trait LocalRead: DeserializeOwned {
    fn read(path_buf: &PathBuf) -> Option<Result<Self>> {
        match File::open(path_buf) {
            Ok(file) => Some(serde_json::from_reader(file).map_err(Into::into)),
            Err(_) => None,
        }
    }
}

pub trait LocalWrite: Serialize {
    fn write(&self, output_file: &Path) -> Result<()> {
        // Create the parent directories if they don't exist
        fs::create_dir_all(output_file.parent().unwrap())?;

        let file = File::create(output_file).map_err(|err| {
            anyhow!(
                r#"Could not create file "{}": {err}"#,
                output_file.display()
            )
        })?;

        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let ser = &mut serde_json::Serializer::with_formatter(file, formatter);
        self.serialize(ser).map_err(|err| {
            anyhow!(
                r#"Could not write file "{}": {err}"#,
                output_file.display()
            )
        })
    }
}
