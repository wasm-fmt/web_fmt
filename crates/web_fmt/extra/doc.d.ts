/**
 * WASM formatter for web development (HTML/CSS/JS/JSON/GraphQL).
 *
 * @example
 * ```ts
 * import { format } from "@wasm-fmt/web_fmt";
 *
 * const input = "<div>  hello  </div>";
 * const output = format(input, "index.html");
 * ```
 *
 * @module
 */

import type { Config } from "./options.d.ts";
import type { Config as GraphqlConfig } from "./graphql_options.d.ts";
import type { Config as JsonConfig } from "./json_options.d.ts";
import type { Config as MarkupConfig } from "./markup_options.d.ts";
import type { Config as ScriptConfig } from "./biome_options.d.ts";
import type { Config as StyleConfig } from "./malva_options.d.ts";
