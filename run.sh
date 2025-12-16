#!/bin/bash
cd "$(dirname "$0")"

echo "=== Gitea UI Clone - Starting Server ==="
echo ""

# Check if the binary exists
if [ ! -f "target/release/gitea-ui-clone" ]; then
    echo "Binary not found. Running install first..."
    ./install.sh
fi

echo "Starting server..."
echo ""
./target/release/gitea-ui-clone
