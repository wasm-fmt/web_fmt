use std::path::Path;

use common::LayoutConfig;

use markup_fmt::Hints;
use wasm_bindgen::prelude::*;

use crate::format_style;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MarkupConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface MarkupConfig extends LayoutConfig {
	/**
	 *  See {@link https://github.com/g-plane/markup_fmt/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
pub fn format_markup(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let default_config: LayoutConfig = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|e| e.to_string())?;

    let markup_config = produce_markup_config(config, &default_config, &default_config);
    let style_config = format_style::produce_style_config(None, &default_config, &default_config);
    let script_config = biome_fmt::BiomeConfig::default().fill_empty_layout_with(&default_config);
    let json_config = LayoutConfig::default().fill_empty_with(&default_config);

    format_markup_with_config(
        src,
        filename,
        markup_config,
        style_config,
        script_config,
        json_config,
    )
}

pub fn format_markup_with_config(
    src: &str,
    filename: &str,
    markup_config: markup_fmt::config::FormatOptions,
    style_config: malva::config::FormatOptions,
    script_config: biome_fmt::BiomeConfig,
    json_config: LayoutConfig,
) -> Result<String, String> {
    let language = detect_language(filename).unwrap_or(markup_fmt::Language::Html);

    markup_fmt::format_text(
        src,
        language,
        &markup_config,
        |src, Hints { print_width, attr, ext, .. }| match ext.as_bytes() {
            b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx" => {
                biome_fmt::format_script_with_config(
                    src,
                    filename,
                    script_config.clone().with_line_width(print_width as u16),
                )
                .map(Into::into)
            }
            b"css" | b"scss" | b"sass" | b"less" => {
                let mut style_config = style_config.clone();
                if attr {
                    if let markup_fmt::config::Quotes::Double = markup_config.language.quotes {
                        style_config.language.quotes = malva::config::Quotes::AlwaysSingle;
                    } else {
                        style_config.language.quotes = malva::config::Quotes::AlwaysDouble;
                    }
                    style_config.language.single_line_top_level_declarations = true;
                }
                format_style::format_style_with_config(
                    src,
                    filename,
                    malva::config::FormatOptions {
                        layout: malva::config::LayoutOptions { print_width, ..style_config.layout },
                        language: style_config.language,
                    },
                )
                .map(Into::into)
            }
            b"json" | b"jsonc" => json_fmt::format_json_with_config(
                src,
                json_config.clone().with_line_width(print_width as u16).into(),
            )
            .map(Into::into),
            _ => Ok(src.into()),
        },
    )
    .map_err(|e| format!("{:?}", e))
}

pub(crate) fn detect_language(path: impl AsRef<Path>) -> Option<markup_fmt::Language> {
    match path.as_ref().extension().map(|x| x.to_ascii_lowercase())?.as_encoded_bytes() {
        b"html" => Some(markup_fmt::Language::Html),
        b"vue" => Some(markup_fmt::Language::Vue),
        b"svelte" => Some(markup_fmt::Language::Svelte),
        b"astro" => Some(markup_fmt::Language::Astro),
        b"jinja" | b"jinja2" | b"twig" => Some(markup_fmt::Language::Jinja),
        _ => Some(markup_fmt::Language::Html),
    }
}

pub(crate) fn produce_markup_config(
    base_config: Option<markup_fmt::config::FormatOptions>,
    config_default: &LayoutConfig,
    global_fallback: &LayoutConfig,
) -> markup_fmt::config::FormatOptions {
    let mut config = base_config.unwrap_or_default();

    if let Some(indent_style) = config_default.indent_style().or(global_fallback.indent_style()) {
        config.layout.use_tabs = indent_style.is_tab();
    }

    if let Some(indent_width) = config_default.indent_width().or(global_fallback.indent_width()) {
        config.layout.indent_width = indent_width as usize;
    }

    if let Some(line_width) = config_default.line_width().or(global_fallback.line_width()) {
        config.layout.print_width = line_width as usize;
    }

    if let Some(line_endings) = config_default.line_ending().or(global_fallback.line_ending()) {
        config.layout.line_break = match line_endings {
            common::LineEnding::Lf => markup_fmt::config::LineBreak::Lf,
            common::LineEnding::Crlf => markup_fmt::config::LineBreak::Crlf,
        };
    }

    config
}
