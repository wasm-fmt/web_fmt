/* @ts-self-types="./malva_fmt_web.d.ts" */
import init from "./malva_fmt_bg.wasm?init";
import * as import_bg from "./malva_fmt_bg.js";
const { __wbg_set_wasm, format, ...wasmImport } = import_bg;

let wasm, wasmModule;

function getImports() {
	return {
		__proto__: null,
		"./malva_fmt_bg.js": wasmImport,
	};
}

function finalize_init(instance, module) {
	wasm = instance.exports;
	wasmModule = module;
	__wbg_set_wasm(wasm);
	return wasm;
}

export default async function initAsync() {
	if (wasm !== void 0) return wasm;
	const instance = await init(getImports());
	return finalize_init(instance);
}

export function initSync(module) {
	if (wasm !== void 0) return wasm;

	if (!(module instanceof WebAssembly.Module)) {
		module = new WebAssembly.Module(module);
	}
	const instance = new WebAssembly.Instance(module, getImports());
	return finalize_init(instance, module);
}

export { format };
