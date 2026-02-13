use common::LayoutConfig;
use markup_fmt_core::config::LanguageOptions;
use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct MarkupConfig {
    #[serde(flatten)]
    pub layout: LayoutConfig,

    #[serde(flatten)]
    markup_layout: MarkupLayoutOptions,

    #[serde(flatten)]
    language: LanguageOptions,
}

#[derive(Deserialize, Default, Clone)]
struct MarkupLayoutOptions {
    #[serde(alias = "useTabs")]
    use_tabs: Option<bool>,
    #[serde(alias = "indentWidth")]
    indent_width: Option<usize>,
    #[serde(alias = "printWidth")]
    print_width: Option<usize>,
    #[serde(alias = "lineBreak")]
    line_break: Option<markup_fmt_core::config::LineBreak>,
}

impl MarkupLayoutOptions {
    fn fill_empty_with(mut self, other: &Self) -> Self {
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
            self.line_break = other.line_break;
        }
        self
    }
}

impl From<LayoutConfig> for MarkupLayoutOptions {
    fn from(config: LayoutConfig) -> Self {
        MarkupLayoutOptions {
            print_width: config.line_width().map(|v| v as usize),
            use_tabs: config.indent_style().map(|s| s.use_tabs()),
            indent_width: config.indent_width().map(|v| v as usize),
            line_break: config.line_ending().map(|le| match le {
                common::LineEnding::Lf => markup_fmt_core::config::LineBreak::Lf,
                common::LineEnding::Crlf => markup_fmt_core::config::LineBreak::Crlf,
            }),
        }
    }
}

impl From<MarkupLayoutOptions> for markup_fmt_core::config::LayoutOptions {
    fn from(options: MarkupLayoutOptions) -> Self {
        let mut value = markup_fmt_core::config::LayoutOptions::default();
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

impl MarkupConfig {
    #[must_use]
    pub fn with_line_width(mut self, line_width: u16) -> Self {
        self.markup_layout.print_width = Some(line_width as usize);
        self
    }

    #[must_use]
    pub fn quotes(&self) -> markup_fmt_core::config::Quotes {
        self.language.quotes
    }

    #[must_use]
    pub fn fill_empty_layout_with(mut self, layout: &LayoutConfig) -> Self {
        self.layout = self.layout.fill_empty_with(layout);
        self
    }
}

impl From<MarkupConfig> for markup_fmt_core::config::FormatOptions {
    fn from(config: MarkupConfig) -> Self {
        let layout: MarkupLayoutOptions = config.layout.into();
        let layout = config.markup_layout.fill_empty_with(&layout);
        let layout: markup_fmt_core::config::LayoutOptions = layout.into();

        Self { layout, language: config.language }
    }
}
