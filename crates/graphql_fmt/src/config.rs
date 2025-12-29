use common::LayoutConfig;
use pretty_graphql::config::{FormatOptions, LanguageOptions, LayoutOptions};
use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
struct GraphqlLayoutOptions {
    #[serde(alias = "printWidth")]
    print_width: Option<usize>,
    #[serde(alias = "useTabs")]
    use_tabs: Option<bool>,
    #[serde(alias = "indentWidth")]
    indent_width: Option<usize>,
    #[serde(alias = "lineBreak")]
    line_break: Option<pretty_graphql::config::LineBreak>,
}

impl GraphqlLayoutOptions {
    pub fn with_print_width(mut self, print_width: usize) -> Self {
        self.print_width = Some(print_width);
        self
    }
}

impl GraphqlLayoutOptions {
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

impl From<GraphqlLayoutOptions> for LayoutOptions {
    fn from(options: GraphqlLayoutOptions) -> Self {
        let mut value = LayoutOptions::default();
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

impl From<LayoutConfig> for GraphqlLayoutOptions {
    fn from(config: LayoutConfig) -> Self {
        GraphqlLayoutOptions {
            print_width: config.line_width().map(|v| v as usize),
            use_tabs: config.indent_style().map(|s| matches!(s, common::IndentStyle::Tab)),
            indent_width: config.indent_width().map(|v| v as usize),
            line_break: config.line_ending().map(|le| match le {
                common::LineEnding::Lf => pretty_graphql::config::LineBreak::Lf,
                common::LineEnding::Crlf => pretty_graphql::config::LineBreak::Crlf,
            }),
        }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct GraphqlConfig {
    #[serde(flatten)]
    pub layout: LayoutConfig,

    #[serde(flatten)]
    graphql_layout: GraphqlLayoutOptions,

    #[serde(flatten)]
    language: LanguageOptions,
}

impl GraphqlConfig {
    pub fn with_print_width(mut self, print_width: usize) -> Self {
        self.graphql_layout = self.graphql_layout.with_print_width(print_width);
        self
    }

    pub fn fill_empty_layout_with(mut self, layout: &LayoutConfig) -> Self {
        self.layout = self.layout.fill_empty_with(layout);
        self
    }
}

impl From<GraphqlConfig> for FormatOptions {
    fn from(config: GraphqlConfig) -> Self {
        let layout: GraphqlLayoutOptions = config.layout.into();
        let layout = config.graphql_layout.fill_empty_with(&layout);
        let layout: LayoutOptions = layout.into();

        Self { layout, language: config.language }
    }
}
