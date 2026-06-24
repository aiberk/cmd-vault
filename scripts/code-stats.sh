#!/bin/bash
# Code Statistics and Visualization Script for cmd-vault
# Shows line counts for all Rust source files with a visual bar graph

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Please run this script from the project root directory (where Cargo.toml is located)"
    echo "💡 Try: cd /path/to/cmd-vault && ./scripts/code-stats.sh"
    exit 1
fi

if [ ! -d "src" ]; then
    echo "❌ ERROR: src directory not found. Are you in the cmd-vault project root?"
    exit 1
fi

echo "🦀 CMD-VAULT CODE STATISTICS 🦀"
echo "================================"
echo

# Get line counts and sort them
echo "📊 LINE COUNT ANALYSIS:"
echo

# Create temporary file for processing
temp_file=$(mktemp)
find src -name "*.rs" -exec wc -l {} \; | sort -nr > "$temp_file"

# Find the maximum line count for scaling
max_lines=$(head -1 "$temp_file" | awk '{print $1}')

# Function to create a bar visualization
create_bar() {
    local count=$1
    local max=$2
    local width=50
    local filled=$(( count * width / max ))
    
    printf "["
    for ((i=1; i<=width; i++)); do
        if [ $i -le $filled ]; then
            printf "█"
        else
            printf "░"
        fi
    done
    printf "]"
}

# Display results with bars
while IFS= read -r line; do
    lines=$(echo "$line" | awk '{print $1}')
    file=$(echo "$line" | awk '{print $2}' | sed 's|src/||')
    
    # Color coding based on file size (Rust-appropriate thresholds)
    if [ "$lines" -gt 600 ]; then
        color="\033[31m"  # Red for very large files
        indicator="🔴"
    elif [ "$lines" -gt 400 ]; then
        color="\033[33m"  # Yellow for large files
        indicator="🟡"
    elif [ "$lines" -gt 200 ]; then
        color="\033[36m"  # Cyan for medium files
        indicator="🔵"
    else
        color="\033[32m"  # Green for small files
        indicator="🟢"
    fi
    
    reset="\033[0m"
    
    printf "%s %s%-20s%s %4d lines " "$indicator" "$color" "$file" "$reset" "$lines"
    create_bar "$lines" "$max_lines"
    echo
done < "$temp_file"

echo
echo "📈 SUMMARY STATISTICS:"
echo "====================="

# Calculate totals
total_files=$(wc -l < "$temp_file")
total_lines=$(awk '{sum += $1} END {print sum}' "$temp_file")
avg_lines=$(( total_lines / total_files ))

echo "Total files: $total_files"
echo "Total lines: $total_lines"
echo "Average lines per file: $avg_lines"
echo

# Identify problematic files
echo "🚨 REFACTORING CANDIDATES:"
echo "========================="
echo "Files over 500 lines (consider splitting):"
awk '$1 > 500 {printf "  🔧 %s (%d lines)\n", $2, $1}' "$temp_file" | sed 's|src/||g'
echo

echo "📝 RECOMMENDATIONS:"
echo "=================="
echo "🔴 URGENT (600+ lines): Consider breaking into smaller modules"
echo "🟡 LARGE (400+ lines): Monitor complexity, consider logical splits if needed"  
echo "🔵 MEDIUM (200+ lines): Good size for Rust modules, keep functions focused"
echo "🟢 SMALL (<200 lines): Excellent - focused and maintainable"
echo

# Cleanup
rm "$temp_file"

echo "Run this script anytime with: ./scripts/code-stats.sh"