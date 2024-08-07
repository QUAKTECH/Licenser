#!/bin/bash

echo "Building project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

if [ ! -f target/release/licenser ]; then
    echo "Error: target/release/licenser not found. Exiting."
    exit 1
fi

TEMPLATE_TARGET_PATH="/home/$USER/.local/share/LICENSER/"

mkdir -p "$TEMPLATE_TARGET_PATH"
mv Templates/ "$TEMPLATE_TARGET_PATH"

echo "Copying licenser to /usr/bin..."
sudo cp target/release/licenser /usr/bin/
if [ $? -ne 0 ]; then
    echo "Failed to copy licenser to /usr/bin/. Exiting."
    exit 1
fi

echo "licenser has been installed.🔥"   