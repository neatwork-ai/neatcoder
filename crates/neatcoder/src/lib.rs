use js_sys::Reflect;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::hash::Hash;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;

pub mod conf;
pub mod endpoints;
pub mod models;
pub mod openai;
pub mod prelude;
pub mod utils;

pub type JsError = JsValue;

pub trait WasmType<ExternType: JsCast> {
    type RustType;

    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError>;

    fn from_extern(extern_type: ExternType) -> Result<Self::RustType, JsError>;
}

impl<
        K: DeserializeOwned + Ord + Into<JsValue> + Clone,
        V: Into<JsValue> + Clone,
        ExternType: JsCast,
    > WasmType<ExternType> for BTreeMap<K, V>
where
    V: DeserializeOwned,
{
    type RustType = BTreeMap<K, V>;

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
            console::error_1(&JsValue::from_str(&error_msg));
            JsValue::from_str(&error_msg)
        })
    }
}

impl<V: DeserializeOwned + Into<JsValue> + Clone, ExternType: JsCast>
    WasmType<ExternType> for VecDeque<V>
{
    type RustType = VecDeque<V>;

    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError> {
        // Create a new JavaScript array
        let js_array = js_sys::Array::new();

        // Add elements from the VecDeque to the JavaScript array
        for value in rust_type.iter() {
            js_array.push(&value.clone().into());
        }

        // Attempt to cast the JsValue (Array) to the ExternType
        // We use unchecked_into here for the reasons mentioned in the BTreeMap implementation
        let extern_type = js_array.unchecked_into::<ExternType>();

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
                "Failed to convert {} JsValue to VecDeque<{}>: {:?}",
                type_name,
                std::any::type_name::<V>(),
                e,
            );
            console::error_1(&JsValue::from_str(&error_msg));
            JsValue::from_str(&error_msg)
        })
    }
}

// TODO: dedup implementation of Vec/VecDeque
impl<V: DeserializeOwned + Into<JsValue> + Clone, ExternType: JsCast>
    WasmType<ExternType> for Vec<V>
{
    type RustType = Vec<V>;

    fn to_extern(rust_type: Self::RustType) -> Result<ExternType, JsError> {
        // Create a new JavaScript array
        let js_array = js_sys::Array::new();

        // Add elements from the Vec to the JavaScript array
        for value in rust_type.iter() {
            js_array.push(&value.clone().into());
        }

        // Attempt to cast the JsValue (Array) to the ExternType
        // We use unchecked_into here for the reasons mentioned in the BTreeMap implementation
        let extern_type = js_array.unchecked_into::<ExternType>();

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
                "Failed to convert {} JsValue to Vec<{}>: {:?}",
                type_name,
                std::any::type_name::<V>(),
                e,
            );
            console::error_1(&JsValue::from_str(&error_msg));
            JsValue::from_str(&error_msg)
        })
    }
}

// TODO: dedup implementation of BTreeMap/HashMap
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
            console::error_1(&JsValue::from_str(&error_msg));
            JsValue::from_str(&error_msg)
        })
    }
}
