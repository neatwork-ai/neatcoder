# Neatcoder

Turn your IDE into an AI Sofware engineer.

Neatcoder is an IDE extension designed to serve as an AI Pair Programmer. In other words, it's the developers' best friend.

As it stands, Neatcoder has the ability to initialise and scaffold any codebase in a programming language of your choice. Neatcoder also allows developers to index external interfaces such as databases and APIs of any given protocol or framework.

## The structure

There are two parts to the codebase: The frontend of the extension which can be found in the `client` folder and the backend of the extension which can be found in `crates/neatcoder`.

The client is written in Typescript and utilizes directly the VSCode API, whereas the backed is written in Rust WebAssembly.

### WASM Library

The WASM library contains the core types and primitives that manage the state of the application. This state management is mostly done via the struct `AppData`, which serves as a central data structure to maintain the state of the application.

It holds information about the project language, initial specifications (from a prompt), project scaffold structure (as a JSON string), interfaces, and a task pool to manage various tasks or jobs in the application.

### VSCE Extension

The VSCE extension contains all the code that interfaces with the VSCode API and therefore is responsible for guising the User Experience as well to perform the function calls to the WASM library to perform state transitions. Given the access to the VSCode API, the VSCE extension is also responsible for keeping touch with the user's file system as well as other embedded VSCode features such as language servers and compiler frontends.

## Dependencies

To run this codebase you will need the following languages installed:

- Rust
- Nodejs
- Typescript

And the following:

- Wasm Pack: `cargo install wasm-pack`
- Wasm Bindgen CLI: `cargo install wasm-bindgen-cli`
- Wasm Compilation Target: `rustup target add wasm32-unknown-unknown`
- VSCE: `npm install --global vsce`
- Yarn: `npm install --global yarn`

## Running

To run tests:

- Wasm: `./bin/test_wasm.sh`
- VSCE: `./bin/test_client.sh`

To build:

- WASM: `./bin/build_wasm.sh`
- webview: `./bin/build_webview.sh`
- VSCE: `./bin/build_client.sh`
- Full: `./bin/build.sh`
- Cross-platform Full: `python ./bin/cross_build.py`

`npm run build`

## Debugging

Install VS extension:

<https://marketplace.visualstudio.com/items?itemName=amodio.tsl-problem-matcher>

Copy `tasks.json` and `launch.json` from vscode to .vscode folder



## To compile oai crate

As default native:
`cargo build`

As a Wasm Lib:
`cargo build --target wasm32-unknown-unknown --features wasm --no-default-features`
