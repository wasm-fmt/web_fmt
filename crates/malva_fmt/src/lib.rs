pub mod config;

use config::MalvaConfig;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config extends LayoutConfig {
	/**
	 *  See {@link https://github.com/g-plane/malva/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(js_name = format)]
pub fn format_style(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config: config::MalvaConfig = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    format_style_with_config(src, filename, config)
}

pub fn format_style_with_config(
    src: &str,
    filename: &str,
    config: MalvaConfig,
) -> Result<String, String> {
    let syntax = malva::detect_syntax(filename).unwrap_or_default();

    malva::format_text(src, syntax, &config.into()).map_err(|e| e.to_string())
}
