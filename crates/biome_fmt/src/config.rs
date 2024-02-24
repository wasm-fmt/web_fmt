use biome_formatter::{IndentStyle as BiomeIndentStyle, LineWidthFromIntError};
use biome_js_formatter::context::JsFormatOptions;
use biome_js_syntax::JsFileSource;

use common::LayoutConfig;
use serde::Deserialize;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config extends LayoutConfig {
    quote_style?: "double" | "single";
    jsx_quote_style?: "double" | "single";
    quote_properties?: "preserve" | "as-needed";
    trailing_comma?: "es5" | "all" | "none";
    semicolons?: "always" | "as-needed";
    arrow_parentheses?: "always" | "as-needed";
    bracket_spacing?: boolean;
    bracket_same_line?: boolean;
}"#;

#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BiomeConfig {
    #[serde(flatten)]
    layout: LayoutConfig,

    /// The style for quotes. Defaults to double.
    quote_style: Option<String>,

    /// The style for JSX quotes. Defaults to double.
    jsx_quote_style: Option<String>,

    /// When properties in objects are quoted. Defaults to as-needed.
    quote_properties: Option<String>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    trailing_comma: Option<String>,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of [ASI](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-automatic-semicolon-insertion).
    semicolons: Option<String>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    arrow_parentheses: Option<String>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    bracket_spacing: Option<bool>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    bracket_same_line: Option<bool>,

    #[serde(skip)]
    source_type: Option<JsFileSource>,
}

impl BiomeConfig {
    pub fn with_source_type(mut self, source_type: JsFileSource) -> Self {
        self.source_type = Some(source_type);
        self
    }

    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.layout = self.layout.with_line_width(line_width);
        self
    }

    pub fn fill_empty_layout_with(mut self, layout: &LayoutConfig) -> Self {
        self.layout = self.layout.fill_empty_with(layout);

        self
    }
}

impl TryFrom<BiomeConfig> for JsFormatOptions {
    type Error = String;

    fn try_from(value: BiomeConfig) -> Result<Self, Self::Error> {
        let source_type = value.source_type.expect("source_type is required");

        let mut option = JsFormatOptions::new(source_type);

        if let Some(indent_style) = value.layout.indent_style() {
            option = option.with_indent_style(indent_style.into());
        };

        if let Some(indent_width) = value.layout.indent_width() {
            option = option.with_indent_width(indent_width.into());
        }

        if let Some(line_width) = value.layout.line_width() {
            let line_width =
                line_width.try_into().map_err(|e: LineWidthFromIntError| e.to_string())?;

            option = option.with_line_width(line_width);
        };

        if let Some(line_ending) = value.layout.line_ending() {
            option = option.with_line_ending(line_ending.into());
        };

        if let Some(quote_style) = value.quote_style {
            let quote_style = quote_style.parse()?;

            option = option.with_quote_style(quote_style);
        };

        if let Some(jsx_quote_style) = value.jsx_quote_style {
            let jsx_quote_style = jsx_quote_style.parse()?;

            option = option.with_jsx_quote_style(jsx_quote_style);
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

        if let Some(arrow_parentheses) = value.arrow_parentheses {
            let arrow_parentheses = arrow_parentheses.parse()?;

            option = option.with_arrow_parentheses(arrow_parentheses);
        };

        if let Some(bracket_spacing) = value.bracket_spacing {
            option = option.with_bracket_spacing(bracket_spacing.into());
        };

        if let Some(bracket_same_line) = value.bracket_same_line {
            option = option.with_bracket_same_line(bracket_same_line.into());
        };

        Ok(option)
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
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
