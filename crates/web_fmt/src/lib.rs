mod format_markup;
mod format_script;
mod format_style;

use std::path::Path;

use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type JSConfig;
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
struct Config {
    markup: Option<markup_fmt::config::FormatOptions>,
    script: Option<biome_fmt::BiomeConfig>,
    style: Option<malva::config::FormatOptions>,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum IndentStyle {
    Tab,
    #[default]
    Space,
}

impl From<biome_fmt::IndentStyle> for IndentStyle {
    fn from(value: biome_fmt::IndentStyle) -> Self {
        match value {
            biome_fmt::IndentStyle::Tab => Self::Tab,
            biome_fmt::IndentStyle::Space => Self::Space,
        }
    }
}

impl From<IndentStyle> for biome_fmt::IndentStyle {
    fn from(value: IndentStyle) -> Self {
        match value {
            IndentStyle::Tab => Self::Tab,
            IndentStyle::Space => Self::Space,
        }
    }
}

impl From<IndentStyle> for bool {
    fn from(value: IndentStyle) -> Self {
        matches!(value, IndentStyle::Tab)
    }
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) struct ConfigLayout {
    indent_style: Option<IndentStyle>,
    indent_width: Option<u8>,
    line_width: Option<u16>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
struct ConfigDefault {
    #[serde(flatten, default)]
    default: ConfigLayout,
    #[serde(default)]
    markup: ConfigLayout,
    #[serde(default)]
    script: ConfigLayout,
    #[serde(default)]
    style: ConfigLayout,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
export interface StyleConfig {
	indent_style?: "tab" | "space";
	indent_width?: number;
	line_width?: number;

	markup?: MarkupConfig;
	script?: ScriptConfig;
	style: StyleConfig;
}"#;

#[wasm_bindgen]
pub fn format(src: &str, filename: &str, config: Option<JSConfig>) -> Result<String, String> {
    let default_config: ConfigDefault = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let config: Config = config
        .as_ref()
        .map(|x| serde_wasm_bindgen::from_value(x.into()))
        .transpose()
        .map_err(|e| e.to_string())?
        .unwrap_or_default();

    let extension = Path::new(&filename).extension().ok_or("expected extension")?;

    let script_config = format_script::produce_script_config(
        config.script,
        &default_config.script,
        &default_config.default,
    );
    let style_config = format_style::produce_style_config(
        config.style,
        &default_config.style,
        &default_config.default,
    );
    let markup_config = format_markup::produce_markup_config(
        config.markup,
        &default_config.markup,
        &default_config.default,
    );

    match extension.as_encoded_bytes() {
        b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx" | b"mtsx"
        | b"ctsx" => biome_fmt::format_script_with_config(src, filename, script_config),
        b"css" | b"scss" | b"sass" | b"less" => {
            format_style::format_style_with_config(src, filename, style_config)
        }
        b"html" | b"vue" | b"svelte" | b"jinja" | b"jinja2" | b"twig" => {
            let language =
                markup_fmt::detect_language(filename).unwrap_or(markup_fmt::Language::Html);

            markup_fmt::format_text(src, language, &markup_config, |filename, src, width| {
                let extension = filename
                    .extension()
                    .map(|x| x.to_ascii_lowercase())
                    .ok_or("expected extension")?;
                let filename: &str = filename.to_str().ok_or("expected filename")?;

                match extension.as_encoded_bytes() {
                    b"js" | b"ts" | b"mjs" | b"cjs" | b"jsx" | b"tsx" | b"mjsx" | b"cjsx"
                    | b"mtsx" => biome_fmt::format_script_with_config(
                        src,
                        filename,
                        script_config.clone().with_line_width(width as u16),
                    )
                    .map(Into::into),
                    b"css" | b"scss" | b"sass" | b"less" => {
                        let style_config = style_config.clone();
                        format_style::format_style_with_config(
                            src,
                            filename,
                            malva::config::FormatOptions {
                                layout: malva::config::LayoutOptions {
                                    print_width: width,
                                    ..style_config.layout
                                },
                                language: style_config.language,
                            },
                        )
                        .map(Into::into)
                    }
                    _ => Ok(src.into()),
                }
            })
            .map_err(|e| format!("{:?}", e))
        }
        _ => Err(format!("unsupported file extension: {}", filename)),
    }
}
