//! InstructionFilterBox: Skip judgment logic for JoinIR instruction rewriting
//!
//! Phase 286C-2: Extracted from instruction_rewriter.rs
//! Provides pure functions to determine which instructions should be skipped during rewriting.

use crate::mir::{types::ConstValue, ValueId};

/// InstructionFilterBox: Skip judgment logic
///
/// Pure functions that determine which instructions should be skipped during
/// JoinIR→MIR rewriting. Each function answers a specific "should I skip this?" question.
pub struct InstructionFilterBox;

impl InstructionFilterBox {
    /// Check if a Copy instruction overwrites a PHI dst (should skip during rewriting)
    ///
    /// In loop headers with PHI nodes, certain Copy instructions would overwrite
    /// the PHI inputs that will be provided by the loop header PHIs themselves.
    /// These copies must be skipped to prevent incorrect values.
    ///
    /// # Arguments
    /// * `dst` - The destination ValueId of the Copy instruction (after remapping)
    /// * `phi_dsts` - Set of PHI destination ValueIds to protect
    ///
    /// # Returns
    /// * `true` if the Copy should be skipped (overwrites a PHI dst)
    /// * `false` if the Copy should be kept
    pub fn should_skip_copy_overwriting_phi(
        dst: ValueId,
        phi_dsts: &std::collections::HashSet<ValueId>,
    ) -> bool {
        phi_dsts.contains(&dst)
    }

    /// Check if a Const String instruction is a function name (should skip, already mapped)
    ///
    /// Function name constants are already mapped via `value_to_func_name` and
    /// don't need to be emitted as instructions in the merged MIR.
    ///
    /// # Arguments
    /// * `value` - The ConstValue to check
    /// * `is_in_func_name_map` - Whether the value's dst is in `value_to_func_name`
    ///
    /// # Returns
    /// * `true` if this Const defines a function name (should skip)
    /// * `false` otherwise
    pub fn should_skip_function_name_const(value: &ConstValue) -> bool {
        matches!(value, ConstValue::String(_))
    }

    /// Check if a Const instruction is a boundary input (should skip, injected by BoundaryInjector)
    ///
    /// Boundary input constants are provided by the BoundaryInjector via Copy instructions,
    /// so the original Const definitions should be skipped.
    ///
    /// # Arguments
    /// * `value_id` - The ValueId of the Const instruction's dst
    /// * `boundary_inputs` - Set of boundary input ValueIds
    /// * `is_loop_entry_point` - Whether this is the loop entry point
    ///
    /// # Returns
    /// * `true` if this Const is a boundary input (should skip)
    /// * `false` otherwise
    pub fn should_skip_boundary_input_const(
        value_id: ValueId,
        boundary_inputs: &[ValueId],
        is_loop_entry_point: bool,
    ) -> bool {
        is_loop_entry_point && boundary_inputs.contains(&value_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_should_skip_copy_overwriting_phi() {
        let mut phi_dsts = HashSet::new();
        phi_dsts.insert(ValueId(100));
        phi_dsts.insert(ValueId(200));

        // Should skip because dst is in phi_dsts
        assert!(InstructionFilterBox::should_skip_copy_overwriting_phi(
            ValueId(100),
            &phi_dsts
        ));

        // Should NOT skip because dst is not in phi_dsts
        assert!(!InstructionFilterBox::should_skip_copy_overwriting_phi(
            ValueId(300),
            &phi_dsts
        ));
    }

    #[test]
    fn test_should_skip_function_name_const() {
        // String const should be skipped (function name)
        assert!(InstructionFilterBox::should_skip_function_name_const(
            &ConstValue::String("main".to_string())
        ));

        // Other consts should NOT be skipped
        assert!(!InstructionFilterBox::should_skip_function_name_const(
            &ConstValue::Integer(42)
        ));
        assert!(!InstructionFilterBox::should_skip_function_name_const(
            &ConstValue::Void
        ));
    }

    #[test]
    fn test_should_skip_boundary_input_const() {
        let boundary_inputs = vec![ValueId(10), ValueId(20), ValueId(30)];

        // Should skip: is_loop_entry_point=true and value_id in boundary_inputs
        assert!(InstructionFilterBox::should_skip_boundary_input_const(
            ValueId(20),
            &boundary_inputs,
            true
        ));

        // Should NOT skip: is_loop_entry_point=false
        assert!(!InstructionFilterBox::should_skip_boundary_input_const(
            ValueId(20),
            &boundary_inputs,
            false
        ));

        // Should NOT skip: value_id not in boundary_inputs
        assert!(!InstructionFilterBox::should_skip_boundary_input_const(
            ValueId(999),
            &boundary_inputs,
            true
        ));
    }
}
