use super::{AsContext, SchemaFile};
use crate::{
    openai::msg::{GptRole, OpenAIMsg},
    typescript::ISchemas,
    JsError, WasmType,
};
use anyhow::Result;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};
use wasm_bindgen::prelude::wasm_bindgen;

/// Struct documenting a Database/DataWarehouse interface. This refers to Database
/// storage solutions or to more classic Data Warehousing solutions such as
/// Snowflake and the likes.
/// The core difference between `Database` and `Storage` variants is that
/// whilst both are storage solutions, the `Database` variant encapsulates
/// storage under a Management system that typically guarantees ACID
/// transactions as well as CAP Theorem guarantees. Usually these solutions
/// provide a declarative framework for accessing and managing data.
// TODO: We can increase the configurations here such as SSL stuff, etc.
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub(crate) name: String,
    pub db_type: DbType,
    /// Field that is only present when the type chose is a custom one
    custom_type: Option<String>,
    pub port: Option<usize>,
    pub(crate) host: Option<String>,
    pub(crate) schemas: BTreeMap<String, SchemaFile>,
}

#[wasm_bindgen]
impl Database {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        db_type: DbType,
        schemas: ISchemas,
    ) -> Result<Database, JsError> {
        let schemas = BTreeMap::from_extern(schemas)?;

        Ok(Database {
            name,
            db_type,
            custom_type: None,
            port: None,
            host: None,
            schemas,
        })
    }

    #[wasm_bindgen(js_name = newCustom)]
    pub fn new_custom(
        name: String,
        custom_type: String,
        port: Option<usize>,
        host: Option<String>,
        schemas: ISchemas,
    ) -> Result<Database, JsError> {
        let schemas = BTreeMap::from_extern(schemas)?;

        Ok(Database {
            name,
            db_type: DbType::Custom,
            custom_type: Some(custom_type),
            port,
            host,
            schemas,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone().into()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // Get the schemas as a ISchemas to return to JavaScript
    #[wasm_bindgen(getter)]
    pub fn schemas(&self) -> Result<ISchemas, JsError> {
        BTreeMap::to_extern(self.schemas.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn host(&self) -> Option<JsString> {
        match &self.host {
            Some(host) => Some(host.clone().into()),
            None => None,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_host(&mut self, host: Option<String>) {
        self.host = host;
    }
}

impl Database {
    pub fn new_(
        name: String,
        db_type: DbType,
        schemas: BTreeMap<String, SchemaFile>,
    ) -> Database {
        Database {
            name,
            db_type,
            custom_type: None,
            port: None,
            host: None,
            schemas,
        }
    }
}

/// Enum documenting the type of Database/DataWarehouse interface.
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DbType {
    // === Tabular Store Types ===
    // Traditional RDBMS systems that store data in rows and columns. Used mainly for OLTP operations.
    //
    /// A high-performance column-oriented database management system.
    ClickHouse,
    /// An embedded analytical database.
    DuckDb,
    /// Microsoft's relational database management system.
    MsSql,
    /// An open-source relational database management system.
    MySql,
    /// An open-source object-relational database system.
    PostgreSql,
    /// A C-language library that provides a lightweight disk-based database.
    SqLite,

    // === DataWarehouse Store ===
    // Systems optimized for analysis and reporting of large datasets.
    //
    /// Google's fully managed, petabyte-scale data warehouse.
    BigQuery,
    /// Amazon's fully managed data warehouse solution.
    Redshift,
    /// A cloud-native data warehousing platform.
    Snowflake,
    /// Data warehouse infrastructure on top of Hadoop.
    Hive,

    // === Columnar Store ===
    // Databases that use a columnar storage approach.
    //
    /// A distributed NoSQL database system.
    Cassandra,
    /// An open-source, non-relational, distributed database modeled after Google's Bigtable.
    Hbase,
    /// A real-time big data database that's fully compatible with Apache Cassandra.
    ScyellaDB,

    // === Time-Series Store ===
    // Databases optimized for handling time-series data.
    //
    /// An open-source time-series database.
    InfluxDB,
    /// SQL time-series database built on PostgreSQL.
    TimescaleDB,
    /// Schema-less JSON-like Time-Series DataBase (TSDB) written on top of HBase.
    OpenTSDB,

    // === Document && Key-Value Store ===
    // Systems that store data as documents or simple key-value pairs.
    //
    /// A document-oriented NoSQL database.
    MongoDB,
    /// A database that uses JSON for documents, JavaScript for MapReduce queries.
    CounchDB,
    /// A NoSQL document database with multi-document ACID transactions.
    RavenDB,
    /// A flexible, scalable database for mobile, web, and server development from Firebase and Google Cloud.
    Firestore,
    /// Amazon's managed NoSQL database service.
    DynamoDB,
    /// Azure's globally distributed, multi-model database service.
    CosmosDB,
    /// An in-memory data structure store, used as a database, cache, and message broker.
    Redis,
    /// A high-performance embedded database for key-value data.
    BerkeleyDB,
    /// A distributed NoSQL key-value data store.
    Riak,
    /// A distributed NoSQL document-oriented database.
    CouchBase,

    // === Object Store ===
    // Databases that use object-oriented approaches to store and query data.
    //
    /// Database for objects.
    Db4o,
    /// An object-oriented database management system.
    Versant,

    // === Graph Store ===
    // Databases optimized for storing data as nodes and relationships.
    //
    /// A graph database management system.
    Neo4j,
    /// An open-source NoSQL database management system written in Java.
    OrientDB,
    /// Amazon's fully managed graph database service.
    AmazonNeptune,
    /// A native multi-model database.
    ArangoDB,

    // === XML Store ===
    // Databases designed for storing, querying, and processing XML data.
    //
    /// A very light-weight, high-performance, and scalable XML database system.
    BaseX,
    /// An open-source database management system entirely built on XML technology.
    EXist,
    /// An enterprise NoSQL database.
    MarkLogic,
    /// A custom database interface
    Custom,
}

impl AsContext for Database {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()> {
        let mut main_prompt = format!(
            "
Have in consideration the following {} Database:

- database name: {}
",
            self.db_type, self.name
        );

        if let Some(port) = &self.port {
            main_prompt =
                format!("{}\n{} {}", main_prompt, "- database port:", port);
        }

        if let Some(host) = &self.host {
            main_prompt =
                format!("{}\n{} {}", main_prompt, "- database host:", host);
        }

        msg_sequence.push(OpenAIMsg {
            role: GptRole::User,
            content: main_prompt,
        });

        for (schema_name, schema) in self.schemas.iter() {
            let prompt = format!("
Consider the following schema as part of the {} database. It's called `{}` and the schema is:\n```\n{}```
            ", self.name, schema_name, schema);

            msg_sequence.push(OpenAIMsg {
                role: GptRole::User,
                content: prompt,
            });
        }

        Ok(())
    }
}

impl Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            DbType::ClickHouse => "ClickHouse",
            DbType::DuckDb => "DuckDb",
            DbType::MsSql => "MsSql",
            DbType::MySql => "MySql",
            DbType::PostgreSql => "PostgreSql",
            DbType::SqLite => "SQLite",
            DbType::BigQuery => "BigQuery",
            DbType::Redshift => "Redshift",
            DbType::Snowflake => "Snowflake",
            DbType::Hive => "Hive",
            DbType::Cassandra => "Cassandra",
            DbType::Hbase => "Hbase",
            DbType::ScyellaDB => "ScyellaDB",
            DbType::InfluxDB => "InfluxDB",
            DbType::TimescaleDB => "TimescaleDB",
            DbType::OpenTSDB => "OpenTSDB",
            DbType::MongoDB => "MongoDB",
            DbType::CounchDB => "CounchDB",
            DbType::RavenDB => "RavenDB",
            DbType::Firestore => "Firestore",
            DbType::DynamoDB => "DynamoDB",
            DbType::CosmosDB => "CosmosDB",
            DbType::Redis => "Redis",
            DbType::BerkeleyDB => "BerkeleyDB",
            DbType::Riak => "Riak",
            DbType::CouchBase => "CouchBase",
            DbType::Db4o => "Db4o",
            DbType::Versant => "Versant",
            DbType::Neo4j => "Neo4j",
            DbType::OrientDB => "OrientDB",
            DbType::AmazonNeptune => "AmazonNeptune",
            DbType::ArangoDB => "ArangoDB",
            DbType::BaseX => "BaseX",
            DbType::EXist => "EXist",
            DbType::MarkLogic => "MarkLogic",
            DbType::Custom => "Custom",
        };

        f.write_str(tag)
    }
}

// This is implemented outside the impl block because abstract data structs
// are not supported in javascript
#[wasm_bindgen(js_name = dbTypeFromFriendlyUX)]
pub fn db_type_from_friendly_ux(database: String) -> DbType {
    let db = match database.as_str() {
        "ClickHouse" => DbType::ClickHouse,
        "DuckDb" => DbType::DuckDb,
        "MS SQL" => DbType::MsSql,
        "MySQL" => DbType::MySql,
        "PostgreSQL" => DbType::PostgreSql,
        "SQLite" => DbType::SqLite,
        "BigQuery" => DbType::BigQuery,
        "Redshift" => DbType::Redshift,
        "Snowflake" => DbType::Snowflake,
        "Hive" => DbType::Hive,
        "Cassandra" => DbType::Cassandra,
        "Hbase" => DbType::Hbase,
        "ScyellaDB" => DbType::ScyellaDB,
        "InfluxDB" => DbType::InfluxDB,
        "TimescaleDB" => DbType::TimescaleDB,
        "OpenTSDB" => DbType::OpenTSDB,
        "MongoDB" => DbType::MongoDB,
        "CounchDB" => DbType::CounchDB,
        "RavenDB" => DbType::RavenDB,
        "Firestore" => DbType::Firestore,
        "DynamoDB" => DbType::DynamoDB,
        "CosmosDB" => DbType::CosmosDB,
        "Redis" => DbType::Redis,
        "BerkeleyDB" => DbType::BerkeleyDB,
        "Riak" => DbType::Riak,
        "CouchBase" => DbType::CouchBase,
        "Db4o" => DbType::Db4o,
        "Versant" => DbType::Versant,
        "Neo4j" => DbType::Neo4j,
        "OrientDB" => DbType::OrientDB,
        "AmazonNeptune" => DbType::AmazonNeptune,
        "ArangoDB" => DbType::ArangoDB,
        "BaseX" => DbType::BaseX,
        "EXist" => DbType::EXist,
        "MarkLogic" => DbType::MarkLogic,
        _ => DbType::Custom,
    };
    db
}

#[wasm_bindgen(js_name = dbTypeToFriendlyUX)]
pub fn db_type_to_friendly_ux(db_type: DbType) -> String {
    let db = match db_type {
        DbType::ClickHouse => "ClickHouse",
        DbType::DuckDb => "DuckDb",
        DbType::MsSql => "MS SQL",
        DbType::MySql => "MySQL",
        DbType::PostgreSql => "PostgreSQL",
        DbType::SqLite => "SQLite",
        DbType::BigQuery => "BigQuery",
        DbType::Redshift => "Redshift",
        DbType::Snowflake => "Snowflake",
        DbType::Hive => "Hive",
        DbType::Cassandra => "Cassandra",
        DbType::Hbase => "Hbase",
        DbType::ScyellaDB => "ScyellaDB",
        DbType::InfluxDB => "InfluxDB",
        DbType::TimescaleDB => "TimescaleDB",
        DbType::OpenTSDB => "OpenTSDB",
        DbType::MongoDB => "MongoDB",
        DbType::CounchDB => "CounchDB",
        DbType::RavenDB => "RavenDB",
        DbType::Firestore => "Firestore",
        DbType::DynamoDB => "DynamoDB",
        DbType::CosmosDB => "CosmosDB",
        DbType::Redis => "Redis",
        DbType::BerkeleyDB => "BerkeleyDB",
        DbType::Riak => "Riak",
        DbType::CouchBase => "CouchBase",
        DbType::Db4o => "Db4o",
        DbType::Versant => "Versant",
        DbType::Neo4j => "Neo4j",
        DbType::OrientDB => "OrientDB",
        DbType::AmazonNeptune => "AmazonNeptune",
        DbType::ArangoDB => "ArangoDB",
        DbType::BaseX => "BaseX",
        DbType::EXist => "EXist",
        DbType::MarkLogic => "MarkLogic",
        DbType::Custom => "Custom",
    };
    db.to_string()
}
