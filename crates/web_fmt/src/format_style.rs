use common::LayoutConfig;
use malva_fmt::config::MalvaConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface StyleConfig extends LayoutConfig {
	/**
	 *  See {@link https://github.com/g-plane/malva/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "StyleConfig")]
    pub type Config;
}

#[wasm_bindgen]
pub fn format_style(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    malva_fmt::format_style_with_config(src, filename, config)
}

pub(crate) fn produce_style_config(
    base_config: Option<MalvaConfig>,
    default_layout: &LayoutConfig,
) -> MalvaConfig {
    base_config.unwrap_or_default().fill_empty_layout_with(default_layout)
}
