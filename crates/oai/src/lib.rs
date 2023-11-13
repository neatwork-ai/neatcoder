///< Client for interacting with the OpenAI API.
pub mod models;
pub mod utils;

#[cfg(feature = "wasm")]
pub mod foreign {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "Record<string, Model>")]
        pub type IModels;

        #[wasm_bindgen(typescript_type = "Array<GptMessage>")]
        pub type IMessages;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(typescript_type = "Array<GptMessage>")]
        pub type IGptMessage;
    }
}
