#!/bin/bash
# Refactoring Progress Monitor
# Tracks progress against refactoring goals

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Please run this script from the project root directory (where Cargo.toml is located)"
    echo "💡 Try: cd /path/to/cmd-vault && ./scripts/refactor-check.sh"
    exit 1
fi

if [ ! -d "src" ]; then
    echo "❌ ERROR: src directory not found. Are you in the cmd-vault project root?"
    exit 1
fi

echo "🎯 REFACTORING PROGRESS CHECK"
echo "============================"
echo

# Define targets (Rust-appropriate)
MAX_LINES=500
TARGET_AVG=250
IDEAL_MAX=400

# Get current stats
temp_file=$(mktemp)
find src -name "*.rs" -exec wc -l {} \; | sort -nr > "$temp_file"

total_files=$(wc -l < "$temp_file")
total_lines=$(awk '{sum += $1} END {print sum}' "$temp_file")
avg_lines=$(( total_lines / total_files ))
max_file_lines=$(head -1 "$temp_file" | awk '{print $1}')

echo "📊 CURRENT METRICS:"
echo "Total files: $total_files"
echo "Total lines: $total_lines"
echo "Average lines/file: $avg_lines"
echo "Largest file: $max_file_lines lines"
echo

# Check against goals
echo "🎯 GOAL PROGRESS:"
echo "================"

# Check average goal
if [ $avg_lines -le $TARGET_AVG ]; then
    echo "✅ Average file size: $avg_lines ≤ $TARGET_AVG (TARGET MET!)"
else
    echo "❌ Average file size: $avg_lines > $TARGET_AVG (need to reduce by $(( avg_lines - TARGET_AVG )))"
fi

# Check no files over 500 lines
over_500=$(awk '$1 > 500' "$temp_file" | wc -l)
if [ $over_500 -eq 0 ]; then
    echo "✅ Files over 500 lines: 0 (GOAL MET!)"
else
    echo "❌ Files over 500 lines: $over_500"
    echo "   Files to refactor:"
    awk '$1 > 500 {printf "   🔧 %s (%d lines)\n", $2, $1}' "$temp_file" | sed 's|src/||g'
fi

# Check ideal goal (most files under 400)
over_400=$(awk '$1 > 400' "$temp_file" | wc -l)
under_400=$(awk '$1 <= 400' "$temp_file" | wc -l)
percent_under_400=$(( under_400 * 100 / total_files ))

if [ $percent_under_400 -ge 80 ]; then
    echo "✅ Files under 400 lines: $percent_under_400% (IDEAL MET!)"
else
    echo "⚠️  Files under 400 lines: $percent_under_400% (ideal: ≥80%)"
fi

echo

# Specific quick wins
echo "🚀 QUICK WINS AVAILABLE:"
echo "======================="

# Check for ui_old.rs
if [ -f "src/ui_old.rs" ]; then
    ui_old_lines=$(grep "src/ui_old.rs" "$temp_file" | awk '{print $1}')
    echo "🗑️  DELETE src/ui_old.rs: -$ui_old_lines lines (unused code)"
else
    echo "✅ ui_old.rs already removed"
fi

# Check for very large files
echo "🔧 LARGE FILES TO SPLIT:"
awk '$1 > 600 {printf "   📦 %s (%d lines) - Split into modules\n", $2, $1}' "$temp_file" | sed 's|src/||g'

echo

# Calculate potential impact
echo "📈 POTENTIAL IMPACT:"
echo "==================="
if [ -f "src/ui_old.rs" ]; then
    ui_old_lines=$(grep "src/ui_old.rs" "$temp_file" | awk '{print $1}')
    new_total=$(( total_lines - ui_old_lines ))
    new_avg=$(( new_total / (total_files - 1) ))
    echo "After removing ui_old.rs:"
    echo "  Total lines: $new_total (saving $ui_old_lines lines)"
    echo "  New average: $new_avg lines/file"
    
    if [ $new_avg -le $TARGET_AVG ]; then
        echo "  🎉 This would meet the average target!"
    fi
fi

echo
echo "💡 NEXT ACTIONS:"
echo "==============="
echo "1. Run './scripts/code-stats.sh' for detailed visualization"
echo "2. Check 'REFACTORING-ROADMAP.md' for detailed plan"
echo "3. Start with quick wins (delete unused files)"
echo "4. Use modular splits for large files"
echo

# Cleanup
rm "$temp_file"