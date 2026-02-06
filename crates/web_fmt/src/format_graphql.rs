use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "GraphqlConfig")]
    pub type Config;
}

/// Formats the given GraphQL code with the provided Configuration.
#[wasm_bindgen]
pub fn format_graphql(
    #[wasm_bindgen(param_description = "The GraphQL code to format")] src: &str,
    #[wasm_bindgen(param_description = "Optional formatter config")] config: Option<Config>,
) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    graphql_fmt::format_graphql_with_config(src, config)
}

pub(crate) fn produce_graphql_config(
    config: Option<graphql_fmt::config::GraphqlConfig>,
    default_layout: &LayoutConfig,
) -> graphql_fmt::config::GraphqlConfig {
    config.unwrap_or_default().fill_empty_layout_with(default_layout)
}
