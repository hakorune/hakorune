//! Minimal global registry for env.modules (Phase 15 P0b)

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::box_trait::NyashBox;

static REGISTRY: Lazy<Mutex<HashMap<String, Box<dyn NyashBox>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn set(name: String, value: Box<dyn NyashBox>) {
    if let Ok(mut map) = REGISTRY.lock() {
        map.insert(name, value);
    }
}

pub fn get(name: &str) -> Option<Box<dyn NyashBox>> {
    if let Ok(mut map) = REGISTRY.lock() {
        if let Some(b) = map.get_mut(name) {
            // clone_box to hand out an owned copy
            return Some(b.clone_box());
        }
    }
    None
}

/// Snapshot names and their stringified values (best‑effort).
/// Intended for diagnostics; values are obtained via to_string_box().value.
pub fn snapshot_names_and_strings() -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Ok(mut map) = REGISTRY.lock() {
        for (k, v) in map.iter_mut() {
            // Best-effort stringify
            let s = v.to_string_box().value;
            out.push((k.clone(), s));
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleRootsUnavailable;

/// Snapshot all Box values as GC roots, rejecting unavailable storage.
/// Uses clone_box() to obtain owned copies and wraps them into Arc for traversal.
pub fn snapshot_boxes() -> Result<Vec<std::sync::Arc<dyn NyashBox>>, ModuleRootsUnavailable> {
    snapshot_box_values(&REGISTRY)
}

fn snapshot_box_values(
    registry: &Mutex<HashMap<String, Box<dyn NyashBox>>>,
) -> Result<Vec<std::sync::Arc<dyn NyashBox>>, ModuleRootsUnavailable> {
    let map = registry.lock().map_err(|_| ModuleRootsUnavailable)?;
    Ok(map
        .values()
        .map(|value| std::sync::Arc::from(value.clone_box()))
        .collect())
}

#[cfg(test)]
#[path = "modules_registry_snapshot_tests.rs"]
mod snapshot_tests;
