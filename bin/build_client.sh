#!/bin/sh

# Navigate to the wasm-lib directory
cd vsce/

# Compile the wasm library (assuming you're using wasm-pack)
vsce package --out ../vsix

cd ../
