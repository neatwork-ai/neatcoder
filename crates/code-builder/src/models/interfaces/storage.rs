use anyhow::Result;
use gluon::ai::openai::msg::{GptRole, OpenAIMsg};
use std::fmt::{self, Display};

use super::AsContext;

#[derive(Debug)]
pub struct Datastore {
    pub name: String,
    pub file_type: FileType,
    pub storage_type: StorageType,
    pub region: Option<String>,
    pub stores: Vec<Store>,
}

#[derive(Debug)]
pub struct Store {
    pub name: String,
    pub path: String,
    pub schema: String,
}

#[derive(Debug)]
pub enum StorageType {
    AwsS3,
    GoogleCloudStorage,
    FirebaseCloudStorage,
    AzureBlobStorage,
    LocalStorage,
}

#[derive(Debug)]
pub enum FileType {
    // === Data Store Formats ===
    /// A simple CSV file with a few rows should allow the LLM
    /// to infer the columns and the types
    Csv,
    /// A free format file that can be acquired via:
    /// `parquet-tools schema /path/to/your/file.parquet`
    Parquet,
    /// JSON File that can be acquired via:
    /// `avro-tools getschema /path/to/your/file.avro`
    Avro,
    // TODO: Feather has no schema definition and it seems
    // tha its schema definition can only be acquired at runtime
    // we should find a way to add compatability with Feather
    /// Free format file containing metadata and the schema definition
    /// for ORC files, it can be acquired via:
    /// `hive --orcfiledump /path/to/file.orc`
    ///
    /// or:
    /// `java -jar orc-tools-*.jar meta /path/to/file.orc``
    Orc,
    /// Protocol Buffers by Google - a method to serialize structured data.
    /// Often accompanied by a `.proto` file that defines the schema.
    ProtoBuf,
    /// Lightweight data-interchange format that's easy for humans to read and write.
    /// Used widely in web applications for data transmission.
    Json,
    /// Newline Delimited JSON - Each line is a valid JSON entry.
    /// Ideal for large datasets and stream processing.
    NdJson,
    /// Extensible Markup Language (XML) is a markup language that defines rules
    /// for encoding documents in a format which is both human-readable and machine-readable.
    Xml,
}

impl AsContext for Datastore {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()> {
        let mut main_prompt = format!(
            "
Have in consideration the following {} data storage:

- datastore name: {}
- file type: {}
",
            self.storage_type, self.name, self.file_type
        );

        if let Some(region) = &self.region {
            main_prompt = format!("{}\n{} {}", main_prompt, "- region:", region);
        }

        msg_sequence.push(OpenAIMsg {
            role: GptRole::User,
            content: main_prompt,
        });

        for store in self.stores.iter() {
            let prompt = format!("
Consider the following {} schema as part of the {} data storage. It's called `{}` and the schema is:\n```\n{}```
            ", self.file_type, self.name, store.name, store.schema);

            msg_sequence.push(OpenAIMsg {
                role: GptRole::User,
                content: prompt,
            });
        }

        Ok(())
    }
}

impl Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            StorageType::AwsS3 => "AWS S3",
            StorageType::GoogleCloudStorage => "Google Cloud Storage",
            StorageType::FirebaseCloudStorage => "Firebase Cloud Storage",
            StorageType::AzureBlobStorage => "Azure Blob Storage",
            StorageType::LocalStorage => "Local Storage",
        };

        f.write_str(tag)
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            FileType::Csv => "CSV",
            FileType::Parquet => "Parquet",
            FileType::Avro => "Avro",
            FileType::Orc => "Orc",
            FileType::ProtoBuf => "Proto Buffer",
            FileType::Json => "JSON",
            FileType::NdJson => "NdJSON",
            FileType::Xml => "XML",
        };

        f.write_str(tag)
    }
}
