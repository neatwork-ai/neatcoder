use crate::process::ComponentPointer;
use anyhow::{anyhow, Result};
use openapiv3::{
    AdditionalProperties, Encoding, Header, MediaType, Operation, Parameter,
    ParameterData, ParameterSchemaOrContent, PathItem, Paths, ReferenceOr,
    RequestBody, Response, Responses, Schema, SchemaKind, Server, Type,
};
use std::{collections::HashMap, ops::DerefMut};

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

pub trait OpenAPIDeref {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()>;
}

impl OpenAPIDeref for Paths {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.paths
            .iter_mut()
            .try_for_each(|(_, path)| path.openapi_deref(ref_map))?;

        Ok(())
    }
}

impl OpenAPIDeref for PathItem {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(get) = &mut self.get {
            get.openapi_deref(ref_map)?;
        }

        if let Some(put) = &mut self.put {
            put.openapi_deref(ref_map)?;
        }

        if let Some(post) = &mut self.post {
            post.openapi_deref(ref_map)?;
        }

        if let Some(delete) = &mut self.delete {
            delete.openapi_deref(ref_map)?;
        }

        if let Some(options) = &mut self.options {
            options.openapi_deref(ref_map)?;
        }

        if let Some(head) = &mut self.head {
            head.openapi_deref(ref_map)?;
        }

        if let Some(patch) = &mut self.patch {
            patch.openapi_deref(ref_map)?;
        }

        if let Some(trace) = &mut self.trace {
            trace.openapi_deref(ref_map)?;
        }

        self.parameters
            .iter_mut()
            .try_for_each(|parameter| parameter.openapi_deref(ref_map))?;

        Ok(())
    }
}

impl OpenAPIDeref for Operation {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.parameters
            .iter_mut()
            .try_for_each(|param| param.openapi_deref(ref_map))?;

        self.responses.openapi_deref(ref_map)?;

        if let Some(request_body) = &mut self.request_body {
            request_body.openapi_deref(ref_map)?;
        }

        Ok(())
    }
}

impl OpenAPIDeref for RequestBody {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.content
            .iter_mut()
            .try_for_each(|(_, media)| media.openapi_deref(ref_map))?;

        Ok(())
    }
}

impl OpenAPIDeref for Parameter {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match self {
            Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => parameter_data.openapi_deref(ref_map),
            Parameter::Path {
                parameter_data,
                style: _,
            } => parameter_data.openapi_deref(ref_map),
            Parameter::Header {
                parameter_data,
                style: _,
            } => parameter_data.openapi_deref(ref_map),
            Parameter::Cookie {
                parameter_data,
                style: _,
            } => parameter_data.openapi_deref(ref_map),
        }?;

        Ok(())
    }
}

impl OpenAPIDeref for ParameterData {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.format.openapi_deref(ref_map)?;

        self.examples.iter_mut().try_for_each::<_, Result<()>>(
            |(_, example)| {
                // Returns early thus preventing uneccessary cloning
                if let ReferenceOr::Item(_) = example {
                    return Ok(());
                }

                if let ReferenceOr::Reference { mut reference } =
                    example.clone()
                {
                    deref_comp(example, &mut reference, ref_map)?;
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

impl OpenAPIDeref for ParameterSchemaOrContent {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match self {
            ParameterSchemaOrContent::Schema(schema) => {
                schema.openapi_deref(ref_map)
            }
            ParameterSchemaOrContent::Content(content) => {
                content.iter_mut().try_for_each::<_, Result<()>>(
                    |(_, media)| {
                        media.openapi_deref(ref_map)?;

                        Ok(())
                    },
                )?;

                Ok(())
            }
        }?;

        Ok(())
    }
}

impl OpenAPIDeref for Responses {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(default) = &mut self.default {
            default.openapi_deref(ref_map)?;
        }

        for (_, response) in self.responses.iter_mut() {
            match_deref(response, ref_map)?;
        }

        Ok(())
    }
}

// &Box<ReferenceOr<Schema>>

// fn match_deref<'a, T, U>(ref_enum: &mut ReferenceOr<U>, ref_map: &mut HashMap<String, ComponentPointer>) -> Result<()>
// where
//     T: 'a + OpenAPIDeref + Clone,
//     U: Deref<Target = T> + Clone,
// {

fn match_deref<T: 'static>(
    ref_enum: &mut ReferenceOr<T>,
    ref_map: &mut HashMap<String, ComponentPointer>,
) -> Result<()>
where
    T: OpenAPIDeref + Clone,
{
    if let ReferenceOr::Item(item) = ref_enum {
        item.openapi_deref(ref_map)?;
    }

    if let ReferenceOr::Reference { reference } = ref_enum.clone() {
        deref_comp(ref_enum, &reference, ref_map)?;
    }

    Ok(())
}

fn match_deref_ignore_item<T: 'static + Clone>(
    ref_enum: &mut ReferenceOr<T>,
    ref_map: &mut HashMap<String, ComponentPointer>,
) -> Result<()> {
    if let ReferenceOr::Item(_) = ref_enum {
        return Ok(());
    }

    if let ReferenceOr::Reference { reference } = ref_enum.clone() {
        deref_comp(ref_enum, &reference, ref_map)?;
    }

    Ok(())
}

fn match_deref_box<T: 'static + Clone>(
    ref_enum: &mut ReferenceOr<impl DerefMut<Target = T> + 'static + Clone>,
    ref_map: &mut HashMap<String, ComponentPointer>,
) -> Result<()>
where
    T: OpenAPIDeref + Clone,
{
    if let ReferenceOr::Item(item) = ref_enum {
        item.openapi_deref(ref_map)?;
    }

    if let ReferenceOr::Reference { reference } = ref_enum.clone() {
        deref_comp(ref_enum, &reference, ref_map)?;
    }

    Ok(())
}

impl OpenAPIDeref for Response {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.headers
            .iter_mut()
            .try_for_each(|(_, header)| match_deref(header, ref_map))?;

        self.content
            .iter_mut()
            .try_for_each(|(_, media)| media.openapi_deref(ref_map))?;

        // self.links.iter_mut().try_for_each(|(_, link)| {
        //     link.openapi_deref(ref_map)
        // })?

        Ok(())
    }
}

impl OpenAPIDeref for MediaType {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(schema) = &mut self.schema {
            match_deref(schema, ref_map)?;
        }

        self.examples.iter_mut().try_for_each(|(_, example)| {
            match_deref_ignore_item(example, ref_map)
        })?;

        self.encoding
            .iter_mut()
            .try_for_each(|(_, encoding)| encoding.openapi_deref(ref_map))?;

        Ok(())
    }
}

impl OpenAPIDeref for Schema {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        println!("Getting refs for schema: {:?}", self);

        match &mut self.schema_kind {
            SchemaKind::Type(schema_type) => match schema_type {
                Type::Object(object) => {
                    object.properties.iter_mut().try_for_each(
                        |(_, property)| match_deref_box(property, ref_map),
                    )?;

                    if let Some(additional_properties) =
                        &mut object.additional_properties
                    {
                        match additional_properties {
                            AdditionalProperties::Any(_) => {}
                            AdditionalProperties::Schema(boxed_schema) => {
                                boxed_schema.openapi_deref(ref_map)?
                            }
                        }
                    }
                }
                Type::Array(array) => {
                    array
                        .items
                        .iter_mut()
                        .try_for_each(|item| match_deref_box(item, ref_map))?;
                }
                _ => {}
            }, // No Inner references
            SchemaKind::OneOf { one_of: of }
            | SchemaKind::AllOf { all_of: of }
            | SchemaKind::AnyOf { any_of: of } => {
                of.iter_mut()
                    .try_for_each(|schema| schema.openapi_deref(ref_map))?;
            }
            SchemaKind::Not { not } => {
                let not_: &mut ReferenceOr<Schema> = not;

                match_deref(not_, ref_map)?;
            }
            SchemaKind::Any(any_schema) => {
                any_schema.properties.iter_mut().try_for_each(
                    |(_, ref_or_schema)| {
                        match_deref_box(ref_or_schema, ref_map)
                    },
                )?;

                any_schema
                    .one_of
                    .iter_mut()
                    .try_for_each(|schema| schema.openapi_deref(ref_map))?;

                any_schema
                    .all_of
                    .iter_mut()
                    .try_for_each(|schema| schema.openapi_deref(ref_map))?;
                any_schema
                    .any_of
                    .iter_mut()
                    .try_for_each(|schema| schema.openapi_deref(ref_map))?;

                if let Some(not) = &mut any_schema.not {
                    let not_: &mut ReferenceOr<Schema> = not;

                    match_deref(not_, ref_map)?;
                }

                if let Some(items) = &mut any_schema.items {
                    match_deref_box(items, ref_map)?;
                }

                if let Some(additional_props) =
                    &mut any_schema.additional_properties
                {
                    match additional_props {
                        AdditionalProperties::Any(_) => {}
                        AdditionalProperties::Schema(boxed_schema) => {
                            let schema: &mut ReferenceOr<Schema> = boxed_schema;

                            match_deref(schema, ref_map)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl<T: 'static + OpenAPIDeref + Clone> OpenAPIDeref for ReferenceOr<T> {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match_deref(self, ref_map)?;

        Ok(())
    }
}

impl OpenAPIDeref for Encoding {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.headers
            .iter_mut()
            .try_for_each(|(_, header)| match_deref(header, ref_map))?;

        Ok(())
    }
}

impl OpenAPIDeref for Header {
    fn openapi_deref(
        &mut self,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.examples.iter_mut().try_for_each(|(_, example)| {
            match_deref_ignore_item(example, ref_map)
        })?;

        Ok(())
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

fn deref_comp<T: 'static + Clone>(
    openapi_elem: &mut ReferenceOr<T>,
    reference: &String,
    ref_map: &mut HashMap<String, ComponentPointer>,
) -> Result<()> {
    let comp: ComponentPointer = ref_map.get(reference).unwrap().clone();

    let actual_comp = comp
        .comp
        .as_ref()
        .downcast_ref::<ReferenceOr<T>>()
        .ok_or_else(|| anyhow!("Failed to downcast component"))?;

    *openapi_elem = actual_comp.clone();

    Ok(())
}
