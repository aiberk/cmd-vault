#!/bin/bash
# Development quality check script

set -e

echo "🦀 Running development quality checks for CmdVault..."

# Format check
echo "📝 Checking code formatting..."
if ! cargo fmt --check; then
    echo "❌ Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi
echo "✅ Code formatting looks good"

# Clippy linting
echo "🦀 Running Clippy linter..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy found issues. Fix them or run 'cargo clippy --fix'."
    exit 1
fi
echo "✅ Clippy checks passed"

# Type checking
echo "🔧 Running type checking..."
if ! cargo check --all-targets --all-features; then
    echo "❌ Type checking failed."
    exit 1
fi
echo "✅ Type checking passed"

# Tests
echo "🧪 Running tests..."
if ! cargo test; then
    echo "❌ Tests failed."
    exit 1
fi
echo "✅ All tests passed"

# Check for outdated dependencies (optional)
if command -v cargo-outdated >/dev/null 2>&1; then
    echo "📦 Checking for outdated dependencies..."
    cargo outdated --exit-code 1 || echo "⚠️  Some dependencies are outdated"
fi

# Security audit (optional)
if command -v cargo-audit >/dev/null 2>&1; then
    echo "🔒 Running security audit..."
    if ! cargo audit; then
        echo "⚠️  Security vulnerabilities found!"
        exit 1
    fi
    echo "✅ No security vulnerabilities found"
fi

echo ""
echo "🎉 All quality checks passed! Ready to commit."