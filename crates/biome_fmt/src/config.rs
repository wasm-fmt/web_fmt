use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, Expand, IndentStyle as BiomeIndentStyle,
    IndentWidthFromIntError, LineWidthFromIntError, QuoteStyle,
};
use biome_js_formatter::context::{
    ArrowParentheses, JsFormatOptions, OperatorLinebreak, QuoteProperties, Semicolons,
    TrailingCommas,
};
use biome_js_syntax::JsFileSource;

use common::LayoutConfig;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Deserialize, Default, Clone)]
pub struct BiomeConfig {
    #[serde(flatten)]
    layout: LayoutConfig,
    #[serde(flatten)]
    language: LanguageOptions,

    #[serde(skip)]
    source_type: Option<JsFileSource>,
}

#[derive(Deserialize, Default, Clone)]
pub struct LanguageOptions {
    /// The style for quotes. Defaults to double.
    #[serde(alias = "quoteStyle", deserialize_with = "deserialize_option_from_str")]
    quote_style: Option<QuoteStyle>,

    /// The style for JSX quotes. Defaults to double.
    #[serde(alias = "jsxQuoteStyle", deserialize_with = "deserialize_option_from_str")]
    jsx_quote_style: Option<QuoteStyle>,

    /// When properties in objects are quoted. Defaults to as-needed.
    #[serde(alias = "quoteProperties", deserialize_with = "deserialize_option_from_str")]
    quote_properties: Option<QuoteProperties>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[serde(alias = "trailingComma", deserialize_with = "deserialize_option_from_str")]
    trailing_comma: Option<TrailingCommas>,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of [ASI](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-automatic-semicolon-insertion).
    #[serde(deserialize_with = "deserialize_option_from_str")]
    semicolons: Option<Semicolons>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[serde(alias = "arrowParentheses", deserialize_with = "deserialize_option_from_str")]
    arrow_parentheses: Option<ArrowParentheses>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[serde(alias = "bracketSpacing", deserialize_with = "deserialize_option_from_str")]
    bracket_spacing: Option<BracketSpacing>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[serde(alias = "bracketSameLine", deserialize_with = "deserialize_option_from_str")]
    bracket_same_line: Option<BracketSameLine>,

    /// Attribute position style. By default auto.
    #[serde(alias = "attributePosition", deserialize_with = "deserialize_option_from_str")]
    attribute_position: Option<AttributePosition>,

    /// Whether to expand object and array literals to multiple lines. Defaults to "auto".
    #[serde(deserialize_with = "deserialize_option_from_str")]
    expand: Option<Expand>,

    /// When formatting binary expressions, whether to break the line before or after the operator. Defaults to "after".
    #[serde(alias = "operatorLinebreak", deserialize_with = "deserialize_option_from_str")]
    operator_linebreak: Option<OperatorLinebreak>,
}

impl BiomeConfig {
    #[must_use]
    pub fn with_source_type(mut self, source_type: JsFileSource) -> Self {
        self.source_type = Some(source_type);
        self
    }

    #[must_use]
    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.layout = self.layout.with_line_width(line_width);
        self
    }

    #[must_use]
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
            option = option.with_indent_style(indent_style.as_str().parse()?);
        }

        if let Some(indent_width) = value.layout.indent_width() {
            let indent_width =
                indent_width.try_into().map_err(|e: IndentWidthFromIntError| e.to_string())?;
            option = option.with_indent_width(indent_width);
        }

        if let Some(line_width) = value.layout.line_width() {
            let line_width =
                line_width.try_into().map_err(|e: LineWidthFromIntError| e.to_string())?;

            option = option.with_line_width(line_width);
        }

        if let Some(line_ending) = value.layout.line_ending() {
            option = option.with_line_ending(line_ending.as_str().parse()?);
        }

        if let Some(quote_style) = value.language.quote_style {
            option = option.with_quote_style(quote_style);
        }

        if let Some(jsx_quote_style) = value.language.jsx_quote_style {
            option = option.with_jsx_quote_style(jsx_quote_style);
        }

        if let Some(quote_properties) = value.language.quote_properties {
            option = option.with_quote_properties(quote_properties);
        }

        if let Some(trailing_comma) = value.language.trailing_comma {
            option = option.with_trailing_commas(trailing_comma);
        }

        if let Some(semicolons) = value.language.semicolons {
            option = option.with_semicolons(semicolons);
        }

        if let Some(arrow_parentheses) = value.language.arrow_parentheses {
            option = option.with_arrow_parentheses(arrow_parentheses);
        }

        if let Some(bracket_spacing) = value.language.bracket_spacing {
            option = option.with_bracket_spacing(bracket_spacing);
        }

        if let Some(bracket_same_line) = value.language.bracket_same_line {
            option = option.with_bracket_same_line(bracket_same_line);
        }

        if let Some(attribute_position) = value.language.attribute_position {
            option = option.with_attribute_position(attribute_position);
        }

        if let Some(expand) = value.language.expand {
            option = option.with_expand(expand);
        }

        if let Some(operator_linebreak) = value.language.operator_linebreak {
            option = option.with_operator_linebreak(operator_linebreak);
        }

        Ok(option)
    }
}

fn deserialize_option_from_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map(Some).map_err(serde::de::Error::custom)
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
