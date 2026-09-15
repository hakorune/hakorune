use crate::runner::NyashRunner;

use super::import_lineage::{ImportLineageEdgeV1, MergedSourceLineageV1, MergedSourceSegmentV1};
use super::prelude::{resolve_normal_prelude_paths_profiled, resolve_prelude_paths_profiled};
use super::using::collect_using_and_strip_with_edges;

struct TextMergePlan {
    merged: String,
    imports: std::collections::HashMap<String, String>,
    lineage: MergedSourceLineageV1,
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
    Ok(plan_text_merge(runner, source, filename, false)?.merged)
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
    let plan = plan_text_merge(runner, source, filename, false)?;
    Ok((plan.merged, plan.imports))
}

/// Selected normal source keeps Local declarations in every merged file.
pub fn merge_normal_prelude_text_with_imports(
    runner: &NyashRunner,
    source: &str,
    filename: &str,
) -> Result<(String, std::collections::HashMap<String, String>), String> {
    let plan = plan_text_merge(runner, source, filename, true)?;
    Ok((plan.merged, plan.imports))
}

pub(crate) fn merge_prelude_text_with_imports_and_lineage(
    runner: &NyashRunner,
    source: &str,
    filename: &str,
    selected_normal: bool,
) -> Result<
    (
        String,
        std::collections::HashMap<String, String>,
        MergedSourceLineageV1,
    ),
    String,
> {
    let plan = plan_text_merge(runner, source, filename, selected_normal)?;
    Ok((plan.merged, plan.imports, plan.lineage))
}

fn plan_text_merge(
    runner: &NyashRunner,
    source: &str,
    filename: &str,
    selected_normal: bool,
) -> Result<TextMergePlan, String> {
    let trace = crate::config::env::resolve_trace();

    // First pass: collect and resolve prelude paths
    let (cleaned_main, prelude_paths_direct, main_imports, main_edges) =
        collect_using_and_strip_with_edges(runner, source, filename)?;
    let discover = if selected_normal {
        resolve_normal_prelude_paths_profiled
    } else {
        resolve_prelude_paths_profiled
    };
    let (_cleaned_ignore, prelude_paths_profiled) = discover(runner, source, filename)?;
    // Expand nested preludes for text-merge too (DFS) so that any `using`
    // inside prelude files (e.g., runner_min -> lower_* boxes) are also
    // included even when NYASH_USING_AST is OFF.
    let mut expanded: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut imports = main_imports;
    let root_path = canonize(filename);
    let mut parent_paths: std::collections::HashMap<String, Option<String>> =
        std::collections::HashMap::new();
    let mut lineage_edges = main_edges;
    for p in prelude_paths_direct.iter() {
        dfs_text_with_imports(
            runner,
            p,
            Some(&root_path),
            &mut expanded,
            &mut seen,
            &mut imports,
            &mut parent_paths,
            &mut lineage_edges,
        )?;
    }
    for p in prelude_paths_profiled.iter() {
        // The resolver returns a transitive closure, while this DFS expands
        // that closure itself.  Skip a path already emitted by this same
        // traversal; duplicate edges encountered inside a file still fail in
        // `dfs_text_with_imports`.
        if seen.contains(&canonize(p)) {
            continue;
        }
        dfs_text_with_imports(
            runner,
            p,
            Some(&root_path),
            &mut expanded,
            &mut seen,
            &mut imports,
            &mut parent_paths,
            &mut lineage_edges,
        )?;
    }
    let prelude_paths = &expanded;
    // Record for enriched diagnostics (parse error context)
    crate::runner::modes::common_util::resolve::set_last_merged_preludes(prelude_paths.clone());

    if prelude_paths.is_empty() {
        let root_lines = source.lines().count();
        let segment = MergedSourceSegmentV1 {
            source: filename.to_owned().into_boxed_str(),
            canonical_path: root_path.clone().into_boxed_str(),
            parent: None,
            dfs_ordinal: 0,
            global_start_line: 1,
            global_line_count: root_lines,
            local_start_line: 1,
            local_line_count: root_lines,
        };
        let lineage = MergedSourceLineageV1::issue(
            root_path.into_boxed_str(),
            vec![segment],
            canonicalize_edges(lineage_edges),
        )
        .map_err(|error| format!("[using/lineage][{:?}]", error))?;
        crate::runner::modes::common_util::resolve::set_last_text_merge_line_spans(vec![
            crate::runner::modes::common_util::resolve::LineSpan {
                file: filename.to_string(),
                start_line: 1,
                line_count: root_lines,
            },
        ]);
        return Ok(TextMergePlan {
            merged: source.to_string(),
            imports,
            lineage,
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
    let mut segments: Vec<MergedSourceSegmentV1> = Vec::new();
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
        let (cleaned_raw, _nested, _nested_imports, _nested_edges) =
            collect_using_and_strip_with_edges(runner, &content, path)?;
        let mut cleaned = normalize_text_for_inline(&cleaned_raw);
        // Legacy normalization only; selected normal source retains Local syntax.
        if !selected_normal
            && (path.ends_with(".hako")
                || crate::runner::modes::common_util::hako::looks_like_hako_code(&cleaned))
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
        let global_count = added.saturating_add(1);
        let canonical_path = canonize(path);
        segments.push(MergedSourceSegmentV1 {
            source: path.clone().into_boxed_str(),
            canonical_path: canonical_path.into_boxed_str(),
            parent: parent_paths
                .get(path)
                .cloned()
                .flatten()
                .map(String::into_boxed_str),
            dfs_ordinal: segments.len() as u32,
            global_start_line: current_line,
            global_line_count: global_count,
            local_start_line: 1,
            local_line_count: added,
        });
        if added > 0 {
            spans.push(crate::runner::modes::common_util::resolve::LineSpan {
                file: path.clone(),
                start_line: current_line,
                line_count: added,
            });
            current_line += global_count; // +1 for extra '\n'
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
        segments.push(MergedSourceSegmentV1 {
            source: "<prelude/main-boundary>".into(),
            canonical_path: "<prelude/main-boundary>".into(),
            parent: None,
            dfs_ordinal: segments.len() as u32,
            global_start_line: current_line,
            global_line_count: boundary_lines,
            local_start_line: 1,
            local_line_count: boundary_lines,
        });
        current_line += boundary_lines;
    }

    // Add main source (already cleaned of using lines) and normalize
    let mut cleaned_main_norm = normalize_text_for_inline(&cleaned_main);
    // Legacy normalization only; caller selection controls source preservation.
    if !selected_normal
        && (filename.ends_with(".hako")
            || crate::runner::modes::common_util::hako::looks_like_hako_code(&cleaned_main_norm))
    {
        cleaned_main_norm =
            crate::runner::modes::common_util::hako::strip_local_decl(&cleaned_main_norm);
    }
    merged.push_str(&cleaned_main_norm);
    let main_lines = cleaned_main_norm.lines().count();
    segments.push(MergedSourceSegmentV1 {
        source: filename.to_string().into_boxed_str(),
        canonical_path: root_path.clone().into_boxed_str(),
        parent: None,
        dfs_ordinal: segments.len() as u32,
        global_start_line: current_line,
        global_line_count: main_lines,
        local_start_line: 1,
        local_line_count: main_lines,
    });
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

    let lineage = MergedSourceLineageV1::issue(
        root_path.into_boxed_str(),
        segments,
        canonicalize_edges(lineage_edges),
    )
    .map_err(|error| format!("[using/lineage][{:?}]", error))?;

    Ok(TextMergePlan {
        merged: normalize_text_for_inline(&merged),
        imports,
        lineage,
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
    parent: Option<&str>,
    out: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
    imports: &mut std::collections::HashMap<String, String>,
    parent_paths: &mut std::collections::HashMap<String, Option<String>>,
    lineage_edges: &mut Vec<ImportLineageEdgeV1>,
) -> Result<(), String> {
    let key = canonize(path);
    if !seen.insert(key.clone()) {
        return Ok(());
    }
    parent_paths.insert(key.clone(), parent.map(str::to_owned));
    // Phase 90-A: fs 系移行
    let ring0 = crate::runtime::ring0::get_global_ring0();
    let src = ring0
        .fs
        .read_to_string(std::path::Path::new(path))
        .map_err(|e| format!("using: failed to read '{}': {}", path, e))?;
    let (_cleaned, nested, nested_imports, edges) =
        collect_using_and_strip_with_edges(runner, &src, path)?;
    merge_imports(imports, nested_imports, path)?;
    lineage_edges.extend(edges);
    for n in nested.iter() {
        dfs_text_with_imports(
            runner,
            n,
            Some(&key),
            out,
            seen,
            imports,
            parent_paths,
            lineage_edges,
        )?;
    }
    out.push(key);
    Ok(())
}

fn canonicalize_edges(mut edges: Vec<ImportLineageEdgeV1>) -> Vec<ImportLineageEdgeV1> {
    for edge in &mut edges {
        edge.origin = canonize(&edge.origin).into_boxed_str();
        edge.resolved = canonize(&edge.resolved).into_boxed_str();
    }
    edges
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
