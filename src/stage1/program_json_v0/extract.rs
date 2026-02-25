use crate::ast::ASTNode;

pub(super) fn find_static_main_body(ast: &ASTNode) -> Option<&[ASTNode]> {
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

        return Some(body.as_slice());
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
