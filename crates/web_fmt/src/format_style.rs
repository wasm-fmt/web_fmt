use common::LayoutConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "StyleConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface StyleConfig extends LayoutConfig {
	/**
	 *  See {@link https://github.com/g-plane/malva/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
pub fn format_style(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let default_config: LayoutConfig = config
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
    config_default: &LayoutConfig,
    global_fallback: &LayoutConfig,
) -> malva::config::FormatOptions {
    let mut config: malva::config::FormatOptions = base_config.unwrap_or_default();

    if let Some(indent_style) = config_default.indent_style().or(global_fallback.indent_style()) {
        config.layout.use_tabs = indent_style.is_tab();
    }

    if let Some(indent_width) = config_default.indent_width().or(global_fallback.indent_width()) {
        config.layout.indent_width = indent_width as usize;
    }

    if let Some(line_width) = config_default.line_width().or(global_fallback.line_width()) {
        config.layout.print_width = line_width as usize;
    }

    if let Some(line_endings) = config_default.line_ending().or(global_fallback.line_ending()) {
        config.layout.line_break = match line_endings {
            common::LineEnding::Lf => malva::config::LineBreak::Lf,
            common::LineEnding::Crlf => malva::config::LineBreak::Crlf,
        };
    }

    config
}
