/**
 * Layout configuration options shared across all formatters.
 */
export interface LayoutConfig {
	/** Indentation style. Defaults to "space". */
	indentStyle?: "tab" | "space";

	/** Number of spaces per indentation level. Defaults to 2. */
	indentWidth?: number;

	/** Maximum line width before wrapping. Defaults to 80. */
	lineWidth?: number;

	/** Line ending style. Defaults to "lf". */
	lineEnding?: "lf" | "crlf";
}
