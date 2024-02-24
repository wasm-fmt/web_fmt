#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
interface LayoutConfig {
	indent_style?: "tab" | "space";
	indent_width?: number;
	line_width?: number;
	line_ending?: "lf" | "crlf";
}"#;

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Clone, Default)]
pub struct LayoutConfig {
    #[cfg_attr(feature = "serde", serde(alias = "indentStyle"))]
    indent_style: Option<IndentStyle>,
    #[cfg_attr(feature = "serde", serde(alias = "indentWidth"))]
    indent_width: Option<u8>,
    #[cfg_attr(feature = "serde", serde(alias = "lineWidth"))]
    line_width: Option<u16>,
    #[cfg_attr(feature = "serde", serde(alias = "lineEnding"))]
    line_ending: Option<LineEnding>,
}

impl LayoutConfig {
    pub fn fill_empty_with(mut self, other: &Self) -> Self {
        if self.indent_style.is_none() {
            self.indent_style = other.indent_style;
        }
        if self.indent_width.is_none() {
            self.indent_width = other.indent_width;
        }
        if self.line_width.is_none() {
            self.line_width = other.line_width;
        }
        if self.line_ending.is_none() {
            self.line_ending = other.line_ending;
        }
        self
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = Some(indent_style);
        self
    }

    pub fn with_indent_width(mut self, indent_width: u8) -> Self {
        self.indent_width = Some(indent_width);
        self
    }

    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.line_width = Some(line_width);
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = Some(line_ending);
        self
    }

    pub fn indent_style(&self) -> Option<IndentStyle> {
        self.indent_style
    }

    pub fn indent_width(&self) -> Option<u8> {
        self.indent_width
    }

    pub fn line_width(&self) -> Option<u16> {
        self.line_width
    }

    pub fn line_ending(&self) -> Option<LineEnding> {
        self.line_ending
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Default)]
pub enum IndentStyle {
    Tab,
    #[default]
    Space,
}

impl IndentStyle {
    pub fn is_tab(&self) -> bool {
        matches!(self, Self::Tab)
    }
}

#[cfg(feature = "biome_formatter")]
impl From<IndentStyle> for biome_formatter::IndentStyle {
    fn from(style: IndentStyle) -> Self {
        match style {
            IndentStyle::Tab => biome_formatter::IndentStyle::Tab,
            IndentStyle::Space => biome_formatter::IndentStyle::Space,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Default)]
pub enum LineEnding {
    #[default]
    Lf,
    Crlf,
}

#[cfg(feature = "biome_formatter")]
impl From<LineEnding> for biome_formatter::LineEnding {
    fn from(ending: LineEnding) -> Self {
        match ending {
            LineEnding::Lf => biome_formatter::LineEnding::Lf,
            LineEnding::Crlf => biome_formatter::LineEnding::Crlf,
        }
    }
}
