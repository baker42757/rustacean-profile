#!/bin/bash
# Setup script to ensure rustup's toolchain is used and wasm32 target is installed
# Run this before trunk serve: source setup-env.sh

# Ensure stable toolchain is set as default
rustup default stable 2>/dev/null || true

# Ensure rustup's binaries are in PATH (works on Windows, Linux, macOS)
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
if [ -d "$CARGO_HOME/bin" ]; then
    export PATH="$CARGO_HOME/bin:$PATH"
fi

# Get rustup's toolchain bin directory and add to PATH (portable across platforms)
if command -v rustup >/dev/null 2>&1; then
    RUSTC_PATH=$(rustup which --toolchain stable rustc 2>/dev/null || echo "")
    if [ -n "$RUSTC_PATH" ]; then
        # Extract directory using dirname (works on Unix and Git Bash on Windows)
        TOOLCHAIN_BIN=$(dirname "$RUSTC_PATH" 2>/dev/null || echo "")
        if [ -n "$TOOLCHAIN_BIN" ] && [ -d "$TOOLCHAIN_BIN" ]; then
            export PATH="$TOOLCHAIN_BIN:$PATH"
        fi
    fi
fi

# Install wasm32-unknown-unknown target if not already installed
if ! rustup target list --installed 2>/dev/null | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
else
    echo "✓ wasm32-unknown-unknown target already installed"
fi
