// TODO: what to do with ParameterSchemaOrContent field in Header?
// TODO: what to do with LinkOperation field in Link?
use std::{collections::HashSet, ops::Deref};

use openapiv3::{
    AdditionalProperties, Callback, Components, Encoding, Header, Link,
    MediaType, OpenAPI, Operation, Parameter, ParameterData,
    ParameterSchemaOrContent, PathItem, Paths, ReferenceOr, RequestBody,
    Response, Responses, Schema, SchemaKind, Server, Type,
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
    fn get_refs(&self) -> HashSet<String>;
}

impl GetRefs for OpenAPI {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        // ref_strings.extend(&mut self.paths.get_refs());
        ref_strings.extend(self.paths.get_refs());

        if let Some(components) = &self.components {
            ref_strings.extend(components.get_refs());
        }

        ref_strings
    }
}

impl GetRefs for Components {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        // Responses
        self.responses
            .iter()
            .for_each(|(_, response)| ref_strings.extend(response.get_refs()));

        // Parameters
        self.parameters.iter().for_each(|(_, parameter)| {
            ref_strings.extend(parameter.get_refs())
        });

        // Request Bodies
        self.request_bodies.iter().for_each(|(_, request_body)| {
            ref_strings.extend(request_body.get_refs())
        });

        // Headers
        self.headers
            .iter()
            .for_each(|(_, hearder)| ref_strings.extend(hearder.get_refs()));

        // Schemas
        self.schemas
            .iter()
            .for_each(|(_, schema)| ref_strings.extend(schema.get_refs()));

        // Callbacks
        self.callbacks
            .iter()
            .for_each(|(_, callback)| ref_strings.extend(callback.get_refs()));

        ref_strings
    }
}

impl GetRefs for Callback {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.iter().for_each(|(_, path_item)| {
            ref_strings.extend(path_item.get_refs())
        });

        ref_strings
    }
}

impl GetRefs for Paths {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.paths
            .iter()
            .for_each(|(_, path)| ref_strings.extend(path.get_refs()));

        ref_strings
    }
}

impl GetRefs for PathItem {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        if let Some(get) = &self.get {
            ref_strings.extend(get.get_refs())
        }

        if let Some(put) = &self.put {
            ref_strings.extend(put.get_refs())
        }

        if let Some(post) = &self.post {
            ref_strings.extend(post.get_refs())
        }

        if let Some(delete) = &self.delete {
            ref_strings.extend(delete.get_refs())
        }

        if let Some(options) = &self.options {
            ref_strings.extend(options.get_refs())
        }

        if let Some(head) = &self.head {
            ref_strings.extend(head.get_refs())
        }

        if let Some(patch) = &self.patch {
            ref_strings.extend(patch.get_refs())
        }

        if let Some(trace) = &self.trace {
            ref_strings.extend(trace.get_refs())
        }

        self.parameters
            .iter()
            .for_each(|parameter| ref_strings.extend(parameter.get_refs()));

        ref_strings
    }
}

impl GetRefs for Operation {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.parameters
            .iter()
            .for_each(|param| ref_strings.extend(param.get_refs()));

        ref_strings.extend(self.responses.get_refs());

        if let Some(request_body) = &self.request_body {
            ref_strings.extend(request_body.get_refs());
        }

        ref_strings
    }
}

impl GetRefs for RequestBody {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.content
            .iter()
            .for_each(|(_, media)| ref_strings.extend(media.get_refs()));

        ref_strings
    }
}

impl GetRefs for Parameter {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        match self {
            Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => ref_strings.extend(parameter_data.get_refs()),
            Parameter::Path {
                parameter_data,
                style: _,
            } => ref_strings.extend(parameter_data.get_refs()),
            Parameter::Header {
                parameter_data,
                style: _,
            } => ref_strings.extend(parameter_data.get_refs()),
            Parameter::Cookie {
                parameter_data,
                style: _,
            } => ref_strings.extend(parameter_data.get_refs()),
        }

        ref_strings
    }
}

impl GetRefs for ParameterData {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        ref_strings.extend(self.format.get_refs());

        self.examples.iter().for_each(|(_, example)| match example {
            ReferenceOr::Reference { reference } => {
                ref_strings.insert(reference.clone());
            }
            _ => {}
        });

        ref_strings
    }
}

impl GetRefs for ParameterSchemaOrContent {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        match self {
            ParameterSchemaOrContent::Schema(schema) => {
                ref_strings.extend(schema.get_refs())
            }
            ParameterSchemaOrContent::Content(content) => {
                content.iter().for_each(|(_, media)| {
                    ref_strings.extend(media.get_refs())
                });
            }
        }

        ref_strings
    }
}

impl GetRefs for Responses {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        if let Some(default) = &self.default {
            ref_strings.extend(default.get_refs())
        }

        for (_, response) in self.responses.iter() {
            match response {
                ReferenceOr::Reference { reference } => {
                    ref_strings.insert(reference.clone());
                }
                ReferenceOr::Item(response) => {
                    ref_strings.extend(response.get_refs())
                }
            }
        }

        ref_strings
    }
}

impl GetRefs for Response {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.headers.iter().for_each(|(_, header)| {
            match header {
                ReferenceOr::Reference { reference } => {
                    ref_strings.insert(reference.clone());
                }
                ReferenceOr::Item(header) => {
                    ref_strings.extend(header.get_refs());
                }
            };
        });

        self.content
            .iter()
            .for_each(|(_, media)| ref_strings.extend(media.get_refs()));

        self.links
            .iter()
            .for_each(|(_, link)| ref_strings.extend(link.get_refs()));

        ref_strings
    }
}

impl GetRefs for Link {
    fn get_refs(&self) -> HashSet<String> {
        let ref_strings = HashSet::new();

        // TODO: handle LinkOperation
        ref_strings
    }
}

impl GetRefs for MediaType {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        if let Some(schema) = &self.schema {
            match schema {
                ReferenceOr::Reference { reference } => {
                    ref_strings.insert(reference.clone());
                }
                ReferenceOr::Item(schema) => {
                    ref_strings.extend(schema.get_refs());
                }
            };
        }

        self.examples.iter().for_each(|(_, example)| match example {
            ReferenceOr::Reference { reference } => {
                ref_strings.insert(reference.clone());
            }
            _ => {}
        });

        self.encoding.iter().for_each(|(_, encoding)| {
            ref_strings.extend(encoding.get_refs());
        });

        ref_strings
    }
}

impl GetRefs for Schema {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        match &self.schema_kind {
            SchemaKind::Type(schema_type) => match schema_type {
                Type::Object(object) => {
                    object.properties.iter().for_each(|(_, property)| {
                        match property {
                            ReferenceOr::Reference { reference } => {
                                ref_strings.insert(reference.clone());
                            }
                            ReferenceOr::Item(boxed_schema) => {
                                ref_strings.extend(boxed_schema.get_refs())
                            }
                        };
                    });

                    if let Some(additional_properties) =
                        &object.additional_properties
                    {
                        match additional_properties {
                            AdditionalProperties::Any(_) => {}
                            AdditionalProperties::Schema(boxed_schema) => {
                                ref_strings.extend(boxed_schema.get_refs())
                            }
                        }
                    }
                }
                Type::Array(array) => {
                    array.items.iter().for_each(|item| {
                        match item {
                            ReferenceOr::Reference { reference } => {
                                ref_strings.insert(reference.clone());
                            }
                            ReferenceOr::Item(boxed_schema) => {
                                ref_strings.extend(boxed_schema.get_refs())
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
                    ref_strings.extend(schema.get_refs());
                })
            }
            SchemaKind::Not { not } => match not.deref() {
                ReferenceOr::Reference { reference } => {
                    ref_strings.insert(reference.clone());
                }
                ReferenceOr::Item(schema) => {
                    ref_strings.extend(schema.get_refs());
                }
            },
            SchemaKind::Any(any_schema) => {
                any_schema.properties.iter().for_each(|(_, ref_or_schema)| {
                    match ref_or_schema {
                        ReferenceOr::Reference { reference } => {
                            ref_strings.insert(reference.clone());
                        }
                        ReferenceOr::Item(boxed_schema) => {
                            ref_strings.extend(boxed_schema.get_refs())
                        }
                    }
                });

                any_schema.one_of.iter().for_each(|schema| {
                    ref_strings.extend(schema.get_refs());
                });
                any_schema.all_of.iter().for_each(|schema| {
                    ref_strings.extend(schema.get_refs());
                });
                any_schema.any_of.iter().for_each(|schema| {
                    ref_strings.extend(schema.get_refs());
                });

                if let Some(not) = &any_schema.not {
                    match not.deref() {
                        ReferenceOr::Reference { reference } => {
                            ref_strings.insert(reference.clone());
                        }
                        ReferenceOr::Item(schema) => {
                            ref_strings.extend(schema.get_refs());
                        }
                    }
                }

                if let Some(items) = &any_schema.items {
                    match items {
                        ReferenceOr::Reference { reference } => {
                            ref_strings.insert(reference.clone());
                        }
                        ReferenceOr::Item(schema) => {
                            ref_strings.extend(schema.get_refs());
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
                                    ref_strings.insert(reference.clone());
                                }
                                ReferenceOr::Item(schema) => {
                                    ref_strings.extend(schema.get_refs());
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
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        match self {
            ReferenceOr::Reference { reference } => {
                ref_strings.insert(reference.clone());
            }
            ReferenceOr::Item(item) => {
                ref_strings.extend(item.get_refs());
            }
        };

        ref_strings
    }
}

impl GetRefs for Encoding {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.headers.iter().for_each(|(_, header)| match header {
            ReferenceOr::Reference { reference } => {
                ref_strings.insert(reference.clone());
            }
            ReferenceOr::Item(header) => ref_strings.extend(header.get_refs()),
        });

        ref_strings
    }
}

impl GetRefs for Header {
    fn get_refs(&self) -> HashSet<String> {
        let mut ref_strings = HashSet::new();

        self.examples.iter().for_each(|(_, example)| match example {
            ReferenceOr::Reference { reference } => {
                ref_strings.insert(reference.clone());
            }
            ReferenceOr::Item(_) => {} // No inner references
        });

        ref_strings
    }
}
