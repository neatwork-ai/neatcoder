use indexmap::IndexMap;
use openapiv3::{
    Components, ExternalDocumentation, Info, OpenAPI, Operation, PathItem,
    Paths, ReferenceOr, SecurityRequirement, Server, Tag,
};
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Serialize)]
pub struct OpenAPIRef<'a> {
    pub openapi: &'a String,
    pub info: &'a Info,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: &'a Vec<Server>,
    pub paths: &'a Paths,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: &'a Option<Components>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: &'a Option<Vec<SecurityRequirement>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: &'a Vec<Tag>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: &'a Option<ExternalDocumentation>,
    // #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    // pub extensions: IndexMap<String, serde_json::Value>,
}

pub fn split_specs<'a>(
    openapi: &'a OpenAPI,
    tag_selected: &'a String,
) -> HashMap<String, OpenAPIRef<'a>> {
    let mut ref_list = HashSet::new(); // example of a ref: "#/components/examples/root"
    let mut ref_map: HashMap<&String, (&String, )> = HashMap::new();

    // 1. For each tag selected
    let mut tag_paths: HashMap<&String, HashSet<&String>> = HashMap::new();
    let mut tag_refs: HashMap<&String, HashSet<&String>> = HashMap::new();

    // 1. For each tag selected
    for tag in tags_selected {
        tag_paths.insert(tag, HashSet::new());
        tag_refs.insert(tag, HashSet::new());
    }

    // 2. Crawl and get the refs
    // Process paths and operations to associate them with tags
    for (path, item) in &openapi.paths.paths {
        match item {
            ReferenceOr::Item(path_item) => {
                let tags_in_path = get_tags_in_path(path_item);

                let relevant_tags: HashSet<&String> = tags_selected
                    .intersection(&tags_in_path)
                    .cloned()
                    .collect();

                let refs = get_refs_in_path(path_item);

                for tag in tags_selected {
                    let this_tag_refs = tag_refs.get_mut(tag).unwrap();
                    this_tag_refs.extend(refs.clone())
                }

                ref_list.extend(refs.clone()); // keep track of all the refs for later
            }
            _ => {} // Skip if it's just a reference at the path level, as we want to inspect operations
        }
    }

    // TODO: This should be if let at the beginning
    let components = openapi.components.unwrap();

    let find_component = |reff: &String, components: &IndexMap<String, ReferenceOr<T>>, name: &String| -> {
        let item = components.get(name).unwrap();
        ref_map.insert(reff, )
    }


    // 3. Match refs to components
    for reff in ref_list.iter() {
        if let Some((field, index)) = parse_ref(reff) {
            let balony: Box<&IndexMap<String, dyn Any>> = match field {
                "security_schemes" => Box::new(&components.security_schemes),
                "responses" => {
                    (Box::new(&components.responses)
                        as Box<&IndexMap<String, dyn Any>>)
                }
                // "parameters" => Box::new(&components.parameters),
                // "examples" => Box::new(&components.examples),
                // "request_bodies" => Box::new(&components.request_bodies),
                // "headers" => Box::new(&components.headers),
                // "schemas" => Box::new(&components.schemas),
                // "links" => Box::new(&components.links),
                // "callbacks" => Box::new(&components.callbacks),
                // "extensions" => Box::new(&components.extensions),
                _ => None,
            };
        }
    }

    // 3. Crawl and get the refs to find which components each tag uses
    crawl_components_for_tags(&openapi.components, &tag_to_refs);

    // 4. Run through the components and collect all that are in the refs
    //    Create a new OpenAPI spec per tag with its own components
    let mut specs: HashMap<String, OpenAPI> = HashMap::new();
    for tag in tags_selected {
        let mut new_components = openapiv3::Components::default();
        if let Some(components) = &openapi.components {
            for (component_type, component_refs) in
                tag_to_refs.get(tag).unwrap()
            {
                components.clone_components_of_type_into(
                    component_type,
                    component_refs,
                    &mut new_components,
                );
            }
        }

        specs.insert(
            tag.to_string(),
            OpenAPI {
                openapi: openapi.openapi.clone(),
                info: openapi.info.clone(),
                servers: openapi.servers.clone(),
                paths: openapiv3::Paths {
                    paths: tag_to_paths.remove(tag).unwrap(),
                    extensions: Default::default(),
                },
                components: Some(new_components),
                security: openapi.security.clone(),
                tags: openapi.tags.clone(),
                external_docs: openapi.external_docs.clone(),
                extensions: openapi.extensions.clone(),
            },
        );
    }

    // 4. Return the structs
    specs
}

// pub fn split_specs<'a>(
//     openapi: &'a OpenAPI,
//     tags_selected: &'a HashSet<&'a String>,
// ) -> HashMap<String, OpenAPIRef<'a>> {
//     let mut ref_list = HashSet::new(); // example of a ref: "#/components/examples/root"
//     let mut ref_map: HashMap<&String, (&String, )> = HashMap::new();

//     // 1. For each tag selected
//     let mut tag_paths: HashMap<&String, HashSet<&String>> = HashMap::new();
//     let mut tag_refs: HashMap<&String, HashSet<&String>> = HashMap::new();

//     // 1. For each tag selected
//     for tag in tags_selected {
//         tag_paths.insert(tag, HashSet::new());
//         tag_refs.insert(tag, HashSet::new());
//     }

//     // 2. Crawl and get the refs
//     // Process paths and operations to associate them with tags
//     for (path, item) in &openapi.paths.paths {
//         match item {
//             ReferenceOr::Item(path_item) => {
//                 let tags_in_path = get_tags_in_path(path_item);

//                 let relevant_tags: HashSet<&String> = tags_selected
//                     .intersection(&tags_in_path)
//                     .cloned()
//                     .collect();

//                 let refs = get_refs_in_path(path_item);

//                 for tag in tags_selected {
//                     let this_tag_refs = tag_refs.get_mut(tag).unwrap();
//                     this_tag_refs.extend(refs.clone())
//                 }

//                 ref_list.extend(refs.clone()); // keep track of all the refs for later
//             }
//             _ => {} // Skip if it's just a reference at the path level, as we want to inspect operations
//         }
//     }

//     // TODO: This should be if let at the beginning
//     let components = openapi.components.unwrap();

//     let find_component = |reff: &String, components: &IndexMap<String, ReferenceOr<T>>, name: &String| -> {
//         let item = components.get(name).unwrap();
//         ref_map.insert(reff, )
//     }

//     // 3. Match refs to components
//     for reff in ref_list.iter() {
//         if let Some((field, index)) = parse_ref(reff) {
//             let balony: Box<&IndexMap<String, dyn Any>> = match field {
//                 "security_schemes" => Box::new(&components.security_schemes),
//                 "responses" => {
//                     (Box::new(&components.responses)
//                         as Box<&IndexMap<String, dyn Any>>)
//                 }
//                 // "parameters" => Box::new(&components.parameters),
//                 // "examples" => Box::new(&components.examples),
//                 // "request_bodies" => Box::new(&components.request_bodies),
//                 // "headers" => Box::new(&components.headers),
//                 // "schemas" => Box::new(&components.schemas),
//                 // "links" => Box::new(&components.links),
//                 // "callbacks" => Box::new(&components.callbacks),
//                 // "extensions" => Box::new(&components.extensions),
//                 _ => None,
//             };
//         }
//     }

//     // 3. Crawl and get the refs to find which components each tag uses
//     crawl_components_for_tags(&openapi.components, &tag_to_refs);

//     // 4. Run through the components and collect all that are in the refs
//     //    Create a new OpenAPI spec per tag with its own components
//     let mut specs: HashMap<String, OpenAPI> = HashMap::new();
//     for tag in tags_selected {
//         let mut new_components = openapiv3::Components::default();
//         if let Some(components) = &openapi.components {
//             for (component_type, component_refs) in
//                 tag_to_refs.get(tag).unwrap()
//             {
//                 components.clone_components_of_type_into(
//                     component_type,
//                     component_refs,
//                     &mut new_components,
//                 );
//             }
//         }

//         specs.insert(
//             tag.to_string(),
//             OpenAPI {
//                 openapi: openapi.openapi.clone(),
//                 info: openapi.info.clone(),
//                 servers: openapi.servers.clone(),
//                 paths: openapiv3::Paths {
//                     paths: tag_to_paths.remove(tag).unwrap(),
//                     extensions: Default::default(),
//                 },
//                 components: Some(new_components),
//                 security: openapi.security.clone(),
//                 tags: openapi.tags.clone(),
//                 external_docs: openapi.external_docs.clone(),
//                 extensions: openapi.extensions.clone(),
//             },
//         );
//     }

//     // 4. Return the structs
//     specs
// }

// Function to parse a reference string and return the component type and name.
fn parse_ref(ref_str: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = ref_str.split('/').collect();
    if parts.len() != 4 || parts[0] != "#" || parts[1] != "components" {
        None
    } else {
        // parts[2] is the component type, parts[3] is the name/key.
        Some((parts[2], parts[3]))
    }
}

// Function to match a ref to the corresponding entry in `Components`.
fn match_ref_to_component<'a>(
    components: &'a Components,
    ref_str: &str,
) -> Option<&'a IndexMap<String, openapiv3::ReferenceOr<openapiv3::Example>>> {
    if let Some((component_type, _component_name)) = parse_ref(ref_str) {
        match component_type {
            "examples" => Some(&components.examples),
            // Add cases for other component types as needed
            _ => None,
        }
    } else {
        None
    }
}

// Helper function to collect references from an operation
fn collect_refs_from_operation(
    operation: &Operation,
    refs: &mut HashSet<String>,
) {
    // Here you would inspect the operation and add its references to the set
    // ...
    todo!();
}

// Helper function to traverse the components and collect references that match the tags
fn crawl_components_for_tags(
    components: &Option<Components>,
    tag_to_refs: &HashMap<String, HashSet<String>>,
) {
    // Here you would traverse the components and collect only those referenced by the selected tags
    // ...
    todo!();
}

fn get_tags_in_path<'a>(path_item: &'a PathItem) -> HashSet<&'a String> {
    let mut all_operations: Vec<&Option<Operation>> = vec![
        &path_item.get,
        &path_item.put,
        &path_item.post,
        &path_item.delete,
        &path_item.options,
        &path_item.head,
        &path_item.patch,
        &path_item.trace,
    ];

    let mut all_tags: HashSet<&String> = HashSet::new();

    // TODO: Crawl through each operation and collect references
    for operation in all_operations.iter() {
        if let Some(operation) = operation {
            operation.tags.iter().for_each(|tag| {
                all_tags.insert(tag);
            });
        }
    }

    all_tags
}

// Helper function to collect operations by tag and referenced components
fn get_refs_in_path<'a>(path_item: &'a PathItem) -> HashSet<&'a String> {
    let mut all_operations: Vec<&Option<Operation>> = vec![
        &path_item.get,
        &path_item.put,
        &path_item.post,
        &path_item.delete,
        &path_item.options,
        &path_item.head,
        &path_item.patch,
        &path_item.trace,
    ];

    let mut all_refs: HashSet<&String> = HashSet::new();

    // TODO: Crawl through each operation and collect references
    for operation in all_operations.iter() {
        if let Some(operation) = operation {
            for param in operation.parameters.iter() {
                match param {
                    ReferenceOr::Reference { reference } => {
                        all_refs.insert(reference);
                    }
                    _ => {}
                }
            }

            if let Some(request_body) = &operation.request_body {
                match request_body {
                    ReferenceOr::Reference { reference } => {
                        all_refs.insert(reference);
                    }
                    _ => {}
                }
            }
        }
    }

    all_refs
}

pub fn find_references(openapi: &OpenAPI) -> HashSet<String> {
    let mut refs = HashSet::new();

    // Check paths to find references.
    for path in openapi.paths.paths.values() {
        match path {
            ReferenceOr::Reference { reference } => {
                refs.insert(reference.clone());
            }
            ReferenceOr::Item(path_item) => {
                // Add logic to process each operation in the path item.
                if let Some(operation) = &path_item.get {
                    find_references_in_operation(operation, &mut refs);
                }
                // ... repeat for other HTTP methods ...
            }
        }
    }

    // Process other parts of the OpenAPI spec that may contain references ...
    // e.g., components, parameters, etc.

    refs
}

fn find_references_in_operation(
    operation: &Operation,
    refs: &mut HashSet<String>,
) {
    // Check each field of the Operation which could contain references.

    // For example, processing parameters:
    for parameter in &operation.parameters {
        match parameter {
            ReferenceOr::Reference { reference } => {
                refs.insert(reference.clone());
            }
            ReferenceOr::Item(param) => {
                // Maybe you'll also need to process parameter fields here...
            }
        }
    }

    // For example, processing request body:
    if let Some(request_body) = &operation.request_body {
        match request_body {
            ReferenceOr::Reference { reference } => {
                refs.insert(reference.clone());
            }
            ReferenceOr::Item(body) => {
                // Process the request body if needed.
            }
        }
    }

    // ... continue for responses and other fields which may contain references.
}
