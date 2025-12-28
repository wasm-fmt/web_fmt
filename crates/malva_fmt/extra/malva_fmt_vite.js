import initAsync from "./malva_fmt.js";
import wasm from "./malva_fmt_bg.wasm?url";

export default function __wbg_init(input = { module_or_path: wasm }) {
	return initAsync(input);
}

export * from "./malva_fmt.js";
