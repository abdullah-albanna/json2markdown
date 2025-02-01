use json2arkdown::MarkdownRenderer;

fn main() {
    let json = include_str!("./input.json");
    let json = serde_json::from_str(json).unwrap_or_else(|e| {
        panic!("couldn't convert the input into serde_json::Value, error: {e}")
    });

    let renderer = MarkdownRenderer::default();
    let markdown = renderer.render(&json);

    println!("{markdown}");
}
