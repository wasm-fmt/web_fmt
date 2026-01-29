/* @ts-self-types="./markup_fmt_web.d.ts" */
import * as import_bg from "./markup_fmt_bg.js";
const { __wbg_set_wasm, format, ...wasmImport } = import_bg;

function getImports() {
	return {
		__proto__: null,
		"./markup_fmt_bg.js": wasmImport,
	};
}

let wasm, wasmModule;

async function load(module, imports) {
	if (typeof Response === "function" && module instanceof Response) {
		if (typeof WebAssembly.instantiateStreaming === "function") {
			try {
				return await WebAssembly.instantiateStreaming(module, imports);
			} catch (e) {
				const validResponse = module.ok && expectedResponseType(module.type);

				if (validResponse && module.headers.get("Content-Type") !== "application/wasm") {
					console.warn(
						"`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
						e,
					);
				} else {
					throw e;
				}
			}
		}

		const bytes = await module.arrayBuffer();
		return await WebAssembly.instantiate(bytes, imports);
	} else {
		const instance = await WebAssembly.instantiate(module, imports);

		if (instance instanceof WebAssembly.Instance) {
			return { instance, module };
		} else {
			return instance;
		}
	}

	function expectedResponseType(type) {
		switch (type) {
			case "basic":
			case "cors":
			case "default":
				return true;
		}
		return false;
	}
}

function finalize_init(instance, module) {
	wasm = instance.exports;
	wasmModule = module;
	__wbg_set_wasm(wasm);
	return wasm;
}

export function initSync(module_or_buffer) {
	if (wasm !== void 0) return wasm;

	if (!(module_or_buffer instanceof WebAssembly.Module)) {
		module_or_buffer = new WebAssembly.Module(module_or_buffer);
	}
	const instance = new WebAssembly.Instance(module_or_buffer, getImports());
	return finalize_init(instance, module_or_buffer);
}

export default async function initAsync(init_input) {
	if (wasm !== void 0) return wasm;

	if (init_input === void 0) {
		init_input = new URL("markup_fmt_bg.wasm", import.meta.url);
	}

	if (
		typeof init_input === "string" ||
		(typeof Request === "function" && init_input instanceof Request) ||
		(typeof URL === "function" && init_input instanceof URL)
	) {
		init_input = fetch(init_input);
	}

	const { instance, module } = await load(await init_input, getImports());

	return finalize_init(instance, module);
}

export { format };
