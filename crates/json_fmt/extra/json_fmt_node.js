import fs from "node:fs/promises";
import initAsync from "./json_fmt.js";

const wasm = new URL("./json_fmt_bg.wasm", import.meta.url);

export default function __wbg_init(init = { module_or_path: fs.readFile(wasm) }) {
	return initAsync(init);
}

export * from "./json_fmt.js";
