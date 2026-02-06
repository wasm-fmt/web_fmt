pub mod config;

use config::MalvaConfig;
#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

/// Formats the given CSS/SCSS/Sass/Less code with the provided Configuration.
#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(js_name = format)]
pub fn format_style(
    #[wasm_bindgen(param_description = "The CSS/SCSS/Sass/Less code to format")] src: &str,
    #[wasm_bindgen(
        param_description = "The filename to determine the syntax (e.g., .css, .scss, .sass, .less)"
    )]
    filename: &str,
    #[wasm_bindgen(param_description = "Optional formatter config")] config: Option<Config>,
) -> Result<String, String> {
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
