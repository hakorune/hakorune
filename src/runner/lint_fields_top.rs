//! Source-structural "fields-at-top of box" lint.
//!
//! Extracted from runner/pipeline.rs as a single-concern peer module
//! (matches the box_index.rs / build.rs / emit.rs peer layout).

/// Lint: enforce "fields must be at the top of box" rule.
/// - Warns by default (when verbose); when `strict` is true, returns Err on any violation.
pub(crate) fn lint_fields_top(code: &str, strict: bool, verbose: bool) -> Result<(), String> {
    let mut brace: i32 = 0;
    let mut in_box = false;
    let mut box_depth: i32 = 0;
    let mut seen_method = false;
    let mut cur_box: String = String::new();
    let mut violations: Vec<(usize, String, String)> = Vec::new(); // (line, field, box)

    for (idx, line) in code.lines().enumerate() {
        let lno = idx + 1;
        let pre_brace = brace;
        let trimmed = line.trim();
        // Count braces for this line
        let opens = line.matches('{').count() as i32;
        let closes = line.matches('}').count() as i32;

        // Enter box on same-line K&R style: `box Name {` or `static box Name {`
        if !in_box && trimmed.starts_with("box ") || trimmed.starts_with("static box ") {
            // capture name
            let mut name = String::new();
            let after = if let Some(rest) = trimmed.strip_prefix("static box ") {
                rest
            } else {
                trimmed.strip_prefix("box ").unwrap_or("")
            };
            for ch in after.chars() {
                if ch.is_alphanumeric() || ch == '_' {
                    name.push(ch);
                } else {
                    break;
                }
            }
            // require K&R brace on same line to start tracking
            if opens > 0 {
                in_box = true;
                cur_box = name;
                box_depth = pre_brace + 1; // assume one level for box body
                seen_method = false;
            }
        }

        if in_box {
            // Top-level inside box only
            if pre_brace == box_depth {
                // Skip empty/comment lines
                if !trimmed.is_empty() && !trimmed.starts_with("//") {
                    // Detect method: name(args) {
                    let is_method = {
                        // starts with identifier then '(' and later '{'
                        let mut it = trimmed.chars();
                        let mut ident = String::new();
                        while let Some(c) = it.next() {
                            if c.is_whitespace() {
                                continue;
                            }
                            if c.is_alphabetic() || c == '_' {
                                ident.push(c);
                                break;
                            } else {
                                break;
                            }
                        }
                        while let Some(c) = it.next() {
                            if c.is_alphanumeric() || c == '_' {
                                ident.push(c);
                            } else {
                                break;
                            }
                        }
                        trimmed.contains('(') && trimmed.ends_with('{') && !ident.is_empty()
                    };
                    if is_method {
                        seen_method = true;
                    }

                    // Detect field: ident ':' Type (rough heuristic)
                    let is_field = {
                        let parts: Vec<&str> = trimmed.split(':').collect();
                        if parts.len() == 2 {
                            let lhs = parts[0].trim();
                            let rhs = parts[1].trim();
                            let lhs_ok = !lhs.is_empty()
                                && lhs
                                    .chars()
                                    .next()
                                    .map(|c| c.is_alphabetic() || c == '_')
                                    .unwrap_or(false);
                            let rhs_ok = !rhs.is_empty()
                                && rhs
                                    .chars()
                                    .next()
                                    .map(|c| c.is_alphabetic() || c == '_')
                                    .unwrap_or(false);
                            lhs_ok && rhs_ok && !trimmed.contains('(') && !trimmed.contains(')')
                        } else {
                            false
                        }
                    };
                    if is_field && seen_method {
                        violations.push((lno, trimmed.to_string(), cur_box.clone()));
                    }
                }
            }
            // Exit box when closing brace reduces depth below box_depth
            let post_brace = pre_brace + opens - closes;
            if post_brace < box_depth {
                in_box = false;
                cur_box.clear();
            }
        }

        // Update brace after processing
        brace += opens - closes;
    }

    if violations.is_empty() {
        return Ok(());
    }
    if strict {
        // Compose error message
        let mut msg =
            String::from("Field declarations must appear at the top of box. Violations:\n");
        for (lno, fld, bx) in violations.iter().take(10) {
            msg.push_str(&format!(
                "  line {} in box {}: '{}",
                lno,
                if bx.is_empty() { "<unknown>" } else { bx },
                fld
            ));
            msg.push_str("'\n");
        }
        if violations.len() > 10 {
            msg.push_str(&format!("  ... and {} more\n", violations.len() - 10));
        }
        return Err(msg);
    }
    if verbose || crate::config::env::env_bool("NYASH_RESOLVE_TRACE") {
        for (lno, fld, bx) in violations {
            eprintln!(
                "[lint] fields-top: line {} in box {} -> {}",
                lno,
                if bx.is_empty() { "<unknown>" } else { &bx },
                fld
            );
        }
    }
    Ok(())
}
