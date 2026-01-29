#!/usr/bin/env node

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { globSync } from "node:fs";

const version = process.argv[2];
if (!version) {
	console.error("Usage: node sync_version.mjs <version>");
	process.exit(1);
}

const cratesDir = path.join(import.meta.dirname, "../crates");

// Use glob to find all crates with package.json
for (const packageJsonPath of globSync("*/package.json", { cwd: cratesDir })) {
	const crate = path.dirname(packageJsonPath);
	const crateDir = path.join(cratesDir, crate);
	const packageJsonFullPath = path.join(crateDir, "package.json");
	const jsrPath = path.join(crateDir, "jsr.jsonc");

	console.log(`Updating ${crate} to version ${version}`);

	const packJson = readJSON(packageJsonFullPath);
	packJson.version = version;
	writeJSON(packageJsonFullPath, packJson);

	const jsr = readJSON(jsrPath);
	jsr.version = version;
	writeJSON(jsrPath, jsr);
}

function readJSON(filePath) {
	const content = fs.readFileSync(filePath, "utf-8");
	return JSON.parse(content);
}

function writeJSON(filePath, data) {
	const content = JSON.stringify(data, null, "\t") + "\n";
	fs.writeFileSync(filePath, content, "utf-8");
}
