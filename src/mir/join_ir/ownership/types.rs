//! Core types for ownership analysis.
//!
//! Phase 56: Interface definitions only (not yet used).

#[cfg(any(debug_assertions, test))]
use std::collections::BTreeSet;

/// Unique identifier for a scope (loop, function, block).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScopeId(pub u32);

/// A variable owned by the current scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeOwnedVar {
    /// Variable name
    pub name: String,
    /// Whether this variable is written within the scope
    pub is_written: bool,
    /// Whether this variable is used in loop conditions
    pub is_condition_only: bool,
}

/// A variable whose updates should be relayed to an ancestor owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayVar {
    /// Variable name
    pub name: String,
    /// Scope that owns this variable
    pub owner_scope: ScopeId,
    /// Intermediate scopes that need to forward this update
    pub relay_path: Vec<ScopeId>,
}

/// A variable captured (read-only) from an ancestor scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedVar {
    /// Variable name
    pub name: String,
    /// Scope that owns this variable
    pub owner_scope: ScopeId,
}

/// Complete ownership analysis result for a scope.
#[derive(Debug, Clone, Default)]
pub struct OwnershipPlan {
    /// ID of this scope
    pub scope_id: ScopeId,

    /// Variables owned by this scope (defined here)
    /// Invariant: carriers = owned_vars where is_written = true
    pub owned_vars: Vec<ScopeOwnedVar>,

    /// Variables written but owned by ancestor (need relay)
    pub relay_writes: Vec<RelayVar>,

    /// Variables read but not owned (read-only capture)
    pub captures: Vec<CapturedVar>,

    /// Subset of captures used in conditions
    pub condition_captures: Vec<CapturedVar>,
}

impl Default for ScopeId {
    fn default() -> Self {
        ScopeId(0)
    }
}

impl OwnershipPlan {
    /// Create empty plan for a scope.
    pub fn new(scope_id: ScopeId) -> Self {
        Self {
            scope_id,
            ..Default::default()
        }
    }

    /// Get carriers (owned AND written).
    pub fn carriers(&self) -> impl Iterator<Item = &ScopeOwnedVar> {
        self.owned_vars.iter().filter(|v| v.is_written)
    }

    /// Get condition-only carriers (owned, written, condition-only).
    pub fn condition_only_carriers(&self) -> impl Iterator<Item = &ScopeOwnedVar> {
        self.owned_vars
            .iter()
            .filter(|v| v.is_written && v.is_condition_only)
    }

    /// Check invariant: no variable appears in multiple categories.
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
    fn test_empty_plan() {
        let plan = OwnershipPlan::new(ScopeId(1));
        assert_eq!(plan.scope_id.0, 1);
        assert!(plan.owned_vars.is_empty());
        assert_eq!(plan.carriers().count(), 0);
    }

    #[test]
    fn test_carriers_filter() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "limit".to_string(),
            is_written: false, // read-only owned
            is_condition_only: false,
        });

        let carriers: Vec<_> = plan.carriers().collect();
        assert_eq!(carriers.len(), 1);
        assert_eq!(carriers[0].name, "sum");
    }

    #[test]
    fn test_invariant_verification() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "x".to_string(),
            is_written: true,
            is_condition_only: false,
        });

        // Valid plan
        assert!(plan.verify_invariants().is_ok());

        // Add conflicting relay
        plan.relay_writes.push(RelayVar {
            name: "x".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![],
        });

        // Now invalid
        assert!(plan.verify_invariants().is_err());
    }
}
