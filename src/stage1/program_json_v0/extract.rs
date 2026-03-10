use crate::ast::ASTNode;
use std::collections::BTreeMap;

pub(super) struct StaticMainBox<'a> {
    pub body: &'a [ASTNode],
    pub helper_methods: Vec<&'a ASTNode>,
}

pub(super) fn find_static_main_box(ast: &ASTNode) -> Option<StaticMainBox<'_>> {
    let ASTNode::Program { statements, .. } = ast else {
        return None;
    };

    for statement in statements.iter().rev() {
        let ASTNode::BoxDeclaration {
            name,
            is_static,
            methods,
            ..
        } = statement
        else {
            continue;
        };

        if !*is_static || name != "Main" {
            continue;
        }

        let declaration = methods
            .get("main/0")
            .or_else(|| methods.get("main"))
            .or_else(|| {
                methods
                    .iter()
                    .find(|(key, _)| key.starts_with("main/"))
                    .map(|(_, value)| value)
            })?;

        let ASTNode::FunctionDeclaration { body, .. } = declaration else {
            return None;
        };

        let mut helper_methods = BTreeMap::new();
        for declaration in methods.values() {
            let ASTNode::FunctionDeclaration {
                name, params, ..
            } = declaration
            else {
                continue;
            };
            if name == "main" {
                continue;
            }
            let signature = format!("{}/{}", name, params.len());
            helper_methods.entry(signature).or_insert(declaration);
        }

        return Some(StaticMainBox {
            body: body.as_slice(),
            helper_methods: helper_methods.into_values().collect(),
        });
    }

    None
}

pub(super) fn extract_static_main_body_text(source: &str) -> Option<String> {
    let box_pos = source.rfind("static box Main")?;
    let main_pos_rel = source[box_pos..].find("main(")?;
    let main_pos = box_pos + main_pos_rel;
    let open_brace = find_next_uncommented_char(source, main_pos, '{')?;
    let close_brace = find_matching_brace(source, open_brace)?;
    if close_brace <= open_brace + 1 {
        return Some(String::new());
    }
    Some(source[open_brace + 1..close_brace].to_string())
}

pub(super) fn collect_using_imports(source: &str) -> BTreeMap<String, String> {
    let mut imports = BTreeMap::new();

    for raw_line in source.lines() {
        let line = strip_line_comment(raw_line).trim();
        let Some(rest) = line.strip_prefix("using ") else {
            continue;
        };
        let Some((target, alias)) = split_using_alias(rest) else {
            continue;
        };
        if target.is_empty() || alias.is_empty() {
            continue;
        }
        imports.insert(alias.to_string(), strip_wrapping_quotes(target).to_string());
    }

    imports
}

pub(super) fn preexpand_dev_local_aliases(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    for raw_line in source.split_inclusive('\n') {
        out.push_str(&preexpand_dev_local_alias_line(raw_line));
    }
    out
}

fn preexpand_dev_local_alias_line(line: &str) -> String {
    let (body, ending) = split_line_ending(line);
    let indent_len = body
        .as_bytes()
        .iter()
        .take_while(|byte| matches!(**byte, b' ' | b'\t'))
        .count();
    let indent = &body[..indent_len];
    let rest = &body[indent_len..];
    let Some(stripped) = rest.strip_prefix('@') else {
        return line.to_string();
    };

    let ident_len = stripped
        .as_bytes()
        .iter()
        .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
        .count();
    if ident_len == 0 {
        return line.to_string();
    }

    let mut cursor = ident_len;
    let bytes = stripped.as_bytes();
    while cursor < bytes.len() && matches!(bytes[cursor], b' ' | b'\t') {
        cursor += 1;
    }
    if cursor < bytes.len() && bytes[cursor] == b':' {
        cursor += 1;
        while cursor < bytes.len() && matches!(bytes[cursor], b' ' | b'\t') {
            cursor += 1;
        }
        let type_len = stripped[cursor..]
            .as_bytes()
            .iter()
            .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
            .count();
        if type_len == 0 {
            return line.to_string();
        }
        cursor += type_len;
        while cursor < bytes.len() && matches!(bytes[cursor], b' ' | b'\t') {
            cursor += 1;
        }
    }
    if cursor >= bytes.len() || bytes[cursor] != b'=' {
        return line.to_string();
    }

    let local_decl = &rest[1..];
    format!("{indent}local {local_decl}{ending}")
}

fn split_line_ending(line: &str) -> (&str, &str) {
    if let Some(body) = line.strip_suffix("\r\n") {
        return (body, "\r\n");
    }
    if let Some(body) = line.strip_suffix('\n') {
        return (body, "\n");
    }
    (line, "")
}

fn split_using_alias(rest: &str) -> Option<(&str, &str)> {
    let (target, alias) = rest.rsplit_once(" as ")?;
    Some((target.trim(), alias.trim()))
}

fn strip_wrapping_quotes(text: &str) -> &str {
    if text.len() >= 2 && text.starts_with('"') && text.ends_with('"') {
        &text[1..text.len() - 1]
    } else {
        text
    }
}

fn strip_line_comment(line: &str) -> &str {
    let mut in_string = false;
    let mut escaped = false;

    for (index, ch) in line.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
            continue;
        }
        if ch == '/' && line[index..].starts_with("//") {
            return &line[..index];
        }
    }

    line
}

fn find_next_uncommented_char(source: &str, start: usize, needle: char) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = start;
    let mut in_string = false;
    let mut escaped = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    while index < bytes.len() {
        let ch = bytes[index] as char;
        let next = if index + 1 < bytes.len() {
            Some(bytes[index + 1] as char)
        } else {
            None
        };

        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            index += 1;
            continue;
        }
        if in_block_comment {
            if ch == '*' && next == Some('/') {
                in_block_comment = false;
                index += 2;
                continue;
            }
            index += 1;
            continue;
        }
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            index += 1;
            continue;
        }

        if ch == '/' && next == Some('/') {
            in_line_comment = true;
            index += 2;
            continue;
        }
        if ch == '/' && next == Some('*') {
            in_block_comment = true;
            index += 2;
            continue;
        }
        if ch == '"' {
            in_string = true;
            index += 1;
            continue;
        }
        if ch == needle {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn find_matching_brace(source: &str, open_pos: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    if open_pos >= bytes.len() || bytes[open_pos] != b'{' {
        return None;
    }
    let mut index = open_pos + 1;
    let mut depth = 1;
    let mut in_string = false;
    let mut escaped = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while index < bytes.len() {
        let ch = bytes[index] as char;
        let next = if index + 1 < bytes.len() {
            Some(bytes[index + 1] as char)
        } else {
            None
        };

        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            index += 1;
            continue;
        }
        if in_block_comment {
            if ch == '*' && next == Some('/') {
                in_block_comment = false;
                index += 2;
                continue;
            }
            index += 1;
            continue;
        }
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            index += 1;
            continue;
        }

        if ch == '/' && next == Some('/') {
            in_line_comment = true;
            index += 2;
            continue;
        }
        if ch == '/' && next == Some('*') {
            in_block_comment = true;
            index += 2;
            continue;
        }
        if ch == '"' {
            in_string = true;
            index += 1;
            continue;
        }
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
        }
        index += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{collect_using_imports, preexpand_dev_local_aliases};
    use std::collections::BTreeMap;

    #[test]
    fn preexpand_dev_local_aliases_rewrites_line_head_locals_only() {
        let source = "  @argc = 0\n@name: String = \"x\"\n@hint(inline)\ncall(@argc)\n";
        let expanded = preexpand_dev_local_aliases(source);
        assert!(expanded.contains("  local argc = 0\n"));
        assert!(expanded.contains("local name: String = \"x\"\n"));
        assert!(expanded.contains("@hint(inline)\n"));
        assert!(expanded.contains("call(@argc)\n"));
    }

    #[test]
    fn collect_using_imports_keeps_alias_to_namespace_path() {
        let source = r#"
using lang.compiler.build.build_box as BuildBox
using "apps/foo/bar.hako" as BarBox
using sh_core as StringHelpers // comment
using lang.compiler.entry.func_scanner
"#;

        let imports = collect_using_imports(source);
        let expected = BTreeMap::from([
            ("BarBox".to_string(), "apps/foo/bar.hako".to_string()),
            (
                "BuildBox".to_string(),
                "lang.compiler.build.build_box".to_string(),
            ),
            ("StringHelpers".to_string(), "sh_core".to_string()),
        ]);
        assert_eq!(imports, expected);
    }
}
