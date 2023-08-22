pub enum Interface {
    Datastore,
    Api,
}

pub enum DataStore {
    Sql(SqlDb),
    NoSql(NoSqlDb),
}

pub enum NoSqlDb {
    /// Data is stored in documents, typically in JSON or BSON format.
    /// Each document has a unique key which is used to retrieve it.
    /// Examples: MongoDB, CouchDB, RavenDB.
    Document,
    /// The simplest NoSQL option, where every single item is stored as an
    /// attribute name (or 'key') together with its value.
    /// Examples: Redis, Riak, DynamoDB, Berkeley DB.
    KeyValue,
    /// Designed to store data tables as columns rather than rows.
    /// Each column is treated separately, which allows for high scalability and performance on operations that involve large volumes of data.
    /// Examples: Apache Cassandra, HBase, ScyllaDB.
    Columnar,
    /// Designed to store data as nodes and edges (relations), allowing for
    /// easy and fast retrieval of complex hierarchical structures.
    /// They are particularly useful for datasets where relationships are key.
    /// Examples: Neo4j, OrientDB, Amazon Neptune, ArangoDB.
    Graph,
    /// Store data in the form of objects, as used by object-oriented programming languages.
    /// Not as commonly used as the other types.
    /// Examples: db4o, Versant Object Database.
    Object,
    /// Store data in the form of XML documents.
    /// They can be queried using XPath or XQuery.
    /// Examples: BaseX, eXist, MarkLogic.
    Xml,
    /// Optimized for time-stamped or time-series data, like log data or monitoring data.
    /// Examples: InfluxDB, TimescaleDB, OpenTSDB.
    TimeSeries,
}

pub struct TabularStore {
    name: String,
    port: usize,
    host: Option<String>,
    store_type: TabularStoreType,
    dialect: SqlDialect,
    tables: Vec<Table>,
}

pub struct Table {
    name: String,
    schema: String,
}

pub enum TabularStoreType {
    /// SQL dialect for Google BigQuery
    BigQuery,
    /// SQL dialect for an open-source column-oriented database management system
    ClickHouse,
    /// SQL dialect for an in-memory analytical database system
    DuckDb,
    /// SQL dialect for Apache Hive, which is built on top of Hadoop
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
}

// OpenTSDB is schema-less in many respects but uses "metrics" to categorize time-series data.

// Cassandra Query Language (CQL): Used by Apache Cassandra, a distributed NoSQL database. CQL resembles SQL in many ways, but it's tailored to the unique architecture and data model of Cassandra.
// HiveQL: Developed for Apache Hive, a data warehousing and SQL-like query language system built on top of Hadoop. HiveQL allows SQL developers to write MapReduce jobs using a SQL-like syntax.
// Cypher: This is the query language for Neo4j, a popular graph database. While tailored to querying graph structures, Cypher's syntax has some resemblances to SQL.
// N1QL (pronounced "nickel"): Developed by Couchbase for its NoSQL database. It's a SQL-like language that allows querying JSON-like documents.
// AQL (ArangoDB Query Language): Used by ArangoDB, a multi-model database. AQL is used to query and modify data stored in ArangoDB.
// Kusto Query Language (KQL): Used primarily by Microsoft's Azure Data Explorer. It's a rich language for querying large datasets quickly and efficiently.
// PartiQL: Developed by Amazon for querying data across relational, semi-structured, and nested data in Amazon Redshift, S3, and other AWS services. PartiQL provides a SQL-compatible syntax to query various data formats.
// PrestoSQL: Presto is a distributed SQL query engine optimized for querying large datasets in sources like Hive, Cassandra, relational databases, or even proprietary data stores. Its syntax is mostly SQL-like.

pub enum TabularDefinition {
    // === Standard SQL Dialects ==
    Generic,
    /// SQL dialect for Google BigQuery
    BigQuery,
    /// SQL dialect for an open-source column-oriented database management system
    ClickHouse,
    /// SQL dialect for an in-memory analytical database system
    DuckDb,
    /// SQL dialect for Apache Hive, which is built on top of Hadoop
    Hive,
    /// SQL dialect for Microsoft SQL Server
    MsSql,
    /// SQL dialect for an open-source relational database management system
    MySql,
    /// SQL dialect for an open-source object-relational database system
    PostgreSql, // Postgres || TimescaleDB
    /// SQL dialect for Amazon Redshift
    RedshiftSql,
    /// SQL dialect for Snowflake data warehousing service
    Snowflake,
    /// A C-language library that implements a small, fast, self-contained,
    /// high-reliability, full-featured, SQL database engine
    SQLite,

    // === NoSQL Columnar ===

    // Cassandra Query Language for Apache Cassandra
    Cql,        // => Cassandra || ScyellaDB
    HbaseShell, // => HBase

    // === Time-Series Store ===
    InfluxDB, // => InfluxQL

    // === Data Store Formats ===
    Csv,
    Parquet,
    Avro,
}

// Web API Description Languages:

// OpenAPI/Swagger Specification Files: JSON or YAML files that describe RESTful APIs, including endpoints, parameters, responses, etc.
// WSDL (Web Services Description Language) Files: XML files that describe SOAP web services, defining operations, messages, and data types.
// RAML (RESTful API Modeling Language): A YAML-based language to describe RESTful APIs.
// Postman Collections: JSON files exported from Postman, detailing API endpoints for testing and documentation.
// Data Serialization and Model Description:

// Protocol Buffers (protobuf) Definition Files: .proto files that describe data structures and services for Protocol Buffers.
// Avro Schemas: JSON format files that describe data structures for Avro serialization.
// JSON Schema: JSON format describing the structure of other JSON data.
// Interface & Integration Description:

// IDL (Interface Definition Language) Files: Used in various systems (e.g., CORBA, gRPC) to describe interfaces for remote procedure calls.
// WebAssembly Text Format (WAT/WAST) Files: Describe the WebAssembly binary format in a readable text format.
// MuleSoft Application Files: XML files that describe the design of integrations and APIs built using MuleSoft.
// Configuration and Deployment:

// Docker Compose Files: YAML files that describe multi-container Docker applications, specifying services, networks, and volumes.
// Kubernetes Configuration Files: YAML or JSON files describing Kubernetes resources, such as pods, services, deployments, etc.
// Modeling & Design:

// UML (Unified Modeling Language) Files: Used to model software systems in various views (structural, behavioral). Saved in formats specific to UML tools, but often can be exported to XML or other formats.
// BPMN (Business Process Model and Notation) Files: XML-based files that describe business processes in detail.
// Infrastructure as Code:

// Terraform Configuration Files: Describe infrastructure resources using HashiCorp Configuration Language (HCL).
// CloudFormation Templates: JSON or YAML files that describe AWS resources for provisioning.
// Azure Resource Manager (ARM) Templates: JSON files to define and deploy Microsoft Azure infrastructure.
// Other Descriptive Files:

// GraphQL SDL (Schema Definition Language) Files: Describe the types and capabilities of a GraphQL API.
// AsyncAPI Specification: Describes asynchronous APIs, extending the OpenAPI spec to cover protocols like MQTT, WebSockets, etc.
// FIDL (Fuchsia Interface Definition Language) Files: Describe system calls in the Fuchsia OS.
