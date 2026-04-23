use crate::runner::NyashRunner;
use crate::runtime::get_global_ring0;

use super::using::collect_using_and_strip;

/// Profile-aware prelude resolution wrapper (single entrypoint).
/// - Delegates to `collect_using_and_strip` for the first pass.
/// - Resolves nested preludes via DFS for the default text-merge route and the
///   optional AST compatibility route, then injects OperatorBox preludes when
///   available (stringify/compare/add).
/// - All runners call this helper; do not fork resolution logic elsewhere.
pub fn resolve_prelude_paths_profiled(
    runner: &NyashRunner,
    code: &str,
    filename: &str,
) -> Result<(String, Vec<String>), String> {
    // First pass: strip using from the main source and collect direct prelude paths
    let (cleaned, direct, _imports) = collect_using_and_strip(runner, code, filename)?;
    // Recursively collect nested preludes (DFS) for the default text-merge
    // route and the optional AST compatibility route.
    // Rationale: even when we merge via text, nested `using` inside preludes
    // must be discovered so that their definitions are present at runtime
    // (e.g., runner_min -> lower_* boxes). Previously this only ran when
    // NYASH_USING_AST=1, which caused unresolved calls in inline flows.
    let _ast_on = crate::config::env::env_bool("NYASH_USING_AST");
    let mut out: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for p in direct.iter() {
        dfs(runner, p, &mut out, &mut seen)?;
    }
    // Operator Boxes prelude injection（観測“常時ON”のため）
    // stringify/compare/add は常に注入（存在時）。その他（bitwise等）は ALL 指定時のみ。
    let opbox_all = crate::config::env::env_bool("NYASH_OPERATOR_BOX_ALL")
        || crate::config::env::env_bool("NYASH_BUILDER_OPERATOR_BOX_ALL_CALL");

    if let Some(root) =
        crate::runner::modes::common_util::resolve::root::resolve_repo_root(Some(filename))
    {
        let must_have = [
            "apps/lib/std/operators/stringify.hako",
            "apps/lib/std/operators/compare.hako",
            "apps/lib/std/operators/add.hako",
        ];
        for rel in must_have.iter() {
            let p = root.join(rel);
            if p.exists() {
                let path = p.to_string_lossy().to_string();
                if !out.iter().any(|x| x == &path) {
                    out.push(path);
                }
            }
        }
    }
    // Inject remaining arithmetic/bitwise/unary operator modules when ALL is requested
    if opbox_all {
        if let Some(root) =
            crate::runner::modes::common_util::resolve::root::resolve_repo_root(Some(filename))
        {
            let rels = vec![
                "apps/lib/std/operators/sub.hako",
                "apps/lib/std/operators/mul.hako",
                "apps/lib/std/operators/div.hako",
                "apps/lib/std/operators/mod.hako",
                // Shifts / bitwise (parser tokens now supported)
                "apps/lib/std/operators/shl.hako",
                "apps/lib/std/operators/shr.hako",
                "apps/lib/std/operators/bitand.hako",
                "apps/lib/std/operators/bitor.hako",
                "apps/lib/std/operators/bitxor.hako",
                "apps/lib/std/operators/neg.hako",
                "apps/lib/std/operators/not.hako",
                "apps/lib/std/operators/bitnot.hako",
            ];
            for rel in rels {
                let p = root.join(rel);
                if p.exists() {
                    let path = p.to_string_lossy().to_string();
                    if !out.iter().any(|x| x == &path) {
                        out.push(path);
                    }
                }
            }
        }
    }
    // Even when the AST compatibility path is disabled, still return the
    // discovered nested prelude list so that the text merger can inline all
    // dependencies. This keeps behavior consistent across strategies and fixes
    // nested `using` resolution.
    Ok((cleaned, out))
}

/// Parse prelude source files into ASTs for the optional compatibility path.
/// - Reads each path, strips nested `using`, and parses to AST.
/// - Returns a Vec of Program ASTs (one per prelude file), preserving DFS order.
pub fn parse_preludes_to_asts(
    runner: &NyashRunner,
    prelude_paths: &[String],
) -> Result<Vec<nyash_rust::ast::ASTNode>, String> {
    let debug = crate::config::env::env_bool("NYASH_STRIP_DEBUG");
    if debug {
        get_global_ring0().log.debug(&format!(
            "[strip-debug] parse_preludes_to_asts: {} files total",
            prelude_paths.len()
        ));
        for (idx, p) in prelude_paths.iter().enumerate() {
            get_global_ring0()
                .log
                .debug(&format!("[strip-debug]   [{}] {}", idx, p));
        }
    }
    let mut out: Vec<nyash_rust::ast::ASTNode> = Vec::with_capacity(prelude_paths.len());
    for (idx, prelude_path) in prelude_paths.iter().enumerate() {
        if debug {
            get_global_ring0().log.debug(&format!(
                "[strip-debug] [{}/{}] Processing: {}",
                idx + 1,
                prelude_paths.len(),
                prelude_path
            ));
        }
        // Phase 90-A: fs 系移行
        let ring0 = crate::runtime::ring0::get_global_ring0();
        let src = ring0
            .fs
            .read_to_string(std::path::Path::new(prelude_path))
            .map_err(|e| format!("using: error reading {}: {}", prelude_path, e))?;
        let (clean_src, _nested, _nested_imports) =
            collect_using_and_strip(runner, &src, prelude_path)?;

        // IMPORTANT: Do not attempt to AST-parse .hako preludes here.
        // .hako is Hakorune surface, not Nyash AST. VM/VM-fallback paths
        // will route to text-merge when any prelude is .hako.
        if prelude_path.ends_with(".hako") {
            if debug {
                get_global_ring0().log.debug(&format!(
                    "[strip-debug] skip AST parse for .hako prelude: {}",
                    prelude_path
                ));
            }
            continue;
        }

        let clean_src = clean_src;

        // Debug: dump clean_src if NYASH_STRIP_DEBUG=1
        if debug {
            get_global_ring0().log.debug(&format!(
                "[strip-debug] [{}/{}] About to parse: {}",
                idx + 1,
                prelude_paths.len(),
                prelude_path
            ));
            get_global_ring0().log.debug(&format!(
                "[strip-debug]   clean_src first 500 chars:\n{}\n---",
                &clean_src.chars().take(500).collect::<String>()
            ));
        }

        match crate::parser::NyashParser::parse_from_string(&clean_src) {
            Ok(ast) => {
                if debug {
                    get_global_ring0().log.debug(&format!(
                        "[strip-debug] [{}/{}] ✅ Parse SUCCESS: {}",
                        idx + 1,
                        prelude_paths.len(),
                        prelude_path
                    ));
                }
                out.push(ast)
            }
            Err(e) => {
                // Always output debug info on parse failure if NYASH_STRIP_DEBUG=1
                let debug = crate::config::env::env_bool("NYASH_STRIP_DEBUG");
                get_global_ring0().log.debug(&format!(
                    "[strip-debug] Parse FAILED for: {} (debug={})",
                    prelude_path, debug
                ));
                if debug {
                    get_global_ring0()
                        .log
                        .debug(&format!("[strip-debug] Error: {}", e));
                    let es = format!("{}", e);
                    let lines: Vec<&str> = clean_src.lines().collect();
                    get_global_ring0()
                        .log
                        .debug(&format!("[strip-debug] Total lines: {}", lines.len()));
                    // Try to extract error line number (e.g., "at line 451") and show local context
                    let mut printed = false;
                    if let Some(pos) = es.rfind("line ") {
                        let mut j = pos + 5; // after "line "
                        let bytes = es.as_bytes();
                        let mut n: usize = 0;
                        let mut had = false;
                        while j < bytes.len() {
                            let c = bytes[j];
                            if c >= b'0' && c <= b'9' {
                                n = n * 10 + (c - b'0') as usize;
                                j += 1;
                                had = true;
                            } else {
                                break;
                            }
                        }
                        if had {
                            let ln = if n == 0 { 1 } else { n };
                            let from = ln.saturating_sub(3);
                            let to = std::cmp::min(lines.len(), ln + 3);
                            get_global_ring0().log.debug(&format!(
                                "[strip-debug] Context around line {} ({}..={}):",
                                ln,
                                from.max(1),
                                to
                            ));
                            for i in from.max(1)..=to {
                                let mark = if i == ln { ">>" } else { "  " };
                                if let Some(line) = lines.get(i - 1) {
                                    get_global_ring0()
                                        .log
                                        .debug(&format!("{} {:4}: {}", mark, i, line));
                                }
                            }
                            printed = true;
                        }
                    }
                    if !printed {
                        get_global_ring0().log.debug("[strip-debug] Lines 15-25:");
                        for (idx, line) in lines.iter().enumerate().skip(14).take(11) {
                            get_global_ring0()
                                .log
                                .debug(&format!("  {:3}: {}", idx + 1, line));
                        }
                    }
                    get_global_ring0().log.debug(&format!(
                        "[strip-debug] Full clean_src:\n{}\n---",
                        clean_src
                    ));
                }
                return Err(format!(
                    "Parse error in using prelude {}: {}",
                    prelude_path, e
                ));
            }
        }
    }
    if debug {
        get_global_ring0().log.debug(&format!(
            "[strip-debug] parse_preludes_to_asts: ✅ All {} files parsed successfully",
            out.len()
        ));
    }
    Ok(out)
}

fn normalize_path(path: &str) -> (String, String) {
    use std::path::PathBuf;
    match PathBuf::from(path).canonicalize() {
        Ok(canon) => {
            let s = canon.to_string_lossy().to_string();
            (s.clone(), s)
        }
        Err(_) => {
            // Fall back to the original path representation.
            (path.to_string(), path.to_string())
        }
    }
}

fn dfs(
    runner: &NyashRunner,
    path: &str,
    out: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
) -> Result<(), String> {
    let (key, real_path) = normalize_path(path);
    if !seen.insert(key.clone()) {
        return Ok(());
    }
    // Phase 90-A: fs 系移行
    let ring0 = crate::runtime::ring0::get_global_ring0();
    let src = ring0
        .fs
        .read_to_string(std::path::Path::new(&real_path))
        .map_err(|e| format!("using: failed to read '{}': {}", real_path, e))?;
    let (_cleaned, nested, _nested_imports) = collect_using_and_strip(runner, &src, &real_path)?;
    for n in nested.iter() {
        dfs(runner, n, out, seen)?;
    }
    out.push(real_path);
    Ok(())
}
