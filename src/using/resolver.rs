use crate::config::env;
use crate::using::errors::UsingError;
use crate::using::policy::UsingPolicy;
use crate::using::spec::{PackageKind, UsingPackage};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

#[derive(Clone, PartialEq, Eq)]
struct PopulateTomlCachePathKey {
    path: String,
    modified: Option<SystemTime>,
    len: Option<u64>,
}

#[derive(Clone, PartialEq, Eq)]
struct PopulateTomlCacheKey {
    manifests: Vec<PopulateTomlCachePathKey>,
}

#[derive(Clone)]
struct PopulateTomlCacheEntry {
    key: PopulateTomlCacheKey,
    using_paths: Vec<String>,
    pending_modules: Vec<(String, String)>,
    aliases: HashMap<String, String>,
    packages: HashMap<String, UsingPackage>,
    module_roots: Vec<(String, String)>,
    policy: UsingPolicy,
}

static POPULATE_TOML_CACHE: OnceLock<Mutex<Option<PopulateTomlCacheEntry>>> = OnceLock::new();

fn populate_toml_cache() -> &'static Mutex<Option<PopulateTomlCacheEntry>> {
    POPULATE_TOML_CACHE.get_or_init(|| Mutex::new(None))
}

fn build_cache_key(paths: &[PathBuf]) -> PopulateTomlCacheKey {
    let manifests = paths
        .iter()
        .map(|path| {
            let (modified, len) = if let Ok(meta) = std::fs::metadata(path) {
                (meta.modified().ok(), Some(meta.len()))
            } else {
                (None, None)
            };
            PopulateTomlCachePathKey {
                path: path.to_string_lossy().to_string(),
                modified,
                len,
            }
        })
        .collect();
    PopulateTomlCacheKey { manifests }
}

fn lookup_cached_populate(key: &PopulateTomlCacheKey) -> Option<PopulateTomlCacheEntry> {
    let guard = populate_toml_cache().lock().ok()?;
    match guard.as_ref() {
        Some(entry) if entry.key == *key => Some(entry.clone()),
        _ => None,
    }
}

fn store_cached_populate(entry: PopulateTomlCacheEntry) {
    if let Ok(mut guard) = populate_toml_cache().lock() {
        *guard = Some(entry);
    }
}

fn apply_cached_populate(
    cache: &PopulateTomlCacheEntry,
    using_paths: &mut Vec<String>,
    pending_modules: &mut Vec<(String, String)>,
    aliases: &mut HashMap<String, String>,
    packages: &mut HashMap<String, UsingPackage>,
    module_roots: &mut Vec<(String, String)>,
) -> UsingPolicy {
    using_paths.extend(cache.using_paths.iter().cloned());
    pending_modules.extend(cache.pending_modules.iter().cloned());
    module_roots.extend(cache.module_roots.iter().cloned());

    for (k, v) in cache.aliases.iter() {
        aliases.insert(k.clone(), v.clone());
    }
    for (k, v) in cache.packages.iter() {
        packages.insert(k.clone(), v.clone());
    }

    cache.policy.clone()
}

fn find_preferred_toml(base: &Path) -> Option<PathBuf> {
    let candidates = ["hako.toml", "hakorune.toml", "nyash.toml"];
    for name in candidates.iter() {
        let candidate = base.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

fn same_manifest_path(lhs: &Path, rhs: &Path) -> bool {
    let lhs = lhs.canonicalize().unwrap_or_else(|_| lhs.to_path_buf());
    let rhs = rhs.canonicalize().unwrap_or_else(|_| rhs.to_path_buf());
    lhs == rhs
}

fn locate_toml_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(local) = find_preferred_toml(Path::new(".")) {
        paths.push(local);
    }
    if let Some(root) = env::env_string("NYASH_ROOT") {
        if let Some(root_manifest) = find_preferred_toml(Path::new(&root)) {
            if !paths
                .iter()
                .any(|existing| same_manifest_path(existing, &root_manifest))
            {
                paths.push(root_manifest);
            }
        }
    }
    paths
}

fn load_toml_content(path: &Path) -> Result<String, UsingError> {
    std::fs::read_to_string(path).map_err(|e| UsingError::ReadToml(e.to_string()))
}

fn should_skip_workspace_member_scan() -> bool {
    let is_stage1_child = std::env::var("NYASH_STAGE1_CLI_CHILD")
        .map(|v| v == "1")
        .unwrap_or(false);
    if !is_stage1_child {
        return false;
    }

    // Stage-1 bridge defaults this to 0. Respect explicit opt-in values.
    let apply_usings_enabled = std::env::var("HAKO_STAGEB_APPLY_USINGS")
        .map(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "True"))
        .unwrap_or(false);
    if apply_usings_enabled {
        return false;
    }

    // Only skip workspace member manifest scans when parent already supplied
    // a concrete module list for the child process.
    std::env::var("HAKO_STAGEB_MODULES_LIST")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}

fn read_stageb_modules_list_from_env() -> Vec<(String, String)> {
    let raw = match std::env::var("HAKO_STAGEB_MODULES_LIST") {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();
    for entry in raw.split("|||") {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((name, path)) = trimmed.split_once('=') {
            let name = name.trim();
            let path = path.trim();
            if !name.is_empty() && !path.is_empty() {
                out.push((name.to_string(), path.to_string()));
            }
        }
    }
    out
}

fn read_stageb_module_roots_list_from_env() -> Vec<(String, String)> {
    let raw = match std::env::var("HAKO_STAGEB_MODULE_ROOTS_LIST") {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut out = Vec::new();
    for entry in raw.split("|||") {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((prefix, path)) = trimmed.split_once('=') {
            let prefix = prefix.trim();
            let path = path.trim();
            if !prefix.is_empty() && !path.is_empty() {
                out.push((prefix.to_string(), path.to_string()));
            }
        }
    }
    out
}

fn resolve_manifest_path(toml_dir: &Path, raw: &str) -> String {
    if raw.is_empty()
        || Path::new(raw).is_absolute()
        || raw.starts_with("builtin:")
        || raw.starts_with("dylib:")
    {
        return raw.to_string();
    }

    let joined = toml_dir.join(raw);
    joined
        .canonicalize()
        .unwrap_or(joined)
        .to_string_lossy()
        .to_string()
}

fn merge_module_entry(entries: &mut Vec<(String, String)>, name: String, path: String) {
    if let Some((_, slot_path)) = entries.iter_mut().find(|(existing, _)| *existing == name) {
        *slot_path = path;
    } else {
        entries.push((name, path));
    }
}

fn merge_module_root(entries: &mut Vec<(String, String)>, prefix: String, path: String) {
    if let Some((_, slot_path)) = entries.iter_mut().find(|(existing, _)| *existing == prefix) {
        *slot_path = path;
    } else {
        entries.push((prefix, path));
    }
}

fn merge_using_path(entries: &mut Vec<String>, path: String) {
    if !entries.iter().any(|existing| existing == &path) {
        entries.push(path);
    }
}

fn merge_manifest_layer(
    toml_path: &Path,
    resolved_using_paths: &mut Vec<String>,
    resolved_pending_modules: &mut Vec<(String, String)>,
    resolved_aliases: &mut HashMap<String, String>,
    resolved_packages: &mut HashMap<String, UsingPackage>,
    resolved_module_roots: &mut Vec<(String, String)>,
    policy: &mut UsingPolicy,
) -> Result<(), UsingError> {
    let text = load_toml_content(toml_path)?;
    let doc =
        toml::from_str::<toml::Value>(&text).map_err(|e| UsingError::ParseToml(e.to_string()))?;
    let toml_dir = toml_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    if let Some(roots_tbl) = doc.get("module_roots").and_then(|v| v.as_table()) {
        for (prefix, path_val) in roots_tbl.iter() {
            if let Some(path_str) = path_val.as_str() {
                merge_module_root(
                    resolved_module_roots,
                    prefix.to_string(),
                    resolve_manifest_path(&toml_dir, path_str),
                );
            }
        }
    }

    if let Some(mods) = doc.get("modules").and_then(|v| v.as_table()) {
        fn visit(
            prefix: &str,
            tbl: &toml::value::Table,
            toml_dir: &Path,
            out: &mut Vec<(String, String)>,
        ) {
            for (k, v) in tbl.iter() {
                let name = if prefix.is_empty() {
                    k.to_string()
                } else {
                    format!("{}.{}", prefix, k)
                };
                if let Some(s) = v.as_str() {
                    merge_module_entry(out, name, resolve_manifest_path(toml_dir, s));
                } else if let Some(t) = v.as_table() {
                    visit(&name, t, toml_dir, out);
                }
            }
        }

        visit("", mods, &toml_dir, resolved_pending_modules);
        if let Some(workspace_tbl) = mods.get("workspace").and_then(|v| v.as_table()) {
            if !should_skip_workspace_member_scan() {
                load_workspace_modules(
                    &toml_dir,
                    workspace_tbl,
                    resolved_pending_modules,
                    resolved_aliases,
                )?;
            }
        }
    }

    if let Some(using_tbl) = doc.get("using").and_then(|v| v.as_table()) {
        if let Some(paths_arr) = using_tbl.get("paths").and_then(|v| v.as_array()) {
            for p in paths_arr {
                if let Some(s) = p.as_str() {
                    let s = s.trim();
                    if !s.is_empty() {
                        let resolved = resolve_manifest_path(&toml_dir, s);
                        merge_using_path(resolved_using_paths, resolved.clone());
                        policy.search_paths.push(resolved);
                    }
                }
            }
        }

        if let Some(alias_tbl) = using_tbl.get("aliases").and_then(|v| v.as_table()) {
            for (k, v) in alias_tbl.iter() {
                if let Some(target) = v.as_str() {
                    resolved_aliases.insert(k.to_string(), target.to_string());
                }
            }
        }

        for (k, v) in using_tbl.iter() {
            if k == "paths" || k == "aliases" {
                continue;
            }
            if let Some(tbl) = v.as_table() {
                let kind = tbl
                    .get("kind")
                    .and_then(|x| x.as_str())
                    .map(PackageKind::from_str)
                    .unwrap_or(PackageKind::Package);
                if let Some(path_s) = tbl.get("path").and_then(|x| x.as_str()) {
                    let path = resolve_manifest_path(&toml_dir, path_s);
                    let main = tbl
                        .get("main")
                        .and_then(|x| x.as_str())
                        .map(|s| s.to_string());
                    let bid = tbl
                        .get("bid")
                        .and_then(|x| x.as_str())
                        .map(|s| s.to_string());
                    resolved_packages.insert(
                        k.to_string(),
                        UsingPackage {
                            kind,
                            path,
                            main,
                            bid,
                        },
                    );
                }
            }
        }
    }

    if let Some(alias_tbl) = doc.get("aliases").and_then(|v| v.as_table()) {
        for (k, v) in alias_tbl.iter() {
            if let Some(target) = v.as_str() {
                resolved_aliases.insert(k.to_string(), target.to_string());
            }
        }
    }

    resolved_module_roots.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    Ok(())
}

/// Populate using context vectors from hako.toml/nyash.toml (if present).
/// Keeps behavior aligned with existing runner pipeline:
///  - Adds [using.paths] entries to `using_paths`
///  - Flattens [modules] into (name, path) pairs appended to `pending_modules`
///  - Reads optional [aliases] table (k -> v)
///  - Reads [module_roots] for prefix-based resolution (Phase 29bq+)
pub fn populate_from_toml(
    using_paths: &mut Vec<String>,
    pending_modules: &mut Vec<(String, String)>,
    aliases: &mut HashMap<String, String>,
    packages: &mut HashMap<String, UsingPackage>,
    module_roots: &mut Vec<(String, String)>,
) -> Result<UsingPolicy, UsingError> {
    // Stage-1 child env-only fast path:
    // when parent already provided a fully expanded modules map and module_roots list,
    // avoid TOML probing/reads entirely (binary-only and no-workspace dependency).
    if should_skip_workspace_member_scan() {
        pending_modules.extend(read_stageb_modules_list_from_env());
        module_roots.extend(read_stageb_module_roots_list_from_env());
        return Ok(UsingPolicy::default());
    }

    let toml_paths = locate_toml_paths();
    let cache_key = build_cache_key(&toml_paths);
    if let Some(cache) = lookup_cached_populate(&cache_key) {
        let policy = apply_cached_populate(
            &cache,
            using_paths,
            pending_modules,
            aliases,
            packages,
            module_roots,
        );
        return Ok(policy);
    }
    let mut policy = UsingPolicy::default();
    let mut resolved_using_paths: Vec<String> = Vec::new();
    let mut resolved_pending_modules: Vec<(String, String)> = Vec::new();
    let mut resolved_aliases: HashMap<String, String> = HashMap::new();
    let mut resolved_packages: HashMap<String, UsingPackage> = HashMap::new();
    let mut resolved_module_roots: Vec<(String, String)> = Vec::new();

    for toml_path in toml_paths.iter().rev() {
        merge_manifest_layer(
            toml_path,
            &mut resolved_using_paths,
            &mut resolved_pending_modules,
            &mut resolved_aliases,
            &mut resolved_packages,
            &mut resolved_module_roots,
            &mut policy,
        )?;
    }

    // Stage-1 child can receive a fully expanded module map via env.
    // Merge it after TOML load so exact entries are always available even
    // when workspace member manifests are skipped.
    for (name, path) in read_stageb_modules_list_from_env() {
        merge_module_entry(&mut resolved_pending_modules, name, path);
    }

    let cache = PopulateTomlCacheEntry {
        key: cache_key,
        using_paths: resolved_using_paths,
        pending_modules: resolved_pending_modules,
        aliases: resolved_aliases,
        packages: resolved_packages,
        module_roots: resolved_module_roots,
        policy,
    };

    let out_policy = apply_cached_populate(
        &cache,
        using_paths,
        pending_modules,
        aliases,
        packages,
        module_roots,
    );
    store_cached_populate(cache);
    Ok(out_policy)
}

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

fn load_workspace_modules(
    nyash_dir: &std::path::Path,
    workspace_tbl: &toml::value::Table,
    pending_modules: &mut Vec<(String, String)>,
    aliases: &mut HashMap<String, String>,
) -> Result<(), UsingError> {
    fn flatten_exports_table(
        prefix: &str,
        tbl: &toml::value::Table,
        out: &mut Vec<(String, String)>,
    ) {
        for (k, v) in tbl {
            let name = if prefix.is_empty() {
                k.to_string()
            } else {
                format!("{}.{}", prefix, k)
            };
            if let Some(path) = v.as_str() {
                out.push((name, path.to_string()));
            } else if let Some(child) = v.as_table() {
                flatten_exports_table(&name, child, out);
            }
        }
    }

    let members = workspace_tbl
        .get("members")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            UsingError::ParseWorkspaceModule(
                "modules.workspace".into(),
                "expected members array".into(),
            )
        })?;

    for entry in members {
        let raw_path = entry.as_str().ok_or_else(|| {
            UsingError::ParseWorkspaceModule(
                "modules.workspace".into(),
                "members must be string paths".into(),
            )
        })?;
        let module_path = if std::path::Path::new(raw_path).is_absolute() {
            std::path::PathBuf::from(raw_path)
        } else {
            nyash_dir.join(raw_path)
        };
        let module_dir = module_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| nyash_dir.to_path_buf());
        let module_text = std::fs::read_to_string(&module_path).map_err(|e| {
            UsingError::ReadWorkspaceModule(
                module_path.to_string_lossy().to_string(),
                e.to_string(),
            )
        })?;
        let module_doc = toml::from_str::<toml::Value>(&module_text).map_err(|e| {
            UsingError::ParseWorkspaceModule(
                module_path.to_string_lossy().to_string(),
                e.to_string(),
            )
        })?;
        let module_name = module_doc
            .get("module")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                UsingError::WorkspaceModuleMissingName(module_path.to_string_lossy().to_string())
            })?;
        if let Some(exports_tbl) = module_doc.get("exports").and_then(|v| v.as_table()) {
            let mut flat_exports: Vec<(String, String)> = Vec::new();
            flatten_exports_table("", exports_tbl, &mut flat_exports);
            for (export_key, rel_path) in flat_exports {
                let mut full_name = module_name.to_string();
                if !export_key.is_empty() {
                    full_name.push('.');
                    full_name.push_str(&export_key);
                }
                if pending_modules.iter().any(|(name, _)| name == &full_name) {
                    continue;
                }
                let resolved_path = module_dir.join(rel_path);
                let resolved_str = resolved_path
                    .canonicalize()
                    .unwrap_or(resolved_path)
                    .to_string_lossy()
                    .to_string();
                pending_modules.push((full_name, resolved_str));
            }
        }
        if let Some(alias_tbl) = module_doc.get("aliases").and_then(|v| v.as_table()) {
            for (alias, target) in alias_tbl {
                if let Some(target_str) = target.as_str() {
                    aliases.insert(alias.to_string(), target_str.to_string());
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
