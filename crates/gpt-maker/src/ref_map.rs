use crate::process::ComponentPointer;
use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use openapiv3::{Components, OpenAPI, ReferenceOr};
use serde_json::Value;
use std::{
    any::type_name,
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub trait RefMap {
    fn build_ref_map(
        &mut self,
        ref_list: &HashSet<String>,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()>;
}

impl RefMap for OpenAPI {
    fn build_ref_map(
        &mut self,
        ref_list: &HashSet<String>,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        if let Some(components) = &mut self.components {
            components.build_ref_map(ref_list, ref_map)?;
        }

        Ok(())
    }
}

impl RefMap for Components {
    fn build_ref_map(
        &mut self,
        ref_list: &HashSet<String>,
        ref_map: &mut HashMap<String, ComponentPointer>,
    ) -> Result<()> {
        // 3. Map the components to references
        for ref_name in ref_list.iter() {
            if let Some((ref_type, short_name)) = parse_ref(ref_name) {
                match ref_type {
                    "security_schemes" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.security_schemes,
                    ),
                    "responses" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.responses,
                    ),
                    "parameters" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.parameters,
                    ),
                    "examples" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.examples,
                    ),
                    "request_bodies" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.request_bodies,
                    ),
                    "headers" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.headers,
                    ),
                    "schemas" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.schemas,
                    ),
                    "links" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.links,
                    ),
                    "callbacks" => link_components_to_refs(
                        ref_map,
                        ref_name.clone(),
                        short_name,
                        &self.callbacks,
                    ),
                    _ => {
                        return Err(anyhow!(
                            "Unknown reference type: '{}'",
                            ref_type
                        ))
                    }
                };
            }
        }

        Ok(())
    }
}

pub fn assemble_ref_map(
    openapi: &OpenAPI,
    ref_list: &mut HashSet<String>,
) -> HashMap<String, ComponentPointer> {
    let mut ref_map = HashMap::new(); // Maps references to components

    if let Some(components) = &openapi.components {
        // 3. Map the components to references
        for ref_name in ref_list.iter() {
            if let Some((ref_type, short_name)) = parse_ref(ref_name) {
                match ref_type {
                    "security_schemes" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.security_schemes,
                    ),
                    "responses" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.responses,
                    ),
                    "parameters" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.parameters,
                    ),
                    "examples" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.examples,
                    ),
                    "request_bodies" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.request_bodies,
                    ),
                    "headers" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.headers,
                    ),
                    "schemas" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.schemas,
                    ),
                    "links" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.links,
                    ),
                    "callbacks" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        short_name,
                        &components.callbacks,
                    ),
                    _ => (),
                };
            }
        }
    }

    ref_map
}

/// Function to parse a reference string and return the component type and name.
/// For an input of: "#/components/schemas/validation-error-simple"
///
/// It returns ("schemas", "validation-error-simple")
///
/// Note that the type here is not equivalent to the Rust typename,
/// because it's in snake case
pub fn parse_ref(ref_str: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = ref_str.split('/').collect();
    if parts.len() != 4 || parts[0] != "#" || parts[1] != "components" {
        None
    } else {
        // parts[2] is the component type, parts[3] is the name/key.
        Some((parts[2], parts[3]))
    }
}

pub fn link_components_to_refs<T: Clone + 'static>(
    ref_map: &mut HashMap<String, ComponentPointer>,
    // e.g. "#/components/schemas/validation-error-simple"
    long_name: String,
    // e.g. "validation-error-simple"
    short_name: &str,
    sub_components: &IndexMap<String, ReferenceOr<T>>,
) {
    let comp = sub_components.get(short_name).unwrap().clone();

    let needs_deref = if let ReferenceOr::Reference { reference: _ } = &comp {
        true
    } else {
        false
    };

    let comp_pointer = ComponentPointer {
        name: short_name.to_owned(),
        comp: Arc::new(comp),
        needs_deref,
        phantom_t: String::from(type_name::<T>()),
    };

    ref_map.insert(long_name, comp_pointer);
}
