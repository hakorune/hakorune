use super::types::{JoinInlineBoundary, JumpArgsLayout, LoopExitBinding};
use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, ExitReconnectMode};
use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
use crate::mir::ValueId;
use std::collections::BTreeSet;

impl JoinInlineBoundary {
    /// Decide jump_args layout from boundary inputs (SSOT)
    pub fn decide_jump_args_layout(
        expr_result: Option<ValueId>,
        exit_bindings: &[LoopExitBinding],
    ) -> JumpArgsLayout {
        if let Some(expr_result_id) = expr_result {
            let expr_is_carrier = exit_bindings.iter().any(|binding| {
                binding.role != CarrierRole::ConditionOnly
                    && binding.join_exit_value == expr_result_id
            });
            if expr_is_carrier {
                JumpArgsLayout::CarriersOnly
            } else {
                JumpArgsLayout::ExprResultPlusCarriers
            }
        } else {
            JumpArgsLayout::CarriersOnly
        }
    }

    /// Validate jump_args layout against boundary contract (Fail-Fast)
    pub fn validate_jump_args_layout(&self) -> Result<(), String> {
        let expected =
            Self::decide_jump_args_layout(self.expr_result, self.exit_bindings.as_slice());
        if self.jump_args_layout != expected {
            return Err(format!(
                "joinir/jump_args_layout_mismatch: expr_result={:?} layout={:?} expected={:?}",
                self.expr_result, self.jump_args_layout, expected
            ));
        }
        Ok(())
    }

    /// Phase 132-R0 Task 1: SSOT for default continuation function names
    /// Phase 256 P1.7: Changed from JoinFuncIds to function names (Strings)
    ///
    /// Returns the default set of continuation functions (k_exit).
    pub fn default_continuations() -> BTreeSet<String> {
        BTreeSet::from([crate::mir::join_ir::lowering::canonical_names::K_EXIT.to_string()])
    }

    /// Create a new boundary with input mappings only
    ///
    /// This is the common case for loops like `LoopSimpleWhile` where:
    /// - Inputs: loop variables (e.g., `i` in `loop(i < 3)`)
    /// - Outputs: none (loop returns void/0)
    pub fn new_inputs_only(join_inputs: Vec<ValueId>, host_inputs: Vec<ValueId>) -> Self {
        assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "join_inputs and host_inputs must have same length"
        );
        Self {
            join_inputs,
            host_inputs,
            loop_invariants: vec![],
            exit_bindings: vec![],
            condition_bindings: vec![],
            expr_result: None,
            jump_args_layout: JumpArgsLayout::CarriersOnly,
            loop_var_name: None,
            loop_header_func_name: None,
            carrier_info: None,
            continuation_func_ids: Self::default_continuations(),
            exit_reconnect_mode: ExitReconnectMode::default(),
        }
    }

    /// Create a new boundary with explicit exit bindings (Phase 190+)
    ///
    /// This is the recommended constructor for loops with exit carriers.
    /// Each exit binding explicitly names the carrier variable and its
    /// source/destination values.
    pub fn new_with_exit_bindings(
        join_inputs: Vec<ValueId>,
        host_inputs: Vec<ValueId>,
        exit_bindings: Vec<LoopExitBinding>,
    ) -> Self {
        assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "join_inputs and host_inputs must have same length"
        );
        Self {
            join_inputs,
            host_inputs,
            loop_invariants: vec![],
            exit_bindings,
            condition_bindings: vec![],
            expr_result: None,
            jump_args_layout: JumpArgsLayout::CarriersOnly,
            loop_var_name: None,
            loop_header_func_name: None,
            carrier_info: None,
            continuation_func_ids: Self::default_continuations(),
            exit_reconnect_mode: ExitReconnectMode::default(),
        }
    }

    /// Phase 171-fix: Create boundary with ConditionBindings (recommended)
    ///
    /// This uses explicit ConditionBindings instead of legacy condition-only inputs.
    pub fn new_with_condition_bindings(
        join_inputs: Vec<ValueId>,
        host_inputs: Vec<ValueId>,
        condition_bindings: Vec<ConditionBinding>,
    ) -> Self {
        assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "join_inputs and host_inputs must have same length"
        );
        Self {
            join_inputs,
            host_inputs,
            loop_invariants: vec![],
            exit_bindings: vec![],
            condition_bindings,
            expr_result: None,
            jump_args_layout: JumpArgsLayout::CarriersOnly,
            loop_var_name: None,
            loop_header_func_name: None,
            carrier_info: None,
            continuation_func_ids: Self::default_continuations(),
            exit_reconnect_mode: ExitReconnectMode::default(),
        }
    }
}
