use inflections::Inflect;
use serde_json::{Map, Value};

/// Enum to represent different Markdown rendering styles.
///
/// This enum is also used to indicate where we are in the rendering process.
#[derive(Clone, Copy)]
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

/// A struct to handle rendering JSON into Markdown format.
///
/// The `MarkdownRenderer` provides customizable options for indentation and depth handling.
pub struct MarkdownRenderer {
    /// Number of spaces used for indentation in the rendered Markdown.
    indent_spaces: usize,
    /// Increment in depth for nested structures.
    depth_increment: usize,
}

impl MarkdownRenderer {
    /// Creates a new `MarkdownRenderer`.
    ///
    /// # Arguments
    ///
    /// * `indent_spaces` - Number of spaces to use for indentation. Always try to use `1` at first.
    /// * `depth_increment` - Increment to apply for nested structures. Always try to use `2` at first
    ///
    ///
    /// You can experiment with different values, it still not perfect.
    ///
    /// # Examples
    ///
    /// ```
    /// let renderer = MarkdownRenderer::new(1, 2);
    /// ```
    pub fn new(indent_spaces: usize, depth_increment: usize) -> Self {
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
    /// # Returns
    ///
    /// A `String` containing the rendered Markdown.
    ///
    /// # Examples
    ///
    /// ```
    /// let renderer = MarkdownRenderer::new(1, 2);
    /// let json = serde_json::json!({"title": "My Document"});
    /// let markdown = renderer.render(&json);
    /// assert_eq!(markdown, "");
    /// ```
    pub fn render(&self, json: &Value) -> String {
        let mut output = String::new();
        output.push_str(&self.render_value(json, 0, RenderStyle::Root));
        output
    }

    /// Handles rendering of different JSON values based on their type.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to render.
    /// * `depth` - Current depth level in the hierarchy.
    /// * `style` - Current rendering style.
    fn render_value(&self, value: &Value, depth: usize, style: RenderStyle) -> String {
        match value {
            Value::Object(obj) => self.render_object(obj, depth, style),
            Value::Array(arr) => self.render_array(arr, depth, style),
            Value::String(s) => self.format_value(s, style),
            Value::Number(n) => self.format_value(&n.to_string(), style),
            Value::Bool(b) => self.format_value(&b.to_string(), style),
            Value::Null => self.format_value("N/A", style),
        }
    }

    /// Renders a JSON object into Markdown format.
    ///
    /// # Arguments
    ///
    /// * `obj` - The JSON object to render.
    /// * `depth` - Current depth level.
    /// * `style` - Rendering style for the object.
    fn render_object(&self, obj: &Map<String, Value>, depth: usize, style: RenderStyle) -> String {
        let mut markdown = String::new();
        let indent = " ".repeat(depth * self.indent_spaces);

        for (key, value) in obj {
            let (new_style, header_marker) = match (depth, style) {
                (0, RenderStyle::Root) => (RenderStyle::Section, "## "),
                (1, RenderStyle::Section) => (RenderStyle::Subsection, "### "),
                _ => (RenderStyle::ListItem, ""),
            };

            let formatted_key = match new_style {
                RenderStyle::Section | RenderStyle::Subsection => {
                    format!("{}{}{}\n\n", indent, header_marker, key.to_title_case())
                }
                RenderStyle::ListItem => format!("{}- **{}**", indent, key),
                _ => key.to_string(),
            };

            markdown.push_str(&formatted_key);

            match value {
                Value::Object(inner_obj) if !inner_obj.is_empty() => {
                    markdown.push_str("\n\n");
                    markdown.push_str(&self.render_object(
                        inner_obj,
                        depth + self.depth_increment,
                        new_style,
                    ));
                }
                Value::Array(arr) if !arr.is_empty() => {
                    markdown.push_str("\n\n");
                    markdown.push_str(&self.render_array(
                        arr,
                        depth + self.depth_increment,
                        RenderStyle::NestedItem,
                    ));
                    markdown.push_str("\n\n");
                }
                Value::String(s) => {
                    markdown.push_str("\n\n");
                    markdown.push_str(&format!("{}   {}", indent, s));
                    markdown.push('\n');
                }
                _ => {
                    markdown.push_str(&self.render_value(
                        value,
                        depth + self.depth_increment,
                        RenderStyle::NestedItem,
                    ));
                }
            }
        }

        markdown
    }

    /// Renders a JSON array into Markdown format.
    ///
    /// # Arguments
    ///
    /// * `arr` - The JSON array to render.
    /// * `depth` - Current depth level.
    /// * `style` - Rendering style for the array.
    fn render_array(&self, arr: &[Value], depth: usize, style: RenderStyle) -> String {
        let indent = " ".repeat(depth * self.indent_spaces);
        let mut markdown = String::new();

        for item in arr.iter() {
            let marker = match style {
                RenderStyle::ListItem => "- ",
                RenderStyle::NestedItem => "  - ",
                _ => "- ",
            };

            markdown.push_str(&format!("{}{}", indent, marker));
            markdown.push_str(&match item {
                Value::Object(obj) if !obj.is_empty() => self.render_object(
                    obj,
                    depth + self.depth_increment + 1,
                    RenderStyle::NestedItem,
                ),
                Value::Array(inner_arr) if !inner_arr.is_empty() => self.render_array(
                    inner_arr,
                    depth + self.depth_increment + 1,
                    RenderStyle::NestedItem,
                ),
                Value::String(s) => {
                    format!("{}\n", s)
                }
                _ => self.render_value(
                    item,
                    depth + self.depth_increment + 1,
                    RenderStyle::NestedItem,
                ),
            });
        }

        markdown
    }

    /// Formats a value based on the rendering style.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to format as a string.
    /// * `style` - The rendering style to apply.
    fn format_value(&self, value: &str, style: RenderStyle) -> String {
        match style {
            RenderStyle::ListItem | RenderStyle::NestedItem => format!("{}\n", value),
            RenderStyle::Root | RenderStyle::Section | RenderStyle::Subsection => {
                format!("{}\n\n", value)
            }
        }
    }
}
