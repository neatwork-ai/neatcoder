#!/bin/sh

# Navigate to the wasm-lib directory
cd crates/neatcoder

# Run tests
wasm-pack test --node

# Go back to workspace directory
cd ../..
