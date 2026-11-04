import type * as InitOutput from "./malva_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./malva_fmt_web.js";
export { format } from "./malva_fmt.js";
