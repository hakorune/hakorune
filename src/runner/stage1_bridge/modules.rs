/*!
 * Stage-1 CLI bridge - modules list collector
 *
 * Provides module mappings for Stage-1 CLI environment variables.
 *
 * Source of truth priority:
 *   1) Embedded snapshot (default; binary-only safe)
 *   2) TOML registry parse when NYASH_STAGE1_MODULES_SOURCE=toml
 *      - [modules.workspace] members (*_module.toml exports)
 *      - [modules] exact entries (override workspace on key collision)
 */

use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub(super) struct Stage1ModuleEnvLists {
    pub(super) modules_list: Option<String>,
    pub(super) module_roots_list: Option<String>,
}

impl Stage1ModuleEnvLists {
    pub(super) fn apply_to_command_if_missing(&self, cmd: &mut Command) {
        if std::env::var("HAKO_STAGEB_MODULES_LIST").is_err() {
            if let Some(modules) = self.modules_list.as_ref() {
                cmd.env("HAKO_STAGEB_MODULES_LIST", modules);
            }
        }

        if std::env::var("HAKO_STAGEB_MODULE_ROOTS_LIST").is_err() {
            if let Some(roots) = self.module_roots_list.as_ref() {
                cmd.env("HAKO_STAGEB_MODULE_ROOTS_LIST", roots);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Stage1ModuleEnvCacheSnapshot {
    signature: u64,
    modules_list: Option<String>,
    module_roots_list: Option<String>,
}

#[derive(Deserialize)]
struct EmbeddedStage1ModuleEnvSnapshot {
    #[allow(dead_code)]
    schema: Option<String>,
    modules_list: Option<String>,
    module_roots_list: Option<String>,
}

const EMBEDDED_STAGE1_MODULES_SNAPSHOT: &str =
    include_str!("embedded_stage1_modules_snapshot.json");

fn module_source_prefers_embedded() -> bool {
    let source = std::env::var("NYASH_STAGE1_MODULES_SOURCE")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .unwrap_or_else(|| "embedded".to_string());
    source != "toml" && source != "runtime"
}

fn load_embedded_snapshot_lists() -> Option<Stage1ModuleEnvLists> {
    static SNAPSHOT: OnceLock<Option<Stage1ModuleEnvLists>> = OnceLock::new();
    SNAPSHOT
        .get_or_init(|| {
            let parsed: EmbeddedStage1ModuleEnvSnapshot =
                serde_json::from_str(EMBEDDED_STAGE1_MODULES_SNAPSHOT).ok()?;
            Some(Stage1ModuleEnvLists {
                modules_list: parsed.modules_list,
                module_roots_list: parsed.module_roots_list,
            })
        })
        .clone()
}

fn cache_file_path() -> PathBuf {
    if let Ok(path) = std::env::var("NYASH_STAGE1_MODULES_CACHE") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    if let Ok(root) = std::env::var("NYASH_ROOT") {
        let trimmed = root.trim();
        if !trimmed.is_empty() {
            return Path::new(trimmed)
                .join("target")
                .join(".cache")
                .join("stage1_module_env.json");
        }
    }
    PathBuf::from("target/.cache/stage1_module_env.json")
}

fn hash_metadata(hasher: &mut std::collections::hash_map::DefaultHasher, path: &Path) {
    path.to_string_lossy().hash(hasher);
    match fs::metadata(path) {
        Ok(meta) => {
            meta.len().hash(hasher);
            let modified_ns = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            modified_ns.hash(hasher);
        }
        Err(_) => {
            0u64.hash(hasher);
            0u128.hash(hasher);
        }
    }
}

fn compute_workspace_signature(doc: &toml::Value, toml_path: &Path) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_metadata(&mut hasher, toml_path);
    let toml_dir = toml_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    if let Some(members) = doc
        .get("modules")
        .and_then(|v| v.get("workspace"))
        .and_then(|v| v.get("members"))
        .and_then(|v| v.as_array())
    {
        for member in members {
            if let Some(member_path_raw) = member.as_str() {
                member_path_raw.hash(&mut hasher);
                let module_path = if Path::new(member_path_raw).is_absolute() {
                    PathBuf::from(member_path_raw)
                } else {
                    toml_dir.join(member_path_raw)
                };
                hash_metadata(&mut hasher, &module_path);
            }
        }
    }
    hasher.finish()
}

fn load_snapshot_cache(signature: u64) -> Option<Stage1ModuleEnvLists> {
    let path = cache_file_path();
    let text = fs::read_to_string(path).ok()?;
    let snapshot = serde_json::from_str::<Stage1ModuleEnvCacheSnapshot>(&text).ok()?;
    if snapshot.signature != signature {
        return None;
    }
    Some(Stage1ModuleEnvLists {
        modules_list: snapshot.modules_list,
        module_roots_list: snapshot.module_roots_list,
    })
}

fn store_snapshot_cache(signature: u64, lists: &Stage1ModuleEnvLists) {
    let path = cache_file_path();
    let snapshot = Stage1ModuleEnvCacheSnapshot {
        signature,
        modules_list: lists.modules_list.clone(),
        module_roots_list: lists.module_roots_list.clone(),
    };
    let Ok(body) = serde_json::to_string(&snapshot) else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, body);
}

/// Find the first existing TOML config file (hako.toml preferred)
fn find_toml_config() -> Option<PathBuf> {
    for name in ["hako.toml", "hakorune.toml", "nyash.toml"] {
        let path = PathBuf::from(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn parse_toml(path: &Path) -> Option<toml::Value> {
    let content = fs::read_to_string(path).ok()?;
    toml::from_str::<toml::Value>(&content).ok()
}

fn load_registry_doc() -> Option<(toml::Value, PathBuf)> {
    let path = find_toml_config()?;
    let doc = parse_toml(&path)?;
    Some((doc, path))
}

fn flatten_modules_table(prefix: &str, tbl: &toml::value::Table, out: &mut Vec<(String, String)>) {
    for (k, v) in tbl.iter() {
        // [modules.workspace] is handled separately as workspace member manifests.
        if prefix.is_empty() && k == "workspace" {
            continue;
        }
        let name = if prefix.is_empty() {
            k.to_string()
        } else {
            format!("{}.{}", prefix, k)
        };
        if let Some(s) = v.as_str() {
            out.push((name, s.to_string()));
        } else if let Some(child) = v.as_table() {
            flatten_modules_table(&name, child, out);
        }
    }
}

fn collect_workspace_module_entries(doc: &toml::Value, toml_dir: &Path) -> Vec<(String, String)> {
    fn normalize_workspace_export_path(path: PathBuf, repo_root: &Path) -> String {
        let normalized = fs::canonicalize(&path).unwrap_or(path);
        if let Ok(relative) = normalized.strip_prefix(repo_root) {
            return relative.to_string_lossy().to_string();
        }
        normalized.to_string_lossy().to_string()
    }

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

    let mut out = Vec::new();
    let repo_root = if toml_dir.as_os_str().is_empty() {
        fs::canonicalize(".").unwrap_or_else(|_| PathBuf::from("."))
    } else {
        fs::canonicalize(toml_dir).unwrap_or_else(|_| toml_dir.to_path_buf())
    };
    let members = match doc
        .get("modules")
        .and_then(|v| v.get("workspace"))
        .and_then(|v| v.get("members"))
        .and_then(|v| v.as_array())
    {
        Some(members) => members,
        None => return out,
    };

    for member in members {
        let Some(member_path_raw) = member.as_str() else {
            continue;
        };
        let module_path = if Path::new(member_path_raw).is_absolute() {
            PathBuf::from(member_path_raw)
        } else {
            toml_dir.join(member_path_raw)
        };
        let Ok(module_text) = fs::read_to_string(&module_path) else {
            continue;
        };
        let Ok(module_doc) = toml::from_str::<toml::Value>(&module_text) else {
            continue;
        };
        let Some(module_name) = module_doc
            .get("module")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
        else {
            continue;
        };
        let Some(exports_tbl) = module_doc.get("exports").and_then(|v| v.as_table()) else {
            continue;
        };
        let mut flat_exports: Vec<(String, String)> = Vec::new();
        flatten_exports_table("", exports_tbl, &mut flat_exports);
        let module_dir = module_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| toml_dir.to_path_buf());
        for (export_key, rel_path) in flat_exports {
            let full_name = if export_key.is_empty() {
                module_name.to_string()
            } else {
                format!("{}.{}", module_name, export_key)
            };
            let resolved = module_dir.join(rel_path);
            out.push((full_name, normalize_workspace_export_path(resolved, &repo_root)));
        }
    }

    out
}

/// Collect modules list from hako.toml/nyash.toml registry
///
/// Returns a "|||"-separated list of "key=value" entries for HAKO_STAGEB_MODULES_LIST.
/// Includes well-known aliases required by Stage-1 CLI if absent.
///
/// Merge policy:
/// - workspace exports are loaded first
/// - [modules] exact entries override workspace entries with same key
fn collect_modules_list_from_doc(doc: &toml::Value, path: &Path) -> Option<String> {
    let toml_dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut entries: Vec<String> = Vec::new();
    let mut values: HashMap<String, String> = HashMap::new();

    for (key, val) in collect_workspace_module_entries(&doc, &toml_dir) {
        if !values.contains_key(&key) {
            entries.push(key.clone());
        }
        values.insert(key, val);
    }

    if let Some(mods) = doc.get("modules").and_then(|v| v.as_table()) {
        let mut flat = Vec::new();
        flatten_modules_table("", mods, &mut flat);
        for (key, val) in flat {
            if !values.contains_key(&key) {
                entries.push(key.clone());
            }
            values.insert(key, val);
        }
    }

    // Add a few well-known aliases required by Stage-1 CLI if they are absent in nyash.toml.
    for (k, v) in [
        (
            "lang.compiler.entry.using_resolver_box",
            "lang/src/compiler/entry/using_resolver_box.hako",
        ),
        (
            "selfhost.shared.host_bridge.codegen_bridge",
            "lang/src/shared/host_bridge/codegen_bridge_box.hako",
        ),
    ] {
        if !values.contains_key(k) {
            entries.push(k.to_string());
            values.insert(k.to_string(), v.to_string());
        }
    }

    let formatted: Vec<String> = entries
        .into_iter()
        .filter_map(|k| values.get(&k).map(|v| format!("{k}={v}")))
        .collect();

    if formatted.is_empty() {
        return None;
    }

    Some(formatted.join("|||"))
}

/// Collect module_roots list from hako.toml/nyash.toml [module_roots] section
///
/// Returns a "|||"-separated list of "prefix=path" entries for HAKO_STAGEB_MODULE_ROOTS_LIST.
/// Entries are sorted by prefix length descending (longest match first) for Stage-B.
fn collect_module_roots_list_from_doc(doc: &toml::Value) -> Option<String> {
    let mut entries: Vec<(String, String)> = Vec::new();
    if let Some(roots) = doc.get("module_roots").and_then(|v| v.as_table()) {
        for (k, v) in roots {
            if let Some(path_str) = v.as_str() {
                entries.push((k.to_string(), path_str.to_string()));
            }
        }
    }
    if entries.is_empty() {
        None
    } else {
        // Sort by prefix length descending for longest-match-first resolution
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        let formatted: Vec<String> = entries
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        Some(formatted.join("|||"))
    }
}

/// Collect both modules and module_roots env payloads with a single TOML parse.
pub(super) fn collect_module_env_lists() -> Stage1ModuleEnvLists {
    // BINARY-ONLY-B03: embedded snapshot is the default source.
    // Set NYASH_STAGE1_MODULES_SOURCE=toml to force runtime TOML collection.
    if module_source_prefers_embedded() {
        if let Some(embedded) = load_embedded_snapshot_lists() {
            return embedded;
        }
    }

    let Some((doc, path)) = load_registry_doc() else {
        return load_embedded_snapshot_lists().unwrap_or_default();
    };

    let signature = compute_workspace_signature(&doc, &path);
    if let Some(cached) = load_snapshot_cache(signature) {
        return cached;
    }

    let lists = Stage1ModuleEnvLists {
        modules_list: collect_modules_list_from_doc(&doc, &path),
        module_roots_list: collect_module_roots_list_from_doc(&doc),
    };
    store_snapshot_cache(signature, &lists);
    lists
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::stage1_bridge::test_support;
    use std::collections::BTreeMap;

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, &'static str)]) -> Self {
            let mut saved = Vec::with_capacity(vars.len());
            for (key, value) in vars {
                saved.push((*key, std::env::var(key).ok()));
                std::env::set_var(key, value);
            }
            Self { saved }
        }

        fn clear(keys: &[&'static str]) -> Self {
            let mut saved = Vec::with_capacity(keys.len());
            for key in keys {
                saved.push((*key, std::env::var(key).ok()));
                std::env::remove_var(key);
            }
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, old_value) in self.saved.drain(..) {
                if let Some(value) = old_value {
                    std::env::set_var(key, value);
                } else {
                    std::env::remove_var(key);
                }
            }
        }
    }

    fn command_env_map(cmd: &Command) -> BTreeMap<String, String> {
        cmd.get_envs()
            .filter_map(|(key, value)| {
                Some((
                    key.to_string_lossy().into_owned(),
                    value?.to_string_lossy().into_owned(),
                ))
            })
            .collect()
    }

    fn parse_kv_map(raw: &str) -> BTreeMap<String, String> {
        raw.split("|||")
            .filter_map(|entry| {
                let trimmed = entry.trim();
                if trimmed.is_empty() {
                    return None;
                }
                let (key, value) = trimmed.split_once('=')?;
                Some((key.to_string(), value.to_string()))
            })
            .collect()
    }

    fn parse_kv_list(raw: &str) -> Vec<(String, String)> {
        let mut entries: Vec<(String, String)> = raw
            .split("|||")
            .filter_map(|entry| {
                let trimmed = entry.trim();
                if trimmed.is_empty() {
                    return None;
                }
                let (key, value) = trimmed.split_once('=')?;
                Some((key.to_string(), value.to_string()))
            })
            .collect();
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
        entries
    }

    #[test]
    fn embedded_snapshot_is_parseable_and_non_empty() {
        let lists = load_embedded_snapshot_lists().expect("embedded snapshot must parse");
        let mods = lists
            .modules_list
            .expect("embedded modules_list must exist");
        let roots = lists
            .module_roots_list
            .expect("embedded module_roots_list must exist");
        assert!(!mods.trim().is_empty());
        assert!(!roots.trim().is_empty());
    }

    #[test]
    fn embedded_snapshot_matches_registry_doc() {
        let (doc, path) = load_registry_doc().expect("registry doc must exist for snapshot parity");
        let expected = Stage1ModuleEnvLists {
            modules_list: collect_modules_list_from_doc(&doc, &path),
            module_roots_list: collect_module_roots_list_from_doc(&doc),
        };
        let embedded = load_embedded_snapshot_lists().expect("embedded snapshot must parse");

        let embedded_modules = embedded
            .modules_list
            .as_deref()
            .map(parse_kv_map);
        let expected_modules = expected
            .modules_list
            .as_deref()
            .map(parse_kv_map);
        let embedded_roots = embedded
            .module_roots_list
            .as_deref()
            .map(parse_kv_list);
        let expected_roots = expected
            .module_roots_list
            .as_deref()
            .map(parse_kv_list);
        let embedded_modules_map = embedded_modules.clone().unwrap_or_default();
        let expected_modules_map = expected_modules.clone().unwrap_or_default();
        let mut only_embedded = Vec::new();
        let mut only_expected = Vec::new();
        let mut value_diff = Vec::new();

        for (key, value) in &embedded_modules_map {
            match expected_modules_map.get(key) {
                Some(expected_value) if expected_value == value => {}
                Some(expected_value) => value_diff.push(format!(
                    "{key}: embedded={value} expected={expected_value}"
                )),
                None => only_embedded.push(format!("{key}={value}")),
            }
        }
        for (key, value) in &expected_modules_map {
            if !embedded_modules_map.contains_key(key) {
                only_expected.push(format!("{key}={value}"));
            }
        }

        assert!(
            only_embedded.is_empty() && only_expected.is_empty() && value_diff.is_empty(),
            "embedded modules_list map is stale; run tools/selfhost/refresh_stage1_module_env_snapshot.sh; only_embedded={only_embedded:?}; only_expected={only_expected:?}; value_diff={value_diff:?}",
        );
        assert_eq!(
            embedded_roots,
            expected_roots,
            "embedded module_roots_list is stale; run tools/selfhost/refresh_stage1_module_env_snapshot.sh",
        );
    }

    #[test]
    fn apply_to_command_if_missing_sets_stageb_lists() {
        let _lock = test_support::env_lock().lock().unwrap();
        let _clear =
            EnvGuard::clear(&["HAKO_STAGEB_MODULES_LIST", "HAKO_STAGEB_MODULE_ROOTS_LIST"]);
        let lists = Stage1ModuleEnvLists {
            modules_list: Some("core=lang/core".into()),
            module_roots_list: Some("core=lang".into()),
        };
        let mut cmd = Command::new("true");

        lists.apply_to_command_if_missing(&mut cmd);
        let envs = command_env_map(&cmd);

        assert_eq!(
            envs.get("HAKO_STAGEB_MODULES_LIST"),
            Some(&"core=lang/core".to_string())
        );
        assert_eq!(
            envs.get("HAKO_STAGEB_MODULE_ROOTS_LIST"),
            Some(&"core=lang".to_string())
        );
    }

    #[test]
    fn apply_to_command_if_missing_preserves_parent_stageb_lists() {
        let _lock = test_support::env_lock().lock().unwrap();
        let _clear =
            EnvGuard::clear(&["HAKO_STAGEB_MODULES_LIST", "HAKO_STAGEB_MODULE_ROOTS_LIST"]);
        let _set = EnvGuard::set(&[
            ("HAKO_STAGEB_MODULES_LIST", "parent-mods"),
            ("HAKO_STAGEB_MODULE_ROOTS_LIST", "parent-roots"),
        ]);
        let lists = Stage1ModuleEnvLists {
            modules_list: Some("core=lang/core".into()),
            module_roots_list: Some("core=lang".into()),
        };
        let mut cmd = Command::new("true");

        lists.apply_to_command_if_missing(&mut cmd);
        let envs = command_env_map(&cmd);

        assert!(!envs.contains_key("HAKO_STAGEB_MODULES_LIST"));
        assert!(!envs.contains_key("HAKO_STAGEB_MODULE_ROOTS_LIST"));
    }
}
