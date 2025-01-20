use json2arkdown::MarkdownRenderer;
use serde_json::Value;

fn main() {
    let json: Value = serde_json::from_str(r#"{"title": "My Project"}"#).unwrap();

    let renderer = MarkdownRenderer::new(1, 2);
    let markdown = renderer.render(&json);

    println!("{}", markdown);
}
