import fs from "node:fs/promises";
import initAsync from "./markup_fmt.js";

const wasm = new URL("./markup_fmt_bg.wasm", import.meta.url);

export default function __wbg_init(init = { module_or_path: fs.readFile(wasm) }) {
	return initAsync(init);
}

export * from "./markup_fmt.js";
