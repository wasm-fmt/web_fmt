import type * as InitOutput from "./biome_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./biome_fmt_web.js";
export { format } from "./biome_fmt.js";
export type * from "./biome_fmt.js";
