use inflections::Inflect;
use serde_json::Value;
use std::fs;
use std::io::{self, Write};

const TAB: &str = "  ";

const LIST_TAG: &str = "* ";

const H_TAG: &str = "#";

fn load_json(file: &str) -> Result<Value, io::Error> {
    let data = fs::read_to_string(file)?;

    let json_data: Value =
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(json_data)
}

pub fn parse_json(json_block: &Value, depth: usize, markdown: &mut String) {
    match json_block {
        Value::Object(d) => parse_dict(d, depth, markdown),

        Value::Array(l) => parse_list(l, depth, markdown),

        _ => {}
    }
}

fn parse_dict(d: &serde_json::Map<String, Value>, depth: usize, markdown: &mut String) {
    for (k, v) in d {
        if v.is_object() || v.is_array() {
            add_header(k, depth, markdown);

            parse_json(v, depth + 1, markdown);
        } else {
            add_value(k, v, depth, markdown);
        }
    }
}

fn parse_list(l: &[Value], depth: usize, markdown: &mut String) {
    for (index, value) in l.iter().enumerate() {
        if !value.is_object() && !value.is_array() {
            add_value(&index.to_string(), value, depth, markdown);
        } else {
            parse_dict(value.as_object().unwrap(), depth, markdown);
        }
    }
}

fn build_header_chain(depth: usize, value: &str) -> String {
    let header = format!(
        "{}{} value {}\n",
        if depth > 0 { LIST_TAG } else { "" },
        H_TAG.repeat(depth + 1),
        H_TAG.repeat(depth + 1)
    );

    header.replace("value", &value.to_title_case())
}

fn build_value_chain(key: &str, value: &Value, depth: usize) -> String {
    format!(
        "{}{}: {}\n",
        TAB.repeat(depth.saturating_sub(1)),
        LIST_TAG,
        value
    )
}

fn add_header(value: &str, depth: usize, markdown: &mut String) {
    let chain = build_header_chain(depth, value);

    markdown.push_str(&chain);
}

fn add_value(key: &str, value: &Value, depth: usize, markdown: &mut String) {
    let chain = build_value_chain(key, value, depth);

    markdown.push_str(&chain);
}

fn write_out(markdown: &str, output_file: &str) -> Result<(), io::Error> {
    let mut file = fs::File::create(output_file)?;

    file.write_all(markdown.as_bytes())?;

    Ok(())
}

fn just_do_it(input_file: &str, output_file: &str) -> Result<(), io::Error> {
    let json_data = load_json(input_file)?;

    let mut markdown = String::new();

    parse_json(&json_data, 0, &mut markdown);

    markdown = markdown.replace("#######", "######");

    write_out(&markdown, output_file)
}
