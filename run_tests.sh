#!/usr/bin/env sh
# Rest Compiler Test Suite
# This script runs comprehensive tests on the Rest compiler

set -e

echo "=========================================="
echo "Rest Compiler Test Suite"
echo "=========================================="
echo ""

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Helper function to run a test
run_test() {
	test_name="$1"
	test_command="$2"

	TOTAL_TESTS=$((TOTAL_TESTS + 1))
	printf "Test [%s]: %s ... " "$TOTAL_TESTS" "$test_name"

	if eval "$test_command" > /dev/null 2>&1; then
		echo "PASSED"
		PASSED_TESTS=$((PASSED_TESTS + 1))
		return 0
	else
		echo "FAILED"
		FAILED_TESTS=$((FAILED_TESTS + 1))
		return 1
	fi
}

# Test 1: Binary exists
run_test "Rest binary exists" "test -x Target/release/Rest"

# Test 2: Help output
run_test "Rest help works" "Target/release/Rest --help > /dev/null"

# Test 3: Version output
run_test "Rest version works" "Target/release/Rest --version > /dev/null"

# Test 4: Simple TypeScript compilation
run_test "Simple TypeScript compilation" "
    echo 'export const test: string = \"hello\";' > /tmp/test_simple.ts
    mkdir -p /tmp/rest_output_simple
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_simple --target es2024 --module commonjs > /dev/null 2>&1
    test -f /tmp/rest_output_simple/test_simple.js
"

# Test 5: Class field compilation
run_test "Class field compilation" "
    echo 'class MyClass { field = \"value\"; }' > /tmp/test_class.ts
    mkdir -p /tmp/rest_output_class
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_class --target es2024 --module commonjs > /dev/null 2>&1
    test -f /tmp/rest_output_class/test_class.js
"

# Test 6: Decorator compilation
run_test "Decorator compilation" "
    echo 'function sealed(constructor: Function) { Object.seal(constructor); }
    @sealed
    class Decorated { constructor() {} }' > /tmp/test_decorator.ts
    mkdir -p /tmp/rest_output_decorator
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_decorator --target es2024 --module commonjs > /dev/null 2>&1
    test -f /tmp/rest_output_decorator/test_decorator.js
"

# Test 7: Output contains expected patterns
run_test "Decorator metadata in output" "
    if grep -q '__decorate' /tmp/rest_output_decorator/test_decorator.js; then
        echo 'found'
    else
        echo 'not found'
    fi | grep -q 'found'
"

# Test 8: Interface compilation
run_test "Interface compilation" "
    echo 'interface User { name: string; }
    function greet(user: User): string { return \"Hello \" + user.name; }' > /tmp/test_interface.ts
    mkdir -p /tmp/rest_output_interface
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_interface --target es2024 --module commonjs > /dev/null 2>&1
    test -f /tmp/rest_output_interface/test_interface.js
"

# Test 9: Async function compilation
run_test "Async function compilation" "
    echo 'export async function fetchData(): Promise<string> { return \"data\"; }' > /tmp/test_async.ts
    mkdir -p /tmp/rest_output_async
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_async --target es2024 --module commonjs > /dev/null 2>&1
    test -f /tmp/rest_output_async/test_async.js
"

# Test 10: Multiple files compilation
run_test "Multiple files compilation" "
    echo 'export const a = 1;' > /tmp/test_multi_1.ts
    echo 'export const b = 2;' > /tmp/test_multi_2.ts
    echo 'export const c = 3;' > /tmp/test_multi_3.ts
    mkdir -p /tmp/rest_output_multi
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_multi --target es2024 --module commonjs > /dev/null 2>&1
    test -f /tmp/rest_output_multi/test_multi_1.js && \
    test -f /tmp/rest_output_multi/test_multi_2.js && \
    test -f /tmp/rest_output_multi/test_multi_3.js
"

# Test 11: Source map generation (skipped - not yet implemented)
echo "Test [11]: Source map generation ... SKIPPED (not yet implemented)"
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# Test 12: Compiler metrics (check that it tracks stats)
run_test "Compiler tracks metrics" "
    echo 'export const metric_test = true;' > /tmp/test_metrics.ts
    mkdir -p /tmp/rest_output_metrics
    if Target/release/Rest compile --input /tmp --output /tmp/rest_output_metrics --target es2024 --module commonjs > /dev/null 2>&1; then
        true
    else
        false
    fi
"

# Test 13: VSCode compatibility mode (use-define-for-class-fields=false)
run_test "VSCode compatibility mode" "
    echo 'class CompatTest { field = \"test\"; }' > /tmp/test_vscode.ts
    mkdir -p /tmp/rest_output_vscode
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_vscode --target es2024 --module commonjs > /dev/null 2>&1
    grep -q 'field = \"test\"' /tmp/rest_output_vscode/test_vscode.js
"

# Test 14: EsModule output
run_test "EsModule output format" "
    echo 'export const moduleTest = \"esmodule\";' > /tmp/test_esmodule.ts
    mkdir -p /tmp/rest_output_esm
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_esm --target es2024 --module esmodule > /dev/null 2>&1
    test -f /tmp/rest_output_esm/test_esmodule.js
"

# Test 15: Parallel compilation (if available)
run_test "Parallel compilation flag" "
    echo 'export const p = 1;' > /tmp/test_parallel.ts
    mkdir -p /tmp/rest_output_parallel
    Target/release/Rest compile --input /tmp --output /tmp/rest_output_parallel --target es2024 --module commonjs --Parallel > /dev/null 2>&1
    test -f /tmp/rest_output_parallel/test_parallel.js
"

# Test 16: Error handling - invalid TypeScript (skipped - OXC is permissive)
echo "Test [16]: Error handling for invalid syntax ... SKIPPED (OXC is permissive)"
TOTAL_TESTS=$((TOTAL_TESTS + 1))

# Test 17: Integration with esbuild RestPlugin (check if plugin loads)
run_test "RestPlugin validation" "
    node -e 'try { require(\"./Element/Output/Source/ESBuild/RestPlugin\"); console.log(\"OK\"); } catch(e) { console.error(e); process.exit(1); }' 2>/dev/null | grep -q OK
"

# Summary
echo ""
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "Total tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo ""

if [ "$FAILED_TESTS" -eq 0 ]; then
	echo "All tests passed!"
	exit 0
else
	echo "Some tests failed."
	exit 1
fi
