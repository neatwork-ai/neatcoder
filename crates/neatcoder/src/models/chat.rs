use anyhow::Result;
use js_sys::Function;
use oai::models::chat::wasm::ChatWasm;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasmer::JsError;

use crate::endpoints::get_chat_title::get_chat_title;

#[wasm_bindgen(js_name = setTitle)]
pub async fn set_title(
    chat: &mut ChatWasm,
    request_callback: &Function,
) -> Result<(), JsError> {
    if !chat.messages.is_empty() {
        // Get the first element using indexing (index 0)
        let first_msg = chat.messages[0].clone();
        let title =
            get_chat_title(&first_msg.payload.content, request_callback)
                .await
                .map_err(|e| JsError::from_str(&e.to_string()))?;

        chat.title = title;

        Ok(())
    } else {
        Err(JsError::from(JsValue::from_str(
            "Unable to create title. No messages in the Chat.",
        )))
    }
}
