#!/bin/bash
# CI dependency installation script
set -euo pipefail

echo "🔧 Installing CI dependencies..."

# Install Rust toolchain components
rustup component add rustfmt clippy

# Install cargo tools
cargo install --locked cargo-audit || true
cargo install --locked cargo-llvm-cov || true
cargo install --locked cargo-deny || true

# Platform-specific dependencies
case "$OSTYPE" in
    linux*)
        echo "📦 Installing Linux dependencies..."
        sudo apt-get update
        sudo apt-get install -y pkg-config libssl-dev
        ;;
    darwin*)
        echo "📦 Installing macOS dependencies..."
        # macOS usually has everything needed
        ;;
    msys*|cygwin*)
        echo "📦 Installing Windows dependencies..."
        # Windows-specific setup if needed
        ;;
esac

echo "✅ Dependencies installed successfully"