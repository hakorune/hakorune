//! Phase 200-2: JoinInlineBoundaryBuilder - Builder pattern for JoinInlineBoundary construction
//!
//! This module provides a fluent Builder API to construct JoinInlineBoundary objects,
//! reducing field manipulation scattering in pattern lowerers.
//!
//! # Design Philosophy
//!
//! - **Centralized Construction**: All boundary field manipulation in one place
//! - **Type Safety**: Builder ensures fields are set correctly
//! - **Readability**: Fluent API makes construction intent clear
//! - **Maintainability**: Changes to boundary structure isolated to this builder
//!
//! # Example Usage
//!
//! ```ignore
//! use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;
//!
//! let boundary = JoinInlineBoundaryBuilder::new()
//!     .with_inputs(vec![ValueId(0)], vec![loop_var_id])
//!     .with_loop_var_name(Some(loop_param_name.to_string()))
//!     .with_condition_bindings(condition_bindings)
//!     .with_exit_bindings(exit_bindings)
//!     .with_expr_result(fragment_meta.expr_result)
//!     .build();
//! ```

use super::condition_to_joinir::ConditionBinding;
use super::inline_boundary::{JoinInlineBoundary, JumpArgsLayout, LoopExitBinding};
use crate::mir::ValueId;

/// Role of a parameter in JoinIR lowering (Phase 200-A)
///
/// This enum explicitly classifies parameters to ensure correct routing
/// during JoinIR → MIR lowering and boundary construction.
///
/// # Invariants
///
/// - **LoopParam**: Participates in header PHI, updated in loop body
///   - Example: `i` in `loop(i < len)` - iteration variable
///   - Routing: join_inputs + host_inputs + header PHI + exit_bindings
///
/// - **Condition**: Used in condition only, NOT in header PHI, NOT in ExitLine
///   - Example: `digits` in `digits.indexOf(ch)` - function-scoped constant
///   - Routing: condition_bindings ONLY (no PHI, no exit_bindings)
///   - Rationale: Condition-only vars are immutable and not updated in loop
///
/// - **Carrier**: Updated in loop body, participates in header PHI and ExitLine
///   - Example: `sum`, `count` in accumulation loops
///   - Routing: join_inputs + host_inputs + header PHI + exit_bindings
///
/// - **ExprResult**: Return value of the loop expression
///   - Example: Loop result in `return loop(...)`
///   - Routing: Handled by exit_phi_builder (set_expr_result)
///
/// # Phase 200-A Status
///
/// Enum is defined but not yet used for routing. Routing implementation
/// will be added in Phase 200-B when CapturedEnv integration is complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamRole {
    /// Loop iteration variable (e.g., `i` in `loop(i < len)`)
    LoopParam,

    /// Condition-only parameter (e.g., `digits` in `digits.indexOf(ch)`)
    /// NOT included in header PHI or ExitLine
    Condition,

    /// State carried across iterations (e.g., `sum`, `count`)
    Carrier,

    /// Expression result returned by the loop
    ExprResult,
}

/// Builder for constructing JoinInlineBoundary objects
///
/// Provides a fluent API to set boundary fields without direct field manipulation.
pub struct JoinInlineBoundaryBuilder {
    boundary: JoinInlineBoundary,
}

impl JoinInlineBoundaryBuilder {
    /// Create a new builder with default empty boundary
    pub fn new() -> Self {
        use crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode;
        Self {
            boundary: JoinInlineBoundary {
                join_inputs: vec![],
                host_inputs: vec![],
                loop_invariants: vec![], // Phase 255 P2: Initialize as empty
                exit_bindings: vec![],
                condition_bindings: vec![],
                expr_result: None,
                jump_args_layout: JumpArgsLayout::CarriersOnly,
                loop_var_name: None,
                loop_header_func_name: None, // Phase 287 P2
                carrier_info: None,          // Phase 228: Initialize as None
                continuation_func_ids: JoinInlineBoundary::default_continuations(),
                exit_reconnect_mode: ExitReconnectMode::default(), // Phase 131 P1.5
            },
        }
    }

    /// Set input mappings (JoinIR-local ↔ Host ValueIds)
    ///
    /// # Arguments
    ///
    /// * `join_inputs` - JoinIR-local ValueIds (e.g., [ValueId(0)])
    /// * `host_inputs` - Host ValueIds (e.g., [loop_var_id])
    ///
    /// # Panics
    ///
    /// Panics if `join_inputs` and `host_inputs` have different lengths.
    pub fn with_inputs(mut self, join_inputs: Vec<ValueId>, host_inputs: Vec<ValueId>) -> Self {
        assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "join_inputs and host_inputs must have same length"
        );
        self.boundary.join_inputs = join_inputs;
        self.boundary.host_inputs = host_inputs;
        self
    }

    /// Set condition bindings (Phase 171-fix)
    ///
    /// Each binding explicitly maps:
    /// - Variable name
    /// - HOST ValueId → JoinIR ValueId
    pub fn with_condition_bindings(mut self, bindings: Vec<ConditionBinding>) -> Self {
        self.boundary.condition_bindings = bindings;
        self
    }

    /// Set exit bindings (Phase 190+)
    ///
    /// Each binding explicitly names the carrier variable and its source/destination.
    pub fn with_exit_bindings(mut self, bindings: Vec<LoopExitBinding>) -> Self {
        self.boundary.exit_bindings = bindings;
        self
    }

    /// Phase 255 P2: Set loop invariants
    ///
    /// Variables that are referenced inside the loop body but do not change
    /// across iterations. These need header PHI nodes (with same value from all
    /// incoming edges) but do NOT need exit PHI nodes.
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.with_loop_invariants(vec![
    ///     ("s".to_string(), ValueId(10)),  // haystack
    ///     ("ch".to_string(), ValueId(11)), // needle
    /// ])
    /// ```
    pub fn with_loop_invariants(mut self, invariants: Vec<(String, ValueId)>) -> Self {
        self.boundary.loop_invariants = invariants;
        self
    }

    /// Set loop variable name (Phase 33-16)
    ///
    /// Used for LoopHeaderPhiBuilder to track which PHI corresponds to the loop variable.
    pub fn with_loop_var_name(mut self, name: Option<String>) -> Self {
        self.boundary.loop_var_name = name;
        self
    }

    /// Phase 287 P2: Set loop header function name (SSOT)
    ///
    /// If omitted for loop routes, `build()` defaults it to `"loop_step"` when `loop_var_name` is set.
    pub fn with_loop_header_func_name(mut self, name: Option<String>) -> Self {
        self.boundary.loop_header_func_name = name;
        self
    }

    /// Set expression result (Phase 33-14)
    ///
    /// If the loop is used as an expression, this is the JoinIR-local ValueId
    /// of k_exit's return value.
    pub fn with_expr_result(mut self, expr: Option<ValueId>) -> Self {
        self.boundary.expr_result = expr;
        self
    }

    /// Build the final JoinInlineBoundary
    pub fn build(self) -> JoinInlineBoundary {
        let mut boundary = self.boundary;
        boundary.jump_args_layout = JoinInlineBoundary::decide_jump_args_layout(
            boundary.expr_result,
            boundary.exit_bindings.as_slice(),
        );

        // Phase 287 P2: Default loop header function name for loop routes.
        // If a pattern sets loop_var_name, it must have a loop header function.
        if boundary.loop_var_name.is_some() && boundary.loop_header_func_name.is_none() {
            boundary.loop_header_func_name =
                Some(crate::mir::join_ir::lowering::canonical_names::LOOP_STEP.to_string());
        }

        boundary
    }

    /// Add a parameter with explicit role (Phase 200-A)
    ///
    /// This method allows adding parameters with explicit role classification,
    /// ensuring correct routing during JoinIR → MIR lowering.
    ///
    /// # Phase 200-A Status
    ///
    /// Currently stores parameters based on role but does not use role for advanced routing.
    /// Full role-based routing will be implemented in Phase 200-B.
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name (e.g., "i", "digits", "sum")
    /// * `host_id` - Host function's ValueId for this variable
    /// * `role` - Parameter role (LoopParam / Condition / Carrier / ExprResult)
    ///
    /// # Routing Rules (Phase 200-B+)
    ///
    /// - **LoopParam**: add_input (join_inputs + host_inputs)
    /// - **Condition**: add to condition_bindings (no PHI, no exit_bindings)
    /// - **Carrier**: add_input + exit_bindings
    /// - **ExprResult**: set_expr_result (handled separately)
    ///
    /// # Example (Future Phase 200-B)
    ///
    /// ```ignore
    /// builder.add_param_with_role("i", ValueId(100), ParamRole::LoopParam);
    /// builder.add_param_with_role("digits", ValueId(42), ParamRole::Condition);
    /// builder.add_param_with_role("sum", ValueId(101), ParamRole::Carrier);
    /// ```
    pub fn add_param_with_role(&mut self, name: &str, host_id: ValueId, role: ParamRole) {
        // Phase 200-B: Full role-based routing implementation
        //
        // Routing rules:
        // - LoopParam: join_inputs + host_inputs (participates in PHI)
        // - Condition: condition_bindings ONLY (no PHI, no ExitLine)
        // - Carrier: join_inputs + host_inputs (participates in PHI + ExitLine)
        // - ExprResult: Handled by set_expr_result

        match role {
            ParamRole::LoopParam | ParamRole::Carrier => {
                // Add to join_inputs + host_inputs
                let join_id = ValueId(self.boundary.join_inputs.len() as u32);
                self.boundary.join_inputs.push(join_id);
                self.boundary.host_inputs.push(host_id);
            }
            ParamRole::Condition => {
                // Phase 200-B: Add to condition_bindings without PHI
                // 1. Allocate JoinIR-local ValueId
                let join_id = ValueId(
                    (self.boundary.join_inputs.len() + self.boundary.condition_bindings.len())
                        as u32,
                );

                // 2. Create ConditionBinding
                let binding = ConditionBinding {
                    name: name.to_string(),
                    host_value: host_id,
                    join_value: join_id,
                };

                // 3. Add to condition_bindings
                self.boundary.condition_bindings.push(binding);
            }
            ParamRole::ExprResult => {
                // Handled separately by with_expr_result
                // No action needed here
            }
        }
    }

    /// Get JoinIR ValueId for a condition-only binding (Phase 200-B)
    ///
    /// Returns the JoinIR-local ValueId for a captured variable that was added
    /// with ParamRole::Condition.
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name to look up
    ///
    /// # Returns
    ///
    /// `Some(ValueId)` if the variable exists in condition_bindings, `None` otherwise.
    pub fn get_condition_binding(&self, name: &str) -> Option<ValueId> {
        self.boundary
            .condition_bindings
            .iter()
            .find(|b| b.name == name)
            .map(|b| b.join_value)
    }

    /// Phase 228: Set carrier metadata
    ///
    /// Provides full carrier information including initialization policies.
    /// This allows header PHI generation to handle ConditionOnly carriers
    /// with explicit bool initialization.
    ///
    /// # Arguments
    ///
    /// * `carrier_info` - Complete carrier metadata from pattern lowerer
    ///
    /// # Example
    ///
    /// ```ignore
    /// let boundary = JoinInlineBoundaryBuilder::new()
    ///     .with_inputs(join_inputs, host_inputs)
    ///     .with_carrier_info(ctx.carrier_info.clone())
    ///     .build();
    /// ```
    pub fn with_carrier_info(mut self, carrier_info: super::carrier_info::CarrierInfo) -> Self {
        self.boundary.carrier_info = Some(carrier_info);
        self
    }

    /// Set continuation function names (Phase 256 P1.7)
    ///
    /// Continuation functions (e.g., k_exit) are functions that should be merged
    /// into the host function. This method registers which JoinIR functions are
    /// continuations, enabling proper merge behavior.
    ///
    /// # Arguments
    ///
    /// * `func_names` - Set of function names (Strings) representing continuation functions
    ///
    /// # Example
    ///
    /// ```ignore
    /// use std::collections::BTreeSet;
    /// use crate::mir::join_ir::lowering::canonical_names as cn;
    /// let boundary = JoinInlineBoundaryBuilder::new()
    ///     .with_inputs(join_inputs, host_inputs)
    ///     .with_continuation_funcs(BTreeSet::from([cn::K_EXIT.to_string()]))
    ///     .build();
    /// ```
    ///
    /// # Why Strings instead of JoinFuncIds
    ///
    /// The MirModule after bridge conversion uses JoinFunction.name as the function key,
    /// not "join_func_{id}". The merge code looks up functions by name, so we must use
    /// actual function names here.
    pub fn with_continuation_funcs(
        mut self,
        func_names: std::collections::BTreeSet<String>,
    ) -> Self {
        self.boundary.continuation_func_ids = func_names;
        self
    }

    /// Phase 256 P1.7: Register k_exit as continuation (convenience method)
    ///
    /// This is a convenience method for the common case of registering "k_exit" as a
    /// continuation function. It's equivalent to:
    /// ```ignore
    /// use crate::mir::join_ir::lowering::canonical_names as cn;
    /// .with_continuation_funcs(BTreeSet::from([cn::K_EXIT.to_string()]))
    /// ```
    ///
    /// # Example
    ///
    /// ```ignore
    /// let boundary = JoinInlineBoundaryBuilder::new()
    ///     .with_inputs(join_inputs, host_inputs)
    ///     .with_k_exit_continuation()
    ///     .build();
    /// ```
    ///
    /// # Note
    ///
    /// For multiple continuations or custom function names, use `with_continuation_funcs()`
    /// instead. This method is specifically for the "k_exit" pattern.
    pub fn with_k_exit_continuation(mut self) -> Self {
        use super::canonical_names as cn;
        self.boundary
            .continuation_func_ids
            .insert(cn::K_EXIT.to_string());
        self
    }
}

impl Default for JoinInlineBoundaryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;

    #[test]
    fn test_builder_basic() {
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![ValueId(0)], vec![ValueId(4)])
            .build();

        assert_eq!(boundary.join_inputs, vec![ValueId(0)]);
        assert_eq!(boundary.host_inputs, vec![ValueId(4)]);
        assert_eq!(boundary.exit_bindings.len(), 0);
        assert_eq!(boundary.condition_bindings.len(), 0);
        assert_eq!(boundary.expr_result, None);
        assert_eq!(boundary.loop_var_name, None);
    }

    #[test]
    fn test_builder_full() {
        let condition_binding = ConditionBinding {
            name: "start".to_string(),
            host_value: ValueId(33),
            join_value: ValueId(1),
        };

        let exit_binding = LoopExitBinding {
            carrier_name: "sum".to_string(),
            join_exit_value: ValueId(18),
            host_slot: ValueId(5),
            role: CarrierRole::LoopState,
        };

        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![ValueId(0)], vec![ValueId(4)])
            .with_loop_var_name(Some("i".to_string()))
            .with_condition_bindings(vec![condition_binding])
            .with_exit_bindings(vec![exit_binding])
            .with_expr_result(Some(ValueId(20)))
            .build();

        assert_eq!(boundary.join_inputs, vec![ValueId(0)]);
        assert_eq!(boundary.host_inputs, vec![ValueId(4)]);
        assert_eq!(boundary.loop_var_name, Some("i".to_string()));
        assert_eq!(boundary.condition_bindings.len(), 1);
        assert_eq!(boundary.exit_bindings.len(), 1);
        assert_eq!(boundary.expr_result, Some(ValueId(20)));
    }

    #[test]
    #[should_panic(expected = "join_inputs and host_inputs must have same length")]
    fn test_builder_mismatched_inputs() {
        JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![ValueId(0), ValueId(1)], vec![ValueId(4)])
            .build();
    }

    #[test]
    fn test_builder_default() {
        let builder = JoinInlineBoundaryBuilder::default();
        let boundary = builder.build();

        assert_eq!(boundary.join_inputs.len(), 0);
        assert_eq!(boundary.host_inputs.len(), 0);
    }

    #[test]
    fn test_builder_if_phi_join_style() {
        // IfPhiJoin style: Two carriers (i + sum), exit_bindings, loop_var_name

        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(0), ValueId(1)],
                vec![ValueId(100), ValueId(101)],
            )
            .with_exit_bindings(vec![LoopExitBinding {
                carrier_name: "sum".to_string(),
                join_exit_value: ValueId(18),
                host_slot: ValueId(101),
                role: CarrierRole::LoopState,
            }])
            .with_loop_var_name(Some("i".to_string()))
            .build();

        assert_eq!(boundary.join_inputs.len(), 2);
        assert_eq!(boundary.host_inputs.len(), 2);
        assert_eq!(boundary.exit_bindings.len(), 1);
        assert_eq!(boundary.exit_bindings[0].carrier_name, "sum");
        assert_eq!(boundary.loop_var_name, Some("i".to_string()));
        assert_eq!(boundary.expr_result, None);
    }

    #[test]
    fn test_builder_loop_continue_only_style() {
        // LoopContinueOnly style: Dynamic carrier count, continue support
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(
                vec![ValueId(0), ValueId(1), ValueId(2)], // i + 2 carriers
                vec![ValueId(100), ValueId(101), ValueId(102)],
            )
            .with_exit_bindings(vec![
                LoopExitBinding {
                    carrier_name: "i".to_string(),
                    join_exit_value: ValueId(11),
                    host_slot: ValueId(100),
                    role: CarrierRole::LoopState,
                },
                LoopExitBinding {
                    carrier_name: "sum".to_string(),
                    join_exit_value: ValueId(20),
                    host_slot: ValueId(101),
                    role: CarrierRole::LoopState,
                },
            ])
            .with_loop_var_name(Some("i".to_string()))
            .build();

        assert_eq!(boundary.exit_bindings.len(), 2);
        assert!(boundary.loop_var_name.is_some());
        assert_eq!(boundary.join_inputs.len(), 3);
        assert_eq!(boundary.host_inputs.len(), 3);
    }

    // Phase 200-A: ParamRole tests
    #[test]
    fn test_param_role_loop_param() {
        let mut builder = JoinInlineBoundaryBuilder::new();
        builder.add_param_with_role("i", ValueId(100), ParamRole::LoopParam);

        let boundary = builder.build();
        assert_eq!(boundary.join_inputs.len(), 1);
        assert_eq!(boundary.host_inputs.len(), 1);
        assert_eq!(boundary.host_inputs[0], ValueId(100));
    }

    #[test]
    fn test_param_role_condition() {
        let mut builder = JoinInlineBoundaryBuilder::new();
        // Phase 200-B: Condition role is added to condition_bindings
        builder.add_param_with_role("digits", ValueId(42), ParamRole::Condition);

        let boundary = builder.build();
        // Phase 200-B: Condition params go to condition_bindings, not join_inputs
        assert_eq!(boundary.join_inputs.len(), 0);
        assert_eq!(boundary.condition_bindings.len(), 1);
        assert_eq!(boundary.condition_bindings[0].name, "digits");
        assert_eq!(boundary.condition_bindings[0].host_value, ValueId(42));
    }

    #[test]
    fn test_param_role_carrier() {
        let mut builder = JoinInlineBoundaryBuilder::new();
        builder.add_param_with_role("sum", ValueId(101), ParamRole::Carrier);

        let boundary = builder.build();
        assert_eq!(boundary.join_inputs.len(), 1);
        assert_eq!(boundary.host_inputs.len(), 1);
        assert_eq!(boundary.host_inputs[0], ValueId(101));
    }

    #[test]
    fn test_with_k_exit_continuation() {
        // Phase 256 P1.7: Test convenience method for k_exit registration
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![ValueId(0)], vec![ValueId(100)])
            .with_k_exit_continuation()
            .build();

        assert_eq!(boundary.continuation_func_ids.len(), 1);
        assert!(boundary.continuation_func_ids.contains("k_exit"));
    }

    #[test]
    fn test_with_continuation_funcs_manual() {
        // Phase 256 P1.7: Test manual continuation registration (should be same as with_k_exit_continuation)
        use std::collections::BTreeSet;
        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![ValueId(0)], vec![ValueId(100)])
            .with_continuation_funcs(BTreeSet::from(["k_exit".to_string()]))
            .build();

        assert_eq!(boundary.continuation_func_ids.len(), 1);
        assert!(boundary.continuation_func_ids.contains("k_exit"));
    }

    #[test]
    fn test_with_k_exit_and_additional_continuation() {
        // Phase 256 P1.7: Test combining convenience method with additional continuations
        use std::collections::BTreeSet;
        let mut continuations = BTreeSet::new();
        continuations.insert("post_k".to_string());

        let boundary = JoinInlineBoundaryBuilder::new()
            .with_inputs(vec![ValueId(0)], vec![ValueId(100)])
            .with_k_exit_continuation()
            .with_continuation_funcs(continuations)
            .build();

        // with_continuation_funcs replaces the set, so only post_k should be present
        assert_eq!(boundary.continuation_func_ids.len(), 1);
        assert!(boundary.continuation_func_ids.contains("post_k"));
        assert!(!boundary.continuation_func_ids.contains("k_exit"));
    }
}
