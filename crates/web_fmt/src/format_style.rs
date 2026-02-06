use common::LayoutConfig;
use malva_fmt::config::MalvaConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "StyleConfig")]
    pub type Config;
}

/// Formats the given CSS/SCSS/Sass/Less code with the provided Configuration.
#[wasm_bindgen]
pub fn format_style(
    #[wasm_bindgen(param_description = "The CSS/SCSS/Sass/Less code to format")] src: &str,
    #[wasm_bindgen(
        param_description = "The filename to determine the syntax (e.g., .css, .scss, .sass, .less)"
    )]
    filename: &str,
    #[wasm_bindgen(param_description = "Optional formatter config")] config: Option<Config>,
) -> Result<String, String> {
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
