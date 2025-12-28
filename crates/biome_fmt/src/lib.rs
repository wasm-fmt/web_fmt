mod config;

pub use config::BiomeConfig;
pub use config::IndentStyle;

use biome_js_formatter::format_node;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, LanguageVariant, ModuleKind};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const FORMAT: &'static str = r#"
type Mod = "" | "m" | "c";
type Lang = "j" | "t";
type X = "" | "x";

export type Filename = `index.${Mod}${"t" | "j"}s${X}` | `index.d.${Mod}ts${X}` | (string & {});

export function format(src: string, filename: Filename, config?: Config): string;
"#;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(skip_typescript)]
pub fn format(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    format_script_with_config(src, filename, config)
}

pub fn format_script_with_config(
    src: &str,
    filename: &str,
    config: BiomeConfig,
) -> Result<String, String> {
    let source_type = source_type_from_filename(filename);

    let tree =
        parse(src, source_type, JsParserOptions::default().with_parse_class_parameter_decorators());

    let option = config.with_source_type(source_type).try_into()?;

    format_node(option, &tree.syntax())
        .map_err(|e| e.to_string())?
        .print()
        .map(|p| p.into_code())
        .map_err(|e| e.to_string())
}

pub(crate) fn source_type_from_filename(mut filename: &str) -> JsFileSource {
    let mut err_flag = false;
    let mut x_flag = false;
    let mut t_flag = false;
    let mut m_flag = None;
    let mut d_flag = false;

    'err: {
        if let Some(s) = filename.strip_suffix(['x', 'X']) {
            filename = s;
            x_flag = true;
        }

        let Some(s) = filename.strip_suffix(['s', 'S']) else {
            err_flag = true;
            break 'err;
        };
        filename = s;

        if let Some(s) = filename.strip_suffix(['t', 'T']) {
            filename = s;
            t_flag = true;
        }

        if let Some(s) = filename.strip_suffix(['m', 'M']) {
            filename = s;
            m_flag = Some(ModuleKind::Module);
        } else if let Some(s) = filename.strip_suffix(['c', 'C']) {
            filename = s;
            m_flag = Some(ModuleKind::Script);
        }

        let Some(s) = filename.strip_suffix(['.']) else {
            err_flag = true;
            break 'err;
        };
        filename = s;

        if filename.strip_suffix(['d', 'D']).and_then(|s| s.strip_suffix(['.'])).is_some() {
            d_flag = true;
        }
    }

    if err_flag {
        return JsFileSource::tsx();
    }

    let source = if t_flag {
        if d_flag {
            JsFileSource::d_ts()
        } else {
            JsFileSource::ts()
        }
    } else {
        JsFileSource::js_module()
    };
    let kind = m_flag.unwrap_or_default();
    let variant = if x_flag || kind.is_module() {
        LanguageVariant::Jsx
    } else if m_flag.is_some() {
        LanguageVariant::StandardRestricted
    } else {
        LanguageVariant::Standard
    };

    source.with_module_kind(kind).with_variant(variant)
}
