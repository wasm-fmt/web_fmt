import initAsync from "./graphql_fmt.js";
import wasm from "./graphql_fmt_bg.wasm?url";

export default function __wbg_init(input = { module_or_path: wasm }) {
	return initAsync(input);
}

export * from "./graphql_fmt.js";
