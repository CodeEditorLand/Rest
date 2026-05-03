#!/usr/bin/env sh
# Rest vs VSCode Output Compatibility Benchmark
# This script compares Rest compiler output with VSCode's gulp/tsb build output

set -e

echo "=========================================="
echo "Rest vs VSCode Compatibility Benchmark"
echo "=========================================="
echo ""

# Configuration
VSCode_SOURCE_DIR="Dependency/Microsoft/Dependency/Editor/src"
VSCode_OUT_DIR="Dependency/Microsoft/Dependency/Editor/out"
VSCode_OUT_BUILD_DIR="Dependency/Microsoft/Dependency/Editor/out-build"
REST_OUT_DIR="Element/Rest/Target/benchmark-out"
REST_OUT_BUILD_DIR="Element/Rest/Target/benchmark-out-build"
REST_BINARY="Element/Rest/Target/debug/Rest"

# Check prerequisites
echo "Checking prerequisites..."
if [ ! -x "$REST_BINARY" ]; then
	echo "ERROR: Rest binary not found at $REST_BINARY"
	echo "Please build Rest first: cd Element/Rest && cargo build"
	exit 1
fi

if [ ! -d "$VSCode_SOURCE_DIR" ]; then
	echo "ERROR: VSCode source not found at $VSCode_SOURCE_DIR"
	exit 1
fi

# Create output directories
mkdir -p "$REST_OUT_DIR"
mkdir -p "$REST_OUT_BUILD_DIR"

echo ""
echo "=========================================="
echo "Benchmark Configuration"
echo "=========================================="
echo "VSCode Source:      $VSCode_SOURCE_DIR"
echo "VSCode Dev Output:  $VSCode_OUT_DIR"
echo "VSCode Prod Output: $VSCode_OUT_BUILD_DIR"
echo "Rest Dev Output:    $REST_OUT_DIR"
echo "Rest Bin:           $REST_BINARY"
echo ""

# Function to count TypeScript files
count_typescript_files() {
	find "$1" -name "*.ts" -type f 2> /dev/null \
		| grep -v "node_modules" \
		| grep -v "Target" \
		| grep -v "/test" \
		| grep -v "\.d\.ts$" \
		| wc -l
}

# Function to compile with Rest
benchmark_rest() {
	input_dir="$1"
	output_dir="$2"
	label="$3"

	echo ""
	echo "=========================================="
	echo "Benchmark: $label"
	echo "=========================================="

	# Clean output directory
	rm -rf "$output_dir"
	mkdir -p "$output_dir"

	echo "Discovering TypeScript files..."
	file_count=$(count_typescript_files "$input_dir")

	if [ "$file_count" -eq 0 ]; then
		echo "WARNING: No TypeScript files found in $input_dir"
		return
	fi

	echo "Found $file_count TypeScript files"
	echo ""

	echo "Running Rest compilation..."
	echo "Command: $REST_BINARY compile -i \"$input_dir\" -o \"$output_dir\""
	start_time=$(date +%s)

	exit_code=0
	$REST_BINARY compile -i "$input_dir" -o "$output_dir" 2>&1 || exit_code=$?

	end_time=$(date +%s)
	elapsed=$((end_time - start_time))

	echo ""
	echo "Results:"
	echo "  Exit code: $exit_code"
	printf "  Total time: %d seconds\n" "$elapsed"

	if [ -d "$output_dir" ]; then
		compiled_count=$(find "$output_dir" -name "*.js" -type f 2> /dev/null | grep -v "map" | wc -l)
		echo "  Output files: $compiled_count"
		echo "  Output size: $(du -sh "$output_dir" | cut -f1)"

		printf "%s,%s,%s,%s\n" "$file_count" "$compiled_count" "$exit_code" "$elapsed" > "$output_dir/benchmark_results.csv"
	else
		echo "  No output directory created"
	fi
}

# Function to compare outputs
compare_outputs() {
	vscode_dir="$1"
	rest_dir="$2"
	label="$3"

	echo ""
	echo "=========================================="
	echo "Comparison: $label"
	echo "=========================================="

	if [ ! -d "$vscode_dir" ] || [ ! -d "$rest_dir" ]; then
		echo "SKIPPED: Directories not ready (vscode: $vscode_dir, rest: $rest_dir)"
		return
	fi

	echo "Finding VSCode JavaScript files..."
	echo "Finding Rest JavaScript files..."

	total_vscode=$(find "$vscode_dir" -name "*.js" -type f 2> /dev/null | grep -v "map" | grep -v "\.d\.js$" | wc -l)
	total_rest=$(find "$rest_dir" -name "*.js" -type f 2> /dev/null | grep -v "map" | grep -v "\.d\.js$" | wc -l)

	echo "VSCode: $total_vscode files, Rest: $total_rest files"

	if [ "$total_vscode" -eq 0 ]; then
		echo "No VSCode JavaScript files found to compare"
		return
	fi

	if [ "$total_rest" -eq 0 ]; then
		echo "No Rest JavaScript files produced"
		return
	fi

	echo "Comparing outputs..."
	match_count=0
	mismatch_count=0
	missing_count=0

	find "$vscode_dir" -name "*.js" -type f 2> /dev/null | grep -v "map" | grep -v "\.d\.js$" | while IFS= read -r vscode_file; do
		rel="${vscode_file#$vscode_dir/}"
		rest_file="$rest_dir/$rel"

		if [ ! -f "$rest_file" ]; then
			missing_count=$((missing_count + 1))
			echo "  MISSING: $rel"
		else
			if cmp -s "$vscode_file" "$rest_file"; then
				match_count=$((match_count + 1))
			else
				mismatch_count=$((mismatch_count + 1))
				vscode_size=$(wc -c < "$vscode_file")
				rest_size=$(wc -c < "$rest_file")
				echo "  DIFF: $rel (VSCode: ${vscode_size}B, Rest: ${rest_size}B)"
			fi
		fi
	done

	echo ""
	echo "Comparison results:"
	echo "  Total VSCode files: $total_vscode"
	echo "  (See output above for mismatches and missing files)"
}

# Main execution
echo "Starting benchmark at $(date)"
echo ""

if [ -d "$VSCode_SOURCE_DIR" ] && [ "$(count_typescript_files "$VSCode_SOURCE_DIR")" -gt 0 ]; then
	benchmark_rest "$VSCode_SOURCE_DIR" "$REST_OUT_DIR" "Development Build (src -> out/)"
	compare_outputs "$VSCode_OUT_DIR" "$REST_OUT_DIR" "Development"
else
	echo "SKIPPING: VSCode source not found or empty"
fi

if [ -d "$VSCode_OUT_BUILD_DIR" ] && [ "$(find "$VSCode_OUT_BUILD_DIR" -name "*.js" 2> /dev/null | wc -l)" -gt 0 ]; then
	echo ""
	echo "NOTE: Production build requires different compile options"
	echo "For now comparing same output with different expected directory"
	compare_outputs "$VSCode_OUT_BUILD_DIR" "$REST_OUT_BUILD_DIR" "Production"
else
	echo "SKIPPING: VSCode production build not ready (out-build/ empty)"
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
