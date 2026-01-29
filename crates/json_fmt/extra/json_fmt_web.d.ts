/**
 * WASM formatter for JSON using Biome.
 *
 * Import this module and call init function before using.
 *
 * @example
 * ```ts
 * import init, { format } from "@wasm-fmt/json_fmt/web";
 *
 * await init();
 *
 * const input = '{"hello": "world"}';
 * const output = format(input);
 * ```
 *
 * @module
 */
import type * as InitOutput from "./json_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

/**
 * Input types for asynchronous WASM initialization.
 * Can be a URL/path to fetch, a Response object, raw bytes, or a pre-compiled WebAssembly.Module.
 */
export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

/**
 * Input types for synchronous WASM initialization.
 * Must be raw bytes (BufferSource) or a pre-compiled WebAssembly.Module.
 */
export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Initializes the WASM module asynchronously.
 * @param init_input - Optional URL/path to the WASM file, or any valid InitInput
 */
export default function initAsync(init_input?: InitInput | Promise<InitInput>): Promise<InitOutput>;
/**
 * Initializes the WASM module synchronously.
 * @param module_or_buffer - The WASM module or buffer source
 */
export declare function initSync(module_or_buffer: SyncInitInput): InitOutput;

export * from "./json_fmt.d.ts";
