use std::path::Path;

use common::LayoutConfig;

use markup_fmt_core::Hints;
use wasm_bindgen::prelude::*;

use crate::format_script;
use crate::format_style;

pub(crate) type ScriptConfig = biome_fmt::BiomeConfig;

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

    let markup_config = produce_markup_config(config, &default_config);
    let style_config = format_style::produce_style_config(None, &default_config);
    let script_config = format_script::produce_script_config(None, &default_config);
    let json_config = LayoutConfig::default().fill_empty_with(&default_config);

    FormatMarkup::new(src, filename)
        .markup(markup_config)
        .style(style_config)
        .script(script_config)
        .json(json_config)
        .format()
}

/// Builder for formatting HTML/Vue/Svelte/Astro markup.
pub struct FormatMarkup<'a> {
    src: &'a str,
    filename: &'a str,
    markup_config: markup_fmt_core::config::FormatOptions,
    style_config: malva::config::FormatOptions,
    script_config: ScriptConfig,
    json_config: LayoutConfig,
}

impl<'a> FormatMarkup<'a> {
    pub fn new(src: &'a str, filename: &'a str) -> Self {
        Self {
            src,
            filename,
            markup_config: Default::default(),
            style_config: Default::default(),
            script_config: Default::default(),
            json_config: Default::default(),
        }
    }

    pub fn markup(mut self, config: markup_fmt_core::config::FormatOptions) -> Self {
        self.markup_config = config;
        self
    }

    pub fn style(mut self, config: malva::config::FormatOptions) -> Self {
        self.style_config = config;
        self
    }

    pub fn script(mut self, config: ScriptConfig) -> Self {
        self.script_config = config;
        self
    }

    pub fn json(mut self, config: LayoutConfig) -> Self {
        self.json_config = config;
        self
    }

    pub fn format(self) -> Result<String, String> {
        let language = detect_language(self.filename).unwrap_or(markup_fmt_core::Language::Html);
        let Self { src, filename, markup_config, style_config, script_config, json_config } = self;

        markup_fmt_core::format_text(src, language, &markup_config, |src, hints| {
            format_embedded(
                src,
                filename,
                hints,
                &markup_config,
                &style_config,
                &script_config,
                &json_config,
            )
        })
        .map_err(|e| format!("{:?}", e))
    }
}

fn format_embedded<'a>(
    src: &'a str,
    filename: &str,
    Hints { print_width, attr, ext, .. }: Hints,
    markup_config: &markup_fmt_core::config::FormatOptions,
    style_config: &malva::config::FormatOptions,
    script_config: &ScriptConfig,
    json_config: &LayoutConfig,
) -> Result<std::borrow::Cow<'a, str>, String> {
    match ext.as_bytes() {
        b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx" => {
            format_script_embedded(src, filename, ext, print_width, script_config, style_config)
        }
        b"css" | b"scss" | b"sass" | b"less" => {
            format_style_embedded(src, filename, print_width, attr, markup_config, style_config)
        }
        b"json" | b"jsonc" => json_fmt::format_json_with_config(
            src,
            json_config.clone().with_line_width(print_width as u16).into(),
        )
        .map(Into::into),
        _ => Ok(src.into()),
    }
}

fn format_script_embedded<'a>(
    src: &'a str,
    filename: &str,
    _ext: &str,
    print_width: usize,
    script_config: &ScriptConfig,
    _style_config: &malva::config::FormatOptions,
) -> Result<std::borrow::Cow<'a, str>, String> {
    biome_fmt::format_script_with_config(
        src,
        filename,
        script_config.clone().with_line_width(print_width as u16),
    )
    .map(Into::into)
}

fn format_style_embedded<'a>(
    src: &'a str,
    filename: &str,
    print_width: usize,
    attr: bool,
    markup_config: &markup_fmt_core::config::FormatOptions,
    style_config: &malva::config::FormatOptions,
) -> Result<std::borrow::Cow<'a, str>, String> {
    let mut style_config = style_config.clone();
    if attr {
        if let markup_fmt_core::config::Quotes::Double = markup_config.language.quotes {
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

pub(crate) fn detect_language(path: impl AsRef<Path>) -> Option<markup_fmt_core::Language> {
    match path.as_ref().extension().map(|x| x.to_ascii_lowercase())?.as_encoded_bytes() {
        b"html" => Some(markup_fmt_core::Language::Html),
        b"vue" => Some(markup_fmt_core::Language::Vue),
        b"svelte" => Some(markup_fmt_core::Language::Svelte),
        b"astro" => Some(markup_fmt_core::Language::Astro),
        b"jinja" | b"jinja2" | b"twig" => Some(markup_fmt_core::Language::Jinja),
        _ => Some(markup_fmt_core::Language::Html),
    }
}

pub(crate) fn produce_markup_config(
    base_config: Option<markup_fmt_core::config::FormatOptions>,
    config_default: &LayoutConfig,
) -> markup_fmt_core::config::FormatOptions {
    let mut config = base_config.unwrap_or_default();

    if let Some(indent_style) = config_default.indent_style() {
        config.layout.use_tabs = indent_style.use_tabs();
    }

    if let Some(indent_width) = config_default.indent_width() {
        config.layout.indent_width = indent_width as usize;
    }

    if let Some(line_width) = config_default.line_width() {
        config.layout.print_width = line_width as usize;
    }

    if let Some(line_endings) = config_default.line_ending() {
        config.layout.line_break = match line_endings {
            common::LineEnding::Lf => markup_fmt_core::config::LineBreak::Lf,
            common::LineEnding::Crlf => markup_fmt_core::config::LineBreak::Crlf,
        };
    }

    config
}
