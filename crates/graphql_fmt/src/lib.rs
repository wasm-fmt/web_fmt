pub mod config;

use config::GraphqlConfig;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface Config extends LayoutConfig {
	/**
	 *  See {@link https://pretty-graphql.netlify.app/}
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
pub fn format_graphql(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config: config::GraphqlConfig = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    format_graphql_with_config(src, filename, config)
}

pub fn format_graphql_with_config(
    src: &str,
    _filename: &str,
    config: GraphqlConfig,
) -> Result<String, String> {
    pretty_graphql::format_text(src, &config.into()).map_err(|e| e.to_string())
}
