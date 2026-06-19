use std::collections::HashSet;

use serde_json::{json, Value};
use syn::Path;

use crate::cli::fail;

const HAKO_RESERVED: &[&str] = &[
    "as", "box", "break", "continue", "else", "enum", "false", "function", "if", "in", "local",
    "loop", "match", "me", "null", "record", "return", "static", "true", "type", "using",
];

pub(crate) fn emitted_ident(source: &str) -> String {
    let source = logical_ident(source);
    let mut out = String::new();
    for (index, ch) in source.chars().enumerate() {
        if (index == 0 && is_ident_start(ch)) || (index > 0 && is_ident_continue(ch)) {
            out.push(ch);
        } else if ch.is_ascii_digit() && index == 0 {
            out.push('_');
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push('_');
    }
    if HAKO_RESERVED.contains(&out.as_str()) {
        format!("rust_{out}")
    } else {
        out
    }
}

pub(crate) fn emitted_path(path: &Path) -> String {
    let segments = path_segments(path);
    if segments.is_empty() {
        "unsupported_path".to_string()
    } else {
        emitted_ident(&segments.join("_"))
    }
}

pub(crate) fn path_segments(path: &Path) -> Vec<String> {
    path.segments
        .iter()
        .map(|segment| logical_ident(&segment.ident.to_string()).to_string())
        .collect()
}

pub(crate) fn name_metadata(source: &str) -> Value {
    let source = logical_ident(source);
    let emitted = emitted_ident(source);
    if emitted == source {
        json!({"name": emitted})
    } else {
        json!({
            "name": emitted,
            "source_name": source,
            "emitted_name": emitted,
        })
    }
}

pub(crate) fn insert_name_metadata(value: &mut Value, source: &str) {
    if let Value::Object(target) = value {
        if let Value::Object(source_fields) = name_metadata(source) {
            for (key, field_value) in source_fields {
                target.insert(key, field_value);
            }
        }
    }
}

pub(crate) fn insert_path_name_metadata(value: &mut Value, path: &Path) {
    if let Value::Object(target) = value {
        if let Value::Object(source_fields) = path_name_metadata(path) {
            for (key, field_value) in source_fields {
                target.insert(key, field_value);
            }
        }
    }
}

pub(crate) fn assert_unique_names(values: &[Value], context: &str) {
    let mut seen = HashSet::new();
    for value in values {
        let Some(name) = value.get("name").and_then(Value::as_str) else {
            continue;
        };
        if !seen.insert(name.to_string()) {
            fail(format!("duplicate emitted_name in {context}: {name}"));
        }
    }
}

pub(crate) fn path_name_metadata(path: &Path) -> Value {
    let segments = path_segments(path);
    let emitted = if segments.len() == 1 {
        emitted_ident(&segments[0])
    } else {
        emitted_path(path)
    };
    let source = segments.last().cloned().unwrap_or_default();
    if segments.len() == 1 && emitted == source {
        json!({"name": emitted})
    } else {
        json!({
            "name": emitted,
            "source_name": source,
            "emitted_name": emitted,
            "source_path": segments,
        })
    }
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn logical_ident(source: &str) -> &str {
    source.strip_prefix("r#").unwrap_or(source)
}
