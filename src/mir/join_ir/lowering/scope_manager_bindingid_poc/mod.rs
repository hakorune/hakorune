//! Phase 73 PoC: BindingId-Based Scope Management (Dev-Only)
//!
//! This module demonstrates the BindingId-based scope design proposed in
//! `docs/development/current/main/phase73-scope-manager-design.md`.
//!
//! **Status**: Proof-of-Concept ONLY
//! - NOT used in production code
//! - Gated by `#[cfg(feature = "normalized_dev")]`
//! - For design validation and testing only

#![cfg(feature = "normalized_dev")]

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 73 PoC: BindingId type
///
/// Unique identifier for a variable binding in lexical scope.
/// Each `local x` declaration creates a new BindingId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindingId(pub u32);

/// Phase 73 PoC: ConditionEnv with BindingId support
///
/// Demonstrates parallel name-based and BindingId-based lookup.
#[derive(Debug, Clone, Default)]
pub struct ConditionEnvV2 {
    // Legacy: name-based lookup (Phase 73 - keep for backward compatibility)
    name_to_join: BTreeMap<String, ValueId>,

    // Phase 73+: NEW - BindingId-based tracking
    binding_to_join: BTreeMap<BindingId, ValueId>, // BindingId → JoinIR ValueId
    name_to_binding: BTreeMap<String, BindingId>,  // Name → current BindingId (shadowing)
}

impl ConditionEnvV2 {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a variable binding (legacy name-based)
    pub fn insert_by_name(&mut self, name: String, join_id: ValueId) {
        self.name_to_join.insert(name, join_id);
    }

    /// Insert a variable binding (BindingId-based - Phase 73+)
    pub fn insert_by_binding(&mut self, name: String, binding: BindingId, join_id: ValueId) {
        // Update BindingId → ValueId mapping
        self.binding_to_join.insert(binding, join_id);

        // Update name → current BindingId (for shadowing)
        self.name_to_binding.insert(name, binding);
    }

    /// Look up a variable by name (legacy)
    pub fn get_by_name(&self, name: &str) -> Option<ValueId> {
        self.name_to_join.get(name).copied()
    }

    /// Look up a variable by BindingId (Phase 73+)
    pub fn get_by_binding(&self, binding: BindingId) -> Option<ValueId> {
        self.binding_to_join.get(&binding).copied()
    }

    /// Get the current BindingId for a name (for shadowing resolution)
    pub fn get_binding_for_name(&self, name: &str) -> Option<BindingId> {
        self.name_to_binding.get(name).copied()
    }

    /// Unified lookup: Try BindingId first, fall back to name
    ///
    /// This demonstrates the transition strategy where new code uses BindingId
    /// and legacy code falls back to name-based lookup.
    pub fn lookup(&self, name: &str) -> Option<ValueId> {
        // 1. Try BindingId lookup (Phase 73+)
        if let Some(binding) = self.get_binding_for_name(name) {
            if let Some(value) = self.get_by_binding(binding) {
                return Some(value);
            }
        }

        // 2. Fallback to legacy name-based lookup
        self.get_by_name(name)
    }
}

/// Phase 73 PoC: CarrierVar with BindingId support
#[derive(Debug, Clone)]
pub struct CarrierVarV2 {
    pub name: String,
    pub host_id: ValueId,
    pub join_id: Option<ValueId>,

    // Phase 73+: NEW - BindingId tracking
    pub host_binding: Option<BindingId>, // HOST function's BindingId
}

/// Phase 73 PoC: CarrierInfo with BindingId-based promotion
#[derive(Debug, Clone)]
pub struct CarrierInfoV2 {
    pub loop_var_name: String,
    pub loop_var_id: ValueId,
    pub carriers: Vec<CarrierVarV2>,

    // Phase 73+: Replace string list with BindingId map
    pub promoted_bindings: BTreeMap<BindingId, BindingId>, // Original → Promoted
}

impl CarrierInfoV2 {
    /// Resolve a promoted LoopBodyLocal binding
    ///
    /// Example: BindingId(5) for "digit_pos" → BindingId(10) for "is_digit_pos"
    pub fn resolve_promoted_binding(&self, original: BindingId) -> Option<BindingId> {
        self.promoted_bindings.get(&original).copied()
    }

    /// Add a promoted binding mapping
    pub fn add_promoted_binding(&mut self, original: BindingId, promoted: BindingId) {
        self.promoted_bindings.insert(original, promoted);
    }
}

/// Phase 73 PoC: ScopeManager with BindingId support
pub trait ScopeManagerV2 {
    /// Look up variable by BindingId (Phase 73+)
    fn lookup_binding(&self, binding: BindingId) -> Option<ValueId>;

    /// Look up variable by name (legacy fallback)
    fn lookup_name(&self, name: &str) -> Option<ValueId>;
}

/// Phase 73 PoC: Pattern2 ScopeManager with BindingId
pub struct Pattern2ScopeManagerV2<'a> {
    pub condition_env: &'a ConditionEnvV2,
    pub carrier_info: &'a CarrierInfoV2,

    // Phase 73+: BindingId context from HOST
    pub host_bindings: Option<&'a BTreeMap<String, BindingId>>,
}

impl<'a> ScopeManagerV2 for Pattern2ScopeManagerV2<'a> {
    fn lookup_binding(&self, binding: BindingId) -> Option<ValueId> {
        // 1. Check condition_env.binding_to_join (direct lookup)
        if let Some(id) = self.condition_env.get_by_binding(binding) {
            return Some(id);
        }

        // 2. Check promoted bindings (LoopBodyLocal → Carrier)
        if let Some(promoted) = self.carrier_info.resolve_promoted_binding(binding) {
            return self.condition_env.get_by_binding(promoted);
        }

        None
    }

    fn lookup_name(&self, name: &str) -> Option<ValueId> {
        // Try BindingId-based lookup first (if host_bindings available)
        if let Some(host_bindings) = self.host_bindings {
            if let Some(binding) = host_bindings.get(name) {
                if let Some(value) = self.lookup_binding(*binding) {
                    return Some(value);
                }
            }
        }

        // Fallback to legacy name-based lookup
        self.condition_env.get_by_name(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_env_v2_basic() {
        let mut env = ConditionEnvV2::new();

        // Legacy name-based insertion
        env.insert_by_name("i".to_string(), ValueId(100));
        assert_eq!(env.get_by_name("i"), Some(ValueId(100)));

        // BindingId-based insertion
        let binding = BindingId(0);
        env.insert_by_binding("sum".to_string(), binding, ValueId(101));
        assert_eq!(env.get_by_binding(binding), Some(ValueId(101)));
        assert_eq!(env.get_binding_for_name("sum"), Some(binding));
    }

    #[test]
    fn test_shadowing_simulation() {
        let mut env = ConditionEnvV2::new();

        // Outer scope: local x = 1 (BindingId(0) → ValueId(10))
        let outer_binding = BindingId(0);
        env.insert_by_binding("x".to_string(), outer_binding, ValueId(10));

        assert_eq!(env.lookup("x"), Some(ValueId(10)));
        assert_eq!(env.get_binding_for_name("x"), Some(outer_binding));

        // Inner scope: local x = 2 (BindingId(1) → ValueId(20), shadows BindingId(0))
        let inner_binding = BindingId(1);
        env.insert_by_binding("x".to_string(), inner_binding, ValueId(20));

        // Name lookup should resolve to inner binding
        assert_eq!(env.lookup("x"), Some(ValueId(20)));
        assert_eq!(env.get_binding_for_name("x"), Some(inner_binding));

        // But we can still access outer binding directly
        assert_eq!(env.get_by_binding(outer_binding), Some(ValueId(10)));
    }

    #[test]
    fn test_promoted_binding_resolution() {
        let mut carrier_info = CarrierInfoV2 {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(5),
            carriers: vec![],
            promoted_bindings: BTreeMap::new(),
        };

        // Promote BindingId(5) "digit_pos" → BindingId(10) "is_digit_pos"
        let original = BindingId(5);
        let promoted = BindingId(10);
        carrier_info.add_promoted_binding(original, promoted);

        assert_eq!(
            carrier_info.resolve_promoted_binding(original),
            Some(promoted)
        );
        assert_eq!(carrier_info.resolve_promoted_binding(BindingId(99)), None);
    }

    #[test]
    fn test_scope_manager_v2_binding_lookup() {
        let mut env = ConditionEnvV2::new();
        let binding_i = BindingId(0);
        let binding_sum = BindingId(1);

        env.insert_by_binding("i".to_string(), binding_i, ValueId(100));
        env.insert_by_binding("sum".to_string(), binding_sum, ValueId(101));

        let carrier_info = CarrierInfoV2 {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(5),
            carriers: vec![],
            promoted_bindings: BTreeMap::new(),
        };

        let mut host_bindings = BTreeMap::new();
        host_bindings.insert("i".to_string(), binding_i);
        host_bindings.insert("sum".to_string(), binding_sum);

        let scope = Pattern2ScopeManagerV2 {
            condition_env: &env,
            carrier_info: &carrier_info,
            host_bindings: Some(&host_bindings),
        };

        // BindingId-based lookup
        assert_eq!(scope.lookup_binding(binding_i), Some(ValueId(100)));
        assert_eq!(scope.lookup_binding(binding_sum), Some(ValueId(101)));

        // Name-based lookup (uses BindingId internally)
        assert_eq!(scope.lookup_name("i"), Some(ValueId(100)));
        assert_eq!(scope.lookup_name("sum"), Some(ValueId(101)));
    }

    #[test]
    fn test_scope_manager_v2_promoted_lookup() {
        let mut env = ConditionEnvV2::new();

        // Promoted binding: is_digit_pos (BindingId(10) → ValueId(102))
        let promoted_binding = BindingId(10);
        env.insert_by_binding("is_digit_pos".to_string(), promoted_binding, ValueId(102));

        let mut carrier_info = CarrierInfoV2 {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(5),
            carriers: vec![],
            promoted_bindings: BTreeMap::new(),
        };

        // Original binding: digit_pos (BindingId(5))
        let original_binding = BindingId(5);
        carrier_info.add_promoted_binding(original_binding, promoted_binding);

        let scope = Pattern2ScopeManagerV2 {
            condition_env: &env,
            carrier_info: &carrier_info,
            host_bindings: None,
        };

        // Lookup original BindingId should resolve to promoted ValueId
        assert_eq!(scope.lookup_binding(original_binding), Some(ValueId(102)));

        // Direct promoted lookup also works
        assert_eq!(scope.lookup_binding(promoted_binding), Some(ValueId(102)));
    }

    #[test]
    fn test_unified_lookup_fallback() {
        let mut env = ConditionEnvV2::new();

        // Legacy name-based entry (no BindingId)
        env.insert_by_name("legacy_var".to_string(), ValueId(999));

        // BindingId-based entry
        let binding = BindingId(0);
        env.insert_by_binding("new_var".to_string(), binding, ValueId(888));

        // Unified lookup should find both
        assert_eq!(env.lookup("legacy_var"), Some(ValueId(999))); // Fallback to name
        assert_eq!(env.lookup("new_var"), Some(ValueId(888))); // BindingId first
    }
}
