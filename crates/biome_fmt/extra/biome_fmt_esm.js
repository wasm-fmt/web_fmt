/* @ts-self-types="./biome_fmt.d.ts" */
// prettier-ignore
import source wasmModule from "./biome_fmt_bg.wasm";

import * as import_bg from "./biome_fmt_bg.js";
const { __wbg_set_wasm, format, formatRange, ...wasmImport } = import_bg;

function getImports() {
	return {
		__proto__: null,
		"./biome_fmt_bg.js": wasmImport,
	};
}

const instance = new WebAssembly.Instance(wasmModule, getImports());

/**
 * @import * as WASM from "./biome_fmt_bg.wasm"
 */

/**
 * @type {WASM}
 */
const wasm = instance.exports;
__wbg_set_wasm(wasm);

export { format, formatRange };
