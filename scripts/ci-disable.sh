#!/bin/bash
# Disable CI/CD workflows  
set -euo pipefail

echo "🛑 Disabling CI/CD workflows..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Please run this script from the project root directory"
    exit 1
fi

# Disable CI workflow
if [ -f ".github/workflows/ci.yml" ]; then
    mv .github/workflows/ci.yml .github/workflows/ci.yml.disabled
    echo "✅ Disabled CI workflow (ci.yml → ci.yml.disabled)"
else
    echo "⚠️  CI workflow already disabled or not found"
fi

# Disable release workflow
if [ -f ".github/workflows/release.yml" ]; then
    mv .github/workflows/release.yml .github/workflows/release.yml.disabled
    echo "✅ Disabled release workflow (release.yml → release.yml.disabled)"
else
    echo "⚠️  Release workflow already disabled or not found"
fi

echo
echo "🔒 CI/CD workflows are now DISABLED!"
echo
echo "📋 What this means:"
echo "   • Pushes to GitHub won't trigger any builds"
echo "   • No automatic testing or releases"
echo "   • All CI/CD code preserved for future use"
echo
echo "💡 To enable again: ./scripts/ci-enable.sh"