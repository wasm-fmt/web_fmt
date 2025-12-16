use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[cfg(feature = "script-biome")]
use biome_fmt;

#[cfg(feature = "script-oxc")]
use oxc_fmt::{EmbeddedFormatter, EmbeddedFormatterCallback, FormatScript, OxcConfig};

#[cfg(feature = "script-oxc")]
use crate::format_style;

#[cfg(feature = "script-biome")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ScriptConfig")]
    pub type Config;
}

#[cfg(feature = "script-oxc")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "OxcScriptConfig")]
    pub type Config;
}

#[cfg(feature = "script-biome")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface ScriptConfig extends LayoutConfig {
	quote_style?: "double" | "single";
	jsx_quote_style?: "double" | "single";
	quote_properties?: "preserve" | "as-needed";
	trailing_comma?: "es5" | "all" | "none";
	semicolons?: "always" | "as-needed";
	arrow_parentheses?: "always" | "as-needed";
	bracket_spacing?: boolean;
	bracket_same_line?: boolean;
}"#;

#[cfg(feature = "script-oxc")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface OxcScriptConfig extends LayoutConfig {
	quote_style?: "double" | "single";
	jsx_quote_style?: "double" | "single";
	quote_properties?: "preserve" | "as-needed" | "consistent";
	trailing_commas?: "es5" | "all" | "none";
	semicolons?: "always" | "as-needed";
	arrow_parentheses?: "always" | "as-needed";
	bracket_spacing?: boolean;
	bracket_same_line?: boolean;
}"#;

#[cfg(feature = "script-biome")]
#[wasm_bindgen]
pub fn format_script(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    biome_fmt::format_script_with_config(src, filename, config)
}

#[cfg(feature = "script-oxc")]
/// Experimental: Format JavaScript/TypeScript code using OXC with embedded CSS support.
///
/// This function uses the OXC formatter and formats embedded CSS in template literals
/// (e.g., `css\`...\``, `styled.div\`...\``) using malva.
#[wasm_bindgen]
pub fn format_script(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let default_config: LayoutConfig = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config: OxcConfig = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config = produce_script_config(Some(config), &default_config, &default_config);
    let style_config = format_style::produce_style_config(None, &default_config, &default_config);

    format_script_with_config(src, filename, config, style_config)
}

#[cfg(feature = "script-biome")]
pub(crate) fn produce_script_config(
    config: Option<biome_fmt::BiomeConfig>,
    config_default: &LayoutConfig,
    global_fallback: &LayoutConfig,
) -> biome_fmt::BiomeConfig {
    let default = LayoutConfig::default()
        .with_indent_style(common::IndentStyle::Space)
        .with_indent_width(2)
        .with_line_width(80)
        .with_line_ending(common::LineEnding::Lf);

    config
        .unwrap_or_default()
        .fill_empty_layout_with(config_default)
        .fill_empty_layout_with(global_fallback)
        .fill_empty_layout_with(&default)
}

#[cfg(feature = "script-oxc")]
pub(crate) fn produce_script_config(
    config: Option<OxcConfig>,
    config_default: &LayoutConfig,
    global_fallback: &LayoutConfig,
) -> OxcConfig {
    let default = LayoutConfig::default()
        .with_indent_style(common::IndentStyle::Space)
        .with_indent_width(2)
        .with_line_width(80)
        .with_line_ending(common::LineEnding::Lf);

    config
        .unwrap_or_default()
        .fill_empty_layout_with(config_default)
        .fill_empty_layout_with(global_fallback)
        .fill_empty_layout_with(&default)
}

#[cfg(feature = "script-oxc")]
pub fn format_script_with_config(
    src: &str,
    filename: &str,
    config: OxcConfig,
    style_config: malva::config::FormatOptions,
) -> Result<String, String> {
    FormatScript::new(src, filename, config)
        .embedded(create_embedded_formatter(style_config))
        .format()
}

#[cfg(feature = "script-oxc")]
/// Format with explicit extension for source type inference.
/// Use when filename doesn't have the correct extension (e.g., embedded scripts in HTML/Vue).
pub fn format_script_with_ext(
    src: &str,
    filename: &str,
    ext: &str,
    config: OxcConfig,
    style_config: malva::config::FormatOptions,
) -> Result<String, String> {
    FormatScript::new(src, filename, config)
        .ext(ext)
        .embedded(create_embedded_formatter(style_config))
        .format()
}

#[cfg(feature = "script-oxc")]
fn create_embedded_formatter(
    style_config: malva::config::FormatOptions,
) -> oxc_fmt::EmbeddedFormatter {
    let callback: EmbeddedFormatterCallback = std::sync::Arc::new(move |tag, source| {
        // Handle CSS tags: "css", "styled", "styled.div", "styled.button", etc.
        if tag == "css" || tag.starts_with("styled") {
            malva::format_text(source, malva::Syntax::Css, &style_config).map_err(|e| e.to_string())
        } else {
            Err(format!("unsupported embedded tag: {tag}"))
        }
    });

    EmbeddedFormatter::new(callback)
}
