use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsonConfig")]
    pub type Config;
}

/// Formats the given JSON code with the provided Configuration.
#[wasm_bindgen]
pub fn format_json(
    #[wasm_bindgen(param_description = "The JSON code to format")] src: &str,
    #[wasm_bindgen(param_description = "Optional formatter config")] config: Option<Config>,
) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    json_fmt::format_json_with_config(src, config)
}
