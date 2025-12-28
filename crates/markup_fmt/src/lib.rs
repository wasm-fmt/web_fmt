pub mod config;

pub use markup_fmt_core::Hints;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config extends LayoutConfig {
	/**
	 *  See {@link https://github.com/g-plane/markup_fmt/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(js_name = format)]
pub fn format_markup(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config: markup_fmt_core::config::FormatOptions = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    FormatMarkup::new(src, filename).config(config).format()
}

/// Trait for formatting embedded code.
pub trait EmbeddedFormatter {
    fn format(&self, src: &str, hints: Hints) -> Result<String, String>;
}

/// Default no-op formatter when none is provided.
pub struct NoneFormatter;

impl EmbeddedFormatter for NoneFormatter {
    fn format(&self, src: &str, _hints: Hints) -> Result<String, String> {
        Ok(src.to_string())
    }
}

/// Builder for formatting HTML/Vue/Svelte/Astro markup.
pub struct FormatMarkup<'a, F = NoneFormatter> {
    src: &'a str,
    filename: &'a str,
    config: markup_fmt_core::config::FormatOptions,
    embed_formatter: F,
}

impl<'a> FormatMarkup<'a, NoneFormatter> {
    pub fn new(src: &'a str, filename: &'a str) -> Self {
        Self { src, filename, config: Default::default(), embed_formatter: NoneFormatter }
    }

    /// Set formatter for embedded code (script, style, json, etc.).
    /// The formatter receives source code and formatting hints (extension, print_width, etc.)
    /// This transforms the builder to use the provided formatter.
    pub fn embed_formatter<F2: EmbeddedFormatter>(self, formatter: F2) -> FormatMarkup<'a, F2> {
        FormatMarkup {
            src: self.src,
            filename: self.filename,
            config: self.config,
            embed_formatter: formatter,
        }
    }
}

impl<'a, F> FormatMarkup<'a, F> {
    pub fn config(mut self, config: markup_fmt_core::config::FormatOptions) -> Self {
        self.config = config;
        self
    }
}

impl<'a, F: EmbeddedFormatter> FormatMarkup<'a, F> {
    pub fn format(self) -> Result<String, String> {
        let language = markup_fmt_core::detect_language(self.filename)
            .unwrap_or(markup_fmt_core::Language::Html);
        let Self { src, config: markup_config, embed_formatter, .. } = self;

        markup_fmt_core::format_text(src, language, &markup_config, |embed_src, hints| {
            format_embedded(embed_src, hints, &embed_formatter)
        })
        .map_err(|e| format!("{:?}", e))
    }
}

fn format_embedded<'a, F: EmbeddedFormatter>(
    src: &'a str,
    hints: Hints,
    embed_formatter: &F,
) -> Result<std::borrow::Cow<'a, str>, String> {
    embed_formatter.format(src, hints).map(Into::into)
}
