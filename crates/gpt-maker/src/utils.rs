use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use serde_json::Value;

pub fn resolve_references(current: &mut Value, root: &Value) {
    if let Some(obj) = current.as_object_mut() {
        // If we find a reference at this level
        if let Some(ref_value) = obj.get("$ref").and_then(Value::as_str) {
            println!("Found reference");
            if let Some(new_value) = resolve_ref_path(ref_value, root) {
                println!("Was able to replace reference ");
                // Replace the entire object containing "$ref" with the resolved value.
                *current = new_value;
                // Recursively resolve references in the newly inserted value.
                resolve_references(current, root);
            } else {
                println!("Unable to replace reference....");
            }
        } else {
            println!("Iterating over object");
            // Else we iterate through the object
            // Recursively resolve references in the fields of this object.
            for value in obj.values_mut() {
                resolve_references(value, root);
            }
        }
    } else if let Some(array) = current.as_array_mut() {
        println!("Iterating over array");
        // Recursively resolve references in each item of the array.
        for item in array {
            resolve_references(item, root);
        }
    }
}

fn resolve_ref_path(ref_path: &str, root: &Value) -> Option<Value> {
    // Ensure the ref_path starts with '#/' which denotes a JSON Pointer.
    println!("Said path is: {:?}", ref_path);

    if !ref_path.starts_with("#/") {
        return None;
    }

    let mut current = root;
    // Skip the first 2 characters ('#/components/') to get the actual path.

    for part in ref_path[13..].split('/') {
        println!("PART! {:?}", part);
        // Attempt to dereference both objects and arrays.
        current = match current {
            Value::Object(map) => map.get(part)?,
            Value::Array(array) => {
                let index: usize = part.parse().ok()?;
                array.get(index)?
            }
            _ => return None,
        };
    }
    // We found a resolved value; return it.
    Some(current.clone())
}

pub fn get_dialoguer_theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_style: Style::new(),
        checked_item_prefix: style("✔".to_string()).green().force_styling(true),
        unchecked_item_prefix: style("✔".to_string())
            .black()
            .force_styling(true),
        ..Default::default()
    }
}

pub fn select<'a>(
    theme: &ColorfulTheme,
    prompt: &str,
    options: &[&'a str],
) -> anyhow::Result<&'a str> {
    let index = Select::with_theme(theme)
        .with_prompt(prompt)
        .items(options)
        .interact()
        .unwrap();

    Ok(options[index])
}

pub fn multi_select(
    theme: &ColorfulTheme,
    prompt: &str,
    option_fields: &Vec<&str>,
) -> anyhow::Result<Vec<String>> {
    let indexes = MultiSelect::with_theme(theme)
        .with_prompt(prompt)
        .items(option_fields)
        .interact()
        .unwrap();

    let borrowed = indexes
        .iter()
        .map(|i| option_fields[*i].to_string())
        .collect::<Vec<_>>();

    Ok(borrowed)
}
