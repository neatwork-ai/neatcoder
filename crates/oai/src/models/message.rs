use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptMessage {
    pub role: Role,
    pub content: String,
}

impl GptMessage {
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: String::from(content),
        }
    }

    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: String::from(content),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: String::from(content),
        }
    }
}

// ===== WASM =====

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::GptMessage;
    use crate::models::role::Role;
    use js_sys::JsString;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::ops::{Deref, DerefMut};
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub struct GptMessageWasm(pub(crate) GptMessage);

    #[wasm_bindgen]
    impl GptMessageWasm {
        #[wasm_bindgen(constructor)]
        pub fn new(role: Role, content: String) -> Self {
            Self(GptMessage { role, content })
        }

        #[wasm_bindgen(getter)]
        pub fn role(&self) -> JsString {
            self.role.as_str().to_string().into()
        }

        #[wasm_bindgen(getter)]
        pub fn content(&self) -> JsString {
            self.content.clone().into()
        }
    }

    impl AsRef<GptMessage> for GptMessageWasm {
        fn as_ref(&self) -> &GptMessage {
            &self.0
        }
    }

    impl Deref for GptMessageWasm {
        type Target = GptMessage;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for GptMessageWasm {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // Implement Serialize for MessageWasm by delegating to Message.
    impl Serialize for GptMessageWasm {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize the inner Message.
            self.0.serialize(serializer)
        }
    }

    // Implement Deserialize for MessageWasm by delegating to Message.
    impl<'de> Deserialize<'de> for GptMessageWasm {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Deserialize as Message and wrap inside MessageWasm.
            GptMessage::deserialize(deserializer).map(GptMessageWasm)
        }
    }
}
