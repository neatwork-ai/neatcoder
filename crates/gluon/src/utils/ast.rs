// // This enum represents the different types of instructions in your language.
// // enum Instruction {
// //     Initialization(InitSet),
// //     Input(InputSet),
// //     Control(ControlSet),
// //     // ...other instructions go here...
// // }

// enum InitSet {
//     OpenAI(InitOpenAIData),
//     HuggingFace(InitHuggingFaceData),
//     Custom(Box<dyn InitData>),
// }

// enum InputSet {
//     OpenAI(InitOpenAIData),
//     HuggingFace(InitHuggingFaceData),
//     Custom(Box<dyn InputData>),
// }

// pub trait InitData {
//     fn execute(&self);
//     // Add other required methods here
// }

// pub trait InputData {
//     fn execute(&self);
//     // Add other required methods here
// }

// // These structs hold the data for each instruction.
// struct InitOpenAIData {
//     model: String,
//     temperature: f32,
//     max_tokens: u32,
//     // ...other initialization parameters...
// }

// // These structs hold the data for each instruction.
// struct InitHuggingFaceData {
//     model: String,
//     temperature: f32,
//     max_tokens: u32,
//     // ...other initialization parameters...
// }

// struct InputData {
//     prompt: String,
//     // ...other input parameters...
// }

// struct ControlData {
//     command: String,
//     // ...other control parameters...
// }

// // You would then build your AST as a vector of instructions.
// struct AST {
//     instructions: Vec<Instruction>,
// }
