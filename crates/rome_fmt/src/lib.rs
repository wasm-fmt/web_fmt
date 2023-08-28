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

#[wasm_bindgen(typescript_custom_section)]
const FORMAT: &'static str = r#"
export type Filename =
  | "index.js"
  | "index.ts"
  | "index.mjs"
  | "index.cjs"
  | "index.mts"
  | "index.cts"
  | "index.mjsx"
  | "index.ctsx"
  | "index.mtsx"
  | "index.ctsx"
  | "index.d.ts"
  | "index.d.ts"
  | "index.d.mts"
  | "index.d.cts"
  | "index.d.mtsx"
  | "index.d.ctsx"
  | (string & {});

export function format(src: string, filename?: Filename, config?: Config): string;
"#;

#[wasm_bindgen(skip_typescript)]
pub fn format(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config: InnerConfig = if let Some(config) = config {
        serde_wasm_bindgen::from_value(config.clone()).map_err(|op| op.to_string())?
    } else {
        Default::default()
    };

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
        "ts" | "mts" => SourceType::ts(),
        "js" | "mjs" | "jsx" => SourceType::jsx(),
        "cjs" | "cjsx" => SourceType::jsx().with_module_kind(ModuleKind::Script),
        "cts" | "ctsx" => SourceType::tsx().with_module_kind(ModuleKind::Script),
        _ => SourceType::tsx(),
    }
}
