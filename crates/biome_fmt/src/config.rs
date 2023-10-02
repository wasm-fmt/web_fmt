use biome_formatter::{IndentStyle as BiomeIndentStyle, LineWidthFromIntError};
use biome_js_formatter::context::JsFormatOptions;
use biome_js_syntax::JsFileSource;

use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config {
    indent_style?: "tab" | "space";
    indent_width?: number;
    line_width?: number;
    quote_style?: "single" | "double";
    quote_properties?: "as-needed" | "preserve";
    trailing_comma?: "es5" | "all" | "none";
    semicolons?: "as-needed" | "always";
}"#;

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) struct Config {
    /// The indent style.
    indent_style: Option<IndentStyle>,

    /// The indent width.
    indent_width: Option<u8>,

    /// What's the max width of a line. Defaults to 80.
    line_width: Option<u16>,

    /// The style for quotes. Defaults to double.
    quote_style: Option<String>,

    /// When properties in objects are quoted. Defaults to as-needed.
    quote_properties: Option<String>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    trailing_comma: Option<String>,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of [ASI](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-automatic-semicolon-insertion).
    semicolons: Option<String>,

    #[serde(skip)]
    source_type: Option<JsFileSource>,
}

impl Config {
    pub fn with_source_type(mut self, source_type: JsFileSource) -> Self {
        self.source_type = Some(source_type);
        self
    }
}

impl TryFrom<Config> for JsFormatOptions {
    type Error = String;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let source_type = value.source_type.expect("source_type is required");

        let mut option = JsFormatOptions::new(source_type);

        if let Some(indent_style) = value.indent_style {
            option = option.with_indent_style(indent_style.into());
        };

        if let Some(indent_width) = value.indent_width {
            option = option.with_indent_width(indent_width.into());
        }

        if let Some(line_width) = value.line_width {
            let line_width =
                line_width.try_into().map_err(|e: LineWidthFromIntError| e.to_string())?;

            option = option.with_line_width(line_width);
        };

        if let Some(quote_style) = value.quote_style {
            let quote_style = quote_style.parse()?;

            option = option.with_quote_style(quote_style);
        };

        if let Some(quote_properties) = value.quote_properties {
            let quote_properties = quote_properties.parse()?;

            option = option.with_quote_properties(quote_properties);
        };

        if let Some(trailing_comma) = value.trailing_comma {
            let trailing_comma = trailing_comma.parse()?;

            option = option.with_trailing_comma(trailing_comma);
        };

        if let Some(semicolons) = value.semicolons {
            let semicolons = semicolons.parse()?;

            option = option.with_semicolons(semicolons);
        };

        Ok(option)
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum IndentStyle {
    Tab,
    Space,
}

impl From<IndentStyle> for BiomeIndentStyle {
    fn from(value: IndentStyle) -> Self {
        match value {
            IndentStyle::Tab => Self::Tab,
            IndentStyle::Space => Self::Space,
        }
    }
}
