#!/usr/bin/env sh
#===============================================================================
# prepublishOnly.sh - Build script for @codeeditorland/Rest NPM publishing
#===============================================================================
#
# This script builds the Rest compiler binary for all supported platforms
# before publishing to NPM. It should be run as part of the prepublishOnly
# lifecycle script.
#
# Usage:
#   sh prepublishOnly.sh
#
# Environment Variables:
#   REST_BUILD_TARGET - Override the build target (default: auto-detect)
#   REST_SKIP_BUILD   - Set to "true" to skip the build
#   Compiler          - Set to "Rest" to enable the Rest compiler
#
#===============================================================================

set -e

log_info() {
	printf "[Rest] %s\n" "$1"
}

log_warn() {
	printf "[Rest] %s\n" "$1"
}

log_error() {
	printf "[Rest] %s\n" "$1" >&2
}

# Ensure Rust toolchain is configured
if ! command -v rustup >/dev/null 2>&1; then
	log_error "rustup is not installed. Please install Rust from https://rustup.rs/"
	exit 1
fi

# Set default toolchain to stable if not already configured
if ! rustup default >/dev/null 2>&1; then
	log_info "Setting Rust default toolchain to stable..."
	rustup default stable
fi

# Check if build should be skipped
if [ "${REST_SKIP_BUILD}" = "true" ]; then
	log_info "Build skipped via REST_SKIP_BUILD environment variable"
	exit 0
fi

# Ensure we're in the Rest directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

log_info "Starting Rest build for NPM publishing..."
log_info "Working directory: $SCRIPT_DIR"

# Check if Cargo is available
if ! command -v cargo >/dev/null 2>&1; then
	log_error "Cargo is not installed. Please install Rust from https://rustup.rs/"
	exit 1
fi

log_info "Cargo version: $(cargo --version)"

# Detect target platform
detect_target() {
	platform=$(uname -s | tr '[:upper:]' '[:lower:]')
	arch=$(uname -m)

	case "$platform" in
	darwin)
		if [ "$arch" = "arm64" ]; then
			echo "aarch64-apple-darwin"
		else
			echo "x86_64-apple-darwin"
		fi
		;;
	linux)
		if [ "$arch" = "aarch64" ]; then
			echo "aarch64-unknown-linux-gnu"
		else
			echo "x86_64-unknown-linux-gnu"
		fi
		;;
	msys* | mingw* | cygwin*)
		echo "x86_64-pc-windows-msvc"
		;;
	*)
		log_error "Unsupported platform: $platform"
		exit 1
		;;
	esac
}

# Override target if environment variable is set
TARGET="${REST_BUILD_TARGET:-$(detect_target)}"

log_info "Build target: $TARGET"

# Build the release binary
log_info "Building Rest compiler in release mode..."
cargo build --release --target "$TARGET"

# Verify the binary was created
case "$TARGET" in
*windows*)
	BINARY_PATH="Target/$TARGET/release/Rest.exe"
	;;
*)
	BINARY_PATH="Target/$TARGET/release/Rest"
	;;
esac

if [ ! -f "$BINARY_PATH" ]; then
	log_error "Binary not found at: $BINARY_PATH"
	exit 1
fi

log_info "Build successful: $BINARY_PATH"

# Create bin directory for NPM package
mkdir -p bin

# Copy the binary to bin directory
BINARY_NAME=$(basename "$BINARY_PATH")
cp "$BINARY_PATH" "bin/$BINARY_NAME"

# Make the binary executable (Unix-like systems)
case "$TARGET" in
*windows*) ;;
*)
	chmod +x "bin/$BINARY_NAME"
	;;
esac

log_info "Binary copied to: bin/$BINARY_NAME"

# Create the bin wrapper script (Rest.js)
cat >bin/Rest.js <<'EOF'
#!/usr/bin/env node
/**
 * Rest.js - CLI wrapper for Rest compiler binary
 */

import { execSync } from 'node:child_process';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { existsSync } from 'node:fs';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Determine the binary path based on platform
const platform = process.platform;
const binaryName = platform === 'win32' ? 'Rest.exe' : 'Rest';
const binaryPath = join(__dirname, binaryName);

if (!existsSync(binaryPath)) {
  console.error(`[Rest] Binary not found at: ${binaryPath}`);
  console.error('[Rest] Please run: npm install or npm rebuild');
  process.exit(1);
}

// Execute the binary with all arguments
try {
  execSync(`"${binaryPath}" ${process.argv.slice(2).map(a => `"${a}"`).join(' ')}`, {
    stdio: 'inherit',
  });
} catch (error) {
  process.exit(error.status || 1);
}
EOF

# Make the wrapper script executable
chmod +x bin/Rest.js

log_info "CLI wrapper created: bin/Rest.js"

# Build summary
log_info "=========================================="
log_info "Build Summary"
log_info "=========================================="
log_info "Target:    $TARGET"
log_info "Binary:    bin/$BINARY_NAME"
log_info "CLI:       bin/Rest.js"
log_info "=========================================="
log_info "Ready for NPM publish!"
log_info "=========================================="

exit 0
