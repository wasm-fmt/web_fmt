#!/usr/bin/env bun test
import { Glob } from "bun";
import { expect, test } from "bun:test";
import { basename } from "node:path";

import init, { format } from "../pkg/web_fmt_web.js";

await init();

const test_root = Bun.fileURLToPath(import.meta.resolve("../test_data"));

for await (const case_name of new Glob("**/*.{json,tsx,vue,html}").scan({ cwd: test_root, dot: true })) {
	const file_name = basename(case_name);
	if (file_name.startsWith(".")) {
		test.skip(case_name, () => {});
		continue;
	}

	const full_path = `${test_root}/${case_name}`;
	const snap_path = full_path + ".snap";

	const [input, expected] = await Promise.all([Bun.file(full_path).text(), Bun.file(snap_path).text()]);

	test(case_name, () => {
		const actual = format(input, case_name);
		expect(actual).toBe(expected);
	});
}
