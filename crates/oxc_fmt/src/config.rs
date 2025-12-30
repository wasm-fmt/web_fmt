use common::LayoutConfig;
pub use oxc_formatter::oxfmtrc::Oxfmtrc;
use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct OxFmtOptions {
    #[serde(flatten)]
    layout: LayoutConfig,

    #[serde(flatten)]
    inner: Oxfmtrc,
}

impl TryFrom<OxFmtOptions> for oxc_formatter::FormatOptions {
    type Error = String;

    fn try_from(mut value: OxFmtOptions) -> Result<Self, Self::Error> {
        if let (None, Some(indent_style)) = (value.inner.use_tabs, value.layout.indent_style()) {
            value.inner.use_tabs = Some(indent_style.use_tabs());
        }

        if let (None, Some(indent_width)) = (value.inner.tab_width, value.layout.indent_width()) {
            value.inner.tab_width = Some(indent_width);
        }

        if let (None, Some(line_width)) = (value.inner.print_width, value.layout.line_width()) {
            value.inner.print_width = Some(line_width);
        }

        let mut option_line_ending = None;

        if let (None, Some(line_ending)) = (value.inner.end_of_line, value.layout.line_ending()) {
            let result = match line_ending {
                common::LineEnding::Lf => oxc_formatter::LineEnding::Lf,
                common::LineEnding::Crlf => oxc_formatter::LineEnding::Crlf,
            };
            option_line_ending = Some(result);
        };

        value.inner.into_options().map(|(mut options, _)| {
            if let Some(line_ending) = option_line_ending {
                options.line_ending = line_ending;
            }
            options
        })
    }
}
