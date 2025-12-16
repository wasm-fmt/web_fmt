mod config;

pub use config::OxcConfig;
pub use oxc_formatter::{EmbeddedFormatter, EmbeddedFormatterCallback};

use oxc_allocator::Allocator;
use oxc_formatter::{get_parse_options, FormatOptions, Formatter};
use oxc_parser::Parser;
use oxc_span::SourceType;

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

export function format(src: string, filename?: Filename, config?: Config): string;
"#;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(skip_typescript)]
pub fn format(src: &str, filename: &str, config: Option<Config>) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value(x.clone()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    FormatScript::new(src, filename, config).format()
}

/// Builder for formatting JavaScript/TypeScript code.
///
/// # Example
/// ```ignore
/// FormatScript::new(src, filename, config)
///     .ext("ts")  // optional: override extension for source type
///     .embedded(formatter)  // optional: embedded language formatter
///     .format()
/// ```
pub struct FormatScript<'a> {
    src: &'a str,
    filename: &'a str,
    ext: Option<&'a str>,
    config: OxcConfig,
    embedded_formatter: Option<EmbeddedFormatter>,
}

impl<'a> FormatScript<'a> {
    pub fn new(src: &'a str, filename: &'a str, config: OxcConfig) -> Self {
        Self { src, filename, ext: None, config, embedded_formatter: None }
    }

    /// Override extension for source type inference.
    /// Use when filename doesn't have the correct extension (e.g., embedded scripts in HTML/Vue).
    pub fn ext(mut self, ext: &'a str) -> Self {
        self.ext = Some(ext);
        self
    }

    /// Set embedded language formatter for template literals.
    pub fn embedded(mut self, formatter: EmbeddedFormatter) -> Self {
        self.embedded_formatter = Some(formatter);
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

        let options: FormatOptions = self.config.try_into()?;
        let formatter = Formatter::new(&allocator, options);

        let formatted = if let Some(ef) = self.embedded_formatter {
            formatter.format_with_embedded(&ret.program, ef)
        } else {
            formatter.format(&ret.program)
        };

        formatted.print().map(|p| p.into_code()).map_err(|e| e.to_string())
    }
}
