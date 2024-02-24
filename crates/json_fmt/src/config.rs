use biome_formatter::LineWidthFromIntError;
use biome_json_formatter::context::JsonFormatOptions;
use common::LayoutConfig;
use serde::Deserialize;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"export type Config = LayoutConfig;"#;

#[derive(Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct JsonConfig(LayoutConfig);

impl From<LayoutConfig> for JsonConfig {
    fn from(config: LayoutConfig) -> Self {
        Self(config)
    }
}

impl JsonConfig {
    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.0 = self.0.with_line_width(line_width);
        self
    }
}

impl TryFrom<JsonConfig> for JsonFormatOptions {
    type Error = String;

    fn try_from(value: JsonConfig) -> Result<Self, Self::Error> {
        let mut option = JsonFormatOptions::default();

        if let Some(indent_style) = value.0.indent_style() {
            option = option.with_indent_style(indent_style.into());
        };

        if let Some(indent_width) = value.0.indent_width() {
            option = option.with_indent_width(indent_width.into());
        }

        if let Some(line_ending) = value.0.line_ending() {
            option = option.with_line_ending(line_ending.into());
        };

        if let Some(line_width) = value.0.line_width() {
            let line_width =
                line_width.try_into().map_err(|e: LineWidthFromIntError| e.to_string())?;

            option = option.with_line_width(line_width);
        };

        Ok(option)
    }
}
