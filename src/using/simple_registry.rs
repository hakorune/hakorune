//! Simple ModuleRegistry for Phase 1 diagnostics
//! Collects published symbols (top-level `static box Name`) from using targets.

use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

static CACHE: Lazy<Mutex<HashMap<String, HashSet<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static PATH_CACHE: Lazy<Mutex<HashMap<String, Vec<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Return candidate using names whose exported symbols contain `symbol`.
/// Uses runtime::modules_registry snapshot (name -> path token) and scans files.
pub fn suggest_using_for_symbol(symbol: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let snap = crate::runtime::modules_registry::snapshot_names_and_strings();
    let wanted = symbol.trim();
    if wanted.is_empty() {
        return results;
    }

    for (name, path_token) in snap {
        // Skip builtin/dylib marker tokens
        if path_token.starts_with("builtin:") || path_token.starts_with("dylib:") {
            continue;
        }
        // Ensure cache for this key
        let mut guard = CACHE.lock().ok();
        let set = guard
            .as_mut()
            .map(|m| m.entry(name.clone()).or_insert_with(HashSet::new))
            .expect("module cache poisoned");
        if set.is_empty() {
            if let Some(p) = resolve_path(&path_token) {
                if let Ok(content) = std::fs::read_to_string(&p) {
                    let syms = scan_static_boxes(&content);
                    for s in syms {
                        set.insert(s);
                    }
                }
            }
        }
        if set.contains(wanted) {
            results.push(name);
        }
    }
    results.sort();
    results.dedup();
    results
}

fn resolve_path(token: &str) -> Option<std::path::PathBuf> {
    let mut p = std::path::PathBuf::from(token);
    if p.is_relative() {
        if let Ok(abs) = std::fs::canonicalize(&p) {
            p = abs;
        }
    }
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

/// Scan a source file and return the top-level `static box` names it publishes.
pub fn published_static_boxes_for_path(path: &str) -> Vec<String> {
    let canon = std::fs::canonicalize(path)
        .ok()
        .map(|pb| pb.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());

    if let Some(hit) = PATH_CACHE
        .lock()
        .ok()
        .and_then(|guard| guard.get(&canon).cloned())
    {
        return hit;
    }

    let boxes = std::fs::read_to_string(&canon)
        .ok()
        .map(|content| scan_static_boxes(&content))
        .unwrap_or_default();

    if let Ok(mut guard) = PATH_CACHE.lock() {
        guard.insert(canon, boxes.clone());
    }

    boxes
}

/// Resolve an explicit `using ... as Alias` binding to a published static box name.
///
/// Contract:
/// - one exported static box => bind the alias to that box
/// - multiple exports => only bind when the alias itself matches one of them
/// - no unambiguous export => return None and let the caller stay explicit/fail-fast
pub fn resolve_imported_static_box(path: &str, alias: &str) -> Option<String> {
    let boxes = published_static_boxes_for_path(path);
    if boxes.len() == 1 {
        return boxes.into_iter().next();
    }
    boxes.into_iter().find(|name| name == alias)
}

fn scan_static_boxes(content: &str) -> Vec<String> {
    // Very simple lexer: find lines like `static box Name {`
    // Avoid matching inside comments by skipping lines that start with //
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("//") {
            continue;
        }
        if let Some(rest) = t.strip_prefix("static box ") {
            let mut name = String::new();
            for ch in rest.chars() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    name.push(ch);
                } else {
                    break;
                }
            }
            if !name.is_empty() {
                out.push(name);
            }
        }
    }
    out
}
