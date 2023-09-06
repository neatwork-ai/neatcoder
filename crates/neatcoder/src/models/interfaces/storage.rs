use crate::{
    openai::msg::{GptRole, OpenAIMsg},
    utils::{from_extern, to_extern},
    JsError,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{AsContext, ISchemas, SchemaFile};

/// Struct documenting a Data storage interface. This refers to more raw storage
/// solutions that usually provide a direct interface to a file or object-store
/// system. This leads to a decoupling of the storage system and the file types
/// themselves. For example, using storage services like AWS S3 we ould build a
/// data-lake that utilizes `parquet` files or `ndjson` files.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[wasm_bindgen]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub(crate) name: String,
    pub file_type: FileType,
    pub storage_type: StorageType,
    /// Field that is only present when the type chose is a custom one
    custom_file_type: Option<String>,
    /// Field that is only present when the type chose is a custom one
    custom_storage_type: Option<String>,
    pub(crate) region: Option<String>,
    pub(crate) schemas: BTreeMap<String, SchemaFile>,
}

/// Enum documenting the type of data storages.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[wasm_bindgen]
pub enum StorageType {
    AwsS3,
    GoogleCloudStorage,
    FirebaseCloudStorage,
    AzureBlobStorage,
    LocalStorage,
    Custom,
}

/// Enum documenting the most popular file types for data storage.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[wasm_bindgen]
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

#[wasm_bindgen]
impl Storage {
    // Create a new Storage instance from JavaScript
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        file_type: FileType,
        storage_type: StorageType,
        region: Option<String>,
        schemas: ISchemas,
    ) -> Result<Storage, JsError> {
        let schemas = from_extern(schemas)?;

        Ok(Storage {
            name,
            file_type,
            storage_type,
            custom_file_type: None,
            custom_storage_type: None,
            schemas,
            region,
        })
    }

    // TODO: New Custom method

    // Get the schemas as ISchemas to return to JavaScript
    #[wasm_bindgen(getter)]
    pub fn schemas(&self) -> Result<ISchemas, JsError> {
        to_extern::<ISchemas>(self.schemas.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn region(&self) -> Option<String> {
        self.region.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_region(&mut self, host: Option<String>) {
        self.region = host;
    }
}

impl AsContext for Storage {
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
            main_prompt =
                format!("{}\n{} {}", main_prompt, "- region:", region);
        }

        msg_sequence.push(OpenAIMsg {
            role: GptRole::User,
            content: main_prompt,
        });

        for (schema_name, schema) in self.schemas.iter() {
            let prompt = format!("
Consider the following {} schema as part of the {} data storage. It's called `{}` and the schema is:\n```\n{}```
            ", self.file_type, self.name, schema_name, schema);

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
            StorageType::Custom => "Custom Storage",
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

// This is implemented outside the impl block because abstract data structs
// are not supported in javascript
#[wasm_bindgen(js_name = storageTypeFromFriendlyUX)]
pub fn storage_type_from_friendly_ux(api: String) -> StorageType {
    let api = match api.as_str() {
        "AWS S3" => StorageType::AwsS3,
        "Google Cloud Storage" => StorageType::GoogleCloudStorage,
        "Firebase Cloud Storage" => StorageType::FirebaseCloudStorage,
        "Azure Blob Storage" => StorageType::AzureBlobStorage,
        "Local Storage" => StorageType::LocalStorage,
        _ => StorageType::Custom,
    };
    api
}
