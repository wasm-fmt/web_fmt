import { assertEquals } from "jsr:@std/assert";
import { walk } from "jsr:@std/fs/walk";

import init, { format } from "../pkg/biome_fmt.js";

await init();

const test_root = new URL(import.meta.resolve("../test_data"));

for await (const entry of walk(test_root, {
	includeDirs: false,
	exts: ["js", "jsx", "ts", "tsx"],
})) {
	if (entry.name.startsWith(".")) {
		continue;
	}

	const input_path = entry.path;
	const expect_path = input_path + ".snap";

	const input = Deno.readTextFileSync(input_path);
	const expected = Deno.readTextFileSync(expect_path);

	Deno.test(input_path, () => {
		const actual = format(input, entry.path);
		assertEquals(actual, expected);
	});
}
