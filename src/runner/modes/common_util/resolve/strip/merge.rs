use crate::runner::NyashRunner;

use super::prelude::resolve_prelude_paths_profiled;
use super::using::collect_using_and_strip;

struct TextMergePlan {
    merged: String,
    imports: std::collections::HashMap<String, String>,
}

/// Legacy/compatibility helper: merge prelude ASTs with the main AST into a single Program node.
/// - Collects statements from each prelude Program in order, then appends
///   statements from the main Program.
/// - If the main AST is not a Program, returns it unchanged (defensive).
pub fn merge_prelude_asts_with_main(
    prelude_asts: Vec<nyash_rust::ast::ASTNode>,
    main_ast: &nyash_rust::ast::ASTNode,
) -> nyash_rust::ast::ASTNode {
    use nyash_rust::ast::{ASTNode, Span};
    let mut combined: Vec<ASTNode> = Vec::new();
    for a in prelude_asts.into_iter() {
        if let ASTNode::Program { statements, .. } = a {
            combined.extend(statements);
        }
    }
    if let ASTNode::Program { statements, .. } = main_ast.clone() {
        let mut all = combined;
        all.extend(statements);
        ASTNode::Program {
            statements: all,
            span: Span::unknown(),
        }
    } else {
        // Defensive: unexpected shape; preserve main AST unchanged.
        main_ast.clone()
    }
}

/// Text-based prelude merge: the default route for `using` expansion.
/// Recursively resolves using dependencies, strips using lines from each file,
/// and concatenates prelude text followed by main source text.
/// Returns merged source text ready for compilation.
pub fn merge_prelude_text(
    runner: &NyashRunner,
    source: &str,
    filename: &str,
) -> Result<String, String> {
    Ok(plan_text_merge(runner, source, filename)?.merged)
}

/// Text-based prelude merge plus explicit imported static-box bindings.
///
/// The returned `imports` map is the Layer 2 runner binding table for
/// `using ... as Alias` after strip/text-merge has removed the original
/// `using` lines. Manifest ownership stays in hako.toml; the MIR builder
/// consumes this table when lowering `Alias.method(...)`.
pub fn merge_prelude_text_with_imports(
    runner: &NyashRunner,
    source: &str,
    filename: &str,
) -> Result<(String, std::collections::HashMap<String, String>), String> {
    let plan = plan_text_merge(runner, source, filename)?;
    Ok((plan.merged, plan.imports))
}

fn plan_text_merge(
    runner: &NyashRunner,
    source: &str,
    filename: &str,
) -> Result<TextMergePlan, String> {
    let trace = crate::config::env::resolve_trace();

    // First pass: collect and resolve prelude paths
    let (cleaned_main, _prelude_paths_direct, main_imports) =
        collect_using_and_strip(runner, source, filename)?;
    let (_cleaned_ignore, prelude_paths_profiled) =
        resolve_prelude_paths_profiled(runner, source, filename)?;
    debug_assert_eq!(cleaned_main, _cleaned_ignore);
    // Expand nested preludes for text-merge too (DFS) so that any `using`
    // inside prelude files (e.g., runner_min -> lower_* boxes) are also
    // included even when NYASH_USING_AST is OFF.
    let mut expanded: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut imports = main_imports;
    for p in prelude_paths_profiled.iter() {
        dfs_text_with_imports(runner, p, &mut expanded, &mut seen, &mut imports)?;
    }
    let prelude_paths = &expanded;
    // Record for enriched diagnostics (parse error context)
    crate::runner::modes::common_util::resolve::set_last_merged_preludes(prelude_paths.clone());

    if prelude_paths.is_empty() {
        // No using statements, return original
        return Ok(TextMergePlan {
            merged: source.to_string(),
            imports,
        });
    }

    if trace {
        crate::runner::trace::log(format!(
            "[using/text-merge] {} prelude files for '{}'",
            prelude_paths.len(),
            filename
        ));
    }

    // Build merged text: preludes first, then main source
    let mut merged = String::new();
    let mut spans: Vec<crate::runner::modes::common_util::resolve::LineSpan> = Vec::new();
    let mut current_line: usize = 1;

    // Add preludes in DFS order
    for (idx, path) in prelude_paths.iter().enumerate() {
        // Phase 90-A: fs 系移行
        let ring0 = crate::runtime::ring0::get_global_ring0();
        let content = ring0
            .fs
            .read_to_string(std::path::Path::new(path))
            .map_err(|e| format!("using: failed to read '{}': {}", path, e))?;

        // Strip using lines from prelude and normalize
        let (cleaned_raw, _nested, _nested_imports) =
            collect_using_and_strip(runner, &content, path)?;
        let mut cleaned = normalize_text_for_inline(&cleaned_raw);
        // Hako-friendly normalize for preludes: always strip leading `local ` at line head
        // when the prelude is a .hako (or looks like Hako code). This prevents top-level
        // `local` from tripping the Nyash parser after text merge.
        if path.ends_with(".hako")
            || crate::runner::modes::common_util::hako::looks_like_hako_code(&cleaned)
        {
            cleaned = crate::runner::modes::common_util::hako::strip_local_decl(&cleaned);
        }

        if trace {
            crate::runner::trace::log(format!(
                "[using/text-merge] [{}] '{}' ({} bytes)",
                idx + 1,
                path,
                cleaned.len()
            ));
        }

        merged.push_str(&cleaned);
        merged.push('\n');

        let added = cleaned.lines().count();
        if added > 0 {
            spans.push(crate::runner::modes::common_util::resolve::LineSpan {
                file: path.clone(),
                start_line: current_line,
                line_count: added,
            });
            current_line += added + 1; // +1 for extra '\n'
        } else {
            current_line += 1;
        }
    }

    // Add boundary marker if debug mode
    if crate::config::env::resolve_seam_debug() {
        merged.push_str("\n/* --- using prelude/main boundary --- */\n\n");
        let boundary_lines = 3usize;
        spans.push(crate::runner::modes::common_util::resolve::LineSpan {
            file: "<prelude/main-boundary>".to_string(),
            start_line: current_line,
            line_count: boundary_lines,
        });
        current_line += boundary_lines;
    }

    // Add main source (already cleaned of using lines) and normalize
    let mut cleaned_main_norm = normalize_text_for_inline(&cleaned_main);
    // Hako-friendly normalize for main: always strip leading `local ` at line head
    // when the merged main looks like Hako code (or file is .hako as a heuristic).
    if filename.ends_with(".hako")
        || crate::runner::modes::common_util::hako::looks_like_hako_code(&cleaned_main_norm)
    {
        cleaned_main_norm =
            crate::runner::modes::common_util::hako::strip_local_decl(&cleaned_main_norm);
    }
    merged.push_str(&cleaned_main_norm);
    let main_lines = cleaned_main_norm.lines().count();
    if main_lines > 0 {
        spans.push(crate::runner::modes::common_util::resolve::LineSpan {
            file: filename.to_string(),
            start_line: current_line,
            line_count: main_lines,
        });
        current_line += main_lines;
    }
    let _ = current_line;

    if trace {
        crate::runner::trace::log(format!(
            "[using/text-merge] final merged: {} bytes ({} prelude + {} main)",
            merged.len(),
            merged.len() - cleaned_main.len(),
            cleaned_main.len()
        ));
    }

    // Optional dump of merged text for diagnostics
    if let Some(dump_path) = crate::config::env::resolve_dump_merged_path() {
        let _ = std::fs::write(&dump_path, &merged);
    }

    crate::runner::modes::common_util::resolve::set_last_text_merge_line_spans(spans);

    Ok(TextMergePlan {
        merged: normalize_text_for_inline(&merged),
        imports,
    })
}

fn canonize(p: &str) -> String {
    std::fs::canonicalize(p)
        .ok()
        .map(|pb| pb.to_string_lossy().to_string())
        .unwrap_or_else(|| p.to_string())
}

fn dfs_text_with_imports(
    runner: &NyashRunner,
    path: &str,
    out: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
    imports: &mut std::collections::HashMap<String, String>,
) -> Result<(), String> {
    let key = canonize(path);
    if !seen.insert(key.clone()) {
        return Ok(());
    }
    // Phase 90-A: fs 系移行
    let ring0 = crate::runtime::ring0::get_global_ring0();
    let src = ring0
        .fs
        .read_to_string(std::path::Path::new(path))
        .map_err(|e| format!("using: failed to read '{}': {}", path, e))?;
    let (_cleaned, nested, nested_imports) = collect_using_and_strip(runner, &src, path)?;
    merge_imports(imports, nested_imports, path)?;
    for n in nested.iter() {
        dfs_text_with_imports(runner, n, out, seen, imports)?;
    }
    out.push(key);
    Ok(())
}

fn merge_imports(
    dst: &mut std::collections::HashMap<String, String>,
    src: std::collections::HashMap<String, String>,
    origin: &str,
) -> Result<(), String> {
    for (alias, box_name) in src {
        if let Some(prev) = dst.get(&alias) {
            if prev != &box_name {
                return Err(format!(
                    "using: imported static box alias '{}' conflicts across merged preludes ({} vs {} from {})",
                    alias, prev, box_name, origin
                ));
            }
            continue;
        }
        dst.insert(alias, box_name);
    }
    Ok(())
}

/// Minimal normalization to improve inline parser robustness.
/// - Normalize CRLF to LF
/// - Remove redundant semicolons before closing braces (`; }` → `}`)
/// - Ensure file ends with a newline
fn normalize_text_for_inline(s: &str) -> String {
    let mut out = s.replace("\r\n", "\n").replace("\r", "\n");
    // Remove `;` before `}` across line boundaries conservatively
    // pattern: `;` followed by optional spaces/newlines then `}`
    // Do a few passes to cover nested cases without regex
    for _ in 0..2 {
        let bytes = out.as_bytes();
        let mut tmp: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b';' {
                // peek ahead skipping spaces/newlines
                let mut j = i + 1;
                while j < bytes.len() {
                    let c = bytes[j];
                    if c == b' ' || c == b'\t' || c == b'\n' {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if j < bytes.len() && bytes[j] == b'}' {
                    // drop ';' (do not advance j here)
                    i += 1;
                    continue;
                }
            }
            tmp.push(bytes[i]);
            i += 1;
        }
        out = String::from_utf8(tmp).expect("normalize_text_for_inline: invalid UTF-8");
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::normalize_text_for_inline;

    #[test]
    fn normalize_text_for_inline_preserves_utf8() {
        let src = "aé𝄞;\n}\n";
        let out = normalize_text_for_inline(src);
        assert_eq!(out, "aé𝄞\n}\n");
    }
}
