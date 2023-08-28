mod config;

use std::path::Path;

use rome_js_formatter::format_node;
use rome_js_parser::parse;
use rome_js_syntax::{ModuleKind, SourceType};

use wasm_bindgen::prelude::*;

use crate::config::Config as InnerConfig;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

#[wasm_bindgen(skip_typescript)]
pub fn format(src: &str, filename: &str, config: &Config) -> Result<String, String> {
    let config: &JsValue = config.as_ref();
    let config: InnerConfig =
        serde_wasm_bindgen::from_value(config.clone()).map_err(|op| op.to_string())?;

    let syntax = source_type_from_filename(filename);

    let tree = parse(src, syntax);

    let option = config.with_source_type(syntax).try_into()?;

    format_node(option, &tree.syntax())
        .map_err(|e| e.to_string())?
        .print()
        .map(|p| p.into_code())
        .map_err(|e| e.to_string())
}

fn source_type_from_filename(filename: &str) -> SourceType {
    if filename.ends_with(".d.ts")
        || filename.ends_with(".d.mts")
        || filename.ends_with(".d.cts")
        || filename.ends_with(".d.mtsx")
        || filename.ends_with(".d.ctsx")
    {
        return SourceType::d_ts();
    }

    let Some(ext) = Path::new(filename).extension().and_then(|ext| ext.to_str()) else {
        return SourceType::tsx();
    };

    match ext {
        "ts" | "mts" | "cts" => SourceType::ts(),
        "js" | "mjs" | "jsx" => SourceType::jsx(),
        "cjs" | "cjsx" => SourceType::jsx().with_module_kind(ModuleKind::Script),
        _ => SourceType::tsx(),
    }
}
