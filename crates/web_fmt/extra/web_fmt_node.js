/* @ts-self-types="./web_fmt.d.ts" */
import { readFileSync } from "node:fs";
import * as import_bg from "./web_fmt_bg.js";
const { __wbg_set_wasm, format, ...wasmImport } = import_bg;

const wasmUrl = new URL("web_fmt_bg.wasm", import.meta.url);
const wasmBytes = readFileSync(wasmUrl);
const wasmModule = new WebAssembly.Module(wasmBytes);

function getImports() {
	return {
		__proto__: null,
		"./web_fmt_bg.js": wasmImport,
	};
}

const instance = new WebAssembly.Instance(wasmModule, getImports());
const wasm = instance.exports;
__wbg_set_wasm(wasm);

export { format };
