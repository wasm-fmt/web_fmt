use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ScriptConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface ScriptConfig extends LayoutConfig {
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
    config: Option<biome_fmt::BiomeConfig>,
    config_default: &LayoutConfig,
    global_fallback: &LayoutConfig,
) -> biome_fmt::BiomeConfig {
    let default = LayoutConfig::default()
        .with_indent_style(common::IndentStyle::Space)
        .with_indent_width(2)
        .with_line_width(80)
        .with_line_ending(common::LineEnding::Lf);

    config
        .unwrap_or_default()
        .fill_empty_layout_with(config_default)
        .fill_empty_layout_with(global_fallback)
        .fill_empty_layout_with(&default)
}
