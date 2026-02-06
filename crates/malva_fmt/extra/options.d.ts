import type { LayoutConfig } from "./layout_config.d.ts";

/**
 * Configuration options for CSS/SCSS/SASS/LESS formatter (malva).
 *
 * @see {@link https://github.com/g-plane/malva/blob/main/docs/config.md}
 */
export interface Config extends LayoutConfig {
	/** Case style for hex colors. Defaults to "lower". */
	hexCase?: "ignore" | "lower" | "upper";

	/** Length style for hex colors. */
	hexColorLength?: "short" | "long";

	/** Quote style for strings. Defaults to "preferDouble". */
	quotes?: "alwaysDouble" | "alwaysSingle" | "preferDouble" | "preferSingle";

	/** Quote style for attribute selectors. Defaults to inherit from quotes. */
	"attrSelector.quotes"?: "alwaysDouble" | "alwaysSingle" | "preferDouble" | "preferSingle";

	/** Operator line break position. Defaults to "after". */
	operatorLinebreak?: "before" | "after";

	/** Block selector line break style. Defaults to "consistent". */
	blockSelectorLinebreak?: "always" | "consistent" | "wrap";

	/** Whether to omit leading zero in numbers. Defaults to false. */
	omitNumberLeadingZero?: boolean;

	/** Whether to add trailing commas. Defaults to false. */
	trailingComma?: boolean;

	/** Whether to format comments. Defaults to false. */
	formatComments?: boolean;

	/** Whether to align comments. Defaults to true. */
	alignComments?: boolean;

	/** Whether to add line breaks in pseudo parentheses. Defaults to false. */
	linebreakInPseudoParens?: boolean;

	/** Declaration order style. */
	declarationOrder?: "alphabetical" | "smacss" | "concentric";

	/** How to group declarations when ordering. Defaults to "nonDeclaration". */
	declarationOrderGroupBy?: "nonDeclaration" | "nonDeclarationAndEmptyLine";

	/** Threshold for single line blocks. */
	singleLineBlockThreshold?: number;

	/** Notation for keyframe selectors. */
	keyframeSelectorNotation?: "keyword" | "percentage";

	/** Quote style for attribute values. Defaults to "always". */
	attrValueQuotes?: "always" | "ignore";

	/** Whether to prefer single line. Defaults to false. */
	preferSingleLine?: boolean;

	/** Whether to prefer single line for selectors. Defaults to inherit from preferSingleLine. */
	"selectors.preferSingleLine"?: boolean;

	/** Whether to prefer single line for function arguments. Defaults to inherit from preferSingleLine. */
	"functionArgs.preferSingleLine"?: boolean;

	/** Whether to prefer single line for SASS content at-rule. Defaults to inherit from preferSingleLine. */
	"sassContentAtRule.preferSingleLine"?: boolean;

	/** Whether to prefer single line for SASS include at-rule. Defaults to inherit from preferSingleLine. */
	"sassIncludeAtRule.preferSingleLine"?: boolean;

	/** Whether to prefer single line for SASS maps. Defaults to inherit from preferSingleLine. */
	"sassMap.preferSingleLine"?: boolean;

	/** Whether to prefer single line for SASS module config. Defaults to inherit from preferSingleLine. */
	"sassModuleConfig.preferSingleLine"?: boolean;

	/** Whether to prefer single line for SASS params. Defaults to inherit from preferSingleLine. */
	"sassParams.preferSingleLine"?: boolean;

	/** Whether to prefer single line for LESS import options. Defaults to inherit from preferSingleLine. */
	"lessImportOptions.preferSingleLine"?: boolean;

	/** Whether to prefer single line for LESS mixin args. Defaults to inherit from preferSingleLine. */
	"lessMixinArgs.preferSingleLine"?: boolean;

	/** Whether to prefer single line for LESS mixin params. Defaults to inherit from preferSingleLine. */
	"lessMixinParams.preferSingleLine"?: boolean;

	/** Whether to format single-line top-level declarations on one line. Defaults to false. */
	singleLineTopLevelDeclarations?: boolean;

	/** Directive for selector override comments. Defaults to "malva-selector-override". */
	selectorOverrideCommentDirective?: string;

	/** Directive to ignore formatting for a section. Defaults to "malva-ignore". */
	ignoreCommentDirective?: string;

	/** Directive to ignore formatting for entire file. Defaults to "malva-ignore-file". */
	ignoreFileCommentDirective?: string;
}
