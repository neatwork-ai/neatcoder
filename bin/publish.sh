#!/bin/bash

# Get the directory
DIR="$(dirname "$0")"

# Run the first script
bash "$DIR/build_wasm.sh"

# Check if the first script ran successfully
if [ $? -eq 0 ]; then
    echo "Built WASM Library successfully"
else
    echo "WASM Lib failed"
    exit 1
fi

# Run the second script
bash "$DIR/build_webview.sh"

# Check if the second script ran successfully
if [ $? -eq 0 ]; then
    echo "Built VSCE successfully"
else
    echo "VSCE failed"
    exit 1
fi

#!/bin/sh

# Navigate to the wasm-lib directory
cd vsce/

# Compile the wasm library (assuming you're using wasm-pack)
vsce publish

cd ../


# # Check if the second script ran successfully
# if [ $? -eq 0 ]; then
#     echo "Built VSCE successfully"
# else
#     echo "VSCE failed"
#     exit 1
# fi

echo "Publishing complete"
