import initAsync from "./markup_fmt.js";
import wasm from "./markup_fmt_bg.wasm?url";

export default function __wbg_init(input = { module_or_path: wasm }) {
	return initAsync(input);
}

export * from "./markup_fmt.js";
