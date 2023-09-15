use js_sys::Reflect;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hash::Hash;
use utils::log_err;
use wasm_bindgen::{JsCast, JsValue};

// Public modules declaration
pub mod consts;
///< Constants used throughout the application.
pub mod endpoints;
///< internal API endpoint definitions.
pub mod models;
///< Data models used in the application.
pub mod openai;
///< Client for interacting with the OpenAI API.
pub mod prelude;
///< Re-exports commonly used items.
pub mod utils;
///< Contains utility functions and helpers.

/// Type alias for JavaScript errors represented as JsValue
pub type JsError = JsValue;

/// Trait to represent a type conversion between Rust and JavaScript types.
///
/// This trait provides methods to convert between a Rust type and a corresponding
/// JavaScript type which can be cast from JsValue.
pub trait WasmType<ExternType: JsCast> {
    /// The type used in Rust representation
    type RustType;

    /// Converts a Rust type into an external (JavaScript) type
    ///
    /// # Parameters
    /// - `rust_type`: The Rust type to be converted.
    ///
    /// # Returns
    /// - A Result containing the JavaScript type or an error.
    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError>;

    /// Converts an external (JavaScript) type into a Rust type
    ///
    /// # Parameters
    /// - `extern_type`: The JavaScript type to be converted.
    ///
    /// # Returns
    /// - A Result containing the Rust type or an error.
    fn from_extern(extern_type: ExternType) -> Result<Self::RustType, JsError>;
}

/// Implementation of WasmType for BTreeMap with methods for converting between Rust and JavaScript types.
impl<
        K: DeserializeOwned + Ord + Into<JsValue> + Clone,
        V: Into<JsValue> + Clone,
        ExternType: JsCast,
    > WasmType<ExternType> for BTreeMap<K, V>
where
    V: DeserializeOwned,
{
    type RustType = BTreeMap<K, V>;

    // Method to convert a BTreeMap to a JavaScript object representation
    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError> {
        // Create a new JavaScript object
        let js_object = js_sys::Object::new();

        // Set properties on the JavaScript object from the BTreeMap
        for (key, value) in rust_type.iter() {
            Reflect::set(
                &js_object,
                &key.clone().into(),
                &value.clone().into(),
            )
            .map_err(|e| {
                JsValue::from_str(&format!(
                    "Failed to set property on JsValue: {:?}",
                    e
                ))
            })?;
        }

        // Try to cast the JsValue to ExternType
        // let ischemas: ExternType = js_object.dyn_into().map_err(|e| {
        //     JsValue::from_str(&format!(
        //         "Failed to cast Object `{:?}` to ExternType. Error: {:?}",
        //         js_object, e
        //     ))
        // })?;

        // Unchecked here can potentially lead to problems down the line.
        // However somehow `js_object.dyn_into()` messes up the casting..
        let extern_type = js_object.unchecked_into::<ExternType>();

        Ok(extern_type)
    }

    // Method to convert a JavaScript object representation to a BTreeMap
    fn from_extern(extern_type: ExternType) -> Result<Self::RustType, JsError> {
        let type_name = std::any::type_name::<ExternType>();

        let js_value = extern_type.dyn_into::<JsValue>().map_err(|_| {
            JsValue::from_str(&format!(
                "Failed to convert {} to JsValue",
                type_name
            ))
        })?;

        serde_wasm_bindgen::from_value(js_value).map_err(|e| {
            let error_msg = format!(
                "Failed to convert {} JsValue to BTreeMap<String, {}>: {:?}",
                type_name,
                std::any::type_name::<V>(),
                e,
            );
            log_err(&error_msg);
            JsValue::from_str(&error_msg)
        })
    }
}

/// Implementation of WasmType for VecDeque with methods for converting between Rust and JavaScript types.
impl<
        V: DeserializeOwned + Serialize + Into<JsValue> + Clone,
        ExternType: JsCast,
    > WasmType<ExternType> for VecDeque<V>
{
    type RustType = VecDeque<V>;

    // Method to convert a VecDeque to a JavaScript object representation
    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError> {
        // Create a new JavaScript array
        let js_array = js_sys::Array::new();

        // Add elements from the VecDeque to the JavaScript array
        // for value in rust_type.iter() {
        //     js_array.push(&value.clone().into());
        // }

        // Add elements from the VecDeque to the JavaScript object as array
        for (index, value) in rust_type.iter().enumerate() {
            let js_index = to_value(&(index as u32)).map_err(|e| {
                JsError::from(JsValue::from_str(&format!(
                    "Failed to convert index to JsValue: {:?}",
                    e
                )))
            })?;

            let js_value = to_value(&value.clone()).map_err(|e| {
                JsError::from(JsValue::from_str(&format!(
                    "Failed to convert value to JsValue: {:?}",
                    e
                )))
            })?;

            Reflect::set(&js_array, &js_index, &js_value).map_err(|e| {
                JsValue::from_str(&format!(
                    "Failed to set property on JsValue: {:?}",
                    e
                ))
            })?;
        }

        // Attempt to cast the JsValue (Array) to the ExternType
        // We use unchecked_into here for the reasons mentioned in the BTreeMap implementation
        let extern_type = js_array.unchecked_into::<ExternType>();

        Ok(extern_type)
    }

    // Method to convert a JavaScript object representation to a VecDeque
    fn from_extern(extern_type: ExternType) -> Result<Self::RustType, JsError> {
        let type_name = std::any::type_name::<ExternType>();

        let js_value = extern_type.dyn_into::<JsValue>().map_err(|_| {
            JsValue::from_str(&format!(
                "Failed to convert {} to JsValue",
                type_name
            ))
        })?;

        serde_wasm_bindgen::from_value(js_value).map_err(|e| {
            let error_msg = format!(
                "Failed to convert {} JsValue to VecDeque<{}>: {:?}",
                type_name,
                std::any::type_name::<V>(),
                e,
            );
            log_err(&error_msg);
            JsValue::from_str(&error_msg)
        })
    }
}

// TODO: dedup implementation of Vec/VecDeque
/// Implementation of WasmType for Vec with methods for converting between Rust and JavaScript types.
impl<
        V: DeserializeOwned + Serialize + Into<JsValue> + Clone,
        ExternType: JsCast,
    > WasmType<ExternType> for Vec<V>
{
    type RustType = Vec<V>;

    // Add elements from the Vec to the JavaScript object as array
    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError> {
        // Create a new JavaScript array
        let js_array = js_sys::Array::new();

        // Add elements from the Vec to the JavaScript array
        // for value in rust_type.iter() {
        //     js_array.push(&value.clone().into());
        // }

        // Add elements from the Vec to the JavaScript object as array
        for (index, value) in rust_type.iter().enumerate() {
            let js_index = to_value(&(index as u32)).map_err(|e| {
                JsError::from(JsValue::from_str(&format!(
                    "Failed to convert index to JsValue: {:?}",
                    e
                )))
            })?;

            let js_value = to_value(&value.clone()).map_err(|e| {
                JsError::from(JsValue::from_str(&format!(
                    "Failed to convert value to JsValue: {:?}",
                    e
                )))
            })?;

            Reflect::set(&js_array, &js_index, &js_value).map_err(|e| {
                JsValue::from_str(&format!(
                    "Failed to set property on JsValue: {:?}",
                    e
                ))
            })?;
        }

        // Attempt to cast the JsValue (Array) to the ExternType
        // We use unchecked_into here for the reasons mentioned in the BTreeMap implementation
        let extern_type = js_array.unchecked_into::<ExternType>();

        Ok(extern_type)
    }

    // Method to convert a JavaScript object representation to a VecDeque
    fn from_extern(extern_type: ExternType) -> Result<Self::RustType, JsError> {
        let type_name = std::any::type_name::<ExternType>();

        let js_value = extern_type.dyn_into::<JsValue>().map_err(|_| {
            JsValue::from_str(&format!(
                "Failed to convert {} to JsValue",
                type_name
            ))
        })?;

        serde_wasm_bindgen::from_value(js_value).map_err(|e| {
            let error_msg = format!(
                "Failed to convert {} JsValue to Vec<{}>: {:?}",
                type_name,
                std::any::type_name::<V>(),
                e,
            );
            log_err(&error_msg);
            JsValue::from_str(&error_msg)
        })
    }
}

// TODO: dedup implementation of BTreeMap/HashMap
/// Implementation of WasmType for BTreeMap with methods for converting between Rust and JavaScript types.
impl<
        K: DeserializeOwned + Ord + Into<JsValue> + Clone + Hash,
        V: Into<JsValue> + Clone,
        ExternType: JsCast,
    > WasmType<ExternType> for HashMap<K, V>
where
    V: DeserializeOwned,
{
    type RustType = HashMap<K, V>;

    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError> {
        // Create a new JavaScript object
        let js_object = js_sys::Object::new();

        // Set properties on the JavaScript object from the HashMap
        for (key, value) in rust_type.iter() {
            Reflect::set(
                &js_object,
                &key.clone().into(),
                &value.clone().into(),
            )
            .map_err(|e| {
                JsValue::from_str(&format!(
                    "Failed to set property on JsValue: {:?}",
                    e
                ))
            })?;
        }

        // Try to cast the JsValue to ExternType
        // let ischemas: ExternType = js_object.dyn_into().map_err(|e| {
        //     JsValue::from_str(&format!(
        //         "Failed to cast Object `{:?}` to ExternType. Error: {:?}",
        //         js_object, e
        //     ))
        // })?;

        // Unchecked here can potentially lead to problems down the line.
        // However somehow `js_object.dyn_into()` messes up the casting..
        let extern_type = js_object.unchecked_into::<ExternType>();

        Ok(extern_type)
    }

    fn from_extern(extern_type: ExternType) -> Result<Self::RustType, JsError> {
        let type_name = std::any::type_name::<ExternType>();

        let js_value = extern_type.dyn_into::<JsValue>().map_err(|_| {
            JsValue::from_str(&format!(
                "Failed to convert {} to JsValue",
                type_name
            ))
        })?;

        serde_wasm_bindgen::from_value(js_value).map_err(|e| {
            let error_msg = format!(
                "Failed to convert {} JsValue to HashMap<String, {}>: {:?}",
                type_name,
                std::any::type_name::<V>(),
                e,
            );
            log_err(&error_msg);
            JsValue::from_str(&error_msg)
        })
    }
}
