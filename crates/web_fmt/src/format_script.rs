use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[cfg(feature = "script-biome")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ScriptConfig")]
    pub type Config;
}

/// Formats the given JavaScript/TypeScript code with the provided Configuration.
#[cfg(feature = "script-biome")]
#[wasm_bindgen]
pub fn format_script(
    #[wasm_bindgen(param_description = "The JavaScript/TypeScript code to format")] src: &str,
    #[wasm_bindgen(
        param_description = "The filename to determine the source type (e.g., .js, .ts, .jsx, .tsx)"
    )]
    filename: &str,
    #[wasm_bindgen(param_description = "Optional formatter config")] config: Option<Config>,
) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    biome_fmt::format_script_with_config(src, filename, config)
}

#[cfg(feature = "script-biome")]
pub(crate) fn produce_script_config(
    config: Option<biome_fmt::BiomeConfig>,
    default_layout: &LayoutConfig,
) -> biome_fmt::BiomeConfig {
    config.unwrap_or_default().fill_empty_layout_with(default_layout)
}
