import init from "./rome_fmt.js";
import wasm from "./rome_fmt_bg.wasm?url";

export default function __wbg_init(input = wasm) {
    return init(input);
}

export * from "./rome_fmt.js";