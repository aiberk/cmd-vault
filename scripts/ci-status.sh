#!/bin/bash
# Check CI/CD workflow status
set -euo pipefail

echo "📊 CI/CD WORKFLOW STATUS"
echo "======================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Please run this script from the project root directory"
    exit 1
fi

echo
echo "🔍 Current Status:"

# Check CI workflow
if [ -f ".github/workflows/ci.yml" ]; then
    echo "   🟢 CI Workflow: ENABLED"
    echo "      • Triggers on pushes to main/develop"
    echo "      • Runs tests, security scans, builds"
elif [ -f ".github/workflows/ci.yml.disabled" ]; then
    echo "   🔴 CI Workflow: DISABLED"
    echo "      • File: .github/workflows/ci.yml.disabled"
else
    echo "   ❓ CI Workflow: NOT FOUND"
fi

# Check release workflow
if [ -f ".github/workflows/release.yml" ]; then
    echo "   🟢 Release Workflow: ENABLED"
    echo "      • Triggers on version tags (v1.0.0)"
    echo "      • Creates releases and publishes binaries"
elif [ -f ".github/workflows/release.yml.disabled" ]; then
    echo "   🔴 Release Workflow: DISABLED"
    echo "      • File: .github/workflows/release.yml.disabled"
else
    echo "   ❓ Release Workflow: NOT FOUND"
fi

echo
echo "🛠️ Available Commands:"
echo "   ./scripts/ci-enable.sh    # Enable CI/CD workflows"
echo "   ./scripts/ci-disable.sh   # Disable CI/CD workflows"
echo "   ./scripts/ci-status.sh    # Check current status (this script)"

echo
echo "📋 What Each Status Means:"
echo
echo "🟢 ENABLED:"
echo "   • GitHub will run workflows on pushes/tags"
echo "   • Automatic testing and building"
echo "   • Uses GitHub Actions minutes"
echo
echo "🔴 DISABLED:"
echo "   • GitHub ignores workflow files"
echo "   • No automatic testing or building"
echo "   • Zero GitHub Actions usage"
echo "   • All code preserved for future use"

# Check if any workflows are currently running (if we can detect)
if command -v gh >/dev/null 2>&1; then
    echo
    echo "🏃 Recent Workflow Runs (via GitHub CLI):"
    gh run list --limit 3 2>/dev/null || echo "   No recent runs or GitHub CLI not authenticated"
fi