use crate::process::ComponentPointer;
use anyhow::{anyhow, Result};
use openapiv3::{
    AdditionalProperties, Callback, Components, Encoding, Header, MediaType,
    OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent,
    PathItem, Paths, ReferenceOr, RequestBody, Response, Responses, Schema,
    SchemaKind, Server, Type,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
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
pub trait DerefAPISpecs {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()>;
}

impl DerefAPISpecs for OpenAPI {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        // Components
        if let Some(components) = &mut self.components {
            components.deref_specs(&ref_map)?;
        }

        // Paths
        self.paths.deref_specs(&ref_map)?;

        Ok(())
    }
}

// TODO: This is not working properly for some unknown reason...
// Use utils::resolve_references for the timebeing...
impl DerefAPISpecs for Components {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        // Responses
        self.responses
            .iter_mut()
            .try_for_each(|(_, response)| match_deref(response, ref_map))?;

        // Parameters
        self.parameters
            .iter_mut()
            .try_for_each(|(_, parameter)| match_deref(parameter, ref_map))?;

        // Request Bodies
        self.request_bodies
            .iter_mut()
            .try_for_each(|(_, request_body)| {
                match_deref(request_body, ref_map)
            })?;

        // Headers
        self.headers
            .iter_mut()
            .try_for_each(|(_, header)| match_deref(header, ref_map))?;

        // Schemas
        self.schemas
            .iter_mut()
            .try_for_each(|(_, schema)| match_deref(schema, ref_map))?;

        // Callbacks
        self.callbacks
            .iter_mut()
            .try_for_each(|(_, callback)| match_deref(callback, ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for Callback {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.iter_mut()
            .try_for_each(|(_, path_item)| path_item.deref_specs(ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for Paths {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.paths
            .iter_mut()
            .try_for_each(|(_, path)| path.deref_specs(ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for PathItem {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(get) = &mut self.get {
            get.deref_specs(ref_map)?;
        }

        if let Some(put) = &mut self.put {
            put.deref_specs(ref_map)?;
        }

        if let Some(post) = &mut self.post {
            post.deref_specs(ref_map)?;
        }

        if let Some(delete) = &mut self.delete {
            delete.deref_specs(ref_map)?;
        }

        if let Some(options) = &mut self.options {
            options.deref_specs(ref_map)?;
        }

        if let Some(head) = &mut self.head {
            head.deref_specs(ref_map)?;
        }

        if let Some(patch) = &mut self.patch {
            patch.deref_specs(ref_map)?;
        }

        if let Some(trace) = &mut self.trace {
            trace.deref_specs(ref_map)?;
        }

        self.parameters
            .iter_mut()
            .try_for_each(|parameter| parameter.deref_specs(ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for Operation {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.parameters
            .iter_mut()
            .try_for_each(|param| param.deref_specs(ref_map))?;

        self.responses.deref_specs(ref_map)?;

        if let Some(request_body) = &mut self.request_body {
            request_body.deref_specs(ref_map)?;
        }

        Ok(())
    }
}

impl DerefAPISpecs for RequestBody {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.content
            .iter_mut()
            .try_for_each(|(_, media)| media.deref_specs(ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for Parameter {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match self {
            Parameter::Query {
                parameter_data,
                allow_reserved: _,
                style: _,
                allow_empty_value: _,
            } => parameter_data.deref_specs(ref_map),
            Parameter::Path {
                parameter_data,
                style: _,
            } => parameter_data.deref_specs(ref_map),
            Parameter::Header {
                parameter_data,
                style: _,
            } => parameter_data.deref_specs(ref_map),
            Parameter::Cookie {
                parameter_data,
                style: _,
            } => parameter_data.deref_specs(ref_map),
        }?;

        Ok(())
    }
}

impl DerefAPISpecs for ParameterData {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.format.deref_specs(ref_map)?;

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

impl DerefAPISpecs for ParameterSchemaOrContent {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match self {
            ParameterSchemaOrContent::Schema(schema) => {
                schema.deref_specs(ref_map)
            }
            ParameterSchemaOrContent::Content(content) => {
                content.iter_mut().try_for_each::<_, Result<()>>(
                    |(_, media)| {
                        media.deref_specs(ref_map)?;

                        Ok(())
                    },
                )?;

                Ok(())
            }
        }?;

        Ok(())
    }
}

impl DerefAPISpecs for Responses {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(default) = &mut self.default {
            default.deref_specs(ref_map)?;
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
//     T: 'a + DerefAPISpecs + Clone,
//     U: Deref<Target = T> + Clone,
// {

fn match_deref<T: 'static>(
    ref_enum: &mut ReferenceOr<T>,
    ref_map: &HashMap<String, ComponentPointer>,
) -> Result<()>
where
    T: DerefAPISpecs + Clone,
{
    if let ReferenceOr::Item(item) = ref_enum {
        item.deref_specs(ref_map)?;
    }

    if let ReferenceOr::Reference { reference } = ref_enum.clone() {
        deref_comp(ref_enum, &reference, ref_map)?;
    }

    Ok(())
}

fn match_deref_ignore_item<T: 'static + Clone>(
    ref_enum: &mut ReferenceOr<T>,
    ref_map: &HashMap<String, ComponentPointer>,
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
    ref_enum: &mut ReferenceOr<Box<T>>,
    ref_map: &HashMap<String, ComponentPointer>,
    // is_signal: bool,
) -> Result<()>
where
    T: DerefAPISpecs + Clone,
{
    if let ReferenceOr::Item(item) = ref_enum {
        // Item here is &mut Box<T>
        item.deref_specs(ref_map)?;
    } else if let ReferenceOr::Reference { reference } = ref_enum.clone() {
        // if is_signal {
        //     println!("REF IS: {}", reference);
        // }
        if reference == "#/components/schemas/nullable-team-simpler" {
            println!("OINK!!!");
        }
        deref_comp_box(ref_enum, &reference, ref_map)?;
    }

    Ok(())
}

impl DerefAPISpecs for Response {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.headers
            .iter_mut()
            .try_for_each(|(_, header)| match_deref(header, ref_map))?;

        self.content
            .iter_mut()
            .try_for_each(|(_, media)| media.deref_specs(ref_map))?;

        // self.links.iter_mut().try_for_each(|(_, link)| {
        //     link.deref_specs(ref_map)
        // })?

        Ok(())
    }
}

impl DerefAPISpecs for MediaType {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(schema) = &mut self.schema {
            match_deref(schema, ref_map)?;
        }

        self.examples.iter_mut().try_for_each(|(_, example)| {
            match_deref_ignore_item(example, ref_map)
        })?;

        self.encoding
            .iter_mut()
            .try_for_each(|(_, encoding)| encoding.deref_specs(ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for Schema {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match &mut self.schema_kind {
            SchemaKind::Type(schema_type) => match schema_type {
                Type::Object(object) => {
                    object.properties.iter_mut().try_for_each(
                        |(prop_name, property)| {
                            println!("Dereferencing {:?}", prop_name);

                            match_deref_box(property, ref_map)
                        },
                    )?;

                    if let Some(additional_properties) =
                        &mut object.additional_properties
                    {
                        match additional_properties {
                            AdditionalProperties::Any(_) => {}
                            AdditionalProperties::Schema(boxed_schema) => {
                                boxed_schema.deref_specs(ref_map)?
                            }
                        }
                    }
                }
                Type::Array(array) => {
                    if let Some(items) = &mut array.items {
                        match_deref_box(items, ref_map)?;
                    }
                }
                _ => {}
            }, // No Inner references
            SchemaKind::OneOf { one_of: of }
            | SchemaKind::AllOf { all_of: of }
            | SchemaKind::AnyOf { any_of: of } => {
                of.iter_mut()
                    .try_for_each(|schema| schema.deref_specs(ref_map))?;
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
                    .try_for_each(|schema| schema.deref_specs(ref_map))?;

                any_schema
                    .all_of
                    .iter_mut()
                    .try_for_each(|schema| schema.deref_specs(ref_map))?;
                any_schema
                    .any_of
                    .iter_mut()
                    .try_for_each(|schema| schema.deref_specs(ref_map))?;

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

impl DerefAPISpecs for Encoding {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.headers
            .iter_mut()
            .try_for_each(|(_, header)| match_deref(header, ref_map))?;

        Ok(())
    }
}

impl DerefAPISpecs for Header {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        self.examples.iter_mut().try_for_each(|(_, example)| {
            match_deref_ignore_item(example, ref_map)
        })?;

        Ok(())
    }
}

fn deref_comp<T: 'static + Clone>(
    openapi_elem: &mut ReferenceOr<T>,
    reference: &String,
    ref_map: &HashMap<String, ComponentPointer>,
) -> Result<()> {
    let comp: ComponentPointer = ref_map
        .get(reference)
        .ok_or_else(|| {
            anyhow!("Error retrieving {:?} from ref_map", reference)
        })?
        .clone();

    let actual_comp = comp
        .comp
        .as_ref()
        .downcast_ref::<ReferenceOr<T>>()
        .ok_or_else(|| {
            anyhow!(
                "Failed to downcast component {:?} to type: {:?}",
                comp,
                std::any::type_name::<T>()
            )
        })?;

    *openapi_elem = actual_comp.clone();

    Ok(())
}

fn deref_comp_box<T: 'static + Clone>(
    openapi_elem: &mut ReferenceOr<Box<T>>,
    reference: &String,
    ref_map: &HashMap<String, ComponentPointer>,
) -> Result<()> {
    let comp: ComponentPointer = ref_map
        .get(reference)
        .ok_or_else(|| {
            anyhow!("Error retrieving {:?} from ref_map", reference)
        })?
        .clone();

    let actual_comp = comp
        .comp
        .as_ref()
        .downcast_ref::<ReferenceOr<T>>()
        .ok_or_else(|| {
            anyhow!(
                "Failed to downcast component {:?} to type: {:?}",
                comp,
                std::any::type_name::<T>()
            )
        })?;

    *openapi_elem = match actual_comp {
        ReferenceOr::Item(item) => ReferenceOr::Item(Box::new(item.clone())),
        ReferenceOr::Reference { reference } => ReferenceOr::Reference {
            reference: reference.clone(),
        },
    };

    Ok(())
}

impl<T: 'static + DerefAPISpecs + Clone> DerefAPISpecs for ReferenceOr<T> {
    fn deref_specs(
        &mut self,
        ref_map: &HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        match_deref(self, ref_map)?;

        Ok(())
    }
}

// // We have ReferenceOr<Box<Schema>>
// // TODO
// impl<T: DerefAPISpecs> DerefAPISpecs for ReferenceOr<Box<T>> {
//     fn deref_specs(
//         &mut self,
//         ref_map: &HashMap<String, ComponentPointer>,
//     ) -> Result<()> {
//         deref_comp_box
//     }
// }
