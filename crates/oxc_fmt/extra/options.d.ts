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

	/** Sort import statements. By default disabled. */
	experimentalSortImports?: SortImportsOptions;

	/** Enable Tailwind CSS class sorting in JSX class/className attributes. Defaults to None (disabled). */
	experimentalTailwindcss?: TailwindcssOptions;
}

/** Options for sorting import statements. */
export interface SortImportsOptions {
	/** Partition imports by newlines. Default is `false`. */
	partitionByNewline?: boolean;

	/** Partition imports by comments. Default is `false`. */
	partitionByComment?: boolean;

	/** Sort side effects imports. Default is `false`. */
	sortSideEffects?: boolean;

	/** Sort order (asc or desc). Default is ascending ("asc"). */
	order?: "asc" | "desc";

	/** Ignore case when sorting. Default is `true`. */
	ignoreCase?: boolean;

	/** Whether to insert blank lines between different import groups. Default is `true`. */
	newlinesBetween?: boolean;

	/** Prefixes for internal imports. Defaults to `["~/", "@/"]`. */
	internalPattern?: string[];

	/**
	 * Groups configuration for organizing imports.
	 * Each array element represents a group, and multiple group names in the same array are treated as one.
	 * Accepts `string`, `string[]`, or `{ newlinesBetween: boolean }` marker objects.
	 * Marker objects override the global `newlinesBetween` setting for the boundary between the adjacent groups.
	 */
	groups?: SortGroupItem[];

	/** Define custom groups for matching specific imports. */
	customGroups?: CustomGroupDefinition[];
}

/** A group item in the sort imports configuration.
 * Can be a single group name, an array of group names, or a newlinesBetween marker.
 */
export type SortGroupItem = string | string[] | { newlinesBetween: boolean };

/** Import selector for custom group matching. */
export type ImportSelector = "value" | "type" | "default" | "namespace" | "side-effect";

/** Import modifier for custom group matching. */
export type ImportModifier = "type" | "value" | "default" | "namespace" | "named" | "side-effect";

/** Custom group definition for sort imports. */
export interface CustomGroupDefinition {
	/** The name of the custom group. */
	groupName: string;

	/** Patterns to match import source names against. */
	elementNamePattern: string[];

	/** Selector to filter imports by type. */
	selector?: ImportSelector;

	/** Modifiers to further filter imports. */
	modifiers?: ImportModifier[];
}

/** Options for Tailwind CSS class sorting. */
export interface TailwindcssOptions {
	/** Path to your Tailwind CSS configuration file (v3). Default: `"./tailwind.config.js"` */
	config?: string;

	/** Path to your Tailwind CSS stylesheet (v4). Example: `"./src/app.css"` */
	stylesheet?: string;

	/** List of custom function names that contain Tailwind CSS classes. Example: `["clsx", "cn", "cva", "tw"]` */
	functions?: string[];

	/** List of additional attributes to sort (beyond `class` and `className`). Example: `["myClassProp", ":class"]` */
	attributes?: string[];

	/** Preserve whitespace around classes. Default: `false` */
	preserveWhitespace?: boolean;

	/** Preserve duplicate classes. Default: `false` */
	preserveDuplicates?: boolean;
}
