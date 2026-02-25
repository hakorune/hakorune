//! Phase 171-fix: Condition Expression Environment
//!
//! This module provides the environment for lowering condition expressions to JoinIR.
//! It maps variable names to JoinIR-local ValueIds, ensuring proper separation between
//! HOST and JoinIR value spaces.
//!
//! ## Design Philosophy
//!
//! **Single Responsibility**: This module ONLY handles variable name → ValueId mapping
//! for condition expressions. It does NOT:
//! - Perform AST lowering (that's condition_lowerer.rs)
//! - Extract variables from AST (that's condition_var_extractor.rs)
//! - Manage HOST ↔ JoinIR bindings (that's inline_boundary.rs)

#[cfg(feature = "normalized_dev")]
use crate::mir::BindingId; // Phase 75: BindingId-based lookup
use crate::mir::ValueId;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

/// Environment for condition expression lowering
///
/// Maps variable names to JoinIR-local ValueIds. Used when lowering
/// condition AST nodes to JoinIR instructions.
///
/// # Phase 200-B Extension
///
/// Added `captured` field to track function-scoped captured variables
/// separately from loop parameters. Captured variables have ParamRole::Condition
/// and do NOT participate in header PHI or ExitLine.
///
/// # Example
///
/// ```ignore
/// let mut env = ConditionEnv::new();
/// env.insert("i".to_string(), ValueId(0));   // Loop parameter
/// env.insert("end".to_string(), ValueId(1)); // Condition-only var
///
/// // Phase 200-B: Add captured variable
/// env.captured.insert("digits".to_string(), ValueId(2));
///
/// // Later during lowering:
/// if let Some(value_id) = env.get("i") {
///     // Use value_id in JoinIR instruction
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ConditionEnv {
    /// Loop parameters and condition-only variables (legacy)
    name_to_join: BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism

    /// Phase 200-B: Captured function-scoped variables (ParamRole::Condition)
    ///
    /// These variables are:
    /// - Declared in function scope before the loop
    /// - Never reassigned (effectively immutable)
    /// - Used in loop condition or body
    /// - NOT included in header PHI or ExitLine (condition-only)
    pub captured: BTreeMap<String, ValueId>, // Phase 222.5-D: HashMap → BTreeMap for determinism

    /// Phase 75: BindingId → ValueId mapping (dev-only, pilot integration)
    ///
    /// This map provides direct BindingId-based lookup, supporting gradual
    /// migration from name-based to BindingId-based variable resolution.
    ///
    /// Populated by lowering code when BindingId information is available
    /// (e.g., from MirBuilder.binding_map).
    #[cfg(feature = "normalized_dev")]
    pub binding_id_map: BTreeMap<BindingId, ValueId>,
}

impl ConditionEnv {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            name_to_join: BTreeMap::new(), // Phase 222.5-D: HashMap → BTreeMap for determinism
            captured: BTreeMap::new(),     // Phase 222.5-D: HashMap → BTreeMap for determinism
            #[cfg(feature = "normalized_dev")]
            binding_id_map: BTreeMap::new(), // Phase 75: BindingId → ValueId mapping
        }
    }

    /// Insert a variable binding
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name (e.g., "i", "end")
    /// * `join_id` - JoinIR-local ValueId for this variable
    pub fn insert(&mut self, name: String, join_id: ValueId) {
        self.name_to_join.insert(name, join_id);
    }

    /// Look up a variable by name
    ///
    /// Phase 200-B: Searches both name_to_join (loop params) and captured fields.
    ///
    /// Returns `Some(ValueId)` if the variable exists in the environment,
    /// `None` otherwise.
    pub fn get(&self, name: &str) -> Option<ValueId> {
        self.name_to_join
            .get(name)
            .copied()
            .or_else(|| self.captured.get(name).copied())
    }

    /// Check if a variable exists in the environment
    ///
    /// Phase 200-B: Checks both name_to_join and captured fields.
    pub fn contains(&self, name: &str) -> bool {
        self.name_to_join.contains_key(name) || self.captured.contains_key(name)
    }

    /// Check if a variable is a captured (Condition role) variable
    ///
    /// Phase 200-B: New method to distinguish captured vars from loop params.
    pub fn is_captured(&self, name: &str) -> bool {
        self.captured.contains_key(name)
    }

    /// Get the number of variables in the environment
    ///
    /// Phase 200-B: Counts both name_to_join and captured fields.
    pub fn len(&self) -> usize {
        self.name_to_join.len() + self.captured.len()
    }

    /// Check if the environment is empty
    ///
    /// Phase 200-B: Checks both name_to_join and captured fields.
    pub fn is_empty(&self) -> bool {
        self.name_to_join.is_empty() && self.captured.is_empty()
    }

    /// Get an iterator over all (name, ValueId) pairs
    ///
    /// Phase 200-B: Note - this only iterates over name_to_join (loop params).
    /// For captured variables, access the `captured` field directly.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &ValueId)> {
        self.name_to_join.iter()
    }

    /// Get all variable names (sorted)
    ///
    /// Phase 200-B: Includes both name_to_join and captured variables.
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<_> = self
            .name_to_join
            .keys()
            .chain(self.captured.keys())
            .cloned()
            .collect();
        names.sort();
        names.dedup(); // Remove duplicates (shouldn't happen, but be safe)
        names
    }

    /// Phase 201-A: Get the maximum ValueId used in this environment
    ///
    /// Returns the highest ValueId.0 value from both name_to_join and captured,
    /// or None if the environment is empty.
    ///
    /// This is used by JoinIR lowering to determine the starting point for
    /// alloc_value() to avoid ValueId collisions.
    pub fn max_value_id(&self) -> Option<u32> {
        let name_max = self.name_to_join.values().map(|v| v.0).max();
        let captured_max = self.captured.values().map(|v| v.0).max();

        match (name_max, captured_max) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    /// Phase 78: Register a carrier binding (BindingId → ValueId)
    ///
    /// Used by Pattern2/4 lowering to register promoted carriers so they can
    /// be looked up via BindingId.
    ///
    /// # Arguments
    ///
    /// * `binding_id` - BindingId for the carrier (allocated by CarrierBindingAssigner)
    /// * `join_value_id` - JoinIR-local ValueId for this carrier (after header PHI)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // After allocating carrier join_id
    /// if let Some(binding_id) = carrier.binding_id {
    ///     condition_env.register_carrier_binding(binding_id, carrier.join_id.unwrap());
    /// }
    /// ```
    #[cfg(feature = "normalized_dev")]
    pub fn register_carrier_binding(&mut self, binding_id: BindingId, join_value_id: ValueId) {
        self.binding_id_map.insert(binding_id, join_value_id);
    }

    /// Phase 79-2: Register loop variable BindingId → ValueId mapping (dev-only)
    ///
    /// This method registers the loop variable (e.g., "i", "p") BindingId
    /// after header PHI allocation, enabling BindingId-based lookup during
    /// condition lowering.
    ///
    /// # Arguments
    ///
    /// * `binding_id` - BindingId for the loop variable
    /// * `value_id` - JoinIR-local ValueId for loop var (after header PHI)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // After allocating loop param in header PHI
    /// if let Some(loop_var_bid) = builder.binding_map.get(&loop_var_name).copied() {
    ///     condition_env.register_loop_var_binding(loop_var_bid, loop_param_value_id);
    /// }
    /// ```
    #[cfg(feature = "normalized_dev")]
    pub fn register_loop_var_binding(&mut self, binding_id: BindingId, value_id: ValueId) {
        self.binding_id_map.insert(binding_id, value_id);
    }

    /// Phase 79-2: Register condition-only variable BindingId → ValueId mapping (dev-only)
    ///
    /// This method registers condition-only variables (variables used only in
    /// loop/break conditions, not in loop body) for BindingId-based lookup.
    ///
    /// # Arguments
    ///
    /// * `binding_id` - BindingId for the condition-only variable
    /// * `value_id` - JoinIR-local ValueId for this variable
    ///
    /// # Example
    ///
    /// ```ignore
    /// // After allocating condition-only carriers
    /// for carrier in &carrier_info.carriers {
    ///     if carrier.role == CarrierRole::ConditionOnly {
    ///         if let Some(bid) = carrier.binding_id {
    ///             condition_env.register_condition_binding(bid, carrier.join_id);
    ///         }
    ///     }
    /// }
    /// ```
    #[cfg(feature = "normalized_dev")]
    pub fn register_condition_binding(&mut self, binding_id: BindingId, value_id: ValueId) {
        self.binding_id_map.insert(binding_id, value_id);
    }

    /// Phase 75: Resolve variable with BindingId priority (dev-only, pilot integration)
    ///
    /// Look up variable by BindingId first, falling back to name-based lookup.
    /// This supports gradual migration from name-based to BindingId-based
    /// variable resolution.
    ///
    /// # Lookup Strategy
    ///
    /// 1. If `binding_id` is Some, try binding_id_map lookup first
    /// 2. If BindingId lookup fails or is None, fall back to name-based lookup (get())
    ///
    /// # Arguments
    ///
    /// * `binding_id` - Optional BindingId for the variable
    /// * `name` - Variable name (used as fallback if BindingId lookup fails)
    ///
    /// # Returns
    ///
    /// `Some(ValueId)` if found via BindingId or name, `None` otherwise.
    ///
    /// # Dev Logging
    ///
    /// When NYASH_JOINIR_DEBUG=1 is set:
    /// - `[binding_pilot/hit]` - BindingId lookup succeeded
    /// - `[binding_pilot/fallback]` - BindingId lookup failed, fell back to name
    /// - `[binding_pilot/legacy]` - No BindingId provided, used name lookup
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut env = ConditionEnv::new();
    /// env.insert("x".to_string(), ValueId(100));
    /// env.binding_id_map.insert(BindingId(5), ValueId(100));
    ///
    /// // BindingId priority
    /// assert_eq!(env.resolve_var_with_binding(Some(BindingId(5)), "x"), Some(ValueId(100)));
    ///
    /// // BindingId miss → name fallback
    /// assert_eq!(env.resolve_var_with_binding(Some(BindingId(99)), "x"), Some(ValueId(100)));
    ///
    /// // Legacy (no BindingId)
    /// assert_eq!(env.resolve_var_with_binding(None, "x"), Some(ValueId(100)));
    /// ```
    #[cfg(feature = "normalized_dev")]
    pub fn resolve_var_with_binding(
        &self,
        binding_id: Option<BindingId>,
        name: &str,
    ) -> Option<ValueId> {
        use super::debug_output_box::DebugOutputBox;
        let debug = DebugOutputBox::new("binding_pilot");

        if let Some(bid) = binding_id {
            // Try BindingId lookup first
            if let Some(&value_id) = self.binding_id_map.get(&bid) {
                debug.log(
                    "hit",
                    &format!(
                        "BindingId({}) -> ValueId({}) for '{}'",
                        bid.0, value_id.0, name
                    ),
                );
                return Some(value_id);
            } else {
                // BindingId miss, fall back to name
                let result = self.get(name);
                debug.log(
                    "fallback",
                    &format!("BindingId({}) miss, name '{}' -> {:?}", bid.0, name, result),
                );
                return result;
            }
        } else {
            // Legacy: no BindingId, use name lookup
            let result = self.get(name);
            debug.log(
                "legacy",
                &format!("No BindingId, name '{}' -> {:?}", name, result),
            );
            return result;
        }
    }
}

/// Binding between HOST and JoinIR ValueIds for condition variables
///
/// This structure explicitly connects a variable name to both its HOST ValueId
/// (from the host function's variable_map) and its JoinIR ValueId (allocated
/// locally within the JoinIR fragment).
///
/// # Example
///
/// For condition variable "start" in `loop(start < end)`:
///
/// ```ignore
/// ConditionBinding {
///     name: "start".to_string(),
///     host_value: ValueId(33),  // HOST function's ValueId for "start"
///     join_value: ValueId(1),   // JoinIR-local ValueId for "start"
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ConditionBinding {
    /// Variable name (e.g., "start", "end")
    pub name: String,

    /// HOST function's ValueId for this variable
    ///
    /// This comes from `builder.variable_map` in the host function.
    pub host_value: ValueId,

    /// JoinIR-local ValueId for this variable
    ///
    /// This is allocated within the JoinIR fragment and must be remapped
    /// when merging into the host function.
    pub join_value: ValueId,
}

impl ConditionBinding {
    /// Create a new condition binding
    pub fn new(name: String, host_value: ValueId, join_value: ValueId) -> Self {
        Self {
            name,
            host_value,
            join_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_env_basic() {
        let mut env = ConditionEnv::new();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);

        env.insert("i".to_string(), ValueId(0));
        assert!(!env.is_empty());
        assert_eq!(env.len(), 1);
        assert!(env.contains("i"));
        assert_eq!(env.get("i"), Some(ValueId(0)));
    }

    #[test]
    fn test_condition_env_multiple_vars() {
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(0));
        env.insert("start".to_string(), ValueId(1));
        env.insert("end".to_string(), ValueId(2));

        assert_eq!(env.len(), 3);
        assert_eq!(env.get("i"), Some(ValueId(0)));
        assert_eq!(env.get("start"), Some(ValueId(1)));
        assert_eq!(env.get("end"), Some(ValueId(2)));
        assert_eq!(env.get("nonexistent"), None);
    }

    #[test]
    fn test_condition_binding() {
        let binding = ConditionBinding::new(
            "start".to_string(),
            ValueId(33), // HOST
            ValueId(1),  // JoinIR
        );

        assert_eq!(binding.name, "start");
        assert_eq!(binding.host_value, ValueId(33));
        assert_eq!(binding.join_value, ValueId(1));
    }

    /// Phase 75: Test BindingId priority lookup (BindingId hit)
    #[test]
    #[cfg(feature = "normalized_dev")]
    fn test_condition_env_binding_id_priority() {
        use crate::mir::BindingId;

        let mut env = ConditionEnv::new();
        env.insert("x".to_string(), ValueId(100));
        env.binding_id_map.insert(BindingId(5), ValueId(100));

        // BindingId should be used if provided
        let result = env.resolve_var_with_binding(Some(BindingId(5)), "x");
        assert_eq!(result, Some(ValueId(100)));
    }

    /// Phase 75: Test BindingId fallback (BindingId miss → name lookup)
    #[test]
    #[cfg(feature = "normalized_dev")]
    fn test_condition_env_binding_id_fallback() {
        use crate::mir::BindingId;

        let mut env = ConditionEnv::new();
        env.insert("x".to_string(), ValueId(100));
        // Note: BindingId(99) is NOT in binding_id_map

        // BindingId miss → should fall back to name lookup
        let result = env.resolve_var_with_binding(Some(BindingId(99)), "x");
        assert_eq!(result, Some(ValueId(100)));
    }

    /// Phase 75: Test legacy name-based lookup (no BindingId)
    #[test]
    #[cfg(feature = "normalized_dev")]
    fn test_condition_env_binding_id_none() {
        let mut env = ConditionEnv::new();
        env.insert("x".to_string(), ValueId(100));

        // No BindingId → should use name lookup
        let result = env.resolve_var_with_binding(None, "x");
        assert_eq!(result, Some(ValueId(100)));
    }
}
