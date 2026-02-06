import type { LayoutConfig } from "./layout_config.d.ts";

/**
 * Configuration options for GraphQL formatter.
 *
 * @see {@link https://pretty-graphql.netlify.app/}
 */
export interface Config extends LayoutConfig {
	/** Print width for formatting. Defaults to 80. */
	printWidth?: number;

	/** Use tabs for indentation. Defaults to false. */
	useTabs?: boolean;

	/** Number of spaces per indentation level. Defaults to 2. */
	indentWidth?: number;

	/** Line break style. Defaults to "lf". */
	lineBreak?: "lf" | "crlf";

	/** Comma style. Defaults to "onlySingleLine". */
	comma?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for arguments. Defaults to "inherit". */
	"arguments.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for arguments definition. Defaults to "inherit". */
	"argumentsDefinition.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for directives. Defaults to "never". */
	"directives.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for enum values definition. Defaults to "never". */
	"enumValuesDefinition.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for fields definition. Defaults to "never". */
	"fieldsDefinition.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for input fields definition. Defaults to "never". */
	"inputFieldsDefinition.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for list values. Defaults to "inherit". */
	"listValue.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for object values. Defaults to "inherit". */
	"objectValue.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for schema definition. Defaults to "never". */
	"schemaDefinition.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for schema extension. Defaults to "never". */
	"schemaExtension.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for selection sets. Defaults to "never". */
	"selectionSet.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Comma style for variable definitions. Defaults to "inherit". */
	"variableDefinitions.comma"?: "always" | "never" | "noTrailing" | "onlySingleLine" | "inherit";

	/** Single line style. Defaults to "smart". */
	singleLine?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for arguments. Defaults to "inherit". */
	"arguments.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for arguments definition. Defaults to "inherit". */
	"argumentsDefinition.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for directive locations. Defaults to "inherit". */
	"directiveLocations.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for directives. Defaults to "inherit". */
	"directives.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for enum values definition. Defaults to "never". */
	"enumValuesDefinition.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for fields definition. Defaults to "never". */
	"fieldsDefinition.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for implements interfaces. Defaults to "inherit". */
	"implementsInterfaces.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for input fields definition. Defaults to "never". */
	"inputFieldsDefinition.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for list values. Defaults to "inherit". */
	"listValue.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for object values. Defaults to "inherit". */
	"objectValue.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for schema definition. Defaults to "never". */
	"schemaDefinition.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for schema extension. Defaults to "never". */
	"schemaExtension.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for selection sets. Defaults to "never". */
	"selectionSet.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for union member types. Defaults to "inherit". */
	"unionMemberTypes.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Single line style for variable definitions. Defaults to "inherit". */
	"variableDefinitions.singleLine"?: "prefer" | "smart" | "never" | "inherit";

	/** Whether to add spaces around parentheses. Defaults to false. */
	parenSpacing?: boolean;

	/** Whether to add spaces around parentheses in arguments. */
	"arguments.parenSpacing"?: boolean;

	/** Whether to add spaces around parentheses in arguments definition. */
	"argumentsDefinition.parenSpacing"?: boolean;

	/** Whether to add spaces around parentheses in variable definitions. */
	"variableDefinitions.parenSpacing"?: boolean;

	/** Whether to add spaces around brackets. Defaults to false. */
	bracketSpacing?: boolean;

	/** Whether to add spaces around braces. Defaults to true. */
	braceSpacing?: boolean;

	/** Whether to add spaces around braces in enum values definition. */
	"enumValuesDefinition.braceSpacing"?: boolean;

	/** Whether to add spaces around braces in fields definition. */
	"fieldsDefinition.braceSpacing"?: boolean;

	/** Whether to add spaces around braces in input fields definition. */
	"inputFieldsDefinition.braceSpacing"?: boolean;

	/** Whether to add spaces around braces in object values. */
	"objectValue.braceSpacing"?: boolean;

	/** Whether to add spaces around braces in schema definition. */
	"schemaDefinition.braceSpacing"?: boolean;

	/** Whether to add spaces around braces in schema extension. */
	"schemaExtension.braceSpacing"?: boolean;

	/** Whether to add spaces around braces in selection sets. */
	"selectionSet.braceSpacing"?: boolean;

	/** Whether to format comments. Defaults to false. */
	formatComments?: boolean;

	/** Directive to ignore comments. Defaults to "pretty-graphql-ignore". */
	ignoreCommentDirective?: string;

	/**
	 *@see {@link https://pretty-graphql.netlify.app/}
	 */
	[key: string]: any;
}
