import type { LayoutConfig } from "./layout_config.d.ts";

/**
 * Configuration options for Biome formatter.
 *
 * @see {@link https://biomejs.dev/reference/configuration/#formatter}
 */
export interface Config extends LayoutConfig {
	/** The style for quotes. Defaults to "double". */
	quoteStyle?: "double" | "single";

	/** The style for JSX quotes. Defaults to "double". */
	jsxQuoteStyle?: "double" | "single";

	/** When properties in objects are quoted. Defaults to "as-needed". */
	quoteProperties?: "preserve" | "as-needed";

	/** Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all". */
	trailingComma?: "es5" | "all" | "none";

	/** Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of ASI. Defaults to "always". */
	semicolons?: "always" | "as-needed";

	/** Whether to add non-necessary parentheses to arrow functions. Defaults to "always". */
	arrowParentheses?: "always" | "as-needed";

	/** Whether to insert spaces around brackets in object literals. Defaults to true. */
	bracketSpacing?: boolean;

	/** Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false. */
	bracketSameLine?: boolean;

	/** Attribute position style. By default "auto". */
	attributePosition?: "auto" | "multiline";

	/** Whether to expand object and array literals to multiple lines. Defaults to "auto". */
	expand?: "always" | "never" | "auto";

	/** When formatting binary expressions, whether to break the line before or after the operator. Defaults to "after". */
	operatorLinebreak?: "before" | "after";
}
