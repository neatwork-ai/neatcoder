use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use openapiv3::{
    Callback, Components, Example, ExternalDocumentation, Header, Info, Link,
    OpenAPI, Operation, Parameter, PathItem, Paths, ReferenceOr, RequestBody,
    Response, Schema, SecurityRequirement, SecurityScheme, Server, Tag,
};
use serde::{
    de::{IgnoredAny, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{
    any::type_name,
    any::Any,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::Arc,
};

#[derive(Serialize)]
pub struct OpenAPIRef {
    pub openapi: Arc<String>,
    pub info: Arc<Info>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Arc<Vec<Server>>,
    pub paths: PathsRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentsRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Arc<Option<Vec<SecurityRequirement>>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Arc<Vec<Tag>>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Arc<Option<ExternalDocumentation>>,
    // #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    // pub extensions: IndexMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct PathsRef {
    /// A map of PathItems or references to them.
    #[serde(flatten, deserialize_with = "deserialize_paths")]
    pub paths: IndexMap<Arc<String>, Arc<ReferenceOr<PathItem>>>,
    pub extensions: IndexMap<Arc<String>, Arc<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentsRef {
    /// An object to hold reusable Security Scheme Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub security_schemes:
        IndexMap<Arc<String>, Arc<ReferenceOr<SecurityScheme>>>,
    /// An object to hold reusable Response Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub responses: IndexMap<Arc<String>, Arc<ReferenceOr<Response>>>,
    /// An object to hold reusable Parameter Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<Arc<String>, Arc<ReferenceOr<Parameter>>>,
    /// An object to hold reusable Example Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub examples: IndexMap<Arc<String>, Arc<ReferenceOr<Example>>>,
    /// An object to hold reusable Request Body Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub request_bodies: IndexMap<Arc<String>, Arc<ReferenceOr<RequestBody>>>,
    /// An object to hold reusable Header Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<Arc<String>, Arc<ReferenceOr<Header>>>,
    /// An object to hold reusable Schema Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub schemas: IndexMap<Arc<String>, Arc<ReferenceOr<Schema>>>,
    /// An object to hold reusable Link Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub links: IndexMap<Arc<String>, Arc<ReferenceOr<Link>>>,
    /// An object to hold reusable Callback Objects.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub callbacks: IndexMap<Arc<String>, Arc<ReferenceOr<Callback>>>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "deserialize_extensions")]
    pub extensions: IndexMap<Arc<String>, Arc<serde_json::Value>>,
}

pub struct TagPath {
    path: Arc<String>,
    item: Arc<ReferenceOr<PathItem>>,
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

pub struct ComponentPointer {
    name: Arc<String>,
    comp: Box<dyn Any>,
    phantom_t: String, // Type reflection
}

pub fn split_specs(
    openapi: Arc<OpenAPI>,
    tags_selected: Arc<HashSet<Arc<String>>>,
) -> Result<HashMap<Arc<String>, OpenAPIRef>> {
    let mut ref_list = HashSet::new(); // example of a ref: "#/components/examples/root"
    let mut tag_paths: HashMap<Arc<String>, HashSet<TagPath>> = HashMap::new(); // Maps tags to paths
    let mut tag_refs: HashMap<Arc<String>, HashSet<Arc<String>>> =
        HashMap::new(); // Maps tags to references
    let mut ref_map: HashMap<Arc<String>, ComponentPointer> = HashMap::new(); // Maps references to components

    for tag in tags_selected.iter() {
        tag_paths.insert(tag.clone(), HashSet::new());
        tag_refs.insert(tag.clone(), HashSet::new());
    }

    // 1. Crawl through all the paths
    for (path, item) in &openapi.paths.paths {
        match item {
            // 2. If a path has a reference then:
            ReferenceOr::Item(path_item) => {
                let tags_in_path = get_tags_in_path(Arc::new(*path_item));

                let relevant_tags: HashSet<Arc<String>> = tags_selected
                    .intersection(&tags_in_path)
                    .cloned()
                    .collect();

                if !relevant_tags.is_empty() {
                    let refs = get_refs_in_path(Arc::new(*path_item));
                    for tag in relevant_tags.iter() {
                        // 2.1. Add those references to the dedicated tags
                        let this_tag_refs = tag_refs.get_mut(tag).unwrap();
                        this_tag_refs.extend(refs.clone());

                        // 2.2. Add those paths to the dedicated tags
                        let this_tag_paths = tag_paths.get_mut(tag).unwrap();
                        this_tag_paths.insert(TagPath {
                            path: Arc::new(*path),
                            item: Arc::new(*item),
                        });
                    }

                    ref_list.extend(refs.clone()); // keep track of all the refs for later
                }
            }
            _ => {} // Skip if it's just a reference at the path level, as we want to inspect operations
        }
    }

    // TODO: This should be if let at the beginning
    let components = openapi.components.map(Arc::new).unwrap();

    // 3. Map the components to references
    for ref_name in ref_list.iter() {
        if let Some((ref_t, ref_index)) = parse_ref(ref_name) {
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

    let mut oapis = HashMap::new();

    for tag in tags_selected.iter() {
        let mut paths = PathsRef {
            paths: IndexMap::new(),
            extensions: IndexMap::new(),
        };

        let paths_to_add = tag_paths.get(tag).unwrap();

        for path_to_add in paths_to_add.iter() {
            paths
                .paths
                .insert(path_to_add.path.clone(), path_to_add.item.clone());
        }

        let mut components = ComponentsRef {
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

        let refs_to_add = tag_refs.get(tag).unwrap();

        for ref_to_add in refs_to_add {
            // Get component from RefMap
            let comp = ref_map.get(ref_to_add).unwrap();

            match comp.phantom_t.as_str() {
                "SecurityScheme" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<SecurityScheme>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.security_schemes.insert(comp.name, *actual_comp);
                }
                "Response" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Response>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.responses.insert(comp.name, *actual_comp);
                }
                "Parameter" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Parameter>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.parameters.insert(comp.name, *actual_comp);
                }
                "Example" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Example>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.examples.insert(comp.name, *actual_comp);
                }
                "RequestBody" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<RequestBody>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.request_bodies.insert(comp.name, *actual_comp);
                }
                "Header" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Header>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.headers.insert(comp.name, *actual_comp);
                }
                "Schema" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Schema>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.schemas.insert(comp.name, *actual_comp);
                }
                "Link" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Link>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.links.insert(comp.name, *actual_comp);
                }
                "Callback" => {
                    let actual_comp = comp
                        .comp
                        .downcast::<Arc<ReferenceOr<Callback>>>()
                        .map_err(|e| anyhow!("{:?}", e))?;
                    components.callbacks.insert(comp.name, *actual_comp);
                }
            }
        }

        let oapi = OpenAPIRef {
            openapi: Arc::new(openapi.openapi),
            info: Arc::new(openapi.info),
            servers: Arc::new(openapi.servers),
            paths,
            components: Some(components),
            security: Arc::new(openapi.security),
            tags: Arc::new(openapi.tags), // TODO: wrong. this should only be selected tags
            external_docs: Arc::new(openapi.external_docs),
        };

        oapis.insert(tag.clone(), oapi);
    }

    // 4. Return the structs
    Ok(oapis)
}

fn link_components_to_refs<T>(
    ref_map: &mut HashMap<Arc<String>, ComponentPointer>,
    reff_name: Arc<String>, // #components/<T>/ref_index
    ref_index: &str,
    sub_components: &IndexMap<String, ReferenceOr<T>>,
) {
    let comp = Box::new(Arc::new(*sub_components.get(&*ref_index).unwrap()));

    let comp_pointer = ComponentPointer {
        name: Arc::new(ref_index.to_string()),
        comp,
        phantom_t: String::from(type_name::<T>()),
    };

    ref_map.insert(reff_name, comp_pointer);
}

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

fn get_tags_in_path(path_item: Arc<PathItem>) -> HashSet<Arc<String>> {
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

    let mut all_tags: HashSet<Arc<String>> = HashSet::new();

    // TODO: Crawl through each operation and collect references
    for operation in all_operations.iter() {
        if let Some(operation) = operation {
            operation.tags.iter().for_each(|tag| {
                all_tags.insert(Arc::new(*tag));
            });
        }
    }

    all_tags
}

// Helper function to collect operations by tag and referenced components
fn get_refs_in_path(path_item: Arc<PathItem>) -> HashSet<Arc<String>> {
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

    let mut all_refs: HashSet<Arc<String>> = HashSet::new();

    // TODO: Crawl through each operation and collect references
    for operation in all_operations.iter() {
        if let Some(operation) = operation {
            for param in operation.parameters.iter() {
                match param {
                    ReferenceOr::Reference { reference } => {
                        all_refs.insert(Arc::new(*reference));
                    }
                    _ => {}
                }
            }

            if let Some(request_body) = &operation.request_body {
                match request_body {
                    ReferenceOr::Reference { reference } => {
                        all_refs.insert(Arc::new(*reference));
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

/// Used to deserialize IndexMap<K, V> that are flattened within other structs.
/// This only adds keys that satisfy the given predicate.
pub(crate) struct PredicateVisitor<F, K, V>(pub F, pub PhantomData<(K, V)>);

impl<'de, F, K, V> Visitor<'de> for PredicateVisitor<F, K, V>
where
    F: Fn(&K) -> bool,
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
{
    type Value = IndexMap<K, V>;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("a map whose fields obey a predicate")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut ret = Self::Value::default();

        loop {
            match map.next_key::<K>() {
                Err(_) => (),
                Ok(None) => break,
                Ok(Some(key)) if self.0(&key) => {
                    let _ = ret.insert(key, map.next_value()?);
                }
                Ok(Some(_)) => {
                    let _ = map.next_value::<IgnoredAny>()?;
                }
            }
        }

        Ok(ret)
    }
}

fn deserialize_paths<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, ReferenceOr<PathItem>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(PredicateVisitor(
        |key: &String| key.starts_with('/'),
        PhantomData,
    ))
}

pub(crate) fn deserialize_extensions<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, serde_json::Value>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(PredicateVisitor(
        |key: &String| key.starts_with("x-"),
        PhantomData,
    ))
}
