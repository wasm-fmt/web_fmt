use common::LayoutConfig;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

// =============================================================================
// Main Options
// =============================================================================

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
        if let Some(single_quote) = value.inner.single_quote {
            options.quote_style = oxc_formatter::QuoteStyle::from_use_single(single_quote);
        }
        if let Some(jsx_single_quote) = value.inner.jsx_single_quote {
            options.jsx_quote_style = oxc_formatter::QuoteStyle::from_use_single(jsx_single_quote);
        }
        if let Some(quote_properties) = value.inner.quote_properties {
            options.quote_properties = quote_properties;
        }
        if let Some(trailing_commas) = value.inner.trailing_commas {
            options.trailing_commas = trailing_commas;
        }
        if let Some(semicolons) = value.inner.semicolons {
            options.semicolons = semicolons;
        }
        if let Some(arrow_parentheses) = value.inner.arrow_parentheses {
            options.arrow_parentheses = arrow_parentheses;
        }
        if let Some(bracket_spacing) = value.inner.bracket_spacing {
            options.bracket_spacing = bracket_spacing;
        }
        if let Some(bracket_same_line) = value.inner.bracket_same_line {
            options.bracket_same_line = bracket_same_line;
        }
        if let Some(attribute_position) = value.inner.attribute_position {
            options.attribute_position = attribute_position;
        }
        if let Some(expand) = value.inner.expand {
            options.expand = expand;
        }
        if let Some(experimental_operator_position) = value.inner.experimental_operator_position {
            options.experimental_operator_position = experimental_operator_position;
        }
        if let Some(experimental_ternaries) = value.inner.experimental_ternaries {
            options.experimental_ternaries = experimental_ternaries;
        }
        if let Some(embedded_language_formatting) = value.inner.embedded_language_formatting {
            options.embedded_language_formatting = embedded_language_formatting;
        }
        options.sort_imports = value.inner.experimental_sort_imports;
        options.sort_tailwindcss = value.inner.experimental_tailwindcss;

        Ok(options)
    }
}

// =============================================================================
// Format Options
// =============================================================================

#[derive(Default, Clone, Deserialize)]
pub struct FormatOptions {
    /// The style for quotes. Defaults to double.
    #[serde(alias = "singleQuote")]
    pub single_quote: Option<bool>,

    /// The style for JSX quotes. Defaults to double.
    #[serde(alias = "jsxSingleQuote")]
    pub jsx_single_quote: Option<bool>,

    /// When properties in objects are quoted. Defaults to as-needed.
    #[serde(alias = "quoteProps", default, deserialize_with = "deserialize_optional_from_str")]
    pub quote_properties: Option<oxc_formatter::QuoteProperties>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[serde(alias = "trailingComma", default, deserialize_with = "deserialize_optional_from_str")]
    pub trailing_commas: Option<oxc_formatter::TrailingCommas>,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of [ASI](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-automatic-semicolon-insertion).
    #[serde(alias = "semi", default, deserialize_with = "deserialize_optional_from_str")]
    pub semicolons: Option<oxc_formatter::Semicolons>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[serde(alias = "arrowParens", default, deserialize_with = "deserialize_optional_from_str")]
    pub arrow_parentheses: Option<oxc_formatter::ArrowParentheses>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[serde(
        alias = "bracketSpacing",
        default,
        deserialize_with = "deserialize_optional_via_def::<_, bool, _>"
    )]
    pub bracket_spacing: Option<oxc_formatter::BracketSpacing>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[serde(
        alias = "bracketSameLine",
        default,
        deserialize_with = "deserialize_optional_via_def::<_, bool, _>"
    )]
    pub bracket_same_line: Option<oxc_formatter::BracketSameLine>,

    /// Attribute position style. By default auto.
    #[serde(
        alias = "singleAttributePerLine",
        default,
        deserialize_with = "deserialize_optional_from_str"
    )]
    pub attribute_position: Option<oxc_formatter::AttributePosition>,

    /// Whether to expand object and array literals to multiple lines. Defaults to "auto".
    #[serde(alias = "objectWrap", default, deserialize_with = "deserialize_optional_from_str")]
    pub expand: Option<oxc_formatter::Expand>,

    /// Controls the position of operators in binary expressions. [**NOT SUPPORTED YET**]
    ///
    /// Accepted values are:
    /// - `"start"`: Places the operator at the beginning of the next line.
    /// - `"end"`: Places the operator at the end of the current line (default).
    #[serde(
        alias = "experimentalOperatorPosition",
        default,
        deserialize_with = "deserialize_optional_from_str"
    )]
    pub experimental_operator_position: Option<oxc_formatter::OperatorPosition>,

    /// Try prettier's new ternary formatting before it becomes the default behavior. [**NOT SUPPORTED YET**]
    ///
    /// Valid options:
    /// - `true` - Use curious ternaries, with the question mark after the condition.
    /// - `false` - Retain the default behavior of ternaries; keep question marks on the same line as the consequent.
    #[serde(alias = "experimentalTernaries")]
    pub experimental_ternaries: Option<bool>,

    /// Enable formatting for embedded languages (e.g., CSS, SQL, GraphQL) within template literals. Defaults to "auto".
    #[serde(
        alias = "embeddedLanguageFormatting",
        default,
        deserialize_with = "deserialize_optional_from_str"
    )]
    pub embedded_language_formatting: Option<oxc_formatter::EmbeddedLanguageFormatting>,

    /// Sort import statements. By default disabled.
    #[serde(
        alias = "experimentalSortImports",
        default,
        deserialize_with = "deserialize_optional_via_def::<_, SortImportsOptionsDef, _>"
    )]
    pub experimental_sort_imports: Option<oxc_formatter::SortImportsOptions>,

    /// Enable Tailwind CSS class sorting in JSX class/className attributes.
    /// When enabled, class strings will be collected and passed to a callback for sorting.
    /// Defaults to None (disabled).
    #[serde(
        alias = "experimentalTailwindcss",
        default,
        deserialize_with = "deserialize_optional_via_def::<_, TailwindcssOptionsDef, _>"
    )]
    pub experimental_tailwindcss: Option<oxc_formatter::SortTailwindcssOptions>,
}

// =============================================================================
// Sort Imports Options
// =============================================================================

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

// =============================================================================
// Tailwindcss Options
// =============================================================================

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

impl From<TailwindcssOptionsDef> for oxc_formatter::SortTailwindcssOptions {
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

// =============================================================================
// Helper Traits
// =============================================================================

trait FromUseSingle {
    fn from_use_single(value: bool) -> Self;
}

impl FromUseSingle for oxc_formatter::QuoteStyle {
    fn from_use_single(value: bool) -> Self {
        if value {
            oxc_formatter::QuoteStyle::Single
        } else {
            oxc_formatter::QuoteStyle::Double
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn default_true() -> bool {
    true
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

/// Deserialize an optional value from a string using FromStr.
fn deserialize_optional_from_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    Option::<String>::deserialize(deserializer)?
        .map(|s| T::from_str(&s).map_err(serde::de::Error::custom))
        .transpose()
}

/// Deserialize an optional value through an intermediate Def type that implements Into<T>.
fn deserialize_optional_via_def<'de, D, Def, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    Def: Deserialize<'de> + Into<T>,
{
    let def = Option::<Def>::deserialize(deserializer)?;
    Ok(def.map(Into::into))
}

// =============================================================================
// Custom Deserialization Modules
// =============================================================================

/// Custom deserialization module for ImportSelector
mod import_selector {
    use serde::Deserialize;
    use serde::Deserializer;

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<oxc_formatter::ImportSelector>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => oxc_formatter::ImportSelector::parse(&s)
                .map(Some)
                .ok_or_else(|| serde::de::Error::custom(format!("Invalid import selector: {s}"))),
            None => Ok(None),
        }
    }
}

/// Custom deserialization module for ImportModifier
mod import_modifiers {
    use serde::de::{SeqAccess, Visitor};
    use serde::Deserializer;
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
                    match oxc_formatter::ImportModifier::parse(&name) {
                        Some(item) => items.push(item),
                        None => {
                            return Err(serde::de::Error::custom(format!(
                                "Invalid import modifier: {name}"
                            )));
                        }
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
    use serde::Deserialize;
    use serde::Deserializer;

    use super::ParsedGroups;

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
                            vec![oxc_formatter::GroupEntry::parse(&s)]
                        }
                        GroupItem::Multiple(v) => {
                            v.into_iter().map(|s| oxc_formatter::GroupEntry::parse(&s)).collect()
                        }
                        GroupItem::NewlinesBetween(_) => unreachable!(),
                    };
                    groups.push(entries);
                }
            }
        }

        Ok(ParsedGroups { groups, newline_boundary_overrides })
    }
}
