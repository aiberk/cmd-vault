#!/bin/bash
# Comprehensive test runner script
set -euo pipefail

echo "🧪 Running comprehensive test suite..."

# Run formatter check
echo "📝 Checking code formatting..."
cargo fmt --all -- --check

# Run clippy
echo "🔍 Running clippy lints..."
cargo clippy --all-targets --all-features -- -D warnings

# Run unit tests
echo "🏃 Running unit tests..."
cargo test --lib --all-features

# Run integration tests
echo "🔗 Running integration tests..."
cargo test --test '*' --all-features

# Run doc tests
echo "📚 Running documentation tests..."
cargo test --doc --all-features

# Run benchmarks (in check mode to avoid long execution)
echo "📊 Checking benchmarks compile..."
cargo bench --no-run

# Security audit
echo "🔒 Running security audit..."
if command -v cargo-audit >/dev/null 2>&1; then
    cargo audit
else
    echo "⚠️  cargo-audit not installed, skipping security audit"
fi

# Check for common issues
echo "🕵️ Running additional checks..."
if command -v cargo-deny >/dev/null 2>&1; then
    cargo deny check
else
    echo "⚠️  cargo-deny not installed, skipping license/dependency checks"
fi

echo "✅ All tests passed!"