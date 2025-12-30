#!/bin/bash

set -e

echo "🚀 Setting up UAV Red Team development environment..."

# Check Rust installation
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust $(rustc --version) found"

# Check cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed"
    exit 1
fi

echo "✓ Cargo $(cargo --version) found"

# Install additional tools
echo "📦 Installing development tools..."
cargo install cargo-watch 2>/dev/null || echo "cargo-watch already installed"
cargo install cargo-edit 2>/dev/null || echo "cargo-edit already installed"

# Check dependencies
echo "📥 Checking dependencies..."
cargo check --quiet 2>&1 | grep -v "Updating" || true

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  cargo build         # Build the project"
echo "  cargo run           # Run the application"
echo "  cargo test          # Run tests"
echo "  cargo watch -x run  # Auto-reload on changes"
