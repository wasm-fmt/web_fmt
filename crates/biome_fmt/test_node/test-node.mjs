import init, { format } from "../pkg/rome_fmt.js";
import { test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

await init();

/**
 * @param {string} dir
 * @returns {Generator<string>}
 */
async function* walk(dir) {
	for await (const d of await fs.opendir(dir)) {
		const entry = path.join(dir, d.name);
		if (d.isDirectory()) yield* walk(entry);
		else if (d.isFile()) yield entry;
	}
}

const test_root = fileURLToPath(new URL("../test_data", import.meta.url));

for await (const input_path of walk(test_root)) {
	const ext = path.extname(input_path);

	switch (ext) {
		case ".js":
		case ".jsx":
		case ".ts":
		case ".tsx":
			break;

		default:
			continue;
	}

	const test_name = path.relative(test_root, input_path);
	const [input, expected] = await Promise.all([
		fs.readFile(input_path, { encoding: "utf-8" }),
		fs.readFile(input_path + ".snap", { encoding: "utf-8" }),
	]);

	const actual = format(input, input_path);

	test(test_name, () => {
		assert.equal(actual, expected);
	});
}
