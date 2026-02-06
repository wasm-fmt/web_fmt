mod config;

pub use oxc_formatter::{EmbeddedFormatterCallback, ExternalCallbacks};

use oxc_allocator::Allocator;
use oxc_formatter::{get_parse_options, FormatOptions, Formatter};
use oxc_parser::Parser;
use oxc_span::SourceType;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

use crate::config::OxFmtOptions;

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

export type Filename = `index.${Mod}${Lang}s${X}` | `index.d.${Mod}ts${X}` | (string & {});

export function format(code: string, filename: Filename, config?: Config): string;
"#;

/// Formats the given JavaScript/TypeScript code with the provided Configuration.
#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(js_name = format, skip_typescript)]
pub fn format_script(
    #[wasm_bindgen(param_description = "The JavaScript/TypeScript code to format")] code: &str,
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

    FormatScript::new(code, filename).config(config)?.format()
}

/// Builder for formatting JavaScript/TypeScript code.
///
/// # Example
/// ```ignore
/// FormatScript::new(src, filename)
///     .config(config)  // required: format configuration
///     .ext("ts")  // optional: override extension for source type
///     .embedded(formatter)  // optional: embedded language formatter
///     .format()
/// ```
pub struct FormatScript<'a> {
    src: &'a str,
    filename: &'a str,
    ext: Option<&'a str>,
    config: Option<FormatOptions>,
    external_callbacks: Option<ExternalCallbacks>,
}

impl<'a> FormatScript<'a> {
    pub fn new(src: &'a str, filename: &'a str) -> Self {
        Self { src, filename, ext: None, config: None, external_callbacks: None }
    }

    /// Set format configuration.
    pub fn config(mut self, config: OxFmtOptions) -> Result<Self, String> {
        let format_options = config.try_into()?;
        self.config = Some(format_options);
        Ok(self)
    }

    /// Override extension for source type inference.
    /// Use when filename doesn't have the correct extension (e.g., embedded scripts in HTML/Vue).
    pub fn ext(mut self, ext: &'a str) -> Self {
        self.ext = Some(ext);
        self
    }

    /// Set embedded language formatter for template literals.
    pub fn external_callbacks(mut self, callbacks: ExternalCallbacks) -> Self {
        self.external_callbacks = Some(callbacks);
        self
    }

    pub fn format(self) -> Result<String, String> {
        let source_type = match self.ext {
            Some(ext) => SourceType::from_path(format!("_.{ext}")).map_err(|e| e.to_string())?,
            None => SourceType::from_path(self.filename).map_err(|e| e.to_string())?,
        };

        let allocator = Allocator::new();
        let ret = Parser::new(&allocator, self.src, source_type)
            .with_options(get_parse_options())
            .parse();

        if !ret.errors.is_empty() {
            return Err(ret
                .errors
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n"));
        }

        let options = self.config.unwrap_or_default();

        let formatted = Formatter::new(&allocator, options)
            .format_with_external_callbacks(&ret.program, self.external_callbacks);

        formatted.print().map(|p| p.into_code()).map_err(|e| e.to_string())
    }
}
