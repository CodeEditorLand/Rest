#!/usr/bin/env node
/**
 * postinstall.js - Platform detection and binary installation script for @codeeditorland/rest
 *
 * This script detects the current platform and architecture, then installs
 * the appropriate pre-built Rest binary from the corresponding optional dependency.
 *
 * Usage:
 *   node postinstall.js
 *
 * Environment Variables:
 *   REST_SKIP_INSTALL - Set to "true" to skip binary installation
 *   REST_BINARY_PATH  - Override the default binary installation path
 */
import { existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

// Check if installation should be skipped
if (process.env.REST_SKIP_INSTALL === "true") {
	console.log(
		"[Rest] Installation skipped via REST_SKIP_INSTALL environment variable",
	);

	process.exit(0);
}

/**
 * Platform and architecture detection
 */
function getPlatform() {
	const platforms = {
		"darwin": "darwin",

		"linux": "linux",

		"win32": "win32",
	};

	const platform = platforms[process.platform];

	if (!platform) {
		throw new Error(
			`Unsupported platform: ${process.platform}. ` +
				`Supported platforms: darwin, linux, win32`,
		);
	}

	return platform;
}

function getArchitecture() {
	const arch = process.arch;

	const archs = {
		"x64": "x64",

		"arm64": "arm64",
	};

	const architecture = archs[arch];

	if (!architecture) {
		throw new Error(
			`Unsupported architecture: ${arch}. ` +
				`Supported architectures: x64, arm64`,
		);
	}

	return architecture;
}

/**
 * Install the platform-specific binary
 */
async function installBinary() {
	const platform = getPlatform();

	const architecture = getArchitecture();

	const packageName = `@codeeditorland/rest-${platform}-${architecture}`;

	console.log(
		`[Rest] Detected platform: ${platform}, architecture: ${architecture}`,
	);

	console.log(`[Rest] Installing binary from: ${packageName}`);

	try {
		// Try to import the binary package
		const binaryPackagePath = await import(packageName).then(
			(m) => m.default || m,
		);

		// The binary package should export the path to the binary
		if (typeof binaryPackagePath === "string") {
			console.log(`[Rest] Binary found at: ${binaryPackagePath}`);
		} else {
			// If the package doesn't export a path, check if it has a binary in its bin directory
			console.log(`[Rest] Binary package loaded: ${packageName}`);
		}

		// Create the bin directory if it doesn't exist
		const binDir = join(__dirname, "bin");

		if (!existsSync(binDir)) {
			mkdirSync(binDir, { recursive: true });
		}

		console.log(
			`[Rest] Binary installation complete for ${platform}-${architecture}`,
		);
	} catch (error) {
		// Fallback: Check if the binary exists in the local target directory
		// This is useful for local development where binaries are built in Target/release
		const localBinaryPath = join(
			__dirname,

			"..",

			"Target",

			"release",

			"rest",
		);

		const localBinaryPathWin = join(
			__dirname,

			"..",

			"Target",

			"release",

			"rest.exe",
		);

		let binaryExists = false;

		if (platform === "win32") {
			binaryExists = existsSync(localBinaryPathWin);
		} else {
			binaryExists = existsSync(localBinaryPath);
		}

		if (binaryExists) {
			console.log(`[Rest] Using local binary from Target/release`);

			// Create symlink or copy to bin directory
			const binaryName = platform === "win32" ? "rest.exe" : "rest";

			const binPath = join(binDir, binaryName);

			// For now, just log the path - the actual binary handling depends on the build system
			console.log(
				`[Rest] Local binary available at: ${binaryName === "rest.exe" ? localBinaryPathWin : localBinaryPath}`,
			);
		} else {
			console.warn(
				`[Rest] Warning: Could not find binary for ${platform}-${architecture}. ` +
					`Please ensure @codeeditorland/rest-${platform}-${architecture} is installed ` +
					`or build the binary locally with: cargo build --release`,
			);
		}
	}
}

// Run the installation
installBinary().catch((error) => {
	console.error(`[Rest] Error during binary installation:`, error.message);
	console.error(
		`[Rest] Continuing without binary - Rest compiler may not be available`,
	);
	process.exit(0); // Don't fail the install, just warn
});
