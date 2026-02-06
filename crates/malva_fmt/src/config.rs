use common::LayoutConfig;
use malva::config::LanguageOptions;
use serde::Deserialize;

pub use malva::config::Quotes;

// malva::config::LayoutOptions wrapper to allow optional fields
#[derive(Deserialize, Default, Clone)]
struct MalvaLayoutOptions {
    #[serde(alias = "printWidth")]
    print_width: Option<usize>,
    #[serde(alias = "useTabs")]
    use_tabs: Option<bool>,
    #[serde(alias = "indentWidth")]
    indent_width: Option<usize>,
    #[serde(alias = "lineBreak")]
    line_break: Option<malva::config::LineBreak>,
}

impl MalvaLayoutOptions {
    pub fn with_print_width(mut self, print_width: usize) -> Self {
        self.print_width = Some(print_width);
        self
    }
}

impl MalvaLayoutOptions {
    pub fn fill_empty_with(mut self, other: &Self) -> Self {
        if self.print_width.is_none() {
            self.print_width = other.print_width;
        }
        if self.use_tabs.is_none() {
            self.use_tabs = other.use_tabs;
        }
        if self.indent_width.is_none() {
            self.indent_width = other.indent_width;
        }
        if self.line_break.is_none() {
            self.line_break = other.line_break.clone();
        }
        self
    }
}

impl From<MalvaLayoutOptions> for malva::config::LayoutOptions {
    fn from(options: MalvaLayoutOptions) -> Self {
        let mut value = malva::config::LayoutOptions::default();
        if let Some(print_width) = options.print_width {
            value.print_width = print_width;
        }
        if let Some(use_tabs) = options.use_tabs {
            value.use_tabs = use_tabs;
        }
        if let Some(indent_width) = options.indent_width {
            value.indent_width = indent_width;
        }
        if let Some(line_break) = options.line_break {
            value.line_break = line_break;
        }
        value
    }
}

impl From<LayoutConfig> for MalvaLayoutOptions {
    fn from(config: LayoutConfig) -> Self {
        MalvaLayoutOptions {
            print_width: config.line_width().map(|v| v as usize),
            use_tabs: config.indent_style().map(|s| matches!(s, common::IndentStyle::Tab)),
            indent_width: config.indent_width().map(|v| v as usize),
            line_break: config.line_ending().map(|le| match le {
                common::LineEnding::Lf => malva::config::LineBreak::Lf,
                common::LineEnding::Crlf => malva::config::LineBreak::Crlf,
            }),
        }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct MalvaConfig {
    #[serde(flatten)]
    pub layout: LayoutConfig,

    #[serde(flatten)]
    malva_layout: MalvaLayoutOptions,

    #[serde(flatten)]
    language: LanguageOptions,
}

impl MalvaConfig {
    #[must_use]
    pub fn with_print_width(mut self, print_width: usize) -> Self {
        self.malva_layout = self.malva_layout.with_print_width(print_width);
        self
    }

    #[must_use]
    pub fn with_quotes(mut self, quotes: malva::config::Quotes) -> Self {
        self.language.quotes = quotes;
        self
    }

    #[must_use]
    pub fn with_single_line_top_level_declarations(mut self, value: bool) -> Self {
        self.language.single_line_top_level_declarations = value;
        self
    }

    #[must_use]
    pub fn fill_empty_layout_with(mut self, layout: &LayoutConfig) -> Self {
        self.layout = self.layout.fill_empty_with(layout);
        self
    }
}

impl From<MalvaConfig> for malva::config::FormatOptions {
    fn from(config: MalvaConfig) -> Self {
        let layout: MalvaLayoutOptions = config.layout.into();
        let layout = config.malva_layout.fill_empty_with(&layout);
        let layout: malva::config::LayoutOptions = layout.into();

        Self { layout, language: config.language }
    }
}
