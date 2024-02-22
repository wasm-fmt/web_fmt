use biome_formatter::{IndentStyle as BiomeIndentStyle, LineWidthFromIntError};
use biome_json_formatter::context::JsonFormatOptions;
use serde::Deserialize;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config {
    indent_style?: "tab" | "space";
    indent_width?: number;
    line_ending?: "lf" | "crlf" | "cr";
    line_width?: number;
}"#;

#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct JsonConfig {
    indent_style: Option<IndentStyle>,
    indent_width: Option<u8>,
    line_ending: Option<String>,
    line_width: Option<u16>,
}

impl JsonConfig {
    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = Some(indent_style);
        self
    }

    pub fn with_indent_width(mut self, indent_width: u8) -> Self {
        self.indent_width = Some(indent_width);
        self
    }

    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.line_width = Some(line_width);
        self
    }

    pub fn indent_style(&self) -> Option<IndentStyle> {
        self.indent_style
    }

    pub fn indent_width(&self) -> Option<u8> {
        self.indent_width
    }

    pub fn line_width(&self) -> Option<u16> {
        self.line_width
    }
}

impl TryFrom<JsonConfig> for JsonFormatOptions {
    type Error = String;

    fn try_from(value: JsonConfig) -> Result<Self, Self::Error> {
        let mut option = JsonFormatOptions::default();

        if let Some(indent_style) = value.indent_style {
            option = option.with_indent_style(indent_style.into());
        };

        if let Some(indent_width) = value.indent_width {
            option = option.with_indent_width(indent_width.into());
        }

        if let Some(line_ending) = value.line_ending {
            let line_ending = line_ending.parse()?;

            option = option.with_line_ending(line_ending);
        };

        if let Some(line_width) = value.line_width {
            let line_width =
                line_width.try_into().map_err(|e: LineWidthFromIntError| e.to_string())?;

            option = option.with_line_width(line_width);
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
