use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[cfg(feature = "script-biome")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ScriptConfig")]
    pub type Config;
}

#[cfg(feature = "script-biome")]
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

#[cfg(feature = "script-biome")]
#[wasm_bindgen]
pub fn format_script(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    biome_fmt::format_script_with_config(src, filename, config)
}

#[cfg(feature = "script-biome")]
pub(crate) fn produce_script_config(
    config: Option<biome_fmt::BiomeConfig>,
    default_layout: &LayoutConfig,
) -> biome_fmt::BiomeConfig {
    config.unwrap_or_default().fill_empty_layout_with(default_layout)
}
