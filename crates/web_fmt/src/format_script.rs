use wasm_bindgen::prelude::*;

use crate::ConfigLayout;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ScriptConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface ScriptConfig {
	indent_style?: "tab" | "space";
	indent_width?: number;
	line_ending?: "lf" | "crlf" | "cr";
	line_width?: number;
	quote_style?: "double" | "single";
	jsx_quote_style?: "double" | "single";
	quote_properties?: "preserve" | "as-needed";
	trailing_comma?: "es5" | "all" | "none";
	semicolons?: "always" | "as-needed";
	arrow_parentheses?: "always" | "as-needed";
	bracket_spacing?: boolean;
	bracket_same_line?: boolean;
}"#;

#[wasm_bindgen]
pub fn format_script(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    biome_fmt::format_script_with_config(src, filename, config)
}

pub(crate) fn produce_script_config(
    base_config: Option<biome_fmt::BiomeConfig>,
    layout_config: &ConfigLayout,
    default_config: &ConfigLayout,
) -> biome_fmt::BiomeConfig {
    let indent_style =
        layout_config.indent_style.or(default_config.indent_style).unwrap_or_default().into();

    let indent_width: u8 = layout_config.indent_width.or(default_config.indent_width).unwrap_or(2);

    let line_width: u16 = layout_config.line_width.or(default_config.line_width).unwrap_or(80);

    base_config
        .unwrap_or_default()
        .with_indent_style(indent_style)
        .with_indent_width(indent_width)
        .with_line_width(line_width)
}
