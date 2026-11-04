import type * as InitOutput from "./markup_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./markup_fmt_web.js";
export { format } from "./markup_fmt.js";
export type * from "./markup_fmt.js";
