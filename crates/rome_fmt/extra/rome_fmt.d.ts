export * from "./_rome_fmt.js";

export type Filename =
    | "file.js"
    | "file.mjs"
    | "file.cjs"
    | "file.ts"
    | "file.mts"
    | "file.cts"
    | "file.jsx"
    | "file.mjsx"
    | "file.cjsx"
    | "file.tsx"
    | "file.mtsx"
    | "file.ctsx"
    | "file.d.ts"
    | "file.d.mts"
    | "file.d.cts"
    | "file.d.mtsx"
    | "file.d.ctsx"
    | (string & {});

import init, { type Config } from "./_rome_fmt.js";
export default init;

/**
 * @param {string} src - The content to format.
 * @param {string} filename - The filename to use for determining the language.
 * @param {Config} config - The Config to use for formatting.
 * @returns {string} The formatted content.
 */
export function format(src: string, filename?: Filename, config?: Config): string;
