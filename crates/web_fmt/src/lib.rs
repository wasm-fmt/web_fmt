mod format_json;
mod format_markup;
mod format_script;
mod format_style;

use std::path::Path;

use common::LayoutConfig;
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
    markup: Option<markup_fmt::config::FormatOptions>,
    script: Option<biome_fmt::BiomeConfig>,
    style: Option<malva::config::FormatOptions>,
    json: Option<LayoutConfig>,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config extends LayoutConfig {
	markup?: MarkupConfig;
	script?: ScriptConfig;
	style?: StyleConfig;
	json?: JsonConfig;
}"#;

#[wasm_bindgen]
pub fn format(src: &str, filename: &str, config: Option<JSConfig>) -> Result<String, String> {
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
        &default_config.script,
        &default_config.default,
    );
    let style_config = format_style::produce_style_config(
        config.style,
        &default_config.style,
        &default_config.default,
    );
    let markup_config = format_markup::produce_markup_config(
        config.markup,
        &default_config.markup,
        &default_config.default,
    );
    let json_config = format_json::produce_json_config(
        config.json,
        &default_config.json,
        &default_config.default,
    );

    match extension.as_encoded_bytes() {
        b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx"
        | b"ctsx" => biome_fmt::format_script_with_config(src, filename, script_config),
        b"css" | b"scss" | b"sass" | b"less" => {
            format_style::format_style_with_config(src, filename, style_config)
        }
        b"html" | b"vue" | b"svelte" | b"astro" | b"jinja" | b"jinja2" | b"twig" => {
            format_markup::format_markup_with_config(
                src,
                filename,
                markup_config,
                style_config,
                script_config,
                json_config,
            )
        }
        b"json" | b"jsonc" => json_fmt::format_json_with_config(src, json_config.into()),
        _ => Err(format!("unsupported file extension: {}", filename)),
    }
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
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
    json: LayoutConfig,
}
