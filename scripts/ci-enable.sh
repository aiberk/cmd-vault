#!/bin/bash
# Enable CI/CD workflows
set -euo pipefail

echo "🚀 Enabling CI/CD workflows..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Please run this script from the project root directory"
    exit 1
fi

# Enable CI workflow
if [ -f ".github/workflows/ci.yml.disabled" ]; then
    mv .github/workflows/ci.yml.disabled .github/workflows/ci.yml
    echo "✅ Enabled CI workflow (ci.yml)"
else
    echo "⚠️  CI workflow already enabled or not found"
fi

# Enable release workflow
if [ -f ".github/workflows/release.yml.disabled" ]; then
    mv .github/workflows/release.yml.disabled .github/workflows/release.yml
    echo "✅ Enabled release workflow (release.yml)"
else
    echo "⚠️  Release workflow already enabled or not found"
fi

echo
echo "🎉 CI/CD workflows are now ENABLED!"
echo
echo "📋 What this means:"
echo "   • Pushes to 'main' or 'develop' will trigger CI"
echo "   • Git tags like 'v1.0.0' will trigger releases"
echo "   • Pull requests will run full test suite"
echo
echo "💡 To disable again: ./scripts/ci-disable.sh"