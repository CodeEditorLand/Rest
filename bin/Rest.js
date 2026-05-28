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
