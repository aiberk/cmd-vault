#!/bin/bash
# Development Environment Setup
# Helps new contributors get started quickly

echo "🦀 CMD-VAULT DEVELOPMENT SETUP"
echo "==============================="
echo

echo "🔍 Checking development environment..."
echo

# Check Rust installation
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✅ Rust installed: $RUST_VERSION"
else
    echo "❌ Rust not found! Please install from https://rustup.rs/"
    exit 1
fi

# Check Cargo
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo "✅ Cargo installed: $CARGO_VERSION"
else
    echo "❌ Cargo not found!"
    exit 1
fi

echo

# Test build
echo "🔨 Testing build..."
if cargo check --quiet; then
    echo "✅ Project compiles successfully"
else
    echo "❌ Build failed - check for errors above"
    exit 1
fi

# Install useful dev tools if not present
echo
echo "🛠️ Checking development tools..."

# Clippy for linting
if cargo clippy --version &> /dev/null; then
    echo "✅ Clippy available for linting"
else
    echo "⚠️  Installing clippy..."
    rustup component add clippy
fi

# Rustfmt for formatting  
if cargo fmt --version &> /dev/null; then
    echo "✅ Rustfmt available for formatting"
else
    echo "⚠️  Installing rustfmt..."
    rustup component add rustfmt
fi

echo

# Run quick checks
echo "🧪 Running development checks..."

# Check formatting
if cargo fmt -- --check &> /dev/null; then
    echo "✅ Code is properly formatted"
else
    echo "⚠️  Code formatting issues found - run 'cargo fmt' to fix"
fi

# Check linting (allow warnings for now)
if cargo clippy --quiet -- -D warnings &> /dev/null; then
    echo "✅ No clippy warnings"
else
    echo "⚠️  Clippy found issues - run 'cargo clippy' to see details"
fi

echo
echo "📊 Current codebase overview:"
if [ -x "./scripts/code-stats.sh" ]; then
    ./scripts/code-stats.sh | head -n 15
else
    echo "Run './scripts/code-stats.sh' for detailed metrics"
fi

echo
echo "🎉 DEVELOPMENT ENVIRONMENT READY!"
echo "================================="
echo
echo "🚀 Quick Start Commands:"
echo "  cargo run           - Run the application"
echo "  cargo test          - Run tests" 
echo "  cargo fmt           - Format code"
echo "  cargo clippy        - Lint code"
echo "  cargo build --release - Build optimized binary"
echo
echo "📖 Useful Scripts:"
echo "  ./scripts/codebase-overview.sh  - Understand project structure"
echo "  ./scripts/code-stats.sh         - View code metrics"
echo "  ./scripts/refactor-check.sh     - Check refactoring progress"
echo
echo "📚 Documentation:"
echo "  docs/README.md               - Documentation index"
echo "  docs/ARCHITECTURE-SUMMARY.md - Architecture guide"  
echo "  docs/UI-CONTRIBUTION-GUIDE.md - UI development guide"
echo "  docs/REFACTORING-ROADMAP.md  - Current improvement plans"
echo
echo "Happy coding! 🦀✨"