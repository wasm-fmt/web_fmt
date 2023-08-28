use std::borrow::Cow;

use rome_formatter::{IndentStyle as RomeIndentStyle, LineWidthFromIntError};
use rome_js_formatter::context::JsFormatOptions;
use rome_js_syntax::SourceType;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config {
    indent_style?: "tab" | number;
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
    source_type: Option<SourceType>,
}

impl Config {
    pub fn with_source_type(mut self, source_type: SourceType) -> Self {
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
            let indent_style = match indent_style {
                IndentStyle::Tab => RomeIndentStyle::Tab,
                IndentStyle::Space(size) => RomeIndentStyle::Space(size),
            };

            option = option.with_indent_style(indent_style);
        };

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

enum IndentStyle {
    Tab,
    Space(u8),
}

impl<'de> Deserialize<'de> for IndentStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum IndentStyle<'a> {
            Tab(Cow<'a, str>),
            Space(u8),
        }

        match IndentStyle::deserialize(deserializer)? {
            IndentStyle::Space(n) => Ok(Self::Space(n)),
            IndentStyle::Tab(c) if c == "tab" => Ok(Self::Tab),
            IndentStyle::Tab(other) => {
                Err(D::Error::invalid_value(Unexpected::Str(&other), &"<number> or `tab`"))
            }
        }
    }
}
