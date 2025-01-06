use inflections::Inflect;
use serde_json::{Map, Value};

#[derive(Clone, Copy)]
enum RenderStyle {
    Root,
    Section,
    Subsection,
    ListItem,
    NestedItem,
}

pub struct MarkdownRenderer {
    indent_spaces: usize,
}

impl MarkdownRenderer {
    /// the amount of indentation to do
    ///
    /// 1 looks nice
    pub fn new(indent_spaces: usize) -> Self {
        MarkdownRenderer { indent_spaces }
    }

    pub fn render(&self, json: &Value) -> String {
        let mut output = String::new();

        output.push_str(&self.render_value(json, 0, RenderStyle::Root));

        output
    }

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

    fn render_object(&self, obj: &Map<String, Value>, depth: usize, style: RenderStyle) -> String {
        let mut markdown = String::new();

        let indent = " ".repeat(depth * self.indent_spaces);

        //let nested_indent = " ".repeat((depth + 1) * 2);

        for (key, value) in obj {
            // Determine appropriate header or list style based on depth and content

            let (new_style, header_marker) = match (depth, style) {
                (0, RenderStyle::Root) => (RenderStyle::Section, "## "),

                (1, RenderStyle::Section) => (RenderStyle::Subsection, "### "),

                _ => (RenderStyle::ListItem, ""),
            };

            // Format the key

            let formatted_key = match new_style {
                RenderStyle::Section | RenderStyle::Subsection => {
                    format!("{}{}{}\n\n", indent, header_marker, key.to_title_case())
                }

                RenderStyle::ListItem => format!("{}- **{}**", indent, key),

                _ => key.to_string(),
            };

            markdown.push_str(&formatted_key);

            // Render the value

            match value {
                Value::Object(inner_obj) if !inner_obj.is_empty() => {
                    markdown.push_str("\n\n");

                    markdown.push_str(&self.render_object(inner_obj, depth + 2, new_style));

                    markdown.push_str("\n\n");
                }

                Value::Array(arr) if !arr.is_empty() => {
                    markdown.push_str("\n\n");

                    markdown.push_str(&self.render_array(arr, depth + 1, RenderStyle::NestedItem));

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
                        depth + 1,
                        RenderStyle::NestedItem,
                    ));
                }
            }
        }

        markdown
    }

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
                Value::Object(obj) if !obj.is_empty() => {
                    self.render_object(obj, depth + 2, RenderStyle::NestedItem)
                }

                Value::Array(inner_arr) if !inner_arr.is_empty() => {
                    self.render_array(inner_arr, depth + 2, RenderStyle::NestedItem)
                }

                Value::String(s) => {
                    format!("{}\n", s)
                }

                _ => self.render_value(item, depth + 2, RenderStyle::NestedItem),
            });
        }

        markdown
    }

    fn format_value(&self, value: &str, style: RenderStyle) -> String {
        match style {
            RenderStyle::ListItem | RenderStyle::NestedItem => format!("{}\n", value),

            RenderStyle::Root | RenderStyle::Section | RenderStyle::Subsection => {
                format!("{}\n\n", value)
            }
        }
    }
}
