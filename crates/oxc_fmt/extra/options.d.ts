import type { LayoutConfig } from "./layout_config.d.ts";

/**
 * Configuration options for OXC formatter.
 *
 * @see {@link https://oxc.rs/docs/guide/usage/formatter/config.html}
 * @see {@link https://prettier.io/docs/options}
 */
export interface Config extends LayoutConfig {
	/** The style for quotes. Defaults to "double". */
	singleQuote?: boolean;

	/** The style for JSX quotes. Defaults to "double". */
	jsxSingleQuote?: boolean;

	/** When properties in objects are quoted. Defaults to "as-needed". */
	quoteProps?: "as-needed" | "consistent" | "preserve";

	/** Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all". */
	trailingComma?: "all" | "es5" | "none";

	/** Whether the formatter prints semicolons for all statements. Defaults to true. */
	semi?: boolean;

	/** Whether to add non-necessary parentheses to arrow functions. Defaults to "always". */
	arrowParens?: "always" | "avoid";

	/** Whether to insert spaces around brackets in object literals. Defaults to true. */
	bracketSpacing?: boolean;

	/** Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line. Defaults to false. */
	bracketSameLine?: boolean;

	/** Attribute position style. Defaults to false. */
	singleAttributePerLine?: boolean;

	/** Whether to expand object and array literals to multiple lines. Defaults to "auto". */
	objectWrap?: "preserve" | "collapse";

	/** Controls the position of operators in binary expressions. Defaults to "end". */
	experimentalOperatorPosition?: "start" | "end";

	/** Try prettier's new ternary formatting before it becomes the default behavior. Defaults to false. */
	experimentalTernaries?: boolean;

	/** Enable formatting for embedded languages (e.g., CSS, SQL, GraphQL) within template literals. Defaults to "auto". */
	embeddedLanguageFormatting?: "auto" | "off";
}
