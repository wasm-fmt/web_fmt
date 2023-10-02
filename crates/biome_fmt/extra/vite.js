import init from "./biome_fmt.js";
import wasm from "./biome_fmt_bg.wasm?url";

export default function __wbg_init(input = wasm) {
    return init(input);
}

export * from "./biome_fmt.js";