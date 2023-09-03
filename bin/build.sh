#!/bin/sh

# Navigate to the wasm-lib directory
cd crates/code-builder

# Compile the wasm library (assuming you're using wasm-pack)
wasm-pack build --target nodejs

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed!"
    exit 1
fi

# Move the pkg directory to ../client
mv pkg ../client/

echo "Compilation successful!"
