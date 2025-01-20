# JSON to Markdown Renderer in Rust

## Overview

A powerful and flexible Rust library for converting complex JSON structures into beautifully formatted Markdown documents. This library provides a highly customizable renderer that can transform nested JSON objects into clean, hierarchical markdown.

## Features

- 🚀 Supports deeply nested JSON structures
- 📝 Configurable indentation and rendering styles
- 🔍 Handles various JSON value types (objects, arrays, strings, numbers, booleans)
- 🧩 Modular design with easy extensibility

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
json2markdown = "0.1.0"
serde_json = "1"
```

## Basic Usage

```rust
use serde_json::Value;
use json2markdown::MarkdownRenderer;

fn main() {
    let json: Value = serde_json::from_str(r#"{"title": "My Project"}"#).unwrap();

    let renderer = MarkdownRenderer::new(1, 2);
    let markdown = renderer.render(&json);

    println!("{}", markdown);
}
```

## Customization

### Indentation

You can customize indentation by adjusting the parameters:

```rust
// 1 space base indentation, 2 spaces for nested items
let renderer = MarkdownRenderer::new(1, 2);
```

## Advanced Example

```rust
let complex_json = serde_json::json!({
    "project": {
        "name": "Advanced Project",
        "goals": [
            "Improve efficiency",
            "Reduce manual work"
        ],
        "team": {
            "size": 5,
            "roles": ["Developer", "Designer"]
        }
    }
});

let renderer = MarkdownRenderer::new(1, 2);
let markdown = renderer.render(&complex_json);
```

## Performance Considerations

- The renderer is designed to handle large, complex JSON structures
- Memory usage scales with JSON complexity
- Recommended for medium to large JSON documents

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

MIT

## Roadmap

- [ ] Add support for custom markdown formatting
- [ ] Implement more rendering options
- [ ] Create configuration presets
- [ ] Improve performance for very large JSON structures
- [ ] Improve the indentations to suit the style
