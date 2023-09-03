#!/bin/sh

# Navigate to the wasm-lib directory
cd client/

# Compile the wasm library (assuming you're using wasm-pack)
vsce package --out ../vsix

cd ../
