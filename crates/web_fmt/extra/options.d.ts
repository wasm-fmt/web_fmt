import type { LayoutConfig } from "./layout_config.d.ts";
import type { Config as MarkupConfig } from "./markup_options.d.ts";
import type { Config as ScriptConfig } from "./biome_options.d.ts";
import type { Config as StyleConfig } from "./malva_options.d.ts";
import type { Config as JsonConfig } from "./json_options.d.ts";
import type { Config as GraphqlConfig } from "./graphql_options.d.ts";

/**
 * Main configuration interface for Web formatter.
 */
export interface Config extends LayoutConfig {
	/** Markup (HTML/Vue/Svelte/Astro) formatter configuration. */
	markup?: MarkupConfig;

	/** Script (JavaScript/TypeScript) formatter configuration. */
	script?: ScriptConfig;

	/** Style (CSS/SCSS/SASS/LESS) formatter configuration. */
	style?: StyleConfig;

	/** JSON formatter configuration (uses LayoutConfig only). */
	json?: JsonConfig;

	/** GraphQL formatter configuration. */
	graphql?: GraphqlConfig;
}
