use crate::ast::ASTNode;
use std::collections::BTreeMap;

pub(super) struct HelperMethod<'a> {
    pub box_name: &'a str,
    pub declaration: &'a ASTNode,
}

pub(super) struct StaticMainBox<'a> {
    pub body: &'a [ASTNode],
    pub helper_methods: Vec<HelperMethod<'a>>,
}

pub(super) fn find_static_main_box(ast: &ASTNode) -> Option<StaticMainBox<'_>> {
    let ASTNode::Program { statements, .. } = ast else {
        return None;
    };

    let mut main_body = None;
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
        main_body = Some(body.as_slice());
        break;
    }

    let mut helper_methods = BTreeMap::new();
    for statement in statements {
        let ASTNode::BoxDeclaration {
            name: box_name,
            methods,
            ..
        } = statement
        else {
            continue;
        };

        for declaration in methods.values() {
            let ASTNode::FunctionDeclaration {
                name: method_name,
                params,
                ..
            } = declaration
            else {
                continue;
            };
            if statement_is_static_main(statement) && method_name == "main" {
                continue;
            }
            let signature = format!("{}::{}:{}", box_name, method_name, params.len());
            helper_methods.insert(
                signature,
                HelperMethod {
                    box_name: box_name.as_str(),
                    declaration,
                },
            );
        }
    }

    Some(StaticMainBox {
        body: main_body?,
        helper_methods: helper_methods.into_values().collect(),
    })
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

pub(super) fn has_dev_local_alias_sugar(source: &str) -> bool {
    source
        .split_inclusive('\n')
        .any(dev_local_alias_line_matches)
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
    if !dev_local_alias_rest_matches(rest) {
        return line.to_string();
    }

    let local_decl = &rest[1..];
    format!("{indent}local {local_decl}{ending}")
}

fn dev_local_alias_line_matches(line: &str) -> bool {
    let (body, _) = split_line_ending(line);
    let indent_len = body
        .as_bytes()
        .iter()
        .take_while(|byte| matches!(**byte, b' ' | b'\t'))
        .count();
    dev_local_alias_rest_matches(&body[indent_len..])
}

fn dev_local_alias_rest_matches(rest: &str) -> bool {
    let Some(stripped) = rest.strip_prefix('@') else {
        return false;
    };

    let ident_len = stripped
        .as_bytes()
        .iter()
        .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
        .count();
    if ident_len == 0 {
        return false;
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
            return false;
        }
        cursor += type_len;
        while cursor < bytes.len() && matches!(bytes[cursor], b' ' | b'\t') {
            cursor += 1;
        }
    }
    cursor < bytes.len() && bytes[cursor] == b'='
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
    if let Some((target, alias)) = rest.rsplit_once(" as ") {
        return Some((target.trim(), alias.trim()));
    }

    let target = rest.trim();
    if target.is_empty() {
        return None;
    }
    Some((target, default_using_alias(target)))
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

fn default_using_alias(target: &str) -> &str {
    let clean = strip_wrapping_quotes(target);
    clean
        .rsplit_once('.')
        .map(|(_, tail)| tail)
        .or_else(|| clean.rsplit_once('/').map(|(_, tail)| tail))
        .unwrap_or(clean)
}

fn statement_is_static_main(statement: &ASTNode) -> bool {
    matches!(
        statement,
        ASTNode::BoxDeclaration {
            name,
            is_static: true,
            ..
        } if name == "Main"
    )
}

#[cfg(test)]
mod tests {
    use super::{collect_using_imports, has_dev_local_alias_sugar, preexpand_dev_local_aliases};
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
    fn has_dev_local_alias_sugar_detects_line_head_locals_only() {
        assert!(has_dev_local_alias_sugar("  @argc = 0\n"));
        assert!(has_dev_local_alias_sugar("@name: String = \"x\"\n"));
        assert!(!has_dev_local_alias_sugar("@hint(inline)\n"));
        assert!(!has_dev_local_alias_sugar("call(@argc)\n"));
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
            (
                "func_scanner".to_string(),
                "lang.compiler.entry.func_scanner".to_string(),
            ),
            ("StringHelpers".to_string(), "sh_core".to_string()),
        ]);
        assert_eq!(imports, expected);
    }
}
