pub struct DataStore {
    name: String,
    port: usize,
    host: Option<String>,
    stores: Vec<Store>,
    store_type: StoreType,
}

pub struct Store {
    name: String,
    schema: String,
}

pub enum StoreType {
    // === Tabular Store Types ===
    /// SQL dialect for Google BigQuery
    BigQuery,
    /// SQL dialect for an open-source column-oriented database management system
    ClickHouse,
    /// SQL dialect for an in-memory analytical database system
    DuckDb,
    /// SQL dialect for Apache Hive, which is built on top of Hadoop TODO: Hive supports more than Tabular
    Hive,
    /// SQL dialect for Microsoft SQL Server
    MsSql,
    /// SQL dialect for an open-source relational database management system
    MySql,
    /// SQL dialect for an open-source object-relational database system
    PostgreSql,
    /// SQL dialect for Amazon Redshift
    RedshiftSql,
    /// SQL dialect for Snowflake data warehousing service
    Snowflake,
    /// A C-language library that implements a small, fast, self-contained,
    /// high-reliability, full-featured, SQL database engine
    SQLite,

    // === Columnar Store ===

    // Cassandra Query Language for Apache Cassandra
    Cassandra,
    Hbase,
    ScyellaDB,

    // === Time-Series Store ===
    InfluxDB,
    TimescaleDB,
    OpenTSDB, // Schema-less JSON-like

    // === Document && Key-Value Store ===
    MongoDB,
    CounchDB,
    RavenDB,
    Firestore,
    DynamoDB,
    CosmosDB,
    Redis,
    BerkeleyDB,
    Riak,
    CouchBase,

    // === Object Store ===
    Db4o,
    Versant,

    // === Graph Store ===
    Neo4j,
    OrientDB,
    AmazonNeptune,
    ArangoDB,

    // === XML Store ===
    BaseX,
    EXist,
    MarkLogic,
}

pub enum FileStoreDefinitions {
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

    ProtoBuf,
}
