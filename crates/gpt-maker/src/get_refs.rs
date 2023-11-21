// TODO: what to do with ParameterSchemaOrContent field in Header?
// TODO: what to do with LinkOperation field in Link?
use std::ops::Deref;

use openapiv3::{
    AdditionalProperties, Encoding, Header, Link, MediaType, Operation,
    Parameter, ParameterData, ParameterSchemaOrContent, PathItem, Paths,
    ReferenceOr, RequestBody, Response, Responses, Schema, SchemaKind, Server,
    Type,
};

/// Paths
/// |__PathItem
///    |__Operation
///       |__Parameter
///          |__ParameterData
///             |__ParameterSchemaOrContent
///             |__ReferenceOr<Example>
///       |__RequestBody
///          |__MediaType
///             |__Schema
///                |__SchemaKind
///                   |__Schema..
///             |__ReferenceOr<Example>
///             |__Encoding
///       |__Responses
///          |__Response
///             |__Header
///             |__MediaType..
///             |__Link
///    |__Parameter..

pub trait GetRefs {
    fn get_refs(&self) -> Vec<String>;
}

impl GetRefs for Paths {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        self.paths
            .iter()
            .for_each(|(_, path)| ref_strings.append(&mut path.get_refs()));

        ref_strings
    }
}

impl GetRefs for PathItem {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        if let Some(get) = &self.get {
            ref_strings.append(&mut get.get_refs())
        }

        if let Some(put) = &self.put {
            ref_strings.append(&mut put.get_refs())
        }

        if let Some(post) = &self.post {
            ref_strings.append(&mut post.get_refs())
        }

        if let Some(delete) = &self.delete {
            ref_strings.append(&mut delete.get_refs())
        }

        if let Some(options) = &self.options {
            ref_strings.append(&mut options.get_refs())
        }

        if let Some(head) = &self.head {
            ref_strings.append(&mut head.get_refs())
        }

        if let Some(patch) = &self.patch {
            ref_strings.append(&mut patch.get_refs())
        }

        if let Some(trace) = &self.trace {
            ref_strings.append(&mut trace.get_refs())
        }

        self.parameters.iter().for_each(|parameter| {
            ref_strings.append(&mut parameter.get_refs())
        });

        ref_strings
    }
}

impl GetRefs for Operation {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        self.parameters
            .iter()
            .for_each(|param| ref_strings.append(&mut param.get_refs()));

        ref_strings.append(&mut self.responses.get_refs());

        if let Some(request_body) = &self.request_body {
            ref_strings.append(&mut request_body.get_refs());
        }

        ref_strings
    }
}

impl GetRefs for RequestBody {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        self.content
            .iter()
            .for_each(|(_, media)| ref_strings.append(&mut media.get_refs()));

        ref_strings
    }
}

impl GetRefs for Parameter {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        match self {
            Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => ref_strings.append(&mut parameter_data.get_refs()),
            Parameter::Path {
                parameter_data,
                style: _,
            } => ref_strings.append(&mut parameter_data.get_refs()),
            Parameter::Header {
                parameter_data,
                style: _,
            } => ref_strings.append(&mut parameter_data.get_refs()),
            Parameter::Cookie {
                parameter_data,
                style: _,
            } => ref_strings.append(&mut parameter_data.get_refs()),
        }

        ref_strings
    }
}

impl GetRefs for ParameterData {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        ref_strings.append(&mut self.format.get_refs());

        self.examples.iter().for_each(|(_, example)| match example {
            ReferenceOr::Reference { reference } => {
                println!("Reference from examples");
                ref_strings.push(reference.clone())
            }
            _ => {}
        });

        ref_strings
    }
}

impl GetRefs for ParameterSchemaOrContent {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        match self {
            ParameterSchemaOrContent::Schema(schema) => {
                ref_strings.append(&mut schema.get_refs())
            }
            ParameterSchemaOrContent::Content(content) => {
                content.iter().for_each(|(_, media)| {
                    ref_strings.append(&mut media.get_refs())
                });
            }
        }

        ref_strings
    }
}

impl GetRefs for Responses {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        if let Some(default) = &self.default {
            ref_strings.append(&mut default.get_refs())
        }

        for (_, response) in self.responses.iter() {
            match response {
                ReferenceOr::Reference { reference } => {
                    ref_strings.push(reference.clone());
                }
                ReferenceOr::Item(response) => {
                    ref_strings.append(&mut response.get_refs())
                }
            }
        }

        ref_strings
    }
}

impl GetRefs for Response {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        self.headers.iter().for_each(|(_, header)| {
            match header {
                ReferenceOr::Reference { reference } => {
                    ref_strings.push(reference.clone());
                }
                ReferenceOr::Item(header) => {
                    ref_strings.append(&mut header.get_refs());
                }
            };
        });

        self.content
            .iter()
            .for_each(|(_, media)| ref_strings.append(&mut media.get_refs()));

        self.links
            .iter()
            .for_each(|(_, link)| ref_strings.append(&mut link.get_refs()));

        ref_strings
    }
}

impl GetRefs for Link {
    fn get_refs(&self) -> Vec<String> {
        let ref_strings = Vec::new();

        // TODO: handle LinkOperation
        ref_strings
    }
}

impl GetRefs for MediaType {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        if let Some(schema) = &self.schema {
            match schema {
                ReferenceOr::Reference { reference } => {
                    ref_strings.push(reference.clone());
                }
                ReferenceOr::Item(schema) => {
                    ref_strings.append(&mut schema.get_refs());
                }
            };
        }

        self.examples.iter().for_each(|(_, example)| match example {
            ReferenceOr::Reference { reference } => {
                println!("Reference from examples");
                ref_strings.push(reference.clone())
            }
            _ => {}
        });

        self.encoding.iter().for_each(|(_, encoding)| {
            ref_strings.append(&mut encoding.get_refs());
        });

        ref_strings
    }
}

impl GetRefs for Schema {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        println!("Getting refs for schema: {:?}", self);

        match &self.schema_kind {
            SchemaKind::Type(schema_type) => match schema_type {
                Type::Object(object) => {
                    object.properties.iter().for_each(|(_, property)| {
                        match property {
                            ReferenceOr::Reference { reference } => {
                                ref_strings.push(reference.clone());
                            }
                            ReferenceOr::Item(boxed_schema) => {
                                ref_strings.append(&mut boxed_schema.get_refs())
                            }
                        };
                    });

                    if let Some(additional_properties) =
                        &object.additional_properties
                    {
                        match additional_properties {
                            AdditionalProperties::Any(_) => {}
                            AdditionalProperties::Schema(boxed_schema) => {
                                ref_strings.append(&mut boxed_schema.get_refs())
                            }
                        }
                    }
                }
                Type::Array(array) => {
                    array.items.iter().for_each(|item| {
                        match item {
                            ReferenceOr::Reference { reference } => {
                                ref_strings.push(reference.clone());
                            }
                            ReferenceOr::Item(boxed_schema) => {
                                ref_strings.append(&mut boxed_schema.get_refs())
                            }
                        };
                    });
                }
                _ => {}
            }, // No Inner references
            SchemaKind::OneOf { one_of: of }
            | SchemaKind::AllOf { all_of: of }
            | SchemaKind::AnyOf { any_of: of } => {
                of.iter().for_each(|schema| {
                    ref_strings.append(&mut schema.get_refs());
                })
            }
            SchemaKind::Not { not } => match not.deref() {
                ReferenceOr::Reference { reference } => {
                    ref_strings.push(reference.clone());
                }
                ReferenceOr::Item(schema) => {
                    ref_strings.append(&mut schema.get_refs());
                }
            },
            SchemaKind::Any(any_schema) => {
                any_schema.properties.iter().for_each(|(_, ref_or_schema)| {
                    match ref_or_schema {
                        ReferenceOr::Reference { reference } => {
                            ref_strings.push(reference.clone());
                        }
                        ReferenceOr::Item(boxed_schema) => {
                            ref_strings.append(&mut boxed_schema.get_refs())
                        }
                    }
                });

                any_schema.one_of.iter().for_each(|schema| {
                    ref_strings.append(&mut schema.get_refs());
                });
                any_schema.all_of.iter().for_each(|schema| {
                    ref_strings.append(&mut schema.get_refs());
                });
                any_schema.any_of.iter().for_each(|schema| {
                    ref_strings.append(&mut schema.get_refs());
                });

                if let Some(not) = &any_schema.not {
                    match not.deref() {
                        ReferenceOr::Reference { reference } => {
                            ref_strings.push(reference.clone());
                        }
                        ReferenceOr::Item(schema) => {
                            ref_strings.append(&mut schema.get_refs());
                        }
                    }
                }

                if let Some(items) = &any_schema.items {
                    match items {
                        ReferenceOr::Reference { reference } => {
                            ref_strings.push(reference.clone());
                        }
                        ReferenceOr::Item(schema) => {
                            ref_strings.append(&mut schema.get_refs());
                        }
                    }
                }

                if let Some(additional_props) =
                    &any_schema.additional_properties
                {
                    match additional_props {
                        AdditionalProperties::Any(_) => {}
                        AdditionalProperties::Schema(boxed_schema) => {
                            match boxed_schema.deref() {
                                ReferenceOr::Reference { reference } => {
                                    ref_strings.push(reference.clone());
                                }
                                ReferenceOr::Item(schema) => {
                                    ref_strings.append(&mut schema.get_refs());
                                }
                            }
                        }
                    }
                }
            }
        }

        ref_strings
    }
}

impl<T: GetRefs> GetRefs for ReferenceOr<T> {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        match self {
            ReferenceOr::Reference { reference } => {
                ref_strings.push(reference.clone());
            }
            ReferenceOr::Item(item) => {
                ref_strings.append(&mut item.get_refs());
            }
        };

        ref_strings
    }
}

impl GetRefs for Encoding {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        self.headers.iter().for_each(|(_, header)| match header {
            ReferenceOr::Reference { reference } => {
                ref_strings.push(reference.clone());
            }
            ReferenceOr::Item(header) => {
                ref_strings.append(&mut header.get_refs())
            }
        });

        ref_strings
    }
}

impl GetRefs for Header {
    fn get_refs(&self) -> Vec<String> {
        let mut ref_strings = Vec::new();

        self.examples.iter().for_each(|(_, example)| match example {
            ReferenceOr::Reference { reference } => {
                println!("Getting reference from examples");
                ref_strings.push(reference.clone());
            }
            ReferenceOr::Item(_) => {} // No inner references
        });

        ref_strings
    }
}

pub fn process_path_item(path_item: &mut PathItem) {
    if let Some(ref mut description) = &mut path_item.description {
        if description.chars().count() > 300 {
            // Take only the first `max_len` characters.
            *description = description.chars().take(300).collect();
        }
    }

    if let Some(get) = &mut path_item.get {
        process_operation(get);
    }

    if let Some(put) = &mut path_item.put {
        process_operation(put);
    }

    if let Some(post) = &mut path_item.post {
        process_operation(post);
    }

    if let Some(delete) = &mut path_item.delete {
        process_operation(delete);
    }

    if let Some(options) = &mut path_item.options {
        process_operation(options);
    }

    if let Some(head) = &mut path_item.head {
        process_operation(head);
    }

    if let Some(patch) = &mut path_item.patch {
        process_operation(patch);
    }

    if let Some(trace) = &mut path_item.trace {
        process_operation(trace);
    }

    path_item
        .servers
        .iter_mut()
        .for_each(|server| process_server(server));
}

fn process_operation(operation: &mut Operation) {
    if let Some(ref mut description) = &mut operation.description {
        if description.chars().count() > 300 {
            // Take only the first `max_len` characters.
            *description = description.chars().take(300).collect();
        }
    }

    if let Some(ref mut operation_id) = &mut operation.operation_id {
        *operation_id = operation_id.replace("/", "--");
    }
}

pub fn process_server(server: &mut Server) {
    if server.url.starts_with("http://") {
        server.url = server.url.replacen("http://", "https://", 1);
    }
}
