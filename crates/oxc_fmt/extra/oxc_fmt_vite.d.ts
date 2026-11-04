import type * as InitOutput from "./oxc_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./oxc_fmt_web.js";
export { format } from "./oxc_fmt.js";
export type * from "./oxc_fmt.js";
