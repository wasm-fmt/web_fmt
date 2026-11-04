import type * as InitOutput from "./graphql_fmt_bg.wasm.d.ts";
declare type InitOutput = typeof InitOutput;

export default function initAsync(): Promise<InitOutput>;

export { initSync, type SyncInitInput } from "./graphql_fmt_web.js";
export { format } from "./graphql_fmt.js";
