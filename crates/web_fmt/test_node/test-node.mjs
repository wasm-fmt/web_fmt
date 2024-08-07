import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import init, { format } from "../pkg/web_fmt_node.js";

await init();

const test_root = fileURLToPath(new URL("../test_data", import.meta.url));

for await (const dirent of await fs.opendir(test_root, { recursive: true })) {
	if (!dirent.isFile()) {
		continue;
	}

	if (dirent.name.startsWith(".")) {
		continue;
	}

	const input_path = path.join(dirent.path, dirent.name);
	const ext = path.extname(input_path);

	switch (ext) {
		case ".html":
		case ".vue":
		case ".json":
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

	test(test_name, () => {
		const actual = format(input, input_path);
		assert.equal(actual, expected);
	});
}
