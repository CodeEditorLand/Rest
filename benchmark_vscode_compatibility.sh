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

# Check prerequisites
echo "Checking prerequisites..."
if [ ! -x "Target/release/Rest" ]; then
    echo -e "${RED}ERROR:${NC} Rest binary not found at Target/release/Rest"
    echo "Please build Rest first: cargo build --release --package=Rest"
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
echo -e "${BLUE}Rest Prod Output:${NC} $REST_OUT_BUILD_DIR"
echo ""

# Function to find all TypeScript files (excluding tests and d.ts)
find_typescript_files() {
    local dir="$1"
    find "$dir" -name "*.ts" -type f \
        | grep -v "node_modules" \
        | grep -v "Target" \
        | grep -v "test" \
        | grep -v "\.d\.ts$" \
        | head -100  # Limit to first 100 for benchmark
}

# Function to measure compilation time
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

    # Find files to compile
    echo "Discovering TypeScript files..."
    local files=($(find_typescript_files "$input_dir"))
    local file_count=${#files[@]}

    if [ $file_count -eq 0 ]; then
        echo -e "${YELLOW}WARNING:${NC} No TypeScript files found, skipping"
        return
    fi

    echo "Found $file_count TypeScript files (first 100)"
    echo ""

    # Warm-up run (compile 1 file)
    echo "Warm-up: Compiling 1 file..."
    local warm_file="${files[0]}"
    local warm_out="$output_dir/warmup.js"
    Target/release/Rest compile_file_to "$warm_file" "$(cat "$warm_file")" "$warm_out" false > /dev/null 2>&1
    rm "$warm_out"

    # Actual benchmark
    echo "Running benchmark..."
    local start_time=$(date +%s.%N)
    local success_count=0
    local fail_count=0

    for file in "${files[@]}"; do
        # Compute relative path
        local rel_path="${file#$input_dir/}"
        local out_file="$output_dir/$rel_path"
        out_file="${out_path%.ts}.js"

        # Create parent directory
        mkdir -p "$(dirname "$out_file")"

        # Compile
        if Target/release/Rest compile_file_to "$file" "$(cat "$file")" "$out_file" false > /dev/null 2>&1; then
            success_count=$((success_count + 1))
        else
            fail_count=$((fail_count + 1))
        fi
    done

    local end_time=$(date +%s.%N)
    local elapsed=$(echo "$end_time - $start_time" | bc)

    echo ""
    echo "Results:"
    echo "  Total files: $file_count"
    echo "  Successful: $success_count"
    echo "  Failed: $fail_count"
    printf "  Total time: %.3f seconds\n" $elapsed
    printf "  Throughput: %.1f files/sec\n" $(echo "$success_count / $elapsed" | bc -l)
    printf "  Avg per file: %.3f ms\n" $(echo "($elapsed * 1000) / $success_count" | bc -l)

    # Save results
    echo "$file_count,$success_count,$fail_count,$elapsed" > "$output_dir/benchmark_results.csv"
    echo "Output size: $(du -sh "$output_dir" | cut -f1)"
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
        echo -e "${YELLOW}SKIPPED:${NC} Directories not ready"
        return
    fi

    # Find common JS files
    echo "Finding common JavaScript files..."
    local vscode_js=($(find "$vscode_dir" -name "*.js" -type f | grep -v "map" | head -50))
    local rest_js=($(find "$rest_dir" -name "*.js" -type f | grep -v "map" | head -50))

    # Build sets
    declare -A rest_set
    for f in "${rest_js[@]}"; do
        rel="${f#$rest_dir/}"
        rest_set["$rel"]="$f"
    done

    # Compare
    local total=${#vscode_js[@]}
    local match_count=0
    local mismatch_count=0
    local missing_count=0

    for vscode_file in "${vscode_js[@]}"; do
        rel="${vscode_file#$vscode_dir/}"
        rest_file="${rest_set["$rel"]}"

        if [ -z "$rest_file" ]; then
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
    echo "  Total VSCode files: $total"
    echo "  Matching: $match_count"
    echo "  Mismatched: $mismatch_count"
    echo "  Missing in Rest: $missing_count"

    local match_pct=0
    if [ $total -gt 0 ]; then
        match_pct=$(echo "scale=2; ($match_count * 100) / $total" | bc)
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
if [ -d "$VSCode_OUT_DIR" ] && [ "$(find "$VSCode_OUT_DIR" -name "*.js" 2>/dev/null | wc -l)" -gt 0 ]; then
    benchmark_rest "$VSCode_OUT_DIR" "$REST_OUT_DIR" "Development Build (out/)"
    compare_outputs "$VSCode_OUT_DIR" "$REST_OUT_DIR" "Development"
else
    echo -e "${YELLOW}SKIPPING:${NC} VSCode development build not ready (out/ empty)"
fi

# Benchmark production build (out-build/)
if [ -d "$VSCode_OUT_BUILD_DIR" ] && [ "$(find "$VSCode_OUT_BUILD_DIR" -name "*.js" 2>/dev/null | wc -l)" -gt 0 ]; then
    benchmark_rest "$VSCode_OUT_BUILD_DIR" "$REST_OUT_BUILD_DIR" "Production Build (out-build/)"
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
echo "  1. Build VSCode if not ready: cd Dependency/Microsoft/Dependency/Editor && npm run compile-build"
echo "  2. Re-run this benchmark to compare outputs"
echo "  3. Investigate any mismatches to ensure 1:1 compatibility"
