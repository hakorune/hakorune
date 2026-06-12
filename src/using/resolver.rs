use crate::using::spec::{PackageKind, UsingPackage};
use std::collections::HashMap;
mod cache;
pub use cache::populate_from_toml;

/// Resolve a using target name into a concrete path token.
/// - Returns plain file path for modules/package files
/// - Returns a marker token `dylib:<path>` for kind="dylib" packages
/// - Searches relative to `context_dir` then `using_paths` for bare names
/// - When `strict` and multiple candidates exist, returns Err
pub fn resolve_using_target_common(
    tgt: &str,
    modules: &[(String, String)],
    module_roots: &[(String, String)],
    using_paths: &[String],
    packages: &HashMap<String, UsingPackage>,
    context_dir: Option<&std::path::Path>,
    strict: bool,
    verbose: bool,
) -> Result<String, String> {
    // 1) modules mapping (exact match)
    if let Some((_, p)) = modules.iter().find(|(n, _)| n == tgt) {
        if verbose {
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[using/resolve] modules '{}' -> '{}'", tgt, p));
        }
        return Ok(p.clone());
    }

    // 2) module_roots: longest prefix match (Phase 29bq+)
    // module_roots is pre-sorted by prefix length descending
    if let Some(resolved) = resolve_via_module_roots(tgt, module_roots, verbose)? {
        return Ok(resolved);
    }

    // When module_roots are configured, dotted namespace targets must resolve via
    // [modules] or [module_roots] only (no fallback to relative/using.paths).
    if !module_roots.is_empty() && tgt.contains('.') {
        return Err(format!(
            "[freeze:contract][module_roots] not_found: '{}' has no matching root",
            tgt
        ));
    }

    // 3) named packages
    if let Some(pkg) = packages.get(tgt) {
        match pkg.kind {
            PackageKind::Dylib => {
                let out = format!("dylib:{}", pkg.path);
                if verbose {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[using/resolve] dylib '{}' -> '{}'", tgt, out));
                }
                return Ok(out);
            }
            PackageKind::Package => {
                let base = std::path::Path::new(&pkg.path);
                let out = if let Some(m) = &pkg.main {
                    if matches!(
                        base.extension().and_then(|s| s.to_str()),
                        Some("nyash") | Some("hako")
                    ) {
                        pkg.path.clone()
                    } else {
                        base.join(m).to_string_lossy().to_string()
                    }
                } else {
                    if matches!(
                        base.extension().and_then(|s| s.to_str()),
                        Some("nyash") | Some("hako")
                    ) {
                        pkg.path.clone()
                    } else {
                        let leaf = base.file_name().and_then(|s| s.to_str()).unwrap_or(tgt);
                        let hako = base.join(format!("{}.hako", leaf));
                        if hako.exists() {
                            hako.to_string_lossy().to_string()
                        } else {
                            base.join(format!("{}.hako", leaf))
                                .to_string_lossy()
                                .to_string()
                        }
                    }
                };
                if verbose {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[using/resolve] package '{}' -> '{}'", tgt, out));
                }
                return Ok(out);
            }
        }
    }
    // 3) relative: prefer cwd > using_paths; .hako first then .nyash
    let rel_hako = tgt.replace('.', "/") + ".hako";
    let rel_ny = tgt.replace('.', "/") + ".nyash";
    let mut cand: Vec<String> = Vec::new();
    if let Some(dir) = context_dir {
        let c1 = dir.join(&rel_hako);
        if c1.exists() {
            cand.push(c1.to_string_lossy().to_string());
        }
        let c2 = dir.join(&rel_ny);
        if c2.exists() {
            cand.push(c2.to_string_lossy().to_string());
        }
    }
    for base in using_paths {
        let p = std::path::Path::new(base);
        let c1 = p.join(&rel_hako);
        if c1.exists() {
            cand.push(c1.to_string_lossy().to_string());
        }
        let c2 = p.join(&rel_ny);
        if c2.exists() {
            cand.push(c2.to_string_lossy().to_string());
        }
    }
    if cand.is_empty() {
        if verbose {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[using] unresolved '{}' (searched: rel+paths)",
                tgt
            ));
        }
        return Err(format!(
            "using: unresolved '{}': searched relative and using.paths",
            tgt
        ));
    }
    if cand.len() > 1 && strict {
        return Err(format!("ambiguous using '{}': {}", tgt, cand.join(", ")));
    }
    let out = cand.remove(0);
    if verbose {
        crate::runtime::get_global_ring0()
            .log
            .debug(&format!("[using/resolve] '{}' -> '{}'", tgt, out));
    }
    Ok(out)
}

/// Resolve via module_roots: longest prefix match → construct path.
/// Returns Ok(Some(path)) if resolved, Ok(None) if no match, Err on ambiguity.
fn resolve_via_module_roots(
    tgt: &str,
    module_roots: &[(String, String)],
    verbose: bool,
) -> Result<Option<String>, String> {
    if module_roots.is_empty() {
        return Ok(None);
    }

    // Find all matching prefixes
    let matches: Vec<_> = module_roots
        .iter()
        .filter(|(prefix, _)| tgt == prefix || tgt.starts_with(&format!("{}.", prefix)))
        .collect();

    if matches.is_empty() {
        return Ok(None);
    }

    // Check for ambiguity: multiple prefixes of the same (longest) length
    let longest_len = matches[0].0.len();
    let same_len_count = matches
        .iter()
        .filter(|(p, _)| p.len() == longest_len)
        .count();
    if same_len_count > 1 {
        let ambiguous: Vec<_> = matches
            .iter()
            .filter(|(p, _)| p.len() == longest_len)
            .map(|(p, _)| p.as_str())
            .collect();
        return Err(format!(
            "[freeze:contract][module_roots] ambiguous: '{}' matches multiple roots: {}",
            tgt,
            ambiguous.join(", ")
        ));
    }

    // Use the longest match
    let (prefix, root_path) = matches[0];

    // Construct the path: root + suffix.replace('.', '/') + ".hako"
    let suffix = if tgt.len() > prefix.len() {
        &tgt[prefix.len() + 1..] // Skip the prefix and the dot
    } else {
        "" // Exact match: tgt == prefix
    };

    let path = if suffix.is_empty() {
        // Exact prefix match - look for index.hako or <prefix_leaf>.hako
        let leaf = prefix.rsplit('.').next().unwrap_or(prefix);
        format!("{}/{}.hako", root_path, leaf)
    } else {
        format!("{}/{}.hako", root_path, suffix.replace('.', "/"))
    };

    if verbose {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[using/resolve] module_roots '{}' -> '{}' (prefix='{}')",
            tgt, path, prefix
        ));
    }

    Ok(Some(path))
}

#[cfg(test)]
mod tests;
