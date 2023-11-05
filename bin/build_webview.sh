#!/bin/sh

# Remove the pkg directory if it exists
if [ -d "webview/build" ]; then
    rm -rf webview/build
    echo "Removed existing build directory."
fi

# Navigate to the wasm-lib directory
cd webview/

# Compile the wasm library (assuming you're using wasm-pack)
npm run build

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed!"
    exit 1
fi

# Move the pkg directory to ../vsce
mv build ../vsce/webview/

cd ../

echo "Compilation successful!"
