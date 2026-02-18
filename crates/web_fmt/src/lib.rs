mod format_graphql;
mod format_json;
mod format_markup;
mod format_script;
mod format_style;

use std::path::Path;

use common::LayoutConfig;
use format_markup::EmbeddedCodeFormatter as MarkupEmbeddedCodeFormatter;
use markup_fmt::FormatMarkup;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type JSConfig;
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
struct Config {
    markup: Option<markup_fmt::config::MarkupConfig>,
    script: Option<biome_fmt::BiomeConfig>,
    style: Option<malva_fmt::config::MalvaConfig>,
    json: Option<LayoutConfig>,
    graphql: Option<graphql_fmt::config::GraphqlConfig>,
}

/// Formats the given code based on the file extension with the provided Configuration.
/// Supports JavaScript, TypeScript, JSX, TSX, CSS, SCSS, Sass, Less, HTML, Vue, Svelte, Astro, JSON, JSONC, GraphQL.
#[wasm_bindgen]
pub fn format(
    #[wasm_bindgen(param_description = "The code to format")] src: &str,
    #[wasm_bindgen(
        param_description = "The filename to determine the language (e.g., .js, .ts, .css, .html, .json, .graphql)"
    )]
    filename: &str,
    #[wasm_bindgen(param_description = "Optional formatter config for different languages")] config: Option<JSConfig>,
) -> Result<String, String> {
    let default_config: ConfigDefault = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config: Config = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let extension = Path::new(&filename).extension().ok_or("expected extension")?;

    let script_config = format_script::produce_script_config(
        config.script,
        &default_config.script.fill_empty_with(&default_config.default),
    );
    let style_config = format_style::produce_style_config(
        config.style,
        &default_config.style.fill_empty_with(&default_config.default),
    );
    let markup_config = format_markup::produce_markup_config(
        config.markup,
        &default_config.markup.fill_empty_with(&default_config.default),
    );
    let json_config = config.json.unwrap_or_default().fill_empty_with(&default_config.default);
    let graphql_config = format_graphql::produce_graphql_config(
        config.graphql,
        &default_config.graphql.fill_empty_with(&default_config.default),
    );

    match extension.as_encoded_bytes() {
        b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx"
        | b"ctsx" => biome_fmt::format_script_with_config(src, filename, script_config),
        b"css" | b"scss" | b"sass" | b"less" => {
            malva_fmt::format_style_with_config(src, filename, style_config)
        }
        b"html" | b"vue" | b"svelte" | b"astro" | b"jinja" | b"jinja2" | b"twig" => {
            let formatter = MarkupEmbeddedCodeFormatter {
                filename: filename.to_string(),
                markup_config: markup_config.clone(),
                script_config,
                style_config,
                json_config,
            };

            FormatMarkup::new(src, filename)
                .config(markup_config.into())
                .embed_formatter(formatter)
                .format()
        }
        b"json" | b"jsonc" => json_fmt::format_json_with_config(src, json_config.into()),
        b"graphql" | b"gql" => graphql_fmt::format_graphql_with_config(src, graphql_config),
        _ => Err(format!("unsupported file extension: {filename}")),
    }
}

#[derive(Deserialize, Default)]
#[serde()]
struct ConfigDefault {
    #[serde(flatten, default)]
    default: LayoutConfig,
    #[serde(default)]
    markup: LayoutConfig,
    #[serde(default)]
    script: LayoutConfig,
    #[serde(default)]
    style: LayoutConfig,
    #[serde(default)]
    graphql: LayoutConfig,
}
