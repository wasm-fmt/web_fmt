use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "GraphqlConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface GraphqlConfig extends LayoutConfig {
	/**
	 *  See {@link https://pretty-graphql.netlify.app/}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
pub fn format_graphql(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    graphql_fmt::format_graphql_with_config(src, filename, config)
}

pub(crate) fn produce_graphql_config(
    config: Option<graphql_fmt::config::GraphqlConfig>,
    default_layout: &LayoutConfig,
) -> graphql_fmt::config::GraphqlConfig {
    config.unwrap_or_default().fill_empty_layout_with(default_layout)
}
