use biome_fmt::BiomeConfig;
use common::LayoutConfig;
use markup_fmt::{EmbeddedFormatter, FormatMarkup, Hints};
use wasm_bindgen::prelude::*;

use crate::format_script;
use crate::format_style;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "MarkupConfig")]
    pub type Config;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface MarkupConfig extends LayoutConfig {
	/**
	 *  See {@link https://github.com/g-plane/markup_fmt/blob/main/docs/config.md}
	 */
	[other: string]: any;
}"#;

#[wasm_bindgen]
pub fn format_markup(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let markup_config: markup_fmt::config::MarkupConfig = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let style_config = format_style::produce_style_config(None, &markup_config.layout);
    let script_config = format_script::produce_script_config(None, &markup_config.layout);
    let json_config = LayoutConfig::default().fill_empty_with(&markup_config.layout);

    let formatter = EmbeddedCodeFormatter {
        filename: filename.to_string(),
        markup_config: markup_config.clone(),
        script_config,
        style_config,
        json_config,
    };

    FormatMarkup::new(src, filename)
        .config(markup_config.into())
        .embed_formatter(formatter)
        .format()
}

pub(crate) fn produce_markup_config(
    base_config: Option<markup_fmt::config::MarkupConfig>,
    default_layout: &LayoutConfig,
) -> markup_fmt::config::MarkupConfig {
    base_config.unwrap_or_default().fill_empty_layout_with(default_layout)
}

pub(crate) struct EmbeddedCodeFormatter {
    pub(crate) filename: String,
    pub(crate) markup_config: markup_fmt::config::MarkupConfig,
    pub(crate) script_config: BiomeConfig,
    pub(crate) style_config: malva_fmt::config::MalvaConfig,
    pub(crate) json_config: LayoutConfig,
}

impl EmbeddedFormatter for EmbeddedCodeFormatter {
    fn format(&self, src: &str, hints: Hints) -> Result<String, String> {
        let Hints { print_width, attr, ext, .. } = hints;
        match ext.as_bytes() {
            b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx" => {
                biome_fmt::format_script_with_config(
                    src,
                    &self.filename,
                    self.script_config.clone().with_line_width(print_width as u16),
                )
            }
            b"css" | b"scss" | b"sass" | b"less" => {
                let mut config = self.style_config.clone().with_print_width(print_width);

                if attr {
                    let quotes = if let markup_fmt_core::config::Quotes::Double =
                        self.markup_config.quotes()
                    {
                        malva::config::Quotes::AlwaysSingle
                    } else {
                        malva::config::Quotes::AlwaysDouble
                    };

                    config =
                        config.with_quotes(quotes).with_single_line_top_level_declarations(true);
                }

                malva_fmt::format_style_with_config(src, &self.filename, config)
            }
            b"json" | b"jsonc" => json_fmt::format_json_with_config(
                src,
                self.json_config.clone().with_line_width(print_width as u16).into(),
            ),
            _ => Ok(src.to_string()),
        }
    }
}
