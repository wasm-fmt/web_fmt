use wasm_bindgen::prelude::*;

use crate::ConfigLayout;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "StyleConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface StyleConfig {
	indent_style?: "tab" | "space";
	indent_width?: number;
	line_width?: number;
	/** 
	 *  See {@link https://github.com/g-plane/malva/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
pub fn format_style(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let default_config: ConfigLayout = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|e| e.to_string())?;

    let config = produce_style_config(config, &default_config, &default_config);

    format_style_with_config(src, filename, config)
}

pub fn format_style_with_config(
    src: &str,
    filename: &str,
    config: malva::config::FormatOptions,
) -> Result<String, String> {
    let syntax = syntax_from_filename(filename).unwrap_or_default();

    malva::format_text(src, syntax, &config).map_err(|e| e.to_string())
}

fn syntax_from_filename(filename: &str) -> Option<malva::Syntax> {
    let filename = filename.strip_suffix(['s', 'S'])?;
    let filename = filename.strip_suffix(['s', 'S'])?;

    let result = 'result: {
        if let Some(filename) = filename.strip_suffix(['a', 'A']) {
            filename.strip_suffix(['s', 'S'])?;
            break 'result malva::Syntax::Sass;
        }

        if let Some(filename) = filename.strip_suffix(['e', 'E']) {
            filename.strip_suffix(['l', 'L'])?;
            break 'result malva::Syntax::Less;
        }

        if filename.strip_suffix(['c', 'C'])?.strip_suffix(['s', 'S']).is_some() {
            break 'result malva::Syntax::Scss;
        }

        malva::Syntax::Css
    };

    filename.strip_suffix('.')?;

    Some(result)
}

pub(crate) fn produce_style_config(
    base_config: Option<malva::config::FormatOptions>,
    layout_config: &ConfigLayout,
    default_config: &ConfigLayout,
) -> malva::config::FormatOptions {
    let use_tabs: bool =
        layout_config.indent_style.or(default_config.indent_style).unwrap_or_default().into();

    let indent_width: u8 = layout_config.indent_width.or(default_config.indent_width).unwrap_or(2);

    let line_width: u16 = layout_config.line_width.or(default_config.line_width).unwrap_or(80);

    let mut config = base_config.unwrap_or_default();

    config.layout.use_tabs = use_tabs;
    config.layout.indent_width = indent_width as usize;
    config.layout.print_width = line_width as usize;

    config
}
