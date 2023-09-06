use wasm_bindgen::JsValue;

pub mod conf;
pub mod endpoints;
pub mod models;
pub mod openai;
pub mod prelude;
pub mod utils;

pub type JsError = JsValue;
