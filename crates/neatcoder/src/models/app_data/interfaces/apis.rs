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

/// Struct documenting an API interface. API here refers to interfaces of
/// executables themselves or execution environments, and therefore it
/// groups RPC APIs, WebSockets, library interfaces, IDLs, etc.
// TODO: We can increase the configurations here such as SSL stuff, etc.
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    pub(crate) name: String,
    pub api_type: ApiType,
    /// Field that is only present when the type chose is a custom one
    custom_type: Option<String>,
    pub port: Option<usize>,
    pub(crate) host: Option<String>,
    pub(crate) schemas: BTreeMap<String, SchemaFile>,
}

#[wasm_bindgen]
impl Api {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        api_type: ApiType,
        schemas: ISchemas,
    ) -> Result<Api, JsError> {
        let schemas = BTreeMap::from_extern(schemas)?;

        Ok(Api {
            name,
            api_type,
            custom_type: None,
            port: None,
            host: None,
            schemas,
        })
    }

    pub fn new_custom(
        name: String,
        custom_type: String,
        port: Option<usize>,
        host: Option<String>,
        schemas: ISchemas,
    ) -> Result<Api, JsError> {
        let schemas = BTreeMap::from_extern(schemas)?;

        Ok(Api {
            name,
            api_type: ApiType::Custom,
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

    // Get the schemas as ISchemas to return to JavaScript
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

/// Enum documenting the type of APIs.
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
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
    Custom,
}

impl Api {
    pub fn new_(
        name: String,
        api_type: ApiType,
        schemas: BTreeMap<String, SchemaFile>,
    ) -> Api {
        Api {
            name,
            api_type,
            custom_type: None,
            port: None,
            host: None,
            schemas,
        }
    }
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
            ApiType::Custom => "Custom",
        };

        f.write_str(tag)
    }
}

// This is implemented outside the impl block because abstract data structs
// are not supported in javascript
#[wasm_bindgen(js_name = apiTypeFromFriendlyUX)]
pub fn api_type_from_friendly_ux(api: String) -> ApiType {
    let api = match api.as_str() {
        "RESTful API" => ApiType::RestfulApi,
        "SOAP API" => ApiType::SoapApi,
        "RPC API" => ApiType::RpcApi,
        "gRPC API" => ApiType::GRpcApi,
        "GraphQL" => ApiType::GraphQL,
        "WebHooks" => ApiType::WebHooks,
        "HTTP Long Polling" => ApiType::HttpLongPolling,
        "Server Sent Events" => ApiType::ServerSentEvents,
        "HTTP Server Push" => ApiType::HttpServerPush,
        "WebSub" => ApiType::WebSub,
        "WebSockets" => ApiType::WebSockets,
        "TCP Socket" => ApiType::TcpSocket,
        "Library IDL" => ApiType::LibraryIDL,
        "MQTT" => ApiType::Mqtt,
        _ => ApiType::Custom,
    };
    api
}

#[wasm_bindgen(js_name = apiTypeToFriendlyUX)]
pub fn api_type_to_friendly_ux(api_type: ApiType) -> String {
    let api = match api_type {
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
        ApiType::Custom => "Custom",
    };
    api.to_string()
}
