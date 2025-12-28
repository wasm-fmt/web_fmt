use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"export type JsonConfig = LayoutConfig;"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsonConfig")]
    pub type Config;
}

#[wasm_bindgen]
pub fn format_json(src: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    json_fmt::format_json_with_config(src, config)
}
