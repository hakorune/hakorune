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
use std::sync::OnceLock;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub(super) struct Stage1ModuleEnvLists {
    pub(super) modules_list: Option<String>,
    pub(super) module_roots_list: Option<String>,
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
            out.push((full_name, resolved.to_string_lossy().to_string()));
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

    #[test]
    fn embedded_snapshot_is_parseable_and_non_empty() {
        let lists = load_embedded_snapshot_lists().expect("embedded snapshot must parse");
        let mods = lists.modules_list.expect("embedded modules_list must exist");
        let roots = lists
            .module_roots_list
            .expect("embedded module_roots_list must exist");
        assert!(!mods.trim().is_empty());
        assert!(!roots.trim().is_empty());
    }
}
