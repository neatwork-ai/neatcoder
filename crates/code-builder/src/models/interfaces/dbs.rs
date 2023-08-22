pub struct Database {
    pub name: String,
    pub port: usize,
    pub host: Option<String>,
    pub stores: Vec<Store>,
    pub db_type: DbType,
}

pub struct Store {
    pub name: String,
    pub schema: String,
}

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
    SQLite,

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
}
