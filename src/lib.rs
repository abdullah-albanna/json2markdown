use fancy_regex::Regex;
use inflections::Inflect;
use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use std::borrow::Cow;

/// A static cached regex that splits at a period only if it’s followed by whitespace
/// that is not immediately followed by an uppercase letter and a dot.
static SPLIT_REGEX: Lazy<Regex> = Lazy::new(|| {
    // This regex uses lookahead assertions supported by fancy_regex.
    Regex::new(r"\.(?=\s+(?![A-Z]\.))")
        .unwrap_or_else(|e| panic!("regex failed to build, error: {e}"))
});

/// Enum to represent different Markdown rendering styles.
#[derive(Clone, Copy, Debug)]
enum RenderStyle {
    /// Root-level rendering style.
    Root,
    /// Section-level rendering style (e.g., first-level headers).
    Section,
    /// Subsection-level rendering style (e.g., second-level headers).
    Subsection,
    /// List item rendering style.
    ListItem,
    /// Nested item rendering style (e.g., items inside a list).
    NestedItem,
}

#[derive(Clone, Copy, Debug)]
pub struct MarkdownRenderer {
    /// Number of spaces used for indentation in the rendered Markdown.
    indent_spaces: usize,
    /// Increment in depth for nested structures.
    depth_increment: usize,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self {
            indent_spaces: 1,
            depth_increment: 2,
        }
    }
}

impl MarkdownRenderer {
    /// Creates a new `MarkdownRenderer`.
    ///
    /// # Arguments
    ///
    /// * `indent_spaces` - Number of spaces to use for indentation.
    /// * `depth_increment` - Increment to apply for nested structures.
    ///
    /// # Examples
    ///
    /// ```
    /// let renderer = MarkdownRenderer::new(1, 2);
    /// ```
    #[must_use]
    pub const fn new(indent_spaces: usize, depth_increment: usize) -> Self {
        MarkdownRenderer {
            indent_spaces,
            depth_increment,
        }
    }

    /// Renders a JSON value into a Markdown string.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON value to render.
    ///
    /// # Errors
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let renderer = MarkdownRenderer::new(1, 2);
    /// let json = serde_json::json!({"title": "My Document"});
    /// let markdown = renderer.render(&json);
    /// ```
    #[must_use]
    pub fn render(&self, json: &Value) -> String {
        let mut output = String::with_capacity(4096); // Pre-allocate memory for large JSON
        self.render_value(json, 0, RenderStyle::Root, &mut output, false);
        output
    }

    /// Handles rendering of different JSON values based on their type.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to render.
    /// * `depth` - Current depth level in the hierarchy.
    /// * `style` - Current rendering style.
    /// * `output` - The output buffer to write the rendered Markdown.
    fn render_value(
        &self,
        value: &Value,
        depth: usize,
        style: RenderStyle,
        output: &mut String,
        written_before: bool,
    ) {
        match value {
            Value::Object(obj) => self.render_object(obj, depth, style, output),
            Value::Array(arr) => self.render_array(arr, depth, style, output),
            Value::String(s) => format_value(s, style, output, written_before),
            Value::Number(n) => format_value(&n.to_string(), style, output, written_before),
            Value::Bool(b) => format_value(&b.to_string(), style, output, written_before),
            Value::Null => format_value("N/A", style, output, written_before),
        }
    }

    fn render_object(
        &self,
        obj: &Map<String, Value>,
        depth: usize,
        style: RenderStyle,
        output: &mut String,
    ) {
        let indent = self.get_indent(depth);

        for (key, value) in obj {
            let (new_style, header_marker, depth_increment) = match (depth, style) {
                (0, RenderStyle::Root) => (RenderStyle::Section, "## ", 0),
                (1, RenderStyle::Section) => (RenderStyle::Subsection, "### ", 0),
                _ => (RenderStyle::ListItem, "", self.depth_increment),
            };

            let formatted_key = match new_style {
                RenderStyle::Section | RenderStyle::Subsection => {
                    format!("{indent}{header_marker}{}\n\n", key.to_title_case())
                }
                RenderStyle::ListItem => format!("{indent}- **{}**", key.to_title_case()),
                _ => key.to_title_case(),
            };

            output.push_str(&formatted_key);

            match value {
                Value::Object(inner_obj) if !inner_obj.is_empty() => {
                    output.push_str("\n\n");
                    self.render_object(inner_obj, depth + depth_increment, new_style, output);
                }
                Value::Array(arr) if !arr.is_empty() => {
                    output.push_str("\n\n");
                    self.render_array(
                        arr,
                        depth + depth_increment,
                        RenderStyle::NestedItem,
                        output,
                    );
                    output.push_str("\n\n");
                }
                Value::String(value) => {
                    output.push_str("\n\n");

                    // we don't touch it if it's a url
                    let s = if value.starts_with("http") {
                        format!("{indent}{value}")
                    } else {
                        let is_in_root = depth_increment == 0;
                        let adjusted_depth = if is_in_root { 0 } else { depth + 2 };

                        let formatted = self.split_at_period(value, adjusted_depth);

                        match formatted {
                            Cow::Owned(owned_s) => owned_s,
                            // we don't add the indent if we are in the root
                            Cow::Borrowed(s) if is_in_root => s.to_string(),
                            Cow::Borrowed(s) => format!("{indent}{s}"),
                        }
                    };

                    output.push_str(&s);
                    output.push('\n');
                }
                _ => {
                    self.render_value(
                        value,
                        depth + depth_increment,
                        RenderStyle::NestedItem,
                        output,
                        true,
                    );
                }
            }
        }
    }

    fn render_array(&self, arr: &[Value], depth: usize, style: RenderStyle, output: &mut String) {
        let indent = self.get_indent(depth);

        for item in arr {
            let marker = match style {
                RenderStyle::NestedItem => "  - ",
                _ => "- ",
            };

            // we only want to do a '-' if it's not an object or an array
            let mut do_hyphen = || output.push_str(&format!("{indent}{marker}"));

            match item {
                Value::Object(obj) if !obj.is_empty() => {
                    self.render_object(
                        obj,
                        depth + self.depth_increment,
                        RenderStyle::NestedItem,
                        output,
                    );
                }
                Value::Array(inner_arr) if !inner_arr.is_empty() => {
                    self.render_array(
                        inner_arr,
                        depth + self.depth_increment,
                        RenderStyle::NestedItem,
                        output,
                    );
                }
                Value::String(s) => {
                    do_hyphen();
                    output.push_str(&format!("{s}\n"));
                }
                _ => {
                    do_hyphen();
                    self.render_value(
                        item,
                        depth + self.depth_increment,
                        RenderStyle::NestedItem,
                        output,
                        false,
                    );
                }
            }
        }
    }

    fn get_indent(&self, depth: usize) -> String {
        " ".repeat(depth * self.indent_spaces)
    }

    /// splits the strings at '.' and adds 2 new lines for readability, we return the given if
    /// there is no '.'
    fn split_at_period<'a>(&self, text: &'a str, depth: usize) -> Cow<'a, str> {
        let indent = self.get_indent(depth);

        if !SPLIT_REGEX.is_match(text).is_ok_and(|b| b) {
            return Cow::Borrowed(text);
        }

        // Split using the regex.
        let splitted = SPLIT_REGEX
            .split(text)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|e| panic!("regex failed to split, error: {e}"));

        let capacity = (splitted.len() * indent.len() + 4) + text.len();

        Cow::Owned(
            splitted
                .into_iter()
                .fold(String::with_capacity(capacity), |mut acc, part| {
                    acc.push_str(&indent);
                    acc.push_str(part.trim());
                    acc.push_str("\n\n");
                    acc
                }),
        )
    }
}

fn format_value(value: &str, style: RenderStyle, output: &mut String, written_before: bool) {
    // we don't want to do ": " if there is nothing before
    let before_value = if written_before { ": " } else { "" };

    match style {
        RenderStyle::ListItem | RenderStyle::NestedItem => {
            output.push_str(&format!("{before_value}{value}\n"));
        }
        RenderStyle::Root | RenderStyle::Section | RenderStyle::Subsection => {
            output.push_str(&format!("{before_value}{value}\n\n"));
        }
    }
}
