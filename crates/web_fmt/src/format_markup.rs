use std::path::Path;

use biome_fmt::BiomeConfig;
use markup_fmt::{self, Language};

use wasm_bindgen::prelude::*;

use crate::{format_style::format_style_with_config, ConfigLayout};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MarkupConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface MarkupConfig {
	indent_style?: "tab" | "space";
	indent_width?: number;
	line_width?: number;
	/** 
	 *  See {@link https://github.com/g-plane/markup_fmt/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
pub fn format_markup(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let default_config: ConfigLayout = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|e| e.to_string())?;

    let config = produce_markup_config(config, &default_config, &default_config);

    format_markup_with_config(src, filename, config)
}

pub fn format_markup_with_config(
    src: &str,
    filename: &str,
    config: markup_fmt::config::FormatOptions,
) -> Result<String, String> {
    let language = detect_language(filename).unwrap_or(markup_fmt::Language::Html);

    markup_fmt::format_text(src, language, &config, |filename, src, width| {
        let extension =
            filename.extension().map(|x| x.to_ascii_lowercase()).ok_or("expected extension")?;
        let filename: &str = filename.to_str().ok_or("expected filename")?;

        match extension.as_encoded_bytes() {
            b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx"
            | b"ctsx" => biome_fmt::format_script_with_config(
                src,
                filename,
                BiomeConfig::default().with_line_width(width as u16),
            )
            .map(Into::into),
            b"css" | b"scss" | b"sass" | b"less" => format_style_with_config(
                src,
                filename,
                malva::config::FormatOptions {
                    layout: malva::config::LayoutOptions {
                        print_width: width,
                        ..Default::default()
                    },
                    language: Default::default(),
                },
            )
            .map(Into::into),
            _ => Ok(src.into()),
        }
    })
    .map_err(|e| format!("{:?}", e))
}

pub(crate) fn detect_language(path: impl AsRef<Path>) -> Option<Language> {
    match path.as_ref().extension().map(|x| x.to_ascii_lowercase())?.as_encoded_bytes() {
        b"html" => Some(Language::Html),
        b"vue" => Some(Language::Vue),
        b"svelte" => Some(Language::Svelte),
        b"astro" => Some(Language::Astro),
        b"jinja" | b"jinja2" | b"twig" => Some(Language::Jinja),
        _ => Some(Language::Html),
    }
}

pub(crate) fn produce_markup_config(
    base_config: Option<markup_fmt::config::FormatOptions>,
    layout_config: &ConfigLayout,
    default_config: &ConfigLayout,
) -> markup_fmt::config::FormatOptions {
    let use_tabs: bool =
        layout_config.indent_style.or(default_config.indent_style).unwrap_or_default().into();

    let indent_width: u8 = layout_config.indent_width.or(default_config.indent_width).unwrap_or(2);

    let line_width: u16 = layout_config.line_width.or(default_config.line_width).unwrap_or(80);

    let mut config = base_config.unwrap_or_default();

    config.layout.use_tabs = use_tabs;
    config.layout.indent_width = indent_width as usize;
    config.layout.print_width = line_width as usize;

    config
}
