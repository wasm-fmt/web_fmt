use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsonConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"export type JsonConfig = LayoutConfig;"#;

#[wasm_bindgen]
pub fn format_json(src: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    json_fmt::format_json_with_config(src, config)
}

pub(crate) fn produce_json_config(
    config: Option<LayoutConfig>,
    config_default: &LayoutConfig,
    global_fallback: &LayoutConfig,
) -> LayoutConfig {
    let default = LayoutConfig::default()
        .with_indent_style(common::IndentStyle::Space)
        .with_indent_width(2)
        .with_line_width(80)
        .with_line_ending(common::LineEnding::Lf);

    config
        .unwrap_or_default()
        .fill_empty_with(config_default)
        .fill_empty_with(global_fallback)
        .fill_empty_with(&default)
}
