#!/bin/bash
# Fix script for segmentation fault issues with trunk build
# Segfaults usually indicate corrupted Rust installation or toolchain issues

echo "=== Fixing Segmentation Fault Issues ==="
echo ""

# Check current toolchain
echo "1. Checking current Rust toolchain..."
rustup show
echo ""

# Check if rust-toolchain.toml exists and what it specifies
echo "2. Checking rust-toolchain.toml..."
if [ -f "rust-toolchain.toml" ]; then
    echo "   Found rust-toolchain.toml:"
    cat rust-toolchain.toml | sed 's/^/   /'
    if grep -q "channel.*=.*nightly" rust-toolchain.toml; then
        CURRENT_TOOLCHAIN=$(rustup show | grep "active toolchain" | awk '{print $NF}')
        if [[ "$CURRENT_TOOLCHAIN" != *"nightly"* ]]; then
            echo "   ⚠ Toolchain file specifies nightly, but $CURRENT_TOOLCHAIN is active"
            echo "   This mismatch can cause segfaults!"
        fi
    fi
else
    echo "   No rust-toolchain.toml found"
fi
echo ""

# 1. Clean everything
echo "3. Cleaning build artifacts..."
cargo clean 2>/dev/null && echo "   ✓ Cargo clean completed" || echo "   ⚠ cargo clean failed"

# Remove target directory completely
if [ -d "target" ]; then
    echo "   Removing target directory..."
    rm -rf target
    echo "   ✓ Target directory removed"
fi
echo ""

# 2. Clear cargo cache
echo "4. Clearing cargo cache..."
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

# Clear registry cache
if [ -d "$CARGO_HOME/registry/.cache" ]; then
    rm -rf "$CARGO_HOME/registry/.cache"
    echo "   ✓ Cleared registry cache"
fi

# Clear git cache
if [ -d "$CARGO_HOME/git/db" ]; then
    echo "   Clearing git cache..."
    find "$CARGO_HOME/git/db" -mindepth 1 -maxdepth 1 -type d -exec rm -rf {} + 2>/dev/null
    echo "   ✓ Cleared git cache"
fi
echo ""

# 3. Reinstall wasm32 target
echo "5. Reinstalling wasm32-unknown-unknown target..."
rustup target remove wasm32-unknown-unknown 2>/dev/null
sleep 1
rustup target add wasm32-unknown-unknown
if [ $? -eq 0 ]; then
    echo "   ✓ Target reinstalled successfully"
else
    echo "   ✗ Failed to reinstall target"
fi
echo ""

# 4. Update rustup and toolchains
echo "6. Updating rustup and toolchains..."
rustup update
echo ""

# 5. Verify toolchain consistency
echo "7. Verifying toolchain consistency..."
ACTIVE_TOOLCHAIN=$(rustup show | grep "active toolchain" | awk '{print $NF}')
echo "   Active: $ACTIVE_TOOLCHAIN"

if [ -f "rust-toolchain.toml" ]; then
    if grep -q "channel.*=.*nightly" rust-toolchain.toml; then
        if [[ "$ACTIVE_TOOLCHAIN" != *"nightly"* ]]; then
            echo "   ⚠ Mismatch detected: rust-toolchain.toml wants nightly"
            echo "   Setting nightly as default..."
            rustup default nightly
            echo "   ✓ Switched to nightly toolchain"
        fi
    elif grep -q "channel.*=.*stable" rust-toolchain.toml; then
        if [[ "$ACTIVE_TOOLCHAIN" != *"stable"* ]]; then
            echo "   Setting stable as default..."
            rustup default stable
            echo "   ✓ Switched to stable toolchain"
        fi
    fi
else
    echo "   No toolchain file, using default"
fi
echo ""

# 6. Test rustc directly
echo "8. Testing rustc directly..."
if rustc --version >/dev/null 2>&1; then
    RUSTC_VERSION=$(rustc --version)
    echo "   ✓ rustc works: $RUSTC_VERSION"
else
    echo "   ✗ rustc test failed - installation may be corrupted"
    echo "   Consider reinstalling Rust: rustup self uninstall && reinstall"
fi
echo ""

# 7. Check for corrupted components
echo "9. Checking for corrupted components..."
COMPONENTS=$(rustup component list --toolchain stable 2>&1)
if echo "$COMPONENTS" | grep -q "broken\|missing"; then
    echo "   ⚠ Found broken components:"
    echo "$COMPONENTS" | grep -E "broken|missing" | sed 's/^/   /'
    echo "   Reinstalling components..."
    rustup component add rustc cargo --toolchain stable
else
    echo "   ✓ No broken components detected"
fi
echo ""

echo "=== Fix Complete ==="
echo ""
echo "Next steps:"
echo "  1. Try: cargo build --target wasm32-unknown-unknown --verbose"
echo "  2. If segfault persists, try: rustup toolchain install stable --force"
echo "  3. If still failing, reinstall Rust completely:"
echo "     rustup self uninstall"
echo "     # Then reinstall from https://rustup.rs"
echo "  4. Check system logs for crash details"
echo "  5. Add cargo/rustc to antivirus exclusions"

