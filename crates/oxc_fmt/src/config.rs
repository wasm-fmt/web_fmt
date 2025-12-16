use common::LayoutConfig;
use oxc_formatter::{
    BracketSameLine, BracketSpacing, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
};
use serde::Deserialize;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config extends LayoutConfig {
    quote_style?: "double" | "single";
    jsx_quote_style?: "double" | "single";
    quote_properties?: "preserve" | "as-needed" | "consistent";
    trailing_commas?: "es5" | "all" | "none";
    semicolons?: "always" | "as-needed";
    arrow_parentheses?: "always" | "as-needed";
    bracket_spacing?: boolean;
    bracket_same_line?: boolean;
}"#;

#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OxcConfig {
    #[serde(flatten)]
    layout: LayoutConfig,

    /// The style for quotes. Defaults to double.
    quote_style: Option<String>,

    /// The style for JSX quotes. Defaults to double.
    jsx_quote_style: Option<String>,

    /// When properties in objects are quoted. Defaults to as-needed.
    quote_properties: Option<String>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    trailing_commas: Option<String>,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of ASI.
    semicolons: Option<String>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    arrow_parentheses: Option<String>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    bracket_spacing: Option<bool>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    bracket_same_line: Option<bool>,
}

impl OxcConfig {
    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.layout = self.layout.with_line_width(line_width);
        self
    }

    pub fn fill_empty_layout_with(mut self, layout: &LayoutConfig) -> Self {
        self.layout = self.layout.fill_empty_with(layout);
        self
    }
}

impl TryFrom<OxcConfig> for FormatOptions {
    type Error = String;

    fn try_from(value: OxcConfig) -> Result<Self, Self::Error> {
        let mut options = FormatOptions::new();

        if let Some(indent_style) = value.layout.indent_style() {
            options.indent_style = match indent_style {
                common::IndentStyle::Tab => IndentStyle::Tab,
                common::IndentStyle::Space => IndentStyle::Space,
            };
        }

        if let Some(indent_width) = value.layout.indent_width() {
            options.indent_width =
                IndentWidth::try_from(indent_width).map_err(|e| e.to_string())?;
        }

        if let Some(line_width) = value.layout.line_width() {
            options.line_width = LineWidth::try_from(line_width).map_err(|e| e.to_string())?;
        }

        if let Some(line_ending) = value.layout.line_ending() {
            options.line_ending = match line_ending {
                common::LineEnding::Lf => LineEnding::Lf,
                common::LineEnding::Crlf => LineEnding::Crlf,
            };
        }

        if let Some(quote_style) = value.quote_style {
            options.quote_style = quote_style.parse().map_err(|e: &str| e.to_string())?;
        }

        if let Some(jsx_quote_style) = value.jsx_quote_style {
            options.jsx_quote_style = jsx_quote_style.parse().map_err(|e: &str| e.to_string())?;
        }

        if let Some(quote_properties) = value.quote_properties {
            options.quote_properties = quote_properties.parse().map_err(|e: &str| e.to_string())?;
        }

        if let Some(trailing_commas) = value.trailing_commas {
            options.trailing_commas = trailing_commas.parse().map_err(|e: &str| e.to_string())?;
        }

        if let Some(semicolons) = value.semicolons {
            options.semicolons = semicolons.parse().map_err(|e: &str| e.to_string())?;
        }

        if let Some(arrow_parentheses) = value.arrow_parentheses {
            options.arrow_parentheses =
                arrow_parentheses.parse().map_err(|e: &str| e.to_string())?;
        }

        if let Some(bracket_spacing) = value.bracket_spacing {
            options.bracket_spacing = BracketSpacing::from(bracket_spacing);
        }

        if let Some(bracket_same_line) = value.bracket_same_line {
            options.bracket_same_line = BracketSameLine::from(bracket_same_line);
        }

        Ok(options)
    }
}
