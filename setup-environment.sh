#!/bin/bash
# NFL Project Setup Script for Phase 0 & 1
# 
# This script initializes the complete Rust development environment
# for the NFL (Nafal Faturizki Listener) hearing aid project.
# 
# Usage: bash setup-environment.sh

set -e

echo "🦻 NFL - Nafal Faturizki Listener"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Phase 0: Project Initialization & Environment Setup"
echo ""

# Step 1: Check if Rust is installed
echo "✓ Step 1: Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust installed: $(rustc --version)"
fi

# Step 2: Update Rust
echo ""
echo "✓ Step 2: Updating Rust toolchain..."
rustup update stable

# Step 3: Add embedded targets
echo ""
echo "✓ Step 3: Adding bare-metal targets for embedded..."
rustup target add thumbv8m.main-none-eabihf
rustup component add rust-analyzer

# Step 4: Install probe-rs for firmware flashing
echo ""
echo "✓ Step 4: Installing probe-rs (firmware flasher)..."
cargo install probe-rs-tools --locked

# Step 5: Install cargo-flash-erase (optional but useful)
echo ""
echo "✓ Step 5: Installing useful embedded tools..."
cargo install cargo-tree
cargo install cargo-expand
cargo install cargo-generate

# Step 6: Verify workspace structure
echo ""
echo "✓ Step 6: Verifying workspace structure..."
cargo metadata --format-version 1 > /dev/null 2>&1 && echo "✅ Workspace is valid" || echo "❌ Workspace validation failed"

# Step 7: Check workspace members
echo ""
echo "✓ Step 7: Checking workspace members..."
echo ""
cargo tree --depth 0

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Phase 0 Setup Complete!"
echo ""
echo "📝 Next steps for Phase 1:"
echo "  1. Connect nRF5340 DK via USB"
echo "  2. Run: cargo build --release -p nfl-firmware"
echo "  3. Run: cargo run --release -p nfl-firmware"
echo ""
echo "📚 Repository structure (Phase 0):"
echo "  ├── firmware/        → nRF5340 bare-metal Rust"
echo "  ├── mobile/rust/     → Mobile FFI engine"
echo "  ├── hardware/        → PCB & casing design"
echo "  └── docs/            → Documentation"
echo ""
echo "🔗 Documentation: See blueprint_nfl.md for architecture details"
echo ""
