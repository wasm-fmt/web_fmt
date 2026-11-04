import type * as InitOutput from "./web_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./web_fmt_web.js";
export { format } from "./web_fmt.js";
