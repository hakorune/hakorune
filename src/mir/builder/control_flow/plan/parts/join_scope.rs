//! Join-scope helpers for branch-local variables.
//!
//! Centralizes branch-local collection and map filtering so join obligations
//! treat branch-scoped locals consistently across parts.

use std::collections::{BTreeMap, BTreeSet};

pub(super) fn filter_branch_locals_from_maps(
    pre_if_map: &BTreeMap<String, crate::mir::ValueId>,
    then_map: &BTreeMap<String, crate::mir::ValueId>,
    else_map: &BTreeMap<String, crate::mir::ValueId>,
    branch_locals: &BTreeSet<String>,
) -> (
    BTreeMap<String, crate::mir::ValueId>,
    BTreeMap<String, crate::mir::ValueId>,
) {
    let mut then_map = then_map.clone();
    let mut else_map = else_map.clone();
    for name in branch_locals {
        if let Some(pre_val) = pre_if_map.get(name).copied() {
            then_map.insert(name.clone(), pre_val);
            else_map.insert(name.clone(), pre_val);
        } else {
            then_map.remove(name);
            else_map.remove(name);
        }
    }
    (then_map, else_map)
}

pub(super) fn collect_branch_local_vars_from_maps(
    pre_if_map: &BTreeMap<String, crate::mir::ValueId>,
    then_map: &BTreeMap<String, crate::mir::ValueId>,
    else_map: &BTreeMap<String, crate::mir::ValueId>,
) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    for name in then_map.keys() {
        if !pre_if_map.contains_key(name) && !else_map.contains_key(name) {
            locals.insert(name.clone());
        }
    }
    for name in else_map.keys() {
        if !pre_if_map.contains_key(name) && !then_map.contains_key(name) {
            locals.insert(name.clone());
        }
    }
    locals
}
