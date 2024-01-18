#!/usr/bin/env node
import process from "node:process";
import path from "node:path";
import fs from "node:fs";

const pkg_path = path.resolve(process.cwd(), process.argv[2]);
const pkg_text = fs.readFileSync(pkg_path, { encoding: "utf-8" });
const pkg_json = JSON.parse(pkg_text);

delete pkg_json.files;

pkg_json.main = pkg_json.module;
pkg_json.type = "module";
pkg_json.publishConfig = {
	access: "public",
};
pkg_json.exports = {
	".": {
		types: "./biome_fmt.d.ts",
		node: "./biome_fmt_node.js",
		default: "./biome_fmt.js",
	},
	"./vite": {
		types: "./biome_fmt.d.ts",
		default: "./biome_fmt_vite.js",
	},
	"./package.json": "./package.json",
	"./*": "./*",
};

fs.writeFileSync(pkg_path, JSON.stringify(pkg_json, null, 4));
