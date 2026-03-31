#!/usr/bin/env bash
# Rest vs VSCode Output Compatibility Benchmark
# This script compares Rest compiler output with VSCode's gulp/tsb build output

set -e

echo "=========================================="
echo "Rest vs VSCode Compatibility Benchmark"
echo "=========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
VSCode_SOURCE_DIR="Dependency/Microsoft/Dependency/Editor/src"
VSCode_OUT_DIR="Dependency/Microsoft/Dependency/Editor/out"
VSCode_OUT_BUILD_DIR="Dependency/Microsoft/Dependency/Editor/out-build"
REST_OUT_DIR="Element/Rest/Target/benchmark-out"
REST_OUT_BUILD_DIR="Element/Rest/Target/benchmark-out-build"
REST_BINARY="Element/Rest/Target/debug/Rest"  # Use debug binary to avoid optimizer bugs

# Check prerequisites
echo "Checking prerequisites..."
if [ ! -x "$REST_BINARY" ]; then
    echo -e "${RED}ERROR:${NC} Rest binary not found at $REST_BINARY"
    echo "Please build Rest first: cd Element/Rest && cargo build"
    exit 1
fi

if [ ! -d "$VSCode_SOURCE_DIR" ]; then
    echo -e "${RED}ERROR:${NC} VSCode source not found at $VSCode_SOURCE_DIR"
    exit 1
fi

# Create output directories
mkdir -p "$REST_OUT_DIR"
mkdir -p "$REST_OUT_BUILD_DIR"

echo ""
echo "=========================================="
echo "Benchmark Configuration"
echo "=========================================="
echo -e "${BLUE}VSCode Source:${NC} $VSCode_SOURCE_DIR"
echo -e "${BLUE}VSCode Dev Output:${NC} $VSCode_OUT_DIR"
echo -e "${BLUE}VSCode Prod Output:${NC} $VSCode_OUT_BUILD_DIR"
echo -e "${BLUE}Rest Dev Output:${NC} $REST_OUT_DIR"
echo -e "${BLUE}Rest Bin:${NC} $REST_BINARY"
echo ""

# Function to count TypeScript files
count_typescript_files() {
    local dir="$1"
    find "$dir" -name "*.ts" -type f 2>/dev/null | \
        grep -v "node_modules" | \
        grep -v "Target" | \
        grep -v "/test" | \
        grep -v "\.d\.ts$" | \
        wc -l
}

# Function to compile with Rest
benchmark_rest() {
    local input_dir="$1"
    local output_dir="$2"
    local label="$3"

    echo ""
    echo "=========================================="
    echo "Benchmark: $label"
    echo "=========================================="

    # Clean output directory
    rm -rf "$output_dir"
    mkdir -p "$output_dir"

    # Count files to compile
    echo "Discovering TypeScript files..."
    local file_count=$(count_typescript_files "$input_dir")

    if [ "$file_count" -eq 0 ]; then
        echo -e "${YELLOW}WARNING:${NC} No TypeScript files found in $input_dir"
        return
    fi

    echo "Found $file_count TypeScript files"
    echo ""

    # Run compilation
    echo "Running Rest compilation..."
    echo "Command: $REST_BINARY compile -i \"$input_dir\" -o \"$output_dir\""
    local start_time=$(date +%s.%N)

    if $REST_BINARY compile -i "$input_dir" -o "$output_dir" 2>&1; then
        local exit_code=0
    else
        local exit_code=$?
    fi

    local end_time=$(date +%s.%N)
    local elapsed=$(echo "$end_time - $start_time" | bc)

    echo ""
    echo "Results:"
    echo "  Exit code: $exit_code"
    printf "  Total time: %.3f seconds\n" $elapsed

    if [ -d "$output_dir" ]; then
        local compiled_count=$(find "$output_dir" -name "*.js" -type f 2>/dev/null | grep -v "map" | wc -l)
        echo "  Output files: $compiled_count"
        echo "  Output size: $(du -sh "$output_dir" | cut -f1)"

        # Save results
        echo "$file_count,$compiled_count,$exit_code,$elapsed" > "$output_dir/benchmark_results.csv"
    else
        echo "  No output directory created"
    fi
}

# Function to compare outputs
compare_outputs() {
    local vscode_dir="$1"
    local rest_dir="$2"
    local label="$3"

    echo ""
    echo "=========================================="
    echo "Comparison: $label"
    echo "=========================================="

    if [ ! -d "$vscode_dir" ] || [ ! -d "$rest_dir" ]; then
        echo -e "${YELLOW}SKIPPED:${NC} Directories not ready (vscode: $vscode_dir, rest: $rest_dir)"
        return
    fi

    # Find common JS files (exclude .d.js declaration files to match VSCode's build output)
    echo "Finding VSCode JavaScript files..."
    local vscode_js=($(find "$vscode_dir" -name "*.js" -type f 2>/dev/null | grep -v "map" | grep -v "\.d\.js$"))
    echo "Finding Rest JavaScript files..."
    local rest_js=($(find "$rest_dir" -name "*.js" -type f 2>/dev/null | grep -v "map" | grep -v "\.d\.js$"))

    local total_vscode=${#vscode_js[@]}
    local total_rest=${#rest_js[@]}

    echo "VSCode: $total_vscode files, Rest: $total_rest files"

    if [ $total_vscode -eq 0 ]; then
        echo "No VSCode JavaScript files found to compare"
        return
    fi

    if [ $total_rest -eq 0 ]; then
        echo "No Rest JavaScript files produced"
        return
    fi

    # Compare file by file
    echo "Comparing outputs..."
    local match_count=0
    local mismatch_count=0
    local missing_count=0

    for vscode_file in "${vscode_js[@]}"; do
        local rel="${vscode_file#$vscode_dir/}"
        local rest_file="$rest_dir/$rel"

        if [ ! -f "$rest_file" ]; then
            missing_count=$((missing_count + 1))
            echo "  MISSING: $rel"
        else
            # Compare content
            if cmp -s "$vscode_file" "$rest_file"; then
                match_count=$((match_count + 1))
            else
                mismatch_count=$((mismatch_count + 1))
                local vscode_size=$(wc -c < "$vscode_file")
                local rest_size=$(wc -c < "$rest_file")
                echo "  DIFF: $rel (VSCode: ${vscode_size}B, Rest: ${rest_size}B)"
            fi
        fi
    done

    echo ""
    echo "Comparison results:"
    echo "  Total VSCode files: $total_vscode"
    echo "  Matching: $match_count"
    echo "  Mismatched: $mismatch_count"
    echo "  Missing in Rest: $missing_count"

    local match_pct=0
    if [ $total_vscode -gt 0 ]; then
        match_pct=$(echo "scale=2; ($match_count * 100) / $total_vscode" | bc)
    fi
    echo "  Match rate: ${match_pct}%"

    if [ $mismatch_count -eq 0 ] && [ $missing_count -eq 0 ]; then
        echo -e "  Status: ${GREEN}PERFECT MATCH${NC}"
    else
        echo -e "  Status: ${RED}DIFFERENCES DETECTED${NC}"
    fi
}

# Main execution
echo "Starting benchmark at $(date)"
echo ""

# Benchmark development build (out/)
if [ -d "$VSCode_SOURCE_DIR" ] && [ "$(count_typescript_files "$VSCode_SOURCE_DIR")" -gt 0 ]; then
    benchmark_rest "$VSCode_SOURCE_DIR" "$REST_OUT_DIR" "Development Build (src -> out/)"
    compare_outputs "$VSCode_OUT_DIR" "$REST_OUT_DIR" "Development"
else
    echo -e "${YELLOW}SKIPPING:${NC} VSCode source not found or empty"
fi

# Benchmark production build (out-build/)
if [ -d "$VSCode_OUT_BUILD_DIR" ] && [ "$(find "$VSCode_OUT_BUILD_DIR" -name "*.js" 2>/dev/null | wc -l)" -gt 0 ]; then
    echo ""
    echo -e "${YELLOW}NOTE:${NC} Production build requires different compile options"
    echo "For now comparing same output with different expected directory"
    compare_outputs "$VSCode_OUT_BUILD_DIR" "$REST_OUT_BUILD_DIR" "Production"
else
    echo -e "${YELLOW}SKIPPING:${NC} VSCode production build not ready (out-build/ empty)"
fi

echo ""
echo "=========================================="
echo "Benchmark Complete"
echo "=========================================="
echo "Results saved to:"
echo "  $REST_OUT_DIR/benchmark_results.csv"
echo "  $REST_OUT_BUILD_DIR/benchmark_results.csv"
echo ""
echo "Next steps:"
echo "  1. If no output: Check if VSCode source exists and contains .ts files"
echo "  2. If output differs: Investigate mismatches to ensure 1:1 compatibility"
echo "  3. Re-run after making changes to Rest compiler"
