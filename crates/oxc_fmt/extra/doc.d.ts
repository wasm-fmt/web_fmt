/**
 * WASM formatter for JavaScript/TypeScript using OXC (experimental).
 *
 * @example
 * ```ts
 * import { format } from "@wasm-fmt/oxc_fmt";
 *
 * const input = "const x=1";
 * const output = format(input, "index.js");
 * ```
 *
 * @module
 */

import type { Config } from "./options.d.ts";
