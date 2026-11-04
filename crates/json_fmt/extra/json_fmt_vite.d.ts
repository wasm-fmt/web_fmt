import type * as InitOutput from "./json_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./json_fmt_web.js";
export { format } from "./json_fmt.js";
export type * from "./json_fmt.js";
