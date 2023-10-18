#!/bin/sh

# Remove the pkg directory if it exists
if [ -d "client/pkg" ]; then
    rm -rf client/pkg
    echo "Removed existing pkg directory."
fi

# Navigate to the wasm-lib directory
cd crates/neatcoder

# Compile the wasm library (assuming you're using wasm-pack)
wasm-pack build --target nodejs --dev

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed!"
    exit 1
fi

# Call the Python script
python3 ../../bin/create_interface.py

# Move the generated interface file to the desired directory
mv neatcoderInterface.d.ts ../../webview/wasm/

# Move the pkg directory to ../client
mv pkg ../../client/

cd ../../

echo "Compilation successful!"
