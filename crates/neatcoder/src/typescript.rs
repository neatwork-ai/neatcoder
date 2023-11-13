///< Provides foreign typescript types
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Record<string, Interface>")]
    pub type IInterfaces;

    #[wasm_bindgen(typescript_type = "Record<string, string>")]
    pub type ICodebase;

    #[wasm_bindgen(typescript_type = "Array<Task>")]
    pub type ITasksVec;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Record<string, string>")]
    pub type ISchemas;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Record<number, Task>")]
    pub type ITasks;

    #[wasm_bindgen(typescript_type = "Array<number>")]
    pub type IOrder;
}
