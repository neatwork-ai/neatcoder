pub struct Datastore {
    pub name: String,
    pub region: String,
    pub stores: Vec<Store>,
    pub storage_type: StorageType,
    pub file_type: FileType,
}

pub struct Store {
    pub name: String,
    pub path: String,
    pub schema: String,
}

pub enum StorageType {
    AwsS3,
    GoogleCloudStorage,
    FirebaseCloudStorage,
    AzureBlobStorage,
}

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
