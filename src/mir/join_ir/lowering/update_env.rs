//! Phase 184: Update Expression Environment
//! Phase 247-EX: Extended with promoted variable resolution for dual-value carriers
//!
//! This module provides a unified variable resolution layer for carrier update expressions.
//! It combines ConditionEnv (condition variables) and LoopBodyLocalEnv (body-local variables)
//! with clear priority order.
//!
//! ## Phase 247-EX: Promoted Variable Resolution
//!
//! For promoted LoopBodyLocal variables (e.g., `digit_pos` → `is_digit_pos` + `digit_value`):
//! - When resolving `digit_pos` in update expressions (e.g., `result = result * 10 + digit_pos`)
//! - Try `<var>_value` first (e.g., `digit_value`)
//! - Then fall back to `is_<var>` (boolean carrier, less common in updates)
//! - Finally fall back to standard resolution
//!
//! ## Design Philosophy
//!
//! **Single Responsibility**: This module ONLY handles variable resolution priority logic.
//! It does NOT:
//! - Store variables (that's ConditionEnv and LoopBodyLocalEnv)
//! - Lower AST to JoinIR (that's JoinIrBuilder)
//! - Emit update instructions (that's CarrierUpdateEmitter)
//!
//! ## Box-First Design
//!
//! Following 箱理論 (Box Theory) principles:
//! - **Composition**: Combines two environments without owning them
//! - **Clear priority**: Condition variables take precedence
//! - **Lightweight**: No allocation, just references

use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

/// Unified environment for carrier update expression variable resolution
///
/// This structure provides a composition layer that resolves variables
/// with the following priority order:
///
/// 1. **Condition variables** (ConditionEnv) - Highest priority
///    - Loop parameters (e.g., `i`, `end`, `p`)
///    - Variables used in condition expressions
///
/// 2. **Body-local variables** (LoopBodyLocalEnv) - Fallback priority
///    - Variables declared in loop body (e.g., `local temp`)
///    - Only accessible if not shadowed by condition variables
///
/// # Example
///
/// ```nyash
/// loop(i < 5) {           // i in ConditionEnv
///     local temp = i * 2  // temp in LoopBodyLocalEnv
///     sum = sum + temp    // Resolves: sum (cond), temp (body)
///     i = i + 1
/// }
/// ```
///
/// ```ignore
/// let condition_env = /* ... i, sum ... */;
/// let body_local_env = /* ... temp ... */;
/// let update_env = UpdateEnv::new(&condition_env, &body_local_env, &[]);
///
/// // Resolve "sum" → ConditionEnv (priority 1)
/// assert_eq!(update_env.resolve("sum"), Some(ValueId(X)));
///
/// // Resolve "temp" → LoopBodyLocalEnv (priority 2)
/// assert_eq!(update_env.resolve("temp"), Some(ValueId(Y)));
///
/// // Resolve "unknown" → None
/// assert_eq!(update_env.resolve("unknown"), None);
/// ```
///
/// # Phase 247-EX Example
///
/// ```ignore
/// // digit_pos promoted → is_digit_pos (bool) + digit_value (i64)
/// let promoted = vec!["digit_pos".to_string()];
/// let update_env = UpdateEnv::new(&condition_env, &body_local_env, &promoted);
///
/// // Resolve "digit_pos" in NumberAccumulation → digit_value (integer carrier)
/// assert_eq!(update_env.resolve("digit_pos"), Some(ValueId(X))); // digit_value
/// ```
#[derive(Debug)]
pub struct UpdateEnv<'a> {
    /// Condition variable environment (priority 1)
    condition_env: &'a ConditionEnv,

    /// Body-local variable environment (priority 2)
    body_local_env: &'a LoopBodyLocalEnv,

    /// Phase 247-EX: List of promoted LoopBodyLocal variable names
    /// For these variables, resolve to <var>_value carrier instead of is_<var>
    promoted_loopbodylocals: &'a [String],
}

impl<'a> UpdateEnv<'a> {
    /// Create a new UpdateEnv with priority-ordered resolution
    ///
    /// # Arguments
    ///
    /// * `condition_env` - Condition variable environment (highest priority)
    /// * `body_local_env` - Body-local variable environment (fallback)
    /// * `promoted_loopbodylocals` - Phase 247-EX: List of promoted variable names
    pub fn new(
        condition_env: &'a ConditionEnv,
        body_local_env: &'a LoopBodyLocalEnv,
        promoted_loopbodylocals: &'a [String],
    ) -> Self {
        Self {
            condition_env,
            body_local_env,
            promoted_loopbodylocals,
        }
    }

    /// Resolve a variable name to JoinIR ValueId
    ///
    /// Resolution order (Phase 247-EX extended):
    /// 1. If name is in promoted_loopbodylocals:
    ///    a. Try condition_env.get("<name>_value")  // Integer carrier for accumulation
    ///    b. If not found, try condition_env.get("is_<name>")  // Boolean carrier (rare in updates)
    /// 2. Try condition_env.get(name)
    /// 3. If not found, try body_local_env.get(name)
    /// 4. If still not found, return None
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name to resolve
    ///
    /// # Returns
    ///
    /// * `Some(ValueId)` - Variable found in one of the environments
    /// * `None` - Variable not found in either environment
    ///
    /// # Phase 247-EX Example
    ///
    /// ```ignore
    /// // digit_pos promoted → is_digit_pos + digit_value
    /// // When resolving "digit_pos" in update expr:
    /// env.resolve("digit_pos") → env.get("digit_value") → Some(ValueId(X))
    /// ```
    pub fn resolve(&self, name: &str) -> Option<ValueId> {
        // Phase 247-EX: Check if this is a promoted variable (digit_pos) or its derived carrier name (digit_value)
        let promoted_key = if self.promoted_loopbodylocals.iter().any(|v| v == name) {
            Some(name.to_string())
        } else if let Some(base) = name.strip_suffix("_value") {
            let candidate = format!("{}_pos", base);
            if self.promoted_loopbodylocals.iter().any(|v| v == &candidate) {
                Some(candidate)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(promoted_name) = promoted_key {
            // Prefer the freshly computed body-local value if it exists (digit_pos)
            if let Some(val) = self.body_local_env.get(&promoted_name) {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(&format!(
                        "[update_env/phase247ex] Resolved promoted '{}' from body-local env: {:?}",
                        name, val
                    ));
                }
                return Some(val);
            }

            // Phase 247-EX: Naming convention - "digit_pos" → "digit_value" (not "digit_pos_value")
            // Extract base name: "digit_pos" → "digit", "pos" → "pos"
            let base_name = if promoted_name.ends_with("_pos") {
                &promoted_name[..promoted_name.len() - 4] // Remove "_pos" suffix
            } else {
                promoted_name.as_str()
            };

            // Priority 1a: Try <base>_value (integer carrier for NumberAccumulation)
            let int_carrier_name = format!("{}_value", base_name);
            if let Some(value_id) = self.condition_env.get(&int_carrier_name) {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(&format!(
                        "[update_env/phase247ex] Resolved promoted '{}' → '{}' (integer carrier): {:?}",
                        name, int_carrier_name, value_id
                    ));
                }
                return Some(value_id);
            }

            // Priority 1b: Try is_<name> (boolean carrier, less common in updates)
            let bool_carrier_name = format!("is_{}", name);
            if let Some(value_id) = self.condition_env.get(&bool_carrier_name) {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(&format!(
                        "[update_env/phase247ex] Resolved promoted '{}' → '{}' (boolean carrier): {:?}",
                        name, bool_carrier_name, value_id
                    ));
                }
                return Some(value_id);
            }

            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(&format!(
                    "[update_env/phase247ex] WARNING: Promoted variable '{}' not found as carrier ({} or {})",
                    name, int_carrier_name, bool_carrier_name
                ));
            }
        }

        // Standard resolution (Phase 184)
        self.condition_env
            .get(name)
            .or_else(|| self.body_local_env.get(name))
    }

    /// Check if a variable exists in either environment
    pub fn contains(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }

    /// Get reference to condition environment (for debugging/diagnostics)
    pub fn condition_env(&self) -> &ConditionEnv {
        self.condition_env
    }

    /// Get reference to body-local environment (for debugging/diagnostics)
    pub fn body_local_env(&self) -> &LoopBodyLocalEnv {
        self.body_local_env
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: Create a test ConditionEnv
    fn test_condition_env() -> ConditionEnv {
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(10));
        env.insert("sum".to_string(), ValueId(20));
        env.insert("end".to_string(), ValueId(30));
        env
    }

    // Helper: Create a test LoopBodyLocalEnv
    fn test_body_local_env() -> LoopBodyLocalEnv {
        let mut env = LoopBodyLocalEnv::new();
        env.insert("temp".to_string(), ValueId(50));
        env.insert("digit".to_string(), ValueId(60));
        env
    }

    #[test]
    fn test_resolve_condition_priority() {
        // Condition variables should be found first
        let cond_env = test_condition_env();
        let body_env = LoopBodyLocalEnv::new(); // Empty
        let promoted: Vec<String> = vec![]; // Phase 247-EX: No promoted variables
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        assert_eq!(update_env.resolve("i"), Some(ValueId(10)));
        assert_eq!(update_env.resolve("sum"), Some(ValueId(20)));
        assert_eq!(update_env.resolve("end"), Some(ValueId(30)));
    }

    #[test]
    fn test_resolve_body_local_fallback() {
        // Body-local variables should be found when not in condition env
        let cond_env = ConditionEnv::new(); // Empty
        let body_env = test_body_local_env();
        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        assert_eq!(update_env.resolve("temp"), Some(ValueId(50)));
        assert_eq!(update_env.resolve("digit"), Some(ValueId(60)));
    }

    #[test]
    fn test_resolve_priority_order() {
        // Condition env takes priority over body-local env
        let mut cond_env = ConditionEnv::new();
        cond_env.insert("x".to_string(), ValueId(100)); // Condition: x=100

        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("x".to_string(), ValueId(200)); // Body-local: x=200

        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        // Should resolve to condition env value (100), not body-local (200)
        assert_eq!(update_env.resolve("x"), Some(ValueId(100)));
    }

    #[test]
    fn test_resolve_not_found() {
        // Variable not in either environment → None
        let cond_env = test_condition_env();
        let body_env = test_body_local_env();
        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        assert_eq!(update_env.resolve("unknown"), None);
        assert_eq!(update_env.resolve("nonexistent"), None);
    }

    #[test]
    fn test_resolve_combined_lookup() {
        // Mixed lookup: some in condition, some in body-local
        let cond_env = test_condition_env();
        let body_env = test_body_local_env();
        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        // Condition variables
        assert_eq!(update_env.resolve("i"), Some(ValueId(10)));
        assert_eq!(update_env.resolve("sum"), Some(ValueId(20)));

        // Body-local variables
        assert_eq!(update_env.resolve("temp"), Some(ValueId(50)));
        assert_eq!(update_env.resolve("digit"), Some(ValueId(60)));

        // Not found
        assert_eq!(update_env.resolve("unknown"), None);
    }

    #[test]
    fn test_contains() {
        let cond_env = test_condition_env();
        let body_env = test_body_local_env();
        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        assert!(update_env.contains("i"));
        assert!(update_env.contains("temp"));
        assert!(!update_env.contains("unknown"));
    }

    #[test]
    fn test_empty_environments() {
        // Both environments empty
        let cond_env = ConditionEnv::new();
        let body_env = LoopBodyLocalEnv::new();
        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        assert_eq!(update_env.resolve("anything"), None);
        assert!(!update_env.contains("anything"));
    }

    #[test]
    fn test_accessor_methods() {
        // Test diagnostic accessor methods
        let cond_env = test_condition_env();
        let body_env = test_body_local_env();
        let promoted: Vec<String> = vec![];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        // Should return references to underlying environments
        assert_eq!(update_env.condition_env().len(), 3);
        assert_eq!(update_env.body_local_env().len(), 2);
    }

    // Phase 247-EX: Test promoted variable resolution (dual-value carriers)
    #[test]
    fn test_promoted_variable_resolution_digit_pos() {
        // Test case: digit_pos promoted → is_digit_pos (bool) + digit_value (i64)
        // Naming: "digit_pos" → "is_digit_pos" + "digit_value" (base_name="_pos" removed)
        let mut cond_env = ConditionEnv::new();

        // Register both carriers in ConditionEnv
        cond_env.insert("is_digit_pos".to_string(), ValueId(100)); // Boolean carrier
        cond_env.insert("digit_value".to_string(), ValueId(200)); // Integer carrier (digit_pos → digit)

        let body_env = LoopBodyLocalEnv::new();
        let promoted: Vec<String> = vec!["digit_pos".to_string()];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        // When resolving "digit_pos" in update expr → should get digit_value (integer carrier)
        assert_eq!(update_env.resolve("digit_pos"), Some(ValueId(200)));

        // Direct carrier access still works
        assert_eq!(update_env.resolve("is_digit_pos"), Some(ValueId(100)));
        assert_eq!(update_env.resolve("digit_value"), Some(ValueId(200)));
    }

    #[test]
    fn test_promoted_variable_resolution_fallback_to_bool() {
        // Test case: Only boolean carrier exists (integer carrier missing)
        let mut cond_env = ConditionEnv::new();
        cond_env.insert("is_pos".to_string(), ValueId(150)); // Only boolean carrier

        let body_env = LoopBodyLocalEnv::new();
        let promoted: Vec<String> = vec!["pos".to_string()];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        // Should fall back to is_pos (boolean carrier)
        assert_eq!(update_env.resolve("pos"), Some(ValueId(150)));
    }

    #[test]
    fn test_promoted_variable_not_a_carrier() {
        // Test case: Variable in promoted list but no carrier exists
        let cond_env = ConditionEnv::new(); // Empty
        let body_env = LoopBodyLocalEnv::new();
        let promoted: Vec<String> = vec!["missing_var".to_string()];
        let update_env = UpdateEnv::new(&cond_env, &body_env, &promoted);

        // Should return None (with warning logged)
        assert_eq!(update_env.resolve("missing_var"), None);
    }
}
