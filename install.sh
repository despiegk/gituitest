#!/bin/bash
cd "$(dirname "$0")"

echo "=== Gitea UI Clone - Installation ==="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed: $(cargo --version)"
fi

# Build the project
echo ""
echo "Building the project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "=== Installation Complete ==="
    echo ""
    echo "Run './run.sh' to start the server"
else
    echo ""
    echo "Build failed. Please check the errors above."
    exit 1
fi
