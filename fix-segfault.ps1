# Fix script for segmentation fault issues with trunk build
# Segfaults usually indicate corrupted Rust installation or toolchain issues

Write-Host "=== Fixing Segmentation Fault Issues ===" -ForegroundColor Cyan
Write-Host ""

# Check current toolchain
Write-Host "1. Checking current Rust toolchain..." -ForegroundColor Yellow
rustup show
Write-Host ""

# Check if rust-toolchain.toml exists and what it specifies
Write-Host "2. Checking rust-toolchain.toml..." -ForegroundColor Yellow
if (Test-Path "rust-toolchain.toml") {
    Write-Host "   Found rust-toolchain.toml:" -ForegroundColor Gray
    Get-Content "rust-toolchain.toml" | ForEach-Object { Write-Host "   $_" -ForegroundColor Gray }
    $toolchainContent = Get-Content "rust-toolchain.toml" -Raw
    if ($toolchainContent -match "channel\s*=\s*[\""']?nightly") {
        Write-Host "   ⚠ Toolchain file specifies nightly, but stable is active" -ForegroundColor Yellow
        Write-Host "   This mismatch can cause segfaults!" -ForegroundColor Red
    }
} else {
    Write-Host "   No rust-toolchain.toml found" -ForegroundColor Gray
}
Write-Host ""

# 1. Clean everything
Write-Host "3. Cleaning build artifacts..." -ForegroundColor Yellow
try {
    cargo clean 2>&1 | Out-Null
    Write-Host "   ✓ Cargo clean completed" -ForegroundColor Green
} catch {
    Write-Host "   ⚠ cargo clean failed" -ForegroundColor Yellow
}

# Remove target directory completely
if (Test-Path "target") {
    Write-Host "   Removing target directory..." -ForegroundColor Gray
    Remove-Item -Path "target" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "   ✓ Target directory removed" -ForegroundColor Green
}
Write-Host ""

# 2. Clear cargo cache
Write-Host "4. Clearing cargo cache..." -ForegroundColor Yellow
$cargoHome = $env:CARGO_HOME
if (-not $cargoHome) {
    $cargoHome = "$env:USERPROFILE\.cargo"
}

# Clear registry cache
if (Test-Path "$cargoHome\registry\.cache") {
    Remove-Item -Path "$cargoHome\registry\.cache" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "   ✓ Cleared registry cache" -ForegroundColor Green
}

# Clear git cache
if (Test-Path "$cargoHome\git\db") {
    Write-Host "   Clearing git cache..." -ForegroundColor Gray
    Get-ChildItem "$cargoHome\git\db" -Directory | ForEach-Object {
        Remove-Item $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
    }
    Write-Host "   ✓ Cleared git cache" -ForegroundColor Green
}
Write-Host ""

# 3. Reinstall wasm32 target
Write-Host "5. Reinstalling wasm32-unknown-unknown target..." -ForegroundColor Yellow
rustup target remove wasm32-unknown-unknown 2>&1 | Out-Null
Start-Sleep -Seconds 1
rustup target add wasm32-unknown-unknown
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✓ Target reinstalled successfully" -ForegroundColor Green
} else {
    Write-Host "   ✗ Failed to reinstall target" -ForegroundColor Red
}
Write-Host ""

# 4. Update rustup and toolchains
Write-Host "6. Updating rustup and toolchains..." -ForegroundColor Yellow
rustup update
Write-Host ""

# 5. Verify toolchain consistency
Write-Host "7. Verifying toolchain consistency..." -ForegroundColor Yellow
$activeToolchain = (rustup show | Select-String "active toolchain" -Context 0,2 | Select-Object -Last 1).Line
Write-Host "   Active: $activeToolchain" -ForegroundColor Gray

if (Test-Path "rust-toolchain.toml") {
    $toolchainContent = Get-Content "rust-toolchain.toml" -Raw
    if ($toolchainContent -match "channel\s*=\s*[\""']?nightly") {
        Write-Host "   ⚠ Mismatch detected: rust-toolchain.toml wants nightly" -ForegroundColor Yellow
        Write-Host "   Setting nightly as default..." -ForegroundColor Yellow
        rustup default nightly
        Write-Host "   ✓ Switched to nightly toolchain" -ForegroundColor Green
    } elseif ($toolchainContent -match "channel\s*=\s*[\""']?stable") {
        Write-Host "   Setting stable as default..." -ForegroundColor Yellow
        rustup default stable
        Write-Host "   ✓ Switched to stable toolchain" -ForegroundColor Green
    }
} else {
    Write-Host "   No toolchain file, using default (stable)" -ForegroundColor Gray
}
Write-Host ""

# 6. Test rustc directly
Write-Host "8. Testing rustc directly..." -ForegroundColor Yellow
try {
    $rustcTest = rustc --version 2>&1
    Write-Host "   ✓ rustc works: $rustcTest" -ForegroundColor Green
} catch {
    Write-Host "   ✗ rustc test failed - installation may be corrupted" -ForegroundColor Red
    Write-Host "   Consider reinstalling Rust: rustup self uninstall && reinstall" -ForegroundColor Yellow
}
Write-Host ""

# 7. Check for corrupted components
Write-Host "9. Checking for corrupted components..." -ForegroundColor Yellow
$components = rustup component list --toolchain stable 2>&1
$broken = $components | Select-String "broken|missing"
if ($broken) {
    Write-Host "   ⚠ Found broken components:" -ForegroundColor Yellow
    $broken | ForEach-Object { Write-Host "   $_" -ForegroundColor Red }
    Write-Host "   Reinstalling components..." -ForegroundColor Yellow
    rustup component add rustc cargo --toolchain stable
} else {
    Write-Host "   ✓ No broken components detected" -ForegroundColor Green
}
Write-Host ""

Write-Host "=== Fix Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Try: cargo build --target wasm32-unknown-unknown --verbose" -ForegroundColor White
Write-Host "  2. If segfault persists, try: rustup toolchain install stable --force" -ForegroundColor White
Write-Host "  3. If still failing, reinstall Rust completely:" -ForegroundColor White
Write-Host "     rustup self uninstall" -ForegroundColor Gray
Write-Host "     # Then reinstall from https://rustup.rs" -ForegroundColor Gray
Write-Host "  4. Check Windows Event Viewer for crash details" -ForegroundColor White
Write-Host "  5. Add cargo/rustc to antivirus exclusions" -ForegroundColor White

