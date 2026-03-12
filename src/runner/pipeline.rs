/*!
 * Runner pipeline helpers — using/modules/env pre-processing
 *
 * Extracts the early-phase setup from runner/mod.rs:
 *  - load nyash.toml [modules] and [using.paths]
 *  - merge with defaults and env overrides
 *  - expose context (using_paths, pending_modules) for downstream resolution
 */

use super::*;
use crate::using::spec::{PackageKind, UsingPackage};
use crate::using::ssot_bridge::{call_using_resolve_ssot, SsotCtx};
use std::collections::HashMap;

/// Using/module resolution context accumulated from config/env/hako.toml/nyash.toml
pub(super) struct UsingContext {
    pub using_paths: Vec<String>,
    pub pending_modules: Vec<(String, String)>,
    pub module_roots: Vec<(String, String)>,
    pub aliases: std::collections::HashMap<String, String>,
    pub packages: std::collections::HashMap<String, UsingPackage>,
}

impl NyashRunner {
    /// Initialize using/module context from defaults, hako.toml/nyash.toml and env
    pub(super) fn init_using_context(&self) -> UsingContext {
        let mut using_paths: Vec<String> = Vec::new();
        let mut pending_modules: Vec<(String, String)> = Vec::new();
        let mut module_roots: Vec<(String, String)> = Vec::new();
        let mut aliases: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut packages: std::collections::HashMap<String, UsingPackage> =
            std::collections::HashMap::new();

        // Defaults
        using_paths.extend(["apps", "lib", "."].into_iter().map(|s| s.to_string()));

        // hako.toml/nyash.toml: delegate to using resolver (keeps existing behavior)
        let toml_result = crate::using::resolver::populate_from_toml(
            &mut using_paths,
            &mut pending_modules,
            &mut aliases,
            &mut packages,
            &mut module_roots,
        );

        // 🔍 Debug: Check if aliases are loaded
        if crate::config::env::debug_using() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[DEBUG/using] populate_from_toml result: {:?}",
                toml_result
            ));
            ring0
                .log
                .debug(&format!("[DEBUG/using] Loaded {} aliases", aliases.len()));
            for (k, v) in aliases.iter() {
                ring0
                    .log
                    .debug(&format!("[DEBUG/using] alias: '{}' => '{}'", k, v));
            }
            ring0
                .log
                .debug(&format!("[DEBUG/using] Loaded {} packages", packages.len()));
            for (k, v) in packages.iter() {
                ring0.log.debug(&format!(
                    "[DEBUG/using] package: '{}' => path='{}'",
                    k, v.path
                ));
            }
        }

        // Debug-only: keep TOML parse failures observable without spamming release/gates.
        if let Err(e) = &toml_result {
            if crate::config::env::debug_using() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[using/workspace] failed to load toml modules: {e}"
                ));
            }
        }

        // Env overrides: modules and using paths
        if let Some(ms) = crate::config::env::modules_env() {
            for ent in ms.split(',') {
                if let Some((k, v)) = ent.split_once('=') {
                    let k = k.trim();
                    let v = v.trim();
                    if !k.is_empty() && !v.is_empty() {
                        pending_modules.push((k.to_string(), v.to_string()));
                    }
                }
            }
        }
        if let Some(p) = crate::config::env::using_path_env() {
            for s in p.split(':') {
                let s = s.trim();
                if !s.is_empty() {
                    using_paths.push(s.to_string());
                }
            }
        }
        // Env aliases: comma-separated k=v pairs
        if let Some(raw) = crate::config::env::aliases_env() {
            for ent in raw.split(',') {
                if let Some((k, v)) = ent.split_once('=') {
                    let k = k.trim();
                    let v = v.trim();
                    if !k.is_empty() && !v.is_empty() {
                        aliases.insert(k.to_string(), v.to_string());
                    }
                }
            }
        }

        UsingContext {
            using_paths,
            pending_modules,
            module_roots,
            aliases,
            packages,
        }
    }
}

/// Suggest candidate files by leaf name within limited bases (apps/lib/.)
#[allow(dead_code)]
pub(super) fn suggest_in_base(base: &str, leaf: &str, out: &mut Vec<String>) {
    use std::fs;
    fn walk(dir: &std::path::Path, leaf: &str, out: &mut Vec<String>, depth: usize) {
        if depth == 0 || out.len() >= 5 {
            return;
        }
        if let Ok(entries) = fs::read_dir(dir) {
            for e in entries.flatten() {
                let path = e.path();
                if path.is_dir() {
                    walk(&path, leaf, out, depth - 1);
                    if out.len() >= 5 {
                        return;
                    }
                } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext == "nyash" || ext == "hako" {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            if stem == leaf {
                                out.push(path.to_string_lossy().to_string());
                                if out.len() >= 5 {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let p = std::path::Path::new(base);
    walk(p, leaf, out, 4);
}

/// Resolve a using target according to priority: modules > module_roots > relative > using-paths
/// Returns Ok(resolved_path_or_token). On strict mode, ambiguous matches cause error.
pub(super) fn resolve_using_target(
    tgt: &str,
    is_path: bool,
    modules: &[(String, String)],
    module_roots: &[(String, String)],
    using_paths: &[String],
    aliases: &HashMap<String, String>,
    packages: &HashMap<String, UsingPackage>,
    context_dir: Option<&std::path::Path>,
    strict: bool,
    verbose: bool,
) -> Result<String, String> {
    // Dev toggle: try SSOT common resolver first, then fall back to legacy path.
    // This helps migrate behavior gradually without changing defaults.
    if crate::config::env::using_resolver_first() {
        match crate::using::resolver::resolve_using_target_common(
            tgt,
            modules,
            module_roots,
            using_paths,
            packages,
            context_dir,
            strict,
            verbose,
        ) {
            Ok(val) => return Ok(val),
            Err(_) => { /* fall through to legacy path */ }
        }
    }
    // Phase 22.1: Thin SSOT hook (future wiring). No behavior change for now.
    if crate::config::env::using_ssot_enabled() && !crate::config::env::using_ssot_invoking() {
        if let Some(ssot_res) = try_resolve_using_target_ssot(
            tgt,
            is_path,
            modules,
            module_roots,
            using_paths,
            aliases,
            packages,
            context_dir,
            strict,
            verbose,
        ) {
            return Ok(ssot_res);
        }
    }
    // Invalidate and rebuild index/cache if env or nyash.toml changed
    super::box_index::rebuild_if_env_changed();
    if is_path {
        return Ok(tgt.to_string());
    }
    let trace = verbose || crate::config::env::env_bool("NYASH_RESOLVE_TRACE");
    let idx = super::box_index::get_box_index();
    let mut strict_effective = strict || idx.plugins_require_prefix_global;
    if crate::config::env::env_bool("NYASH_PLUGIN_REQUIRE_PREFIX") {
        strict_effective = true;
    }
    let meta_for_target = idx.plugin_meta_by_box.get(tgt).cloned();
    let mut require_prefix_target = meta_for_target
        .as_ref()
        .map(|m| m.require_prefix)
        .unwrap_or(false);
    if let Some(m) = &meta_for_target {
        if !m.expose_short_names {
            require_prefix_target = true;
        }
    }
    let mut is_plugin_short = meta_for_target.is_some();
    if !is_plugin_short {
        is_plugin_short = idx.plugin_boxes.contains(tgt)
            || super::box_index::BoxIndex::is_known_plugin_short(tgt);
    }
    if (strict_effective || require_prefix_target) && is_plugin_short && !tgt.contains('.') {
        let mut msg = format!("plugin short name '{}' requires prefix", tgt);
        if let Some(meta) = &meta_for_target {
            if let Some(pref) = &meta.prefix {
                msg.push_str(&format!(" (use '{}.{}')", pref, tgt));
            }
        }
        return Err(msg);
    }
    let key = {
        let base = context_dir.and_then(|p| p.to_str()).unwrap_or("");
        format!(
            "{}|{}|{}|{}",
            tgt,
            base,
            strict as i32,
            using_paths.join(":")
        )
    };
    if let Some(hit) = crate::runner::box_index::cache_get(&key) {
        if trace {
            crate::runner::trace::log(format!("[using/cache] '{}' -> '{}'", tgt, hit));
        }
        return Ok(hit);
    }
    // Resolve aliases early（推移的に）
    // - ループ/循環を検出して早期エラー
    // - 10段まで（防衛的）
    if let Some(_) = aliases.get(tgt) {
        use std::collections::HashSet;
        let mut seen: HashSet<String> = HashSet::new();
        let mut cur = tgt.to_string();
        let mut depth = 0usize;
        while let Some(next) = aliases.get(&cur).cloned() {
            if trace {
                crate::runner::trace::log(format!("[using/resolve] alias '{}' -> '{}'", cur, next));
            }
            if !seen.insert(cur.clone()) {
                return Err(format!("alias cycle detected at '{}'", cur));
            }
            cur = next;
            depth += 1;
            if depth > 10 {
                return Err(format!("alias resolution too deep starting at '{}'", tgt));
            }
            // Continue while next is also an alias; break when concrete
            if !aliases.contains_key(&cur) {
                break;
            }
        }
        // Recurse once into final target to materialize path/token
        let rec = resolve_using_target(
            &cur,
            false,
            modules,
            module_roots,
            using_paths,
            aliases,
            packages,
            context_dir,
            strict,
            verbose,
        )?;
        crate::runner::box_index::cache_put(&key, rec.clone());
        return Ok(rec);
    }
    // Named packages (nyash.toml [using.<name>])
    if let Some(pkg) = packages.get(tgt) {
        match pkg.kind {
            PackageKind::Dylib => {
                // Return a marker token to avoid inlining attempts; loader will consume later stages
                let out = format!("dylib:{}", pkg.path);
                if trace {
                    crate::runner::trace::log(format!(
                        "[using/resolve] dylib '{}' -> '{}'",
                        tgt, out
                    ));
                }
                crate::runner::box_index::cache_put(&key, out.clone());
                return Ok(out);
            }
            PackageKind::Package => {
                // Compute entry: main or <dir_last>.hako
                let base = std::path::Path::new(&pkg.path);
                let out = if let Some(m) = &pkg.main {
                    if matches!(
                        base.extension().and_then(|s| s.to_str()),
                        Some("nyash") | Some("hako")
                    ) {
                        // path is a file; ignore main and use as-is
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
                        // prefer .hako when package path points to a directory without explicit main
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
                if trace {
                    crate::runner::trace::log(format!(
                        "[using/resolve] package '{}' -> '{}'",
                        tgt, out
                    ));
                }
                crate::runner::box_index::cache_put(&key, out.clone());
                return Ok(out);
            }
        }
    }
    // Also consult env aliases
    if let Some(raw) = crate::config::env::aliases_env() {
        for ent in raw.split(',') {
            if let Some((k, v)) = ent.split_once('=') {
                if k.trim() == tgt {
                    let out = v.trim().to_string();
                    if trace {
                        crate::runner::trace::log(format!(
                            "[using/resolve] env-alias '{}' -> '{}'",
                            tgt, out
                        ));
                    }
                    crate::runner::box_index::cache_put(&key, out.clone());
                    return Ok(out);
                }
            }
        }
    }
    // 2) Special handling for built-in namespaces
    if tgt == "nyashstd" {
        let out = "builtin:nyashstd".to_string();
        if trace {
            crate::runner::trace::log(format!("[using/resolve] builtin '{}' -> '{}'", tgt, out));
        }
        crate::runner::box_index::cache_put(&key, out.clone());
        return Ok(out);
    }
    // 3) delegate resolution to using::resolver (SSOT)
    match crate::using::resolver::resolve_using_target_common(
        tgt,
        modules,
        module_roots,
        using_paths,
        packages,
        context_dir,
        strict,
        verbose,
    ) {
        Ok(val) => {
            crate::runner::box_index::cache_put(&key, val.clone());
            Ok(val)
        }
        Err(e) => {
            // Fail-fast (contract): missing module registration causes long-distance failures
            // (e.g., unknown box type later), so surface it early with a stable tag.
            if e.starts_with("using: unresolved") {
                return Err(format!(
                    "[freeze:contract][module_registry] missing module: {} hint=hako.toml:[modules]",
                    tgt
                ));
            }

            // Strict: propagate resolver errors instead of silently continuing.
            if strict {
                return Err(e);
            }

            // Non-strict: keep legacy behavior (log + keep original name).
            if trace {
                crate::runner::trace::log(format!("[using] unresolved '{}' ({})", tgt, e));
            }
            Ok(tgt.to_string())
        }
    }
}

/// Thin SSOT wrapper — returns Some(resolved) when an alternative SSOT path is available.
/// MVP: return None to keep current behavior. Future: call into Hako `UsingResolveSSOTBox`.
#[allow(clippy::too_many_arguments)]
fn try_resolve_using_target_ssot(
    tgt: &str,
    is_path: bool,
    modules: &[(String, String)],
    module_roots: &[(String, String)],
    using_paths: &[String],
    aliases: &HashMap<String, String>,
    packages: &HashMap<String, UsingPackage>,
    context_dir: Option<&std::path::Path>,
    strict: bool,
    verbose: bool,
) -> Option<String> {
    // Phase 22.1 MVP: Build context and consult SSOT bridge (modules-only).
    let trace = verbose || crate::config::env::env_bool("NYASH_RESOLVE_TRACE");
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in modules {
        map.insert(k.clone(), v.clone());
    }
    let cwd_str = context_dir.and_then(|p| p.to_str()).map(|s| s.to_string());
    let ctx = SsotCtx {
        modules: map,
        using_paths: using_paths.to_vec(),
        cwd: cwd_str,
    };
    if let Some(hit) = call_using_resolve_ssot(tgt, &ctx) {
        if trace {
            crate::runner::trace::log(format!("[using/ssot] '{}' -> '{}'", tgt, hit));
        }
        return Some(hit);
    }
    // Optional relative inference (Runner-side, guarded): prefer cwd > using_paths
    if crate::config::env::using_ssot_relative() {
        let rel_hako = tgt.replace('.', "/") + ".hako";
        let rel_ny = tgt.replace('.', "/") + ".nyash";
        let mut try_paths: Vec<std::path::PathBuf> = Vec::new();
        if let Some(dir) = context_dir {
            try_paths.push(dir.join(&rel_hako));
            try_paths.push(dir.join(&rel_ny));
        }
        for base in using_paths {
            let p = std::path::Path::new(base);
            try_paths.push(p.join(&rel_hako));
            try_paths.push(p.join(&rel_ny));
        }
        let mut found: Vec<String> = Vec::new();
        for p in try_paths {
            if p.exists() {
                found.push(p.to_string_lossy().to_string());
            }
        }
        if !found.is_empty() {
            if found.len() > 1 && strict {
                if trace {
                    let total = found.len();
                    // Allow customizing the number of shown candidates via env (bounded 1..=10)
                    let n_show: usize = crate::config::env::using_ssot_relative_ambig_first_n()
                        .map(|n| n.clamp(1, 10))
                        .unwrap_or(3);
                    let shown: Vec<String> = found.iter().take(n_show).cloned().collect();
                    // Standardized message: count + first N + explicit delegation policy
                    crate::runner::trace::log(format!(
                        "[using/ssot:relative ambiguous] name='{}' count={} first=[{}] -> delegate=legacy(strict)",
                        tgt,
                        total,
                        shown.join(", ")
                    ));
                }
                // Strict ambiguity: delegate to legacy resolver (behavior unchanged)
            } else {
                let out = found.remove(0);
                if trace {
                    crate::runner::trace::log(format!(
                        "[using/ssot:relative] '{}' -> '{}' (priority=cwd>using_paths)",
                        tgt, out
                    ));
                }
                return Some(out);
            }
        }
    }
    // Fallback: keep parity by delegating to existing resolver within the same gate
    let prev = crate::config::env::using_ssot_invoking_raw();
    crate::config::env::set_using_ssot_invoking(Some("1"));
    let res = resolve_using_target(
        tgt,
        is_path,
        modules,
        module_roots,
        using_paths,
        aliases,
        packages,
        context_dir,
        strict,
        verbose,
    );
    crate::config::env::set_using_ssot_invoking(prev.as_deref());
    res.ok()
}

/// Lint: enforce "fields must be at the top of box" rule.
/// - Warns by default (when verbose); when `strict` is true, returns Err on any violation.
pub(super) fn lint_fields_top(code: &str, strict: bool, verbose: bool) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn plugin_meta_requires_prefix_even_when_relaxed() {
        let dir = tempdir().expect("tempdir");
        let old = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(dir.path()).expect("chdir");
        let toml = r#"
[plugins]
require_prefix = false

[plugins."test-plugin"]
prefix = "test"
require_prefix = true
expose_short_names = false
boxes = ["ArrayBox"]
"#;
        std::fs::write("nyash.toml", toml).expect("write nyash.toml");
        crate::runner::box_index::refresh_box_index();
        crate::runner::box_index::cache_clear();

        let res = resolve_using_target(
            "ArrayBox",
            false,
            &[],
            &[],
            &[],
            &HashMap::new(),
            &std::collections::HashMap::<String, crate::using::spec::UsingPackage>::new(),
            None,
            false,
            false,
        );
        assert!(res.is_err(), "expected prefix enforcement");
        let err = res.err().unwrap();
        assert!(err.contains("requires prefix"));
        assert!(err.contains("test."));

        std::env::set_current_dir(old).expect("restore cwd");
        crate::runner::box_index::refresh_box_index();
        crate::runner::box_index::cache_clear();
    }
}
