use common::LayoutConfig;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Deserialize, Default, Clone)]
pub struct OxFmtOptions {
    #[serde(flatten)]
    layout: LayoutConfig,

    #[serde(flatten)]
    inner: FormatOptions,
}

impl TryFrom<OxFmtOptions> for oxc_formatter::FormatOptions {
    type Error = String;

    fn try_from(value: OxFmtOptions) -> Result<Self, Self::Error> {
        let mut options = oxc_formatter::FormatOptions::new();

        // Handle use_tabs from LayoutConfig if not explicitly set
        if let Some(indent_style) = value.layout.indent_style() {
            options.indent_style = if indent_style.use_tabs() {
                oxc_formatter::IndentStyle::Tab
            } else {
                oxc_formatter::IndentStyle::Space
            };
        }

        // Handle tab_width from LayoutConfig if not explicitly set
        if let Some(indent_width) = value.layout.indent_width() {
            options.indent_width = oxc_formatter::IndentWidth::try_from(indent_width)
                .map_err(|e| format!("Invalid indent_width: {e}"))?;
        }

        // Handle print_width from LayoutConfig if not explicitly set
        if let Some(line_width) = value.layout.line_width() {
            options.line_width = oxc_formatter::LineWidth::try_from(line_width)
                .map_err(|e| format!("Invalid print_width: {e}"))?;
        }

        // Handle line_ending from LayoutConfig if not explicitly set
        if let Some(line_ending) = value.layout.line_ending() {
            options.line_ending = match line_ending {
                common::LineEnding::Lf => oxc_formatter::LineEnding::Lf,
                common::LineEnding::Crlf => oxc_formatter::LineEnding::Crlf,
            };
        }

        // Apply all other options from FormatOptions
        options.quote_style = value.inner.quote_style;
        options.jsx_quote_style = value.inner.jsx_quote_style;
        options.quote_properties = value.inner.quote_properties;
        options.trailing_commas = value.inner.trailing_commas;
        options.semicolons = value.inner.semicolons;
        options.arrow_parentheses = value.inner.arrow_parentheses;
        options.bracket_spacing = value.inner.bracket_spacing;
        options.bracket_same_line = value.inner.bracket_same_line;
        options.attribute_position = value.inner.attribute_position;
        options.expand = value.inner.expand;
        options.experimental_operator_position = value.inner.experimental_operator_position;
        options.experimental_ternaries = value.inner.experimental_ternaries;
        options.embedded_language_formatting = value.inner.embedded_language_formatting;
        options.experimental_sort_imports = value.inner.experimental_sort_imports;
        options.experimental_tailwindcss = value.inner.experimental_tailwindcss;

        Ok(options)
    }
}

fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Default, Clone, Deserialize)]
pub struct FormatOptions {
    /// The style for quotes. Defaults to double.
    #[serde(alias = "singleQuote", deserialize_with = "deserialize_from_str")]
    pub quote_style: oxc_formatter::QuoteStyle,

    /// The style for JSX quotes. Defaults to double.
    #[serde(alias = "jsxSingleQuote", deserialize_with = "deserialize_from_str")]
    pub jsx_quote_style: oxc_formatter::QuoteStyle,

    /// When properties in objects are quoted. Defaults to as-needed.
    #[serde(alias = "quoteProps", deserialize_with = "deserialize_from_str")]
    pub quote_properties: oxc_formatter::QuoteProperties,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[serde(alias = "trailingComma", deserialize_with = "deserialize_from_str")]
    pub trailing_commas: oxc_formatter::TrailingCommas,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of [ASI](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-automatic-semicolon-insertion).
    #[serde(alias = "semi", deserialize_with = "deserialize_from_str")]
    pub semicolons: oxc_formatter::Semicolons,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[serde(alias = "arrowParens", deserialize_with = "deserialize_from_str")]
    pub arrow_parentheses: oxc_formatter::ArrowParentheses,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[serde(alias = "bracketSpacing", deserialize_with = "deserialize_from_str")]
    pub bracket_spacing: oxc_formatter::BracketSpacing,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[serde(alias = "bracketSameLine", deserialize_with = "deserialize_from_str")]
    pub bracket_same_line: oxc_formatter::BracketSameLine,

    /// Attribute position style. By default auto.
    #[serde(alias = "singleAttributePerLine", deserialize_with = "deserialize_from_str")]
    pub attribute_position: oxc_formatter::AttributePosition,

    /// Whether to expand object and array literals to multiple lines. Defaults to "auto".
    #[serde(alias = "objectWrap", deserialize_with = "deserialize_from_str")]
    pub expand: oxc_formatter::Expand,

    /// Controls the position of operators in binary expressions. [**NOT SUPPORTED YET**]
    ///
    /// Accepted values are:
    /// - `"start"`: Places the operator at the beginning of the next line.
    /// - `"end"`: Places the operator at the end of the current line (default).
    #[serde(alias = "experimentalOperatorPosition", deserialize_with = "deserialize_from_str")]
    pub experimental_operator_position: oxc_formatter::OperatorPosition,

    /// Try prettier's new ternary formatting before it becomes the default behavior. [**NOT SUPPORTED YET**]
    ///
    /// Valid options:
    /// - `true` - Use curious ternaries, with the question mark after the condition.
    /// - `false` - Retain the default behavior of ternaries; keep question marks on the same line as the consequent.
    #[serde(alias = "experimentalTernaries")]
    pub experimental_ternaries: bool,

    /// Enable formatting for embedded languages (e.g., CSS, SQL, GraphQL) within template literals. Defaults to "auto".
    #[serde(alias = "embeddedLanguageFormatting", deserialize_with = "deserialize_from_str")]
    pub embedded_language_formatting: oxc_formatter::EmbeddedLanguageFormatting,

    /// Sort import statements. By default disabled.
    #[serde(skip)]
    pub experimental_sort_imports: Option<oxc_formatter::SortImportsOptions>,

    /// Enable Tailwind CSS class sorting in JSX class/className attributes.
    /// When enabled, class strings will be collected and passed to a callback for sorting.
    /// Defaults to None (disabled).
    #[serde(skip)]
    pub experimental_tailwindcss: Option<oxc_formatter::TailwindcssOptions>,
}
