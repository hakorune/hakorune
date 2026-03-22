use super::super::{CapturedVar, OwnershipPlan, RelayVar, ScopeId, ScopeOwnedVar};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Scope kind for ownership analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Function,
    Loop,
    Block,
    If,
}

/// Internal scope representation during JSON analysis.
#[derive(Debug)]
pub(super) struct ScopeInfo {
    pub(super) id: ScopeId,
    pub(super) kind: ScopeKind,
    pub(super) parent: Option<ScopeId>,
    pub(super) defined: BTreeSet<String>,
    pub(super) reads: BTreeSet<String>,
    pub(super) writes: BTreeSet<String>,
    pub(super) condition_reads: BTreeSet<String>,
}

/// Analyzes ProgramJSON and produces `OwnershipPlan`.
pub struct OwnershipAnalyzer {
    pub(super) scopes: BTreeMap<ScopeId, ScopeInfo>,
    pub(super) next_scope_id: u32,
}

impl OwnershipAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: BTreeMap::new(),
            next_scope_id: 0,
        }
    }

    pub(super) fn alloc_scope(&mut self, kind: ScopeKind, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId(self.next_scope_id);
        self.next_scope_id += 1;
        self.scopes.insert(
            id,
            ScopeInfo {
                id,
                kind,
                parent,
                defined: BTreeSet::new(),
                reads: BTreeSet::new(),
                writes: BTreeSet::new(),
                condition_reads: BTreeSet::new(),
            },
        );
        id
    }

    pub(super) fn analyze_function(
        &mut self,
        func: &Value,
        parent: Option<ScopeId>,
    ) -> Result<ScopeId, String> {
        let scope_id = self.alloc_scope(ScopeKind::Function, parent);

        if let Some(params) = func.get("params").and_then(|p| p.as_array()) {
            for param in params {
                if let Some(name) = param.as_str() {
                    self.scopes
                        .get_mut(&scope_id)
                        .unwrap()
                        .defined
                        .insert(name.to_string());
                }
            }
        }

        if let Some(body) = func.get("body") {
            self.analyze_statement(body, scope_id)?;
        }

        Ok(scope_id)
    }

    pub(super) fn is_defined_in_scope_chain(&self, from_scope: ScopeId, name: &str) -> bool {
        let mut current = Some(from_scope);
        while let Some(id) = current {
            let scope = match self.scopes.get(&id) {
                Some(scope) => scope,
                None => break,
            };
            if scope.defined.contains(name) {
                return true;
            }
            current = scope.parent;
        }
        false
    }

    pub(super) fn find_enclosing_loop_or_function(&self, scope_id: ScopeId) -> ScopeId {
        let scope = &self.scopes[&scope_id];
        match scope.kind {
            ScopeKind::Loop | ScopeKind::Function => scope_id,
            _ => {
                if let Some(parent) = scope.parent {
                    self.find_enclosing_loop_or_function(parent)
                } else {
                    scope_id
                }
            }
        }
    }

    pub(super) fn propagate_to_parent(&mut self, child_id: ScopeId) {
        let child = &self.scopes[&child_id];
        let reads = child.reads.clone();
        let writes = child.writes.clone();
        let cond_reads = child.condition_reads.clone();
        let child_kind = child.kind;
        let child_defined = child.defined.clone();

        if let Some(parent_id) = child.parent {
            let parent = self.scopes.get_mut(&parent_id).unwrap();
            parent.reads.extend(reads);
            parent.condition_reads.extend(cond_reads);

            if child_kind == ScopeKind::Loop || child_kind == ScopeKind::Function {
                for write in writes {
                    if !child_defined.contains(&write) {
                        parent.writes.insert(write);
                    }
                }
            } else {
                parent.writes.extend(writes);
            }
        }
    }

    pub(super) fn build_plans(&self) -> Result<Vec<OwnershipPlan>, String> {
        let mut plans = Vec::new();

        for (_, scope) in &self.scopes {
            if scope.kind != ScopeKind::Loop && scope.kind != ScopeKind::Function {
                continue;
            }

            let mut plan = OwnershipPlan::new(scope.id);

            for name in &scope.defined {
                let is_written = scope.writes.contains(name);
                let is_condition_only = scope.condition_reads.contains(name)
                    && !scope
                        .writes
                        .iter()
                        .any(|w| w == name && !scope.condition_reads.contains(w));

                plan.owned_vars.push(ScopeOwnedVar {
                    name: name.clone(),
                    is_written,
                    is_condition_only: is_condition_only && is_written,
                });
            }

            for name in &scope.writes {
                if scope.defined.contains(name) {
                    continue;
                }

                if let Some((owner_scope, relay_path)) = self.find_owner(scope.id, name) {
                    plan.relay_writes.push(RelayVar {
                        name: name.clone(),
                        owner_scope,
                        relay_path,
                    });
                } else {
                    return Err(format!(
                        "Relay violation: variable '{}' written in scope {:?} has no owner",
                        name, scope.id
                    ));
                }
            }

            for name in &scope.reads {
                if scope.defined.contains(name) || scope.writes.contains(name) {
                    continue;
                }

                if let Some((owner_scope, _)) = self.find_owner(scope.id, name) {
                    plan.captures.push(CapturedVar {
                        name: name.clone(),
                        owner_scope,
                    });
                }
            }

            for cap in &plan.captures {
                if scope.condition_reads.contains(&cap.name) {
                    plan.condition_captures.push(cap.clone());
                }
            }

            #[cfg(debug_assertions)]
            plan.verify_invariants()?;

            plans.push(plan);
        }

        Ok(plans)
    }

    fn find_owner(&self, from_scope: ScopeId, name: &str) -> Option<(ScopeId, Vec<ScopeId>)> {
        let mut current = from_scope;
        let mut path = Vec::new();

        loop {
            let scope = &self.scopes[&current];

            if scope.defined.contains(name) {
                return Some((current, path));
            }

            if let Some(parent) = scope.parent {
                if scope.kind == ScopeKind::Loop {
                    path.push(current);
                }
                current = parent;
            } else {
                return None;
            }
        }
    }
}

impl Default for OwnershipAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
