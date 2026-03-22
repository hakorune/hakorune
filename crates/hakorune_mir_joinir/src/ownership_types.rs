//! Core types for JoinIR ownership analysis.

#[cfg(any(debug_assertions, test))]
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeOwnedVar {
    pub name: String,
    pub is_written: bool,
    pub is_condition_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayVar {
    pub name: String,
    pub owner_scope: ScopeId,
    pub relay_path: Vec<ScopeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedVar {
    pub name: String,
    pub owner_scope: ScopeId,
}

#[derive(Debug, Clone, Default)]
pub struct OwnershipPlan {
    pub scope_id: ScopeId,
    pub owned_vars: Vec<ScopeOwnedVar>,
    pub relay_writes: Vec<RelayVar>,
    pub captures: Vec<CapturedVar>,
    pub condition_captures: Vec<CapturedVar>,
}

impl Default for ScopeId {
    fn default() -> Self {
        ScopeId(0)
    }
}

impl OwnershipPlan {
    pub fn new(scope_id: ScopeId) -> Self {
        Self {
            scope_id,
            ..Default::default()
        }
    }

    pub fn carriers(&self) -> impl Iterator<Item = &ScopeOwnedVar> {
        self.owned_vars.iter().filter(|v| v.is_written)
    }

    pub fn condition_only_carriers(&self) -> impl Iterator<Item = &ScopeOwnedVar> {
        self.owned_vars
            .iter()
            .filter(|v| v.is_written && v.is_condition_only)
    }

    #[cfg(any(debug_assertions, test))]
    pub fn verify_invariants(&self) -> Result<(), String> {
        let mut all_names: BTreeSet<&str> = BTreeSet::new();

        for v in &self.owned_vars {
            if !all_names.insert(&v.name) {
                return Err(format!("Duplicate owned var: {}", v.name));
            }
        }

        for v in &self.relay_writes {
            if self.owned_vars.iter().any(|o| o.name == v.name) {
                return Err(format!("Relay var '{}' conflicts with owned", v.name));
            }
        }

        for v in &self.captures {
            if self.owned_vars.iter().any(|o| o.name == v.name) {
                return Err(format!("Captured var '{}' conflicts with owned", v.name));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_plan() {
        let plan = OwnershipPlan::new(ScopeId(1));
        assert_eq!(plan.scope_id.0, 1);
        assert!(plan.owned_vars.is_empty());
        assert_eq!(plan.carriers().count(), 0);
    }

    #[test]
    fn carriers_filter() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "limit".to_string(),
            is_written: false,
            is_condition_only: false,
        });

        let carriers: Vec<_> = plan.carriers().collect();
        assert_eq!(carriers.len(), 1);
        assert_eq!(carriers[0].name, "sum");
    }

    #[test]
    fn invariant_verification() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "x".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        assert!(plan.verify_invariants().is_ok());

        plan.relay_writes.push(RelayVar {
            name: "x".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![],
        });

        assert!(plan.verify_invariants().is_err());
    }
}
