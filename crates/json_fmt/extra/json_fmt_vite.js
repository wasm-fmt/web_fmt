import initAsync from "./json_fmt.js";
import wasm from "./json_fmt_bg.wasm?url";

export default function __wbg_init(input = { module_or_path: wasm }) {
	return initAsync(input);
}

export * from "./json_fmt.js";
