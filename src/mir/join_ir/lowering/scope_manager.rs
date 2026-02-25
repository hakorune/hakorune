//! Phase 231: Scope Manager for Unified Variable Lookup
//!
//! This module provides a unified interface for variable lookup across different
//! scopes in JoinIR lowering. It abstracts over the complexity of multiple
//! environments (ConditionEnv, LoopBodyLocalEnv, CapturedEnv, CarrierInfo).
//!
//! ## Design Philosophy
//!
//! **Box-First**: ScopeManager is a trait-based "box" that encapsulates variable
//! lookup logic, making it easy to swap implementations or test in isolation.
//!
//! **Single Responsibility**: Variable resolution only. Does NOT:
//! - Lower AST to JoinIR (that's ExprLowerer)
//! - Manage ValueId allocation (that's JoinValueSpace)
//! - Handle HOST ↔ JoinIR bindings (that's InlineBoundary)
//!
//! ## Pattern2 Pilot Implementation
//!
//! Phase 231 starts with Pattern2-specific implementation to validate the design.
//! Future phases will generalize to Pattern1, Pattern3, etc.

use super::carrier_info::CarrierInfo;
use super::condition_env::ConditionEnv;
use super::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::loop_pattern_detection::function_scope_capture::CapturedEnv;
#[cfg(feature = "normalized_dev")]
use crate::mir::BindingId; // Phase 75: BindingId-based lookup pilot
use crate::mir::ValueId;
#[cfg(feature = "normalized_dev")]
use crate::runtime::get_global_ring0;

/// Phase 231: Scope kind for variables
///
/// Helps distinguish where a variable comes from, which affects how it's
/// treated during lowering (e.g., PHI generation, exit handling).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarScopeKind {
    /// Loop control variable (i, p)
    LoopVar,
    /// Carrier variable (sum, count, is_digit_pos)
    Carrier,
    /// Loop body-local variable (ch, digit_pos before promotion)
    LoopBodyLocal,
    /// Captured from outer function scope (digits, s, len)
    Captured,
}

/// Phase 231: Scope manager trait for unified variable lookup
///
/// This trait provides a unified interface for looking up variables across
/// multiple environments. Implementations can aggregate different environment
/// types (ConditionEnv, LoopBodyLocalEnv, etc.) and provide consistent lookup.
///
/// # Example
///
/// ```ignore
/// let scope: &dyn ScopeManager = &Pattern2ScopeManager { ... };
/// if let Some(value_id) = scope.lookup("sum") {
///     // Use value_id in expression lowering
/// }
/// ```
pub trait ScopeManager {
    /// Look up variable by name, return ValueId if found
    ///
    /// This method searches across all available scopes and returns the first
    /// match. The search order is implementation-defined but should be
    /// documented in the implementing struct.
    fn lookup(&self, name: &str) -> Option<ValueId>;

    /// Get the scope kind of a variable
    ///
    /// This helps the caller understand where the variable comes from, which
    /// can affect code generation (e.g., PHI node generation, exit handling).
    fn scope_of(&self, name: &str) -> Option<VarScopeKind>;

    /// Phase 75: BindingId-based lookup (dev-only, pilot integration)
    ///
    /// Look up variable by BindingId first, falling back to name-based lookup.
    /// This supports gradual migration from name-based to BindingId-based
    /// variable resolution.
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
    /// # Example
    ///
    /// ```ignore
    /// // BindingId available - priority lookup
    /// let value_id = scope.lookup_with_binding(Some(BindingId(5)), "x");
    ///
    /// // BindingId not available - legacy name-based fallback
    /// let value_id = scope.lookup_with_binding(None, "x");
    /// ```
    #[cfg(feature = "normalized_dev")]
    fn lookup_with_binding(&self, binding_id: Option<BindingId>, name: &str) -> Option<ValueId> {
        // Default implementation: BindingId not supported, use name lookup
        let _ = binding_id; // Suppress unused warning
        self.lookup(name)
    }
}

/// Phase 231: Pattern2-specific scope manager (pilot implementation)
///
/// This implementation aggregates all the environments used in Pattern2 loop
/// lowering and provides unified variable lookup.
///
/// ## Lookup Order
///
/// 1. ConditionEnv (includes loop var, carriers, condition-only vars)
/// 2. LoopBodyLocalEnv (body-local variables before promotion)
/// 3. CapturedEnv (function-scoped captured variables)
/// 4. Promoted LoopBodyLocal → Carrier (using naming convention)
///
/// ## Naming Convention for Promoted Variables
///
/// - DigitPos pattern: `"digit_pos"` → `"is_digit_pos"`
/// - Trim pattern: `"ch"` → `"is_ch_match"`
///
/// # Example
///
/// ```ignore
/// let scope = Pattern2ScopeManager {
///     condition_env: &env,
///     loop_body_local_env: Some(&body_local_env),
///     captured_env: Some(&captured_env),
///     carrier_info: &carrier_info,
/// };
///
/// // Lookup loop variable
/// assert_eq!(scope.lookup("i"), Some(ValueId(100)));
///
/// // Lookup carrier
/// assert_eq!(scope.lookup("sum"), Some(ValueId(101)));
///
/// // Lookup promoted variable (uses naming convention)
/// assert_eq!(scope.lookup("digit_pos"), Some(ValueId(102))); // Resolves to "is_digit_pos"
/// ```
pub struct Pattern2ScopeManager<'a> {
    /// Condition environment (loop var + carriers + condition-only vars)
    pub condition_env: &'a ConditionEnv,

    /// Loop body-local environment (optional, may be empty)
    pub loop_body_local_env: Option<&'a LoopBodyLocalEnv>,

    /// Captured environment (optional, may be empty)
    pub captured_env: Option<&'a CapturedEnv>,

    /// Carrier information (includes promoted_loopbodylocals list)
    pub carrier_info: &'a CarrierInfo,
}

impl<'a> ScopeManager for Pattern2ScopeManager<'a> {
    fn lookup(&self, name: &str) -> Option<ValueId> {
        // 1. ConditionEnv (highest priority: loop var, carriers, condition-only)
        if let Some(id) = self.condition_env.get(name) {
            return Some(id);
        }

        // 2. LoopBodyLocalEnv (body-local variables)
        if let Some(env) = self.loop_body_local_env {
            if let Some(id) = env.get(name) {
                return Some(id);
            }
        }

        // 3. CapturedEnv (function-scoped captured variables)
        if let Some(env) = self.captured_env {
            for var in &env.vars {
                if var.name == name {
                    // Captured variables are already in condition_env, so this
                    // should have been caught in step 1. But check here for safety.
                    return self.condition_env.get(name);
                }
            }
        }

        // 4. Promoted LoopBodyLocal → Carrier lookup（命名規約は CarrierInfo 側に集約）
        // Phase 77: promoted_bindings は導入済みだが、ここ（ScopeManager::lookup）は依然として
        // “name-only” 入力なので、legacy の name-based promoted 解決を残す。
        #[allow(deprecated)]
        {
            self.carrier_info.resolve_promoted_join_id(name)
        }
    }

    fn scope_of(&self, name: &str) -> Option<VarScopeKind> {
        // Check loop variable first
        if name == self.carrier_info.loop_var_name {
            return Some(VarScopeKind::LoopVar);
        }

        // Check carriers
        if self.carrier_info.carriers.iter().any(|c| c.name == name) {
            return Some(VarScopeKind::Carrier);
        }

        // Check body-local
        if let Some(env) = self.loop_body_local_env {
            if env.contains(name) {
                return Some(VarScopeKind::LoopBodyLocal);
            }
        }

        // Check captured
        if let Some(env) = self.captured_env {
            if env.vars.iter().any(|v| v.name == name) {
                return Some(VarScopeKind::Captured);
            }
        }

        None
    }

    /// Phase 76: BindingId-based lookup with promoted binding support (dev-only)
    ///
    /// Extends Phase 75's BindingId priority lookup to check promoted_bindings map.
    /// This eliminates name-based hacks (`format!("is_{}", name)`) by using type-safe
    /// BindingId → BindingId mapping from CarrierInfo.
    ///
    /// ## Lookup Order
    ///
    /// 1. Direct BindingId lookup in ConditionEnv (if BindingId provided)
    /// 2. **NEW (Phase 76)**: Promoted BindingId lookup in CarrierInfo.promoted_bindings
    /// 3. Fallback to legacy name-based lookup (Phase 75 behavior)
    ///
    /// # Arguments
    ///
    /// * `binding_id` - Optional BindingId from MirBuilder's binding_map
    /// * `name` - Variable name (fallback for legacy paths)
    ///
    /// # Returns
    ///
    /// `Some(ValueId)` if found via BindingId/promoted/name, `None` otherwise.
    ///
    /// # Example (Phase 76 Promotion Path)
    ///
    /// ```ignore
    /// // Given:
    /// // - "digit_pos" has BindingId(5)
    /// // - "is_digit_pos" has BindingId(10)
    /// // - CarrierInfo.promoted_bindings[BindingId(5)] = BindingId(10)
    /// // - ConditionEnv.binding_id_map[BindingId(10)] = ValueId(102)
    ///
    /// let scope = Pattern2ScopeManager { ... };
    ///
    /// // Phase 76: BindingId-based promoted resolution (NEW!)
    /// let result = scope.lookup_with_binding(Some(BindingId(5)), "digit_pos");
    /// // Step 1: ConditionEnv[BindingId(5)] → None (not a carrier)
    /// // Step 2: CarrierInfo.promoted_bindings[BindingId(5)] → BindingId(10) ✓
    /// // Step 3: ConditionEnv[BindingId(10)] → ValueId(102) ✓
    /// assert_eq!(result, Some(ValueId(102)));
    ///
    /// // Phase 75: Legacy name-based fallback still works
    /// let result = scope.lookup_with_binding(None, "digit_pos");
    /// // → Falls back to format!("is_digit_pos") lookup
    /// assert_eq!(result, Some(ValueId(102)));
    /// ```
    ///
    /// # Phase 77 Migration Note
    ///
    /// The legacy name-based path (step 3) will be removed in Phase 77 after all
    /// promoters populate promoted_bindings map and all call sites provide BindingId.
    #[cfg(feature = "normalized_dev")]
    fn lookup_with_binding(&self, binding_id: Option<BindingId>, name: &str) -> Option<ValueId> {
        use super::debug_output_box::DebugOutputBox;
        let debug = DebugOutputBox::new("phase76");

        if let Some(bid) = binding_id {
            // Step 1: Try direct BindingId lookup in ConditionEnv (Phase 75)
            if let Some(value_id) = self.condition_env.resolve_var_with_binding(Some(bid), name) {
                debug.log(
                    "direct",
                    &format!(
                        "BindingId({}) -> ValueId({}) for '{}'",
                        bid.0, value_id.0, name
                    ),
                );
                return Some(value_id);
            }

            // Step 2: **NEW (Phase 76)**: Check promoted_bindings map
            if let Some(promoted_bid) = self.carrier_info.resolve_promoted_with_binding(bid) {
                // Promoted BindingId found, lookup in ConditionEnv
                if let Some(value_id) = self
                    .condition_env
                    .resolve_var_with_binding(Some(promoted_bid), name)
                {
                    debug.log(
                        "promoted",
                        &format!(
                            "BindingId({}) promoted to BindingId({}) -> ValueId({}) for '{}'",
                            bid.0, promoted_bid.0, value_id.0, name
                        ),
                    );
                    return Some(value_id);
                }
            }

            // Step 3: Fallback to legacy name-based lookup (Phase 75 fallback path)
            // Phase 77: DEPRECATED - Will be removed in Phase 78+
            #[cfg(feature = "normalized_dev")]
            get_global_ring0().log.debug(&format!(
                "[phase77/fallback] WARNING: BindingId({}) for '{}' not resolved, falling back to name-based lookup (DEPRECATED)",
                bid.0, name
            ));
            #[cfg(not(feature = "normalized_dev"))]
            debug.log(
                "fallback",
                &format!(
                    "BindingId({}) miss, falling back to name '{}' lookup",
                    bid.0, name
                ),
            );
        }

        // Step 4: Legacy name-based lookup (Phase 75 behavior)
        // Phase 77: DEPRECATED - Will be removed in Phase 78+ after all call sites use BindingId
        self.lookup(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};
    use crate::mir::loop_pattern_detection::function_scope_capture::CapturedVar;

    #[test]
    fn test_pattern2_scope_manager_loop_var() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_loopbodylocals: vec![],
            #[cfg(feature = "normalized_dev")]
            promoted_bindings: std::collections::BTreeMap::new(),
        };

        let scope = Pattern2ScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("i"), Some(ValueId(100)));
        assert_eq!(scope.scope_of("i"), Some(VarScopeKind::LoopVar));
    }

    #[test]
    fn test_pattern2_scope_manager_carrier() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));
        condition_env.insert("sum".to_string(), ValueId(101));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![CarrierVar {
                name: "sum".to_string(),
                host_id: ValueId(2),
                join_id: Some(ValueId(101)),
                role: CarrierRole::LoopState,
                init: CarrierInit::FromHost,
                #[cfg(feature = "normalized_dev")]
                binding_id: None,
            }],
            trim_helper: None,
            promoted_loopbodylocals: vec![],
            #[cfg(feature = "normalized_dev")]
            promoted_bindings: std::collections::BTreeMap::new(),
        };

        let scope = Pattern2ScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("sum"), Some(ValueId(101)));
        assert_eq!(scope.scope_of("sum"), Some(VarScopeKind::Carrier));
    }

    #[test]
    fn test_pattern2_scope_manager_promoted_variable() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![CarrierVar {
                name: "is_digit_pos".to_string(),
                host_id: ValueId(2),
                join_id: Some(ValueId(102)),
                role: CarrierRole::ConditionOnly,
                init: CarrierInit::BoolConst(false),
                #[cfg(feature = "normalized_dev")]
                binding_id: None,
            }],
            trim_helper: None,
            promoted_loopbodylocals: vec!["digit_pos".to_string()],
            #[cfg(feature = "normalized_dev")]
            promoted_bindings: std::collections::BTreeMap::new(),
        };

        let scope = Pattern2ScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        // Lookup "digit_pos" should resolve to "is_digit_pos" carrier
        assert_eq!(scope.lookup("digit_pos"), Some(ValueId(102)));
    }

    #[test]
    fn test_pattern2_scope_manager_body_local() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let mut body_local_env = LoopBodyLocalEnv::new();
        body_local_env.insert("temp".to_string(), ValueId(200));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_loopbodylocals: vec![],
            #[cfg(feature = "normalized_dev")]
            promoted_bindings: std::collections::BTreeMap::new(),
        };

        let scope = Pattern2ScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: Some(&body_local_env),
            captured_env: None,
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("temp"), Some(ValueId(200)));
        assert_eq!(scope.scope_of("temp"), Some(VarScopeKind::LoopBodyLocal));
    }

    #[test]
    fn test_pattern2_scope_manager_captured() {
        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));
        condition_env.insert("len".to_string(), ValueId(201));

        let mut captured_env =
            crate::mir::loop_pattern_detection::function_scope_capture::CapturedEnv::new();
        captured_env.add_var(CapturedVar {
            name: "len".to_string(),
            host_id: ValueId(42),
            is_immutable: true,
            kind: crate::mir::loop_pattern_detection::function_scope_capture::CapturedKind::Explicit,
        });

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_loopbodylocals: vec![],
            #[cfg(feature = "normalized_dev")]
            promoted_bindings: std::collections::BTreeMap::new(),
        };

        let scope = Pattern2ScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: Some(&captured_env),
            carrier_info: &carrier_info,
        };

        assert_eq!(scope.lookup("len"), Some(ValueId(201)));
        assert_eq!(scope.scope_of("len"), Some(VarScopeKind::Captured));
    }
}
