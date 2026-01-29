#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const crateDir = process.argv[2];
if (!crateDir) {
	console.error("Usage: node patch.mjs <crate-dir>");
	process.exit(1);
}

const crateName = path.basename(crateDir);
const pkgDir = path.resolve(crateDir, "pkg");
const dtsPath = path.join(pkgDir, `${crateName}.d.ts`);

// Use crate-specific doc.d.ts if available, otherwise fall back to default
const crateDocPath = path.resolve(crateDir, "extra/doc.d.ts");
const defaultDocPath = fileURLToPath(import.meta.resolve("./doc.d.ts"));
const docPath = fs.existsSync(crateDocPath) ? crateDocPath : defaultDocPath;

if (fs.existsSync(docPath)) {
	const docText = fs.readFileSync(docPath, { encoding: "utf-8" });
	prependTextToFile(docText + "\n", dtsPath);
}

function prependTextToFile(text, filePath) {
	const originalContent = fs.readFileSync(filePath, { encoding: "utf-8" });
	const newContent = text + originalContent;
	fs.writeFileSync(filePath, newContent);
}
