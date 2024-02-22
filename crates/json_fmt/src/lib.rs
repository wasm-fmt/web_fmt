mod config;

use biome_formatter::Printed;
use biome_json_formatter::format_node;
use biome_json_parser::{parse_json, JsonParserOptions};

use config::JsonConfig;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(skip_typescript)]
pub fn format(src: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    format_json_with_config(src, config)
}

pub fn format_json_with_config(src: &str, config: JsonConfig) -> Result<String, String> {
    let options = JsonParserOptions::default().with_allow_comments().with_allow_trailing_commas();
    let parse = parse_json(src, options);

    let options = config.try_into()?;

    format_node(options, &parse.syntax())
        .map_err(|e| e.to_string())?
        .print()
        .map(Printed::into_code)
        .map_err(|e| e.to_string())
}
