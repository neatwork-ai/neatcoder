use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Specs {
    pub language: Language,
    pub interfaces: Vec<Interface>,
    pub upstream_services: Vec<Service>,
    pub downstream_services: Vec<Service>,
    pub blocked_features: Vec<AdvancedFeatures>,
    // TODO: Add prefered_libraries
}

#[derive(Deserialize, Serialize)]
pub enum Language {
    Rust,
}

/// `Interface` is an enum representing various ways a program can interact
/// with the external world. Depending on the use case, these can include network
/// protocols, file I/O, shared memory, and more.
#[derive(Deserialize, Serialize)]
pub enum Interface {
    /// RESTful API interface, typically HTTP-based and uses CRUD operations.
    RestfulAPI,
    /// Remote Procedure Call API, used to execute code across network boundaries.
    RpcAPI,
    /// WebHooks, a method of augmenting or altering the behavior of a web page,
    /// or web application, with custom callbacks.
    WebHooks,
    /// WebSockets, a protocol providing full-duplex communication over a single, long-lived connection.
    WebSockets,
    /// Library interface, where the program provides an API as a library for other programs to use.
    Lib,
    /// Command Line Interface, for interacting with the program via textual commands.
    Cli,
}

#[derive(Deserialize, Serialize)]
pub enum Service {
    Api(Api),
    Database(Database),
    MesssageQueue(MessageQueue),
}

#[derive(Deserialize, Serialize)]
pub enum Database {
    Sql(SqlFlavour),
    Document,
    KeyValue,
    WideColumn,
    Graph,
    Search,
    TimeSeries,
}

#[derive(Deserialize, Serialize)]
pub enum SqlFlavour {
    MySQL,
    PostgreSQL,
}

#[derive(Deserialize, Serialize)]
pub enum Api {
    Restful,
    Rpc,
    GRpc,
}

#[derive(Deserialize, Serialize)]
pub enum MessageQueue {
    RabbitMQ,
    Kafka,
    AmazonSQS,
    GooglePubSub,
    AzureServiceBus,
}

#[derive(Deserialize, Serialize)]
pub enum AdvancedFeatures {
    Concurrency,
    DynamicTypes,
    AdvancedTraits,
    FFI,
    Unsafe,
}
