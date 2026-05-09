#!/usr/bin/env node
/**
 * index.js - Main entry point for @codeeditorland/rest NPM package
 *
 * This file provides the main module export for the Rest compiler.
 * It re-exports the CLI wrapper functionality.
 */
import { execSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Execute the Rest compiler with the provided arguments
 * @param {...string} args - Command line arguments to pass to the Rest binary
 * @returns {Buffer} The output from the Rest compiler
 */
export function run(...args) {
	const platform = process.platform;

	const binaryName = platform === "win32" ? "Rest.exe" : "Rest";

	const binaryPath = join(__dirname, "bin", binaryName);

	if (!existsSync(binaryPath)) {
		throw new Error(
			`[Rest] Binary not found at: ${binaryPath}\n` +
				"[Rest] Please ensure the package is properly installed with: npm install",
		);
	}

	const command = `"${binaryPath}" ${args.map((a) => `"${a}"`).join(" ")}`;

	return execSync(command, { encoding: "utf8" });
}

/**
 * Get the version of the installed Rest compiler
 * @returns {string} The version string
 */
export function getVersion() {
	try {
		const output = run("--version");

		return output.match(/v?\d+\.\d+\.\d+/)?.[0] || "unknown";
	} catch (_error) {
		return "unknown";
	}
}

// Export CLI functionality for programmatic use
export default {
	run,

	getVersion,
};
