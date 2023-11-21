use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use openapiv3::{
    Callback, Components, Example, Header, Link, OpenAPI, Operation, Parameter,
    PathItem, Paths, ReferenceOr, RequestBody, Response, Schema,
    SecurityScheme,
};
use std::{
    any::type_name,
    any::Any,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Deref,
    sync::Arc,
};

use crate::get_refs::{process_path_item, process_server, GetRefs};

pub struct TagPath<'a> {
    path: &'a str,
    item: &'a ReferenceOr<PathItem>,
}

impl<'a> PartialEq for TagPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<'a> Eq for TagPath<'a> {}

impl<'a> Hash for TagPath<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only the path field is used to compute the hash
        self.path.hash(state);
    }
}

#[derive(Clone)]
pub struct ComponentPointer {
    pub name: String,
    pub comp: Arc<dyn Any>, // &'a ReferenceOr<>
    pub phantom_t: String,  // Type reflection
}

pub fn process_specs(openapi: &mut OpenAPI) {
    openapi.paths.paths.iter_mut().for_each(|(_, path)| {
        if let ReferenceOr::Item(path) = path {
            process_path_item(path)
        }
    });

    openapi
        .servers
        .iter_mut()
        .for_each(|server| process_server(server));
}

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

pub fn split_specs(
    openapi: Arc<OpenAPI>,
    tags_selected: Arc<HashSet<&str>>,
) -> Result<HashMap<&str, OpenAPI>> {
    let mut ref_list = HashSet::new(); // example of a ref: "#/components/examples/root"
    let mut tag_paths: HashMap<&str, HashSet<TagPath>> = HashMap::new(); // Maps tags to paths
    let mut tag_refs: HashMap<&str, HashSet<String>> = HashMap::new(); // Maps tags to references
    let mut ref_map: HashMap<String, ComponentPointer> = HashMap::new(); // Maps references to components

    for tag in tags_selected.iter() {
        tag_paths.insert(&tag, HashSet::new());
        tag_refs.insert(&tag, HashSet::new());
    }

    // 1. Crawl through all the paths
    for (path, item) in &openapi.paths.paths {
        match item {
            // 2. If a path has a reference then:
            ReferenceOr::Item(path_item) => {
                let tags_in_path = get_tags_in_path(path_item);

                let relevant_tags: HashSet<&str> = tags_selected
                    .intersection(&tags_in_path)
                    .cloned()
                    .collect();

                if !relevant_tags.is_empty() {
                    let refs = path_item.get_refs();
                    for tag in relevant_tags.iter() {
                        // 2.1. Add those references to the dedicated tags
                        let this_tag_refs = tag_refs.get_mut(tag).unwrap();
                        this_tag_refs.extend(refs.clone());

                        // 2.2. Add those paths to the dedicated tags
                        let this_tag_paths = tag_paths.get_mut(tag).unwrap();
                        this_tag_paths.insert(TagPath { path, item });
                    }

                    ref_list.extend(refs.clone()); // keep track of all the refs for later
                }
            }
            _ => {} // Skip if it's just a reference at the path level, as we want to inspect operations
        }
    }

    // TODO: This should be if let at the beginning
    // let components = openapi.components.map(Arc::new).unwrap();
    // let components = &openapi.components;

    if let Some(components) = &openapi.components {
        // 3. Map the components to references
        for ref_name in ref_list.iter() {
            if let Some((ref_t, ref_index)) = parse_ref(ref_name) {
                println!("Reference type: {}", ref_t);
                println!("Reference index: {}", ref_index);
                match ref_t {
                    "security_schemes" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.security_schemes,
                    ),
                    "responses" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.responses,
                    ),
                    "parameters" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.parameters,
                    ),
                    "examples" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.examples,
                    ),
                    "request_bodies" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.request_bodies,
                    ),
                    "headers" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.headers,
                    ),
                    "schemas" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.schemas,
                    ),
                    "links" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.links,
                    ),
                    "callbacks" => link_components_to_refs(
                        &mut ref_map,
                        ref_name.clone(),
                        ref_index,
                        &components.callbacks,
                    ),
                    _ => (),
                };
            }
        }
    }

    let mut oapis = HashMap::new();

    for tag in tags_selected.iter() {
        let mut paths = Paths {
            paths: IndexMap::new(),
            extensions: IndexMap::new(),
        };

        let paths_to_add = tag_paths.get(tag).unwrap();

        for path_to_add in paths_to_add.iter() {
            let key = path_to_add.path.to_string();
            let val = path_to_add.item.clone();
            paths.paths.insert(key, val);
        }

        let mut components = Components {
            security_schemes: IndexMap::new(),
            responses: IndexMap::new(),
            parameters: IndexMap::new(),
            examples: IndexMap::new(),
            request_bodies: IndexMap::new(),
            headers: IndexMap::new(),
            schemas: IndexMap::new(),
            links: IndexMap::new(),
            callbacks: IndexMap::new(),
            extensions: IndexMap::new(),
        };

        if let Some(_) = openapi.components {
            let refs_to_add = tag_refs.get(tag).unwrap();

            for ref_to_add in refs_to_add {
                fill_components(&mut components, ref_to_add, &mut ref_map)?;
            }
        }

        let oapi = OpenAPI {
            openapi: openapi.openapi.clone(),
            info: openapi.info.clone(),
            servers: openapi.servers.clone(),
            paths,
            components: Some(components),
            security: openapi.security.clone(),
            tags: openapi.tags.clone(), // TODO: wrong. this should only be selected tags
            external_docs: openapi.external_docs.clone(),
            extensions: openapi.extensions.clone(),
        };

        oapis.insert(*tag, oapi);
    }

    // 4. Return the structs
    Ok(oapis)
}

pub fn fill_components(
    components: &mut Components,
    ref_to_add: &String,
    ref_map: &HashMap<String, ComponentPointer>,
) -> Result<()> {
    // Get component from RefMap
    let comp: ComponentPointer = ref_map.get(ref_to_add).unwrap().clone();

    match comp.phantom_t.as_str() {
        "openapiv3::security_scheme::SecurityScheme" => {
            let actual_comp = comp
                .comp
                .as_ref()
                .downcast_ref::<ReferenceOr<SecurityScheme>>()
                .ok_or_else(|| {
                    anyhow!("Failed to downcast SecurityScheme component")
                })?;
            components
                .security_schemes
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::responses::Response" => {
            let actual_comp = comp
                .comp
                .downcast_ref::<ReferenceOr<Response>>()
                .ok_or_else(|| {
                    anyhow!("Failed to downcast Response component")
                })?;
            components
                .responses
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::parameter::Parameter" => {
            let actual_comp = comp
                .comp
                .downcast_ref::<ReferenceOr<Parameter>>()
                .ok_or_else(|| {
                    anyhow!("Failed to downcast Parameter component")
                })?;
            components
                .parameters
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::example::Example" => {
            let actual_comp = comp
                .comp
                .downcast_ref::<ReferenceOr<Example>>()
                .ok_or_else(|| {
                    anyhow!("Failed to downcast Example component")
                })?;
            components
                .examples
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::request_body::RequestBody" => {
            let actual_comp = comp
                .comp
                .downcast_ref::<ReferenceOr<RequestBody>>()
                .ok_or_else(|| {
                    anyhow!("Failed to downcast RequestBody component")
                })?;
            components
                .request_bodies
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::header::Header" => {
            let actual_comp =
                comp.comp.downcast_ref::<ReferenceOr<Header>>().ok_or_else(
                    || anyhow!("Failed to downcast Header component"),
                )?;
            components
                .headers
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::schema::Schema" => {
            let actual_comp =
                comp.comp.downcast_ref::<ReferenceOr<Schema>>().ok_or_else(
                    || anyhow!("Failed to downcast Schema component"),
                )?;
            components
                .schemas
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::link::Link" => {
            let actual_comp = comp
                .comp
                .downcast_ref::<ReferenceOr<Link>>()
                .ok_or_else(|| anyhow!("Failed to downcast Link component"))?;
            components
                .links
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        "openapiv3::callback::Callback" => {
            let actual_comp = comp
                .comp
                .downcast_ref::<ReferenceOr<Callback>>()
                .ok_or_else(|| {
                    anyhow!("Failed to downcast Callback component")
                })?;
            components
                .callbacks
                .insert(comp.name.to_string(), actual_comp.deref().clone());
        }
        _ => {
            return Err(anyhow!("Unknown component: {:?}", comp.phantom_t));
        }
    }
    Ok(())
}

pub fn link_components_to_refs<T: Clone + 'static>(
    ref_map: &mut HashMap<String, ComponentPointer>,
    reff_name: String,
    ref_index: &str,
    sub_components: &IndexMap<String, ReferenceOr<T>>,
) {
    let comp_pointer = ComponentPointer {
        name: ref_index.to_owned(),
        comp: Arc::new(sub_components.get(ref_index).unwrap().clone()),
        phantom_t: String::from(type_name::<T>()),
    };

    ref_map.insert(reff_name, comp_pointer);
}

// Function to parse a reference string and return the component type and name.
pub fn parse_ref(ref_str: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = ref_str.split('/').collect();
    if parts.len() != 4 || parts[0] != "#" || parts[1] != "components" {
        None
    } else {
        // parts[2] is the component type, parts[3] is the name/key.
        Some((parts[2], parts[3]))
    }
}

fn get_tags_in_path(path_item: &PathItem) -> HashSet<&str> {
    let all_operations: Vec<&Option<Operation>> = vec![
        &path_item.get,
        &path_item.put,
        &path_item.post,
        &path_item.delete,
        &path_item.options,
        &path_item.head,
        &path_item.patch,
        &path_item.trace,
    ];

    let mut all_tags: HashSet<&str> = HashSet::new();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::sync::Arc;

    #[test]
    fn test_split_specs() {
        // Step 1: Load the OpenAPI spec from the file
        let file_path = "tests/data/gh.json";
        let file_contents = fs::read_to_string(file_path)
            .expect("Failed to read OpenAPI spec file");
        let mut openapi: OpenAPI = serde_json::from_str(&file_contents)
            .expect("Failed to parse OpenAPI spec from JSON");

        process_specs(&mut openapi);

        // Step 2: Create a HashSet containing the tags you want to filter by
        let tags: HashSet<&str> = HashSet::from(["repos"]); // Replace with actual tags

        // Step 3: Wrap values in Arc
        let openapi_arc = Arc::new(openapi);
        let tags_arc = Arc::new(tags);

        // Step 4: Call the split_specs function
        let result = split_specs(openapi_arc.clone(), tags_arc.clone());

        // Step 5: Verify the function output
        match result {
            Ok(specs_map) => {
                println!("Yey!");

                let output_dir = Path::new("tests/output");
                if !output_dir.exists() {
                    fs::create_dir_all(output_dir)
                        .expect("Failed to create output directory");
                }

                // Perform various assertions depending on what you expect
                // For example, check if the map contains keys for each tag
                for (tag, spec) in specs_map {
                    // Convert the OpenAPI object back to a JSON string
                    let spec_json = serde_json::to_string_pretty(&spec)
                        .expect("Failed to serialize OpenAPI spec to JSON");

                    // Define the output file path
                    let file_path =
                        output_dir.join(format!("{}_spec.json", tag));
                    let mut file = File::create(&file_path)
                        .expect("Failed to create output file");

                    // Write the JSON string to the file
                    file.write_all(spec_json.as_bytes())
                        .expect("Failed to write OpenAPI spec to file");

                    println!(
                        "Saved split spec for tag '{}' to '{}'",
                        tag,
                        file_path.display()
                    );
                }

                // Further checks can include verifying the content of the split specs
            }
            Err(e) => panic!("split_specs function failed: {:?}", e),
        }
    }
}
