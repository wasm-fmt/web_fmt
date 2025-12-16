import initAsync from "./oxc_fmt.js";
import wasm from "./oxc_fmt_bg.wasm?url";

export default function __wbg_init(input = { module_or_path: wasm }) {
	return initAsync(input);
}

export * from "./oxc_fmt.js";
