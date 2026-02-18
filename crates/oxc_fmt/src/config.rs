use common::LayoutConfig;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// Trait for types that can be parsed from a string.
/// This is used to provide a common interface for parsing types from oxc_formatter.
pub trait Parseable: Sized {
    /// Parse a value from a string.
    /// Returns `None` if the string is not a valid representation.
    fn parse_from_str(s: &str) -> Option<Self>;

    /// Returns the name of the type for error messages.
    fn type_name() -> &'static str;
}

impl Parseable for oxc_formatter::ImportSelector {
    fn parse_from_str(s: &str) -> Option<Self> {
        oxc_formatter::ImportSelector::parse(s)
    }

    fn type_name() -> &'static str {
        "import selector"
    }
}

impl Parseable for oxc_formatter::ImportModifier {
    fn parse_from_str(s: &str) -> Option<Self> {
        oxc_formatter::ImportModifier::parse(s)
    }

    fn type_name() -> &'static str {
        "import modifier"
    }
}

impl Parseable for oxc_formatter::GroupEntry {
    fn parse_from_str(s: &str) -> Option<Self> {
        Some(oxc_formatter::GroupEntry::parse(s))
    }

    fn type_name() -> &'static str {
        "group entry"
    }
}

/// Parse a single value using the Parseable trait.
fn parse_single<T: Parseable>(s: &str) -> Result<T, String> {
    T::parse_from_str(s).ok_or_else(|| format!("Invalid {}: {}", T::type_name(), s))
}

/// Parse an optional value using the Parseable trait.
fn parse_optional<T: Parseable>(s: Option<String>) -> Result<Option<T>, String> {
    match s {
        Some(s) => parse_single(&s).map(Some),
        None => Ok(None),
    }
}

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

fn default_true() -> bool {
    true
}

/// Local definition for SortImportsOptions to enable deserialization.
/// This mirrors `oxc_formatter::SortImportsOptions`.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortImportsOptionsDef {
    #[serde(default)]
    pub partition_by_newline: bool,
    #[serde(default)]
    pub partition_by_comment: bool,
    #[serde(default)]
    pub sort_side_effects: bool,
    #[serde(default, deserialize_with = "deserialize_from_str")]
    pub order: oxc_formatter::SortOrder,
    #[serde(default = "default_true")]
    pub ignore_case: bool,
    #[serde(default = "default_true")]
    pub newlines_between: bool,
    #[serde(default = "oxc_formatter::default_internal_patterns")]
    pub internal_pattern: Vec<String>,
    #[serde(default, deserialize_with = "groups::deserialize")]
    pub groups: ParsedGroups,
    #[serde(default)]
    pub custom_groups: Vec<CustomGroupDefinitionDef>,
}

/// Parsed groups result containing both groups and newline boundary overrides.
#[derive(Debug, Default, Clone)]
pub struct ParsedGroups {
    pub groups: Vec<Vec<oxc_formatter::GroupEntry>>,
    pub newline_boundary_overrides: Vec<Option<bool>>,
}

impl Default for SortImportsOptionsDef {
    fn default() -> Self {
        Self {
            partition_by_newline: false,
            partition_by_comment: false,
            sort_side_effects: false,
            order: oxc_formatter::SortOrder::default(),
            ignore_case: true,
            newlines_between: true,
            internal_pattern: oxc_formatter::default_internal_patterns(),
            groups: ParsedGroups {
                groups: oxc_formatter::default_groups(),
                newline_boundary_overrides: vec![],
            },
            custom_groups: vec![],
        }
    }
}

impl From<SortImportsOptionsDef> for oxc_formatter::SortImportsOptions {
    fn from(def: SortImportsOptionsDef) -> Self {
        Self {
            partition_by_newline: def.partition_by_newline,
            partition_by_comment: def.partition_by_comment,
            sort_side_effects: def.sort_side_effects,
            order: def.order,
            ignore_case: def.ignore_case,
            newlines_between: def.newlines_between,
            internal_pattern: def.internal_pattern,
            groups: def.groups.groups,
            custom_groups: def.custom_groups.into_iter().map(Into::into).collect(),
            newline_boundary_overrides: def.groups.newline_boundary_overrides,
        }
    }
}

#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomGroupDefinitionDef {
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub element_name_pattern: Vec<String>,
    #[serde(default, deserialize_with = "import_selector::deserialize")]
    pub selector: Option<oxc_formatter::ImportSelector>,
    #[serde(default, deserialize_with = "import_modifiers::deserialize")]
    pub modifiers: Vec<oxc_formatter::ImportModifier>,
}

impl From<CustomGroupDefinitionDef> for oxc_formatter::CustomGroupDefinition {
    fn from(def: CustomGroupDefinitionDef) -> Self {
        Self {
            group_name: def.group_name,
            element_name_pattern: def.element_name_pattern,
            selector: def.selector,
            modifiers: def.modifiers,
        }
    }
}

/// Local definition for TailwindcssOptions to enable deserialization.
/// This mirrors `oxc_formatter::TailwindcssOptions`.
#[derive(Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TailwindcssOptionsDef {
    pub config: Option<String>,
    pub stylesheet: Option<String>,
    #[serde(default)]
    pub functions: Vec<String>,
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(default)]
    pub preserve_whitespace: bool,
    #[serde(default)]
    pub preserve_duplicates: bool,
}

impl From<TailwindcssOptionsDef> for oxc_formatter::TailwindcssOptions {
    fn from(def: TailwindcssOptionsDef) -> Self {
        Self {
            config: def.config,
            stylesheet: def.stylesheet,
            functions: def.functions,
            attributes: def.attributes,
            preserve_whitespace: def.preserve_whitespace,
            preserve_duplicates: def.preserve_duplicates,
        }
    }
}

/// Custom deserialization module for ImportSelector
mod import_selector {
    use super::*;
    use serde::Deserialize;

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<oxc_formatter::ImportSelector>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        parse_optional::<oxc_formatter::ImportSelector>(s).map_err(serde::de::Error::custom)
    }
}

/// Custom deserialization module for ImportModifier
mod import_modifiers {
    use super::*;
    use serde::de::{SeqAccess, Visitor};
    use std::fmt;

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Vec<oxc_formatter::ImportModifier>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ImportModifiersVisitor;

        impl<'de> Visitor<'de> for ImportModifiersVisitor {
            type Value = Vec<oxc_formatter::ImportModifier>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of import modifier strings")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut items = Vec::new();

                while let Some(name) = seq.next_element::<String>()? {
                    match parse_single::<oxc_formatter::ImportModifier>(&name) {
                        Ok(item) => items.push(item),
                        Err(e) => return Err(serde::de::Error::custom(e)),
                    }
                }

                Ok(items)
            }
        }

        deserializer.deserialize_seq(ImportModifiersVisitor)
    }
}

/// Custom deserialization module for groups
mod groups {
    use super::*;
    use serde::Deserialize;

    /// A marker object for overriding `newlinesBetween` at a specific group boundary.
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct NewlinesBetweenMarker {
        newlines_between: bool,
    }

    /// Intermediate representation for group items.
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum GroupItem {
        /// A `{ "newlinesBetween": bool }` marker object.
        NewlinesBetween(NewlinesBetweenMarker),
        /// A single group name string (e.g. `"value-builtin"`).
        Single(String),
        /// Multiple group names treated as one group (e.g. `["value-builtin", "value-external"]`).
        Multiple(Vec<String>),
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ParsedGroups, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items: Vec<GroupItem> = Vec::deserialize(deserializer)?;

        let mut groups: Vec<Vec<oxc_formatter::GroupEntry>> = Vec::new();
        let mut newline_boundary_overrides: Vec<Option<bool>> = Vec::new();
        let mut pending_override: Option<bool> = None;

        for item in items {
            match item {
                GroupItem::NewlinesBetween(marker) => {
                    pending_override = Some(marker.newlines_between);
                }
                other => {
                    if !groups.is_empty() {
                        newline_boundary_overrides.push(pending_override.take());
                    }
                    let entries = match other {
                        GroupItem::Single(s) => {
                            vec![parse_single::<oxc_formatter::GroupEntry>(&s)
                                .map_err(serde::de::Error::custom)?]
                        }
                        GroupItem::Multiple(v) => v
                            .into_iter()
                            .map(|s| parse_single::<oxc_formatter::GroupEntry>(&s))
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(serde::de::Error::custom)?,
                        GroupItem::NewlinesBetween(_) => unreachable!(),
                    };
                    groups.push(entries);
                }
            }
        }

        Ok(ParsedGroups { groups, newline_boundary_overrides })
    }
}

/// Custom deserialization module for SortImportsOptions
mod sort_imports {
    use super::*;
    use serde::Deserialize;

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<oxc_formatter::SortImportsOptions>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let def = Option::<SortImportsOptionsDef>::deserialize(deserializer)?;
        Ok(def.map(Into::into))
    }
}

/// Custom deserialization module for TailwindcssOptions
mod tailwindcss {
    use super::*;
    use serde::Deserialize;

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<oxc_formatter::TailwindcssOptions>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let def = Option::<TailwindcssOptionsDef>::deserialize(deserializer)?;
        Ok(def.map(Into::into))
    }
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
    #[serde(alias = "experimentalSortImports", deserialize_with = "sort_imports::deserialize")]
    pub experimental_sort_imports: Option<oxc_formatter::SortImportsOptions>,

    /// Enable Tailwind CSS class sorting in JSX class/className attributes.
    /// When enabled, class strings will be collected and passed to a callback for sorting.
    /// Defaults to None (disabled).
    #[serde(alias = "experimentalTailwindcss", deserialize_with = "tailwindcss::deserialize")]
    pub experimental_tailwindcss: Option<oxc_formatter::TailwindcssOptions>,
}
