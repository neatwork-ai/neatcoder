use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use super::{AsContext, SchemaFile};
use gluon::ai::openai::msg::{GptRole, OpenAIMsg};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Api {
    pub name: String,
    pub api_type: ApiType,
    pub port: Option<usize>,
    pub host: Option<String>,
    pub schemas: HashMap<String, SchemaFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApiType {
    /// OpenAPI/Swagger Specification Files: JSON or YAML files that describe
    /// RESTful APIs, including endpoints, parameters, responses, etc.
    ///
    /// Postman Collections: JSON files exported from Postman, detailing API
    /// endpoints for testing and documentation.
    ///
    /// RAML (RESTful API Modeling Language): A YAML-based language to
    /// describe RESTful APIs.
    RestfulApi,
    /// WSDL (Web Services Description Language) Files: XML files that
    /// describe SOAP web services, defining operations, messages, and data types.
    SoapApi,
    RpcApi,
    GRpcApi,
    /// GraphQL SDL (Schema Definition Language) Files: Describe the types
    /// and capabilities of a GraphQL API.
    GraphQL,
    WebHooks,
    HttpLongPolling,
    ServerSentEvents,
    HttpServerPush,
    WebSub,
    /// AsyncAPI Specification: Describes asynchronous APIs, extending the
    /// OpenAPI spec to cover protocols like MQTT, WebSockets, etc.
    WebSockets,
    TcpSocket,
    LibraryIDL,
    /// AsyncAPI Specification: Describes asynchronous APIs, extending the
    /// OpenAPI spec to cover protocols like MQTT, WebSockets, etc.
    Mqtt,
}

impl AsContext for Api {
    fn add_context(&self, msg_sequence: &mut Vec<OpenAIMsg>) -> Result<()> {
        let mut main_prompt = format!(
            "
Have in consideration the following {} communication service:

- service name: {}
",
            self.api_type, self.name
        );

        if let Some(port) = &self.port {
            main_prompt = format!("{}\n{} {}", main_prompt, "- port:", port);
        }

        if let Some(host) = &self.host {
            main_prompt = format!("{}\n{} {}", main_prompt, "- host:", host);
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

impl Display for ApiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            ApiType::RestfulApi => "Restful API",
            ApiType::SoapApi => "Soap API",
            ApiType::RpcApi => "RPC API",
            ApiType::GRpcApi => "gRPC API",
            ApiType::GraphQL => "GraphQL",
            ApiType::WebHooks => "WebHooks",
            ApiType::HttpLongPolling => "HTTP Long-Polling",
            ApiType::ServerSentEvents => "Server-Sent Events",
            ApiType::HttpServerPush => "HTTP Server Push",
            ApiType::WebSub => "WebSub",
            ApiType::WebSockets => "WebSockets",
            ApiType::TcpSocket => "Raw TCP Socket",
            ApiType::LibraryIDL => "Library IDL",
            ApiType::Mqtt => "MQTT",
        };

        f.write_str(tag)
    }
}
