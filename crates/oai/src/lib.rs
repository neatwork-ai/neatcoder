pub mod consts;
///< Client for interacting with the OpenAI API.
pub mod models;
pub mod utils;

#[cfg(feature = "wasm")]
pub mod foreign {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "Record<string, ModelDataWasm>")]
        pub type IModelsData;

        #[wasm_bindgen(typescript_type = "Array<MessageDataWasm>")]
        pub type IMessagesData;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "Array<GptMessage>")]
        pub type IGptMessage;
    }
}
