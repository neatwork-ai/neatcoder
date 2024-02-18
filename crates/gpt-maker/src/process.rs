use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use openapiv3::{
    Callback, Components, Example, Header, Link, OpenAPI, Operation, Parameter,
    PathItem, Paths, ReferenceOr, RequestBody, Response, Schema,
    SecurityScheme,
};
use serde::Serialize;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Deref,
    sync::Arc,
};

pub struct TagPath {
    path: String,
    item: ReferenceOr<PathItem>,
}

impl PartialEq for TagPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for TagPath {}

impl Hash for TagPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only the path field is used to compute the hash
        self.path.hash(state);
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ComponentPointer {
    pub name: String,
    #[serde(skip)]
    pub comp: Arc<dyn Any>, // &'a ReferenceOr<>
    // Due to nested referencing, there will be the need to dereference these
    // items in the ref_map once fully collected
    pub needs_deref: bool,
    pub phantom_t: String, // Type reflection
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

pub fn split_specs_with_refs<'a>(
    openapi: &'a OpenAPI,
    tags_selected: &'a HashSet<&'a str>,
    tag_paths: &'a HashMap<String, HashSet<TagPath>>,
    tag_refs: &'a HashMap<String, HashSet<String>>,
    ref_map: &'a HashMap<String, ComponentPointer>,
) -> Result<HashMap<&'a str, OpenAPI>> {
    let mut oapis = HashMap::new();

    for tag in tags_selected.iter() {
        let mut paths = Paths {
            paths: IndexMap::new(),
            extensions: IndexMap::new(),
        };

        let paths_to_add = tag_paths.get(*tag).unwrap();

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
            let refs_to_add = tag_refs.get(*tag).unwrap();

            for ref_to_add in refs_to_add {
                fill_components(&mut components, ref_to_add, &ref_map)?;
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

pub fn split_specs_raw<'a>(
    openapi: &'a OpenAPI,
    tags_selected: &'a HashSet<&'a str>,
    tag_paths: &'a HashMap<String, HashSet<TagPath>>,
    ref_map: &'a HashMap<String, ComponentPointer>,
) -> Result<HashMap<&'a str, OpenAPI>> {
    let mut oapis = HashMap::new();

    for tag in tags_selected.iter() {
        let mut paths = Paths {
            paths: IndexMap::new(),
            extensions: IndexMap::new(),
        };

        let paths_to_add = tag_paths.get(*tag).unwrap();

        for path_to_add in paths_to_add.iter() {
            let key = path_to_add.path.to_string();
            let val = path_to_add.item.clone();
            paths.paths.insert(key, val);
        }

        let oapi = OpenAPI {
            openapi: openapi.openapi.clone(),
            info: openapi.info.clone(),
            servers: openapi.servers.clone(),
            paths,
            components: None,
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

pub fn build_tag_paths(
    openapi: &OpenAPI,
    tags_selected: &HashSet<&str>,
    tag_paths: &mut HashMap<String, HashSet<TagPath>>,
) -> Result<()> {
    for (path, item) in &openapi.paths.paths {
        match item {
            // 2. If a path has a reference then:
            ReferenceOr::Item(path_item) => {
                let tags_in_path = get_tags_in_path(path_item);

                let relevant_tags: HashSet<&str> = tags_selected
                    .intersection(&tags_in_path)
                    .cloned()
                    .collect();

                for tag in relevant_tags.iter() {
                    // 2.2. Add those paths to the dedicated tags
                    let this_tag_paths = tag_paths.get_mut(*tag).unwrap();

                    this_tag_paths.insert(TagPath {
                        path: path.clone(),
                        item: item.clone(),
                    });
                }
            }
            _ => {
                return Err(anyhow!(
                    "Unexpected reference at the top path level"
                ))
            }
        }
    }

    Ok(())
}

// pub fn assemble_ref_list_todo_old<'a>(
//     openapi: &'a OpenAPI,
//     tags_selected: &'a HashSet<&'a str>,
//     // tag_refs: &mut HashMap<String, HashSet<String>>,
//     tag_paths: &mut HashMap<String, HashSet<TagPath>>,
// ) -> HashSet<String> {
//     let mut ref_list = HashSet::new(); // example of a ref: "#/components/examples/root"

//     for (path, item) in &openapi.paths.paths {
//         match item {
//             // 2. If a path has a reference then:
//             ReferenceOr::Item(path_item) => {
//                 let tags_in_path = get_tags_in_path(path_item);

//                 let relevant_tags: HashSet<&str> = tags_selected
//                     .intersection(&tags_in_path)
//                     .cloned()
//                     .collect();

//                 if !relevant_tags.is_empty() {
//                     let refs = path_item.get_refs();
//                     for tag in relevant_tags.iter() {
//                         // 2.1. Add those references to the dedicated tags
//                         let this_tag_refs = tag_refs.get_mut(*tag).unwrap();
//                         this_tag_refs.extend(refs.clone());

//                         // 2.2. Add those paths to the dedicated tags
//                         let this_tag_paths = tag_paths.get_mut(*tag).unwrap();

//                         this_tag_paths.insert(TagPath {
//                             path: path.clone(),
//                             item: item.clone(),
//                         });
//                     }

//                     ref_list.extend(refs.clone()); // keep track of all the refs for later
//                 }
//             }
//             _ => {} // Skip if it's just a reference at the path level, as we want to inspect operations
//         }
//     }

//     ref_list
// }

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

fn get_tags_in_path<'a>(path_item: &'a PathItem) -> HashSet<&'a str> {
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
    use serde_json::{from_value, to_value};

    use crate::deref::DerefAPISpecs;
    use crate::get_refs::GetRefs;
    use crate::preprocess::PreProcess;
    use crate::ref_map::RefMap;
    use crate::utils::resolve_references;

    use super::*;
    use std::collections::HashSet;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // #[test]
    // fn test_split_specs() -> Result<()> {
    //     // Step 1: Load the OpenAPI spec from the file
    //     let file_path = "tests/data/gh.json";
    //     let file_contents = fs::read_to_string(file_path)
    //         .expect("Failed to read OpenAPI spec file");
    //     let mut openapi: OpenAPI = serde_json::from_str(&file_contents)
    //         .expect("Failed to parse OpenAPI spec from JSON");

    //     // Step 2: Create a HashSet containing the tags you want to filter by
    //     let tags_selected: HashSet<&str> = HashSet::from(["repos"]); // Replace with actual tags

    //     let mut tag_paths: HashMap<String, HashSet<TagPath>> = HashMap::new(); // Maps tags to paths
    //     let mut tag_refs: HashMap<String, HashSet<String>> = HashMap::new(); // Maps tags to references

    //     for tag in tags_selected.iter() {
    //         tag_paths.insert(tag.to_string(), HashSet::new());
    //         tag_refs.insert(tag.to_string(), HashSet::new());
    //     }

    //     // 1. Crawl through all the paths
    //     let mut ref_list = assemble_ref_list(
    //         &openapi, // here first
    //         &tags_selected,
    //         &mut tag_refs,
    //         &mut tag_paths,
    //     );

    //     let ref_map = assemble_ref_map(&openapi, &mut ref_list);

    //     process_specs(&mut openapi);

    //     // Step 4: Call the split_specs function
    //     let mut specs_map = split_specs(
    //         &openapi, // here second
    //         &tags_selected,
    //         &tag_paths,
    //         &tag_refs,
    //         &ref_map,
    //     )?;

    //     for (_, spec) in specs_map.iter_mut() {
    //         spec.openapi_deref(&ref_map)?;
    //     }

    //     println!("Yey!");

    //     let output_dir = Path::new("tests/output");
    //     if !output_dir.exists() {
    //         fs::create_dir_all(output_dir)
    //             .expect("Failed to create output directory");
    //     }

    //     // Perform various assertions depending on what you expect
    //     // For example, check if the map contains keys for each tag
    //     for (tag, spec) in specs_map {
    //         // Convert the OpenAPI object back to a JSON string
    //         let spec_json = serde_json::to_string_pretty(&spec)
    //             .expect("Failed to serialize OpenAPI spec to JSON");

    //         // Define the output file path
    //         let file_path = output_dir.join(format!("{}_spec.json", tag));
    //         let mut file =
    //             File::create(&file_path).expect("Failed to create output file");

    //         // Write the JSON string to the file
    //         file.write_all(spec_json.as_bytes())
    //             .expect("Failed to write OpenAPI spec to file");

    //         println!(
    //             "Saved split spec for tag '{}' to '{}'",
    //             tag,
    //             file_path.display()
    //         );
    //     }

    //     Ok(())
    // }

    // #[test]
    // fn test_split_specs_2() -> Result<()> {
    //     // Step 1: Load the OpenAPI spec from the file
    //     let file_path = "tests/data/gh_0.json";
    //     let file_contents = fs::read_to_string(file_path)
    //         .expect("Failed to read OpenAPI spec file");
    //     let mut openapi: OpenAPI = serde_json::from_str(&file_contents)
    //         .expect("Failed to parse OpenAPI spec from JSON");

    //     // Step...: Preprocess the OpenAPI spec
    //     preprocess_specs(&mut openapi);

    //     // Step 2: Create a HashSet containing the tags you want to filter by
    //     let tags_selected: HashSet<&str> = HashSet::from(["repos"]); // Replace with actual tags

    //     // Step 3: Dereference the Components
    //     if let Some(components) = &mut openapi.components {
    //         // Get reference list
    //         println!("Getting Component Reference List");
    //         let ref_list: HashSet<String> = components.get_refs();

    //         // Build reference map
    //         println!("Building Component Reference Map");
    //         let mut ref_map = HashMap::new();
    //         components.build_ref_map(&ref_list, &mut ref_map)?;

    //         // Dereference components
    //         println!("Dereferencing components");
    //         components.deref_specs(&ref_map)?;
    //     }

    //     // === Step 4: Dereference the OpenAPI specs ===

    //     // Get reference list
    //     println!("Getting Reference List");
    //     let ref_list: HashSet<String> = openapi.get_refs();

    //     // Build reference map
    //     println!("Building Reference Map");
    //     let mut ref_map = HashMap::new();
    //     openapi.build_ref_map(&ref_list, &mut ref_map)?;

    //     // Dereference specs
    //     println!("Dereferencing OpenAPI Specs");
    //     openapi.deref_specs(&ref_map)?;

    //     // TEMP

    //     let output_dir = Path::new("tests/output");
    //     if !output_dir.exists() {
    //         fs::create_dir_all(output_dir)
    //             .expect("Failed to create output directory");
    //     }

    //     let output_dir = Path::new("tests/output");
    //     if !output_dir.exists() {
    //         fs::create_dir_all(output_dir)
    //             .expect("Failed to create output directory");
    //     }

    //     let spec_json = serde_json::to_string_pretty(&openapi.components)
    //         .expect("Failed to serialize OpenAPI spec to JSON");

    //     // Define the output file path
    //     let file_path = output_dir.join(format!("comps.json"));
    //     let mut file =
    //         File::create(&file_path).expect("Failed to create output file");

    //     // Write the JSON string to the file
    //     file.write_all(spec_json.as_bytes())
    //         .expect("Failed to write OpenAPI spec to file");

    //     panic!();

    //     // === Step 5: Split OpenAPI specs ===

    //     // Build mapping between Tags and Paths
    //     println!("Mapping Tags to Paths");
    //     let mut tag_paths: HashMap<String, HashSet<TagPath>> = HashMap::new(); // Maps tags to paths

    //     for tag in tags_selected.iter() {
    //         tag_paths.insert(tag.to_string(), HashSet::new());
    //     }

    //     build_tag_paths(&openapi, &tags_selected, &mut tag_paths)?;

    //     // Split specs accordingly
    //     println!("Splitting OpenAPI Specs");
    //     let specs_map = split_specs_raw(
    //         &openapi, // here second
    //         &tags_selected,
    //         &tag_paths,
    //         &ref_map,
    //     )?;

    //     println!("Yey!");

    //     let output_dir = Path::new("tests/output");
    //     if !output_dir.exists() {
    //         fs::create_dir_all(output_dir)
    //             .expect("Failed to create output directory");
    //     }

    //     // Perform various assertions depending on what you expect
    //     // For example, check if the map contains keys for each tag
    //     for (tag, spec) in specs_map {
    //         // Convert the OpenAPI object back to a JSON string
    //         let spec_json = serde_json::to_string_pretty(&spec)
    //             .expect("Failed to serialize OpenAPI spec to JSON");

    //         // Define the output file path
    //         let file_path = output_dir.join(format!("{}_spec.json", tag));
    //         let mut file =
    //             File::create(&file_path).expect("Failed to create output file");

    //         // Write the JSON string to the file
    //         file.write_all(spec_json.as_bytes())
    //             .expect("Failed to write OpenAPI spec to file");

    //         println!(
    //             "Saved split spec for tag '{}' to '{}'",
    //             tag,
    //             file_path.display()
    //         );
    //     }

    //     Ok(())
    // }

    #[test]
    fn test_split_specs_3() -> Result<()> {
        // Step 1: Load the OpenAPI spec from the file
        let file_path = "tests/data/gh.json";
        let file_contents = fs::read_to_string(file_path)
            .expect("Failed to read OpenAPI spec file");
        let mut openapi: OpenAPI = serde_json::from_str(&file_contents)
            .expect("Failed to parse OpenAPI spec from JSON");

        // Step...: Preprocess the OpenAPI spec
        openapi.pre_process()?;

        // Step 2: Create a HashSet containing the tags you want to filter by
        let tags_selected: HashSet<&str> = HashSet::from(["repos", "projects"]); // Replace with actual tags

        // Step 3: Dereference the Components
        if let Some(components) = &mut openapi.components {
            let mut comp_val = to_value(&components).unwrap();
            let comp_val_copy = comp_val.clone();

            resolve_references(&mut comp_val, &comp_val_copy);

            *components = from_value(comp_val).unwrap();
        }

        // TEMP

        let output_dir = Path::new("tests/output");
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .expect("Failed to create output directory");
        }

        let output_dir = Path::new("tests/output");
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .expect("Failed to create output directory");
        }

        let spec_json = serde_json::to_string_pretty(&openapi.components)
            .expect("Failed to serialize OpenAPI spec to JSON");

        // Define the output file path
        let file_path = output_dir.join(format!("comps.json"));
        let mut file =
            File::create(&file_path).expect("Failed to create output file");

        // Write the JSON string to the file
        file.write_all(spec_json.as_bytes())
            .expect("Failed to write OpenAPI spec to file");

        // === Step 4: Dereference the OpenAPI specs ===

        // Get reference list
        println!("Getting Reference List");
        let ref_list: HashSet<String> = openapi.get_refs();

        // Build reference map
        println!("Building Reference Map");
        let mut ref_map = HashMap::new();
        openapi.build_ref_map(&ref_list, &mut ref_map)?;

        // Dereference specs
        println!("Dereferencing OpenAPI Specs");
        openapi.deref_specs(&ref_map)?;

        // TEMP

        let output_dir = Path::new("tests/output");
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .expect("Failed to create output directory");
        }

        let output_dir = Path::new("tests/output");
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .expect("Failed to create output directory");
        }

        let spec_json = serde_json::to_string_pretty(&openapi)
            .expect("Failed to serialize OpenAPI spec to JSON");

        // Define the output file path
        let file_path = output_dir.join(format!("openapi.json"));
        let mut file =
            File::create(&file_path).expect("Failed to create output file");

        // Write the JSON string to the file
        file.write_all(spec_json.as_bytes())
            .expect("Failed to write OpenAPI spec to file");

        // === Step 5: Split OpenAPI specs ===

        // Build mapping between Tags and Paths
        println!("Mapping Tags to Paths");
        let mut tag_paths: HashMap<String, HashSet<TagPath>> = HashMap::new(); // Maps tags to paths

        for tag in tags_selected.iter() {
            tag_paths.insert(tag.to_string(), HashSet::new());
        }

        build_tag_paths(&openapi, &tags_selected, &mut tag_paths)?;

        // Split specs accordingly
        println!("Splitting OpenAPI Specs");
        let specs_map = split_specs_raw(
            &openapi, // here second
            &tags_selected,
            &tag_paths,
            &ref_map,
        )?;

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
            let file_path = output_dir.join(format!("{}_spec.json", tag));
            let mut file =
                File::create(&file_path).expect("Failed to create output file");

            // Write the JSON string to the file
            file.write_all(spec_json.as_bytes())
                .expect("Failed to write OpenAPI spec to file");

            println!(
                "Saved split spec for tag '{}' to '{}'",
                tag,
                file_path.display()
            );
        }

        Ok(())
    }
}
