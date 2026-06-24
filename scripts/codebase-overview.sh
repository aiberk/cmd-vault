#!/bin/bash
# Codebase Overview Generator
# Helps new contributors understand the project structure

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Please run this script from the project root directory (where Cargo.toml is located)"
    echo "💡 Try: cd /path/to/cmd-vault && ./scripts/codebase-overview.sh"
    exit 1
fi

echo "🦀 CMD-VAULT CODEBASE OVERVIEW"
echo "=============================="
echo

echo "📁 PROJECT STRUCTURE:"
echo "===================="
tree src/ -I target 2>/dev/null || find src -type f -name "*.rs" | sort

echo
echo "🎯 MODULE RESPONSIBILITIES:"
echo "=========================="

echo "📱 USER INTERFACE:"
echo "  src/main.rs          - Application entry point"
echo "  src/app.rs           - Core application state management"
echo "  src/ui/              - Terminal UI components"
echo "    ├── mod.rs         - UI module exports"
echo "    ├── render.rs      - Main rendering logic" 
echo "    ├── layout.rs      - UI layout management"
echo "    ├── components.rs  - Reusable UI widgets"
echo "    └── modals.rs      - Modal dialog windows"
echo

echo "⚙️ CORE FUNCTIONALITY:"
echo "  src/handler.rs       - Event handling (keyboard, input)"
echo "  src/cli.rs           - Command-line interface"
echo "  src/storage.rs       - Data persistence"
echo "  src/models.rs        - Data structures"
echo

echo "🔧 UTILITIES & PLATFORM:"
echo "  src/platform.rs      - Cross-platform operations"
echo "  src/utils.rs         - Helper functions"
echo "  src/placeholders.rs  - Variable substitution"
echo "  src/constants.rs     - Application constants"
echo

echo "📊 CURRENT CODE METRICS:"
echo "======================="
./scripts/code-stats.sh | tail -n +5 | head -n 20

echo
echo "🔍 FINDING YOUR WAY AROUND:"
echo "=========================="
echo "New to the codebase? Start here:"
echo "  1. 📖 README.md - Project overview and setup"
echo "  2. 📚 docs/README.md - Complete documentation index"
echo "  3. 🏗️  docs/ARCHITECTURE-SUMMARY.md - High-level architecture"
echo "  4. 🎨 docs/UI-ARCHITECTURE.md - UI component structure"  
echo "  5. 🔧 docs/REFACTORING-ROADMAP.md - Current improvement plans"
echo

echo "🛠️ DEVELOPMENT TOOLS:"
echo "==================="
echo "  ./scripts/code-stats.sh      - Visual line count analysis"
echo "  ./scripts/refactor-check.sh  - Refactoring progress tracker"
echo "  ./scripts/dev-check.sh       - Development environment check"
echo "  ./scripts/codebase-overview.sh - This overview (run anytime!)"
echo

echo "💡 CONTRIBUTION TIPS:"
echo "=================="
echo "  🎯 Focus areas: Files over 500 lines need refactoring"
echo "  📏 Target: Keep new files under 400 lines"
echo "  🧪 Testing: Run 'cargo test' before submitting"
echo "  📋 Format: Run 'cargo fmt' to maintain style"
echo "  🔍 Lint: Run 'cargo clippy' to catch issues"
echo

echo "For detailed contribution guidelines:"
echo "  📘 docs/UI-CONTRIBUTION-GUIDE.md - UI component development"  
echo "  🦀 docs/RUST-DEV-WORKFLOW.md - Rust-specific practices"
echo "  🖥️  docs/PLATFORM-DEV-GUIDE.md - Cross-platform development"