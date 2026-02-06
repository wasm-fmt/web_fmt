import type { LayoutConfig } from "./layout_config.d.ts";

/**
 * Configuration options for Markup formatter (HTML/Vue/Svelte/Astro).
 *
 * @see {@link https://github.com/g-plane/markup_fmt/blob/main/docs/config.md}
 */
export interface Config extends LayoutConfig {
	/** Whether to use single quotes for attribute values. Defaults to "double". */
	quotes?: "double" | "single";

	/** Whether to format comments. Defaults to false. */
	formatComments?: boolean;

	/** Whether to indent script content. Defaults to false. */
	scriptIndent?: boolean;

	/** Whether to indent script content in HTML. Defaults to inherit from scriptIndent. */
	"html.scriptIndent"?: boolean;

	/** Whether to indent script content in Vue. Defaults to inherit from scriptIndent. */
	"vue.scriptIndent"?: boolean;

	/** Whether to indent script content in Svelte. Defaults to inherit from scriptIndent. */
	"svelte.scriptIndent"?: boolean;

	/** Whether to indent script content in Astro. Defaults to inherit from scriptIndent. */
	"astro.scriptIndent"?: boolean;

	/** Whether to indent style content. Defaults to false. */
	styleIndent?: boolean;

	/** Whether to indent style content in HTML. Defaults to inherit from styleIndent. */
	"html.styleIndent"?: boolean;

	/** Whether to indent style content in Vue. Defaults to inherit from styleIndent. */
	"vue.styleIndent"?: boolean;

	/** Whether to indent style content in Svelte. Defaults to inherit from styleIndent. */
	"svelte.styleIndent"?: boolean;

	/** Whether to indent style content in Astro. Defaults to inherit from styleIndent. */
	"astro.styleIndent"?: boolean;

	/** Whether to place closing bracket on the same line. Defaults to false. */
	closingBracketSameLine?: boolean;

	/** Line break behavior for empty closing tags. Defaults to "fit". */
	closingTagLineBreakForEmpty?: "always" | "fit" | "never";

	/** Maximum number of attributes per line. Defaults to null (no limit). */
	maxAttrsPerLine?: number;

	/** Whether to prefer placing all attributes on a single line. Defaults to false. */
	preferAttrsSingleLine?: boolean;

	/** Whether to place a single attribute on the same line as the tag. Defaults to true. */
	singleAttrSameLine?: boolean;

	/** Whether normal HTML elements should be self-closing. */
	"html.normal.selfClosing"?: boolean;

	/** Whether void HTML elements should be self-closing. */
	"html.void.selfClosing"?: boolean;

	/** Whether components should be self-closing. */
	"component.selfClosing"?: boolean;

	/** Whether SVG elements should be self-closing. */
	"svg.selfClosing"?: boolean;

	/** Whether MathML elements should be self-closing. */
	"mathml.selfClosing"?: boolean;

	/** Whitespace sensitivity mode. Defaults to "css". */
	whitespaceSensitivity?: "css" | "strict" | "ignore";

	/** Whitespace sensitivity mode for components. Defaults to inherit from whitespaceSensitivity. */
	"component.whitespaceSensitivity"?: "css" | "strict" | "ignore";

	/** Case style for DOCTYPE keyword. Defaults to "upper". */
	doctypeKeywordCase?: "ignore" | "upper" | "lower";

	/** Style for v-bind directive in Vue. */
	vBindStyle?: "short" | "long";

	/** Style for v-on directive in Vue. */
	vOnStyle?: "short" | "long";

	/** Delimiter style for v-for directive in Vue. */
	vForDelimiterStyle?: "in" | "of";

	/** Style for v-slot directive in Vue. */
	vSlotStyle?: "short" | "long" | "vSlot";

	/** Style for v-slot in components. Defaults to inherit from vSlotStyle. */
	"component.vSlotStyle"?: "short" | "long" | "vSlot";

	/** Style for default v-slot. Defaults to inherit from vSlotStyle. */
	"default.vSlotStyle"?: "short" | "long" | "vSlot";

	/** Style for named v-slot. Defaults to inherit from vSlotStyle. */
	"named.vSlotStyle"?: "short" | "long" | "vSlot";

	/** Whether to use shorthand for v-bind with same name in Vue. */
	vBindSameNameShortHand?: boolean;

	/** Component naming case style in Vue. Defaults to "ignore". */
	vueComponentCase?: "ignore" | "pascalCase" | "kebabCase";

	/** Whether to use strict attribute checking in Svelte. Defaults to false. */
	strictSvelteAttr?: boolean;

	/** Whether to use attribute shorthand in Svelte. */
	svelteAttrShorthand?: boolean;

	/** Whether to use directive shorthand in Svelte. */
	svelteDirectiveShorthand?: boolean;

	/** Whether to use attribute shorthand in Astro. */
	astroAttrShorthand?: boolean;

	/** Whether to place Angular control flow on same line. Defaults to true. */
	angularNextControlFlowSameLine?: boolean;

	/** Script formatter to use. */
	scriptFormatter?: "dprint" | "biome";

	/** Directive to ignore formatting for a section. Defaults to "markup-fmt-ignore". */
	ignoreCommentDirective?: string;

	/** Directive to ignore formatting for entire file. Defaults to "markup-fmt-ignore-file". */
	ignoreFileCommentDirective?: string;
}
