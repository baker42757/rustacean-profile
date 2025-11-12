#!/bin/bash
# Setup script to ensure rustup's cargo is used instead of standalone Rust installation
# Run this before trunk serve: source setup-env.sh

# Get the rustup home directory (defaults to ~/.rustup)
RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

# Add rustup's cargo to PATH if it exists
if [ -d "$CARGO_HOME/bin" ]; then
    export PATH="$CARGO_HOME/bin:$PATH"
    echo "✓ Updated PATH to use rustup's cargo from $CARGO_HOME/bin"
else
    echo "⚠ Warning: $CARGO_HOME/bin not found. Make sure rustup is installed."
fi

echo "Current cargo: $(which cargo 2>/dev/null || echo 'not found')"
if command -v cargo &> /dev/null; then
    echo "Cargo version: $(cargo --version)"
fi

