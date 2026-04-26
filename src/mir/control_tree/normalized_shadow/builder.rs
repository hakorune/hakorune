//! Phase 121: StepTree → JoinModule shadow lowering (if-only)
//!
//! Navigation:
//! - `src/mir/control_tree/README.md`
//! - `src/mir/control_tree/step_tree/README.md`
//!
//! Boundaries:
//! - StepTree is input SSOT; do not re-run facts extraction here.
//! - This module is dev-only lowering for selected normalized shadow shapes.
//! - Keep fail-fast behavior explicit and avoid hidden fallback heuristics.
//!
//! ## Responsibility
//!
//! - Convert StepTree to JoinModule (Normalized dialect)
//! - Only for if-only patterns (no loops)
//! - Returns None for out-of-scope patterns
//! - Returns Err for patterns that should be supported but conversion failed

use crate::mir::control_tree::normalized_shadow::entry::if_only;
use crate::mir::control_tree::normalized_shadow::env_layout::{
    expected_env_field_count as calc_expected_env_fields, EnvLayout,
};
use crate::mir::control_tree::normalized_shadow::if_as_last_join_k::IfAsLastJoinKLowererBox;
use crate::mir::control_tree::normalized_shadow::loop_true_break_once::LoopTrueBreakOnceBuilderBox; // Phase 131
use crate::mir::control_tree::normalized_shadow::loop_true_if_break_continue::LoopTrueIfBreakContinueBuilderBox; // Phase 143 P0
use crate::mir::control_tree::normalized_shadow::post_if_post_k::PostIfPostKBuilderBox; // Phase 129-C
use crate::mir::control_tree::step_tree::StepTree;
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::JoinModule;
use crate::mir::ValueId; // Phase 126
use std::collections::BTreeMap; // Phase 126

use super::contracts::{check_if_only, CapabilityCheckResult};

/// Box-First: StepTree → Normalized shadow lowering
pub struct StepTreeNormalizedShadowLowererBox;

impl StepTreeNormalizedShadowLowererBox {
    /// Phase 129-B: Expected env field count (writes + inputs)
    pub fn expected_env_field_count(
        step_tree: &StepTree,
        available_inputs: &BTreeMap<String, ValueId>,
    ) -> usize {
        calc_expected_env_fields(step_tree, available_inputs)
    }

    /// Phase 129-B: If-as-last shape detection (no post-if)
    pub fn expects_join_k_as_last(step_tree: &StepTree) -> bool {
        IfAsLastJoinKLowererBox::expects_join_k_as_last(step_tree)
    }

    /// Try to lower an if-only StepTree to normalized form
    ///
    /// - `Ok(None)`: Out of scope (e.g., contains unsupported constructs)
    /// - `Ok(Some(...))`: Shadow generation succeeded
    /// - `Err(...)`: Should be supported but conversion failed (internal error)
    ///
    /// Phase 131: Also supports loop(true) break-once pattern
    pub fn try_lower_if_only(
        step_tree: &StepTree,
        available_inputs: &BTreeMap<String, ValueId>,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        let capability = check_if_only(step_tree);
        match capability {
            CapabilityCheckResult::Supported => {
                // If-only path
                Self::lower_if_only_to_normalized(step_tree, available_inputs)
            }
            CapabilityCheckResult::Unsupported(_reason) => {
                // Phase 131: Try loop(true) break-once pattern
                Self::lower_with_loop_support(step_tree, available_inputs)
            }
        }
    }

    /// Lower if-only StepTree to Normalized JoinModule (Phase 122-126)
    fn lower_if_only_to_normalized(
        step_tree: &StepTree,
        available_inputs: &BTreeMap<String, ValueId>,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        // Phase 126: EnvLayout 生成（available_inputs を使用）
        let env_layout = EnvLayout::from_contract(&step_tree.contract, available_inputs);

        // Phase 129-C: Post-if with post_k continuation (dev-only)
        if let Some((module, meta)) = PostIfPostKBuilderBox::lower(step_tree, &env_layout)? {
            return Ok(Some((module, meta)));
        }

        // Phase 129-B: If-as-last join_k lowering (dev-only)
        if let Some((module, meta)) = IfAsLastJoinKLowererBox::lower(step_tree, &env_layout)? {
            return Ok(Some((module, meta)));
        }

        // Fossil baseline if-only entry (Phase 123-128 scope).
        // New shapes should get a new route before this call; do not widen the
        // historical placeholder/then-branch behavior here.
        if_only::lower_if_only_to_normalized(step_tree, &env_layout)
    }

    /// Phase 131: Lower StepTree with loop support (loop(true) break-once only)
    fn lower_with_loop_support(
        step_tree: &StepTree,
        available_inputs: &BTreeMap<String, ValueId>,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        // Phase 126: EnvLayout 生成（available_inputs を使用）
        let env_layout = EnvLayout::from_contract(&step_tree.contract, available_inputs);

        // Phase 131: loop(true) break-once pattern (simpler, higher priority)
        if let Some((module, meta)) = LoopTrueBreakOnceBuilderBox::lower(step_tree, &env_layout)? {
            return Ok(Some((module, meta)));
        }

        // Phase 143 P0: loop(true) + if + break pattern
        if let Some((module, meta)) =
            LoopTrueIfBreakContinueBuilderBox::lower(step_tree, &env_layout)?
        {
            return Ok(Some((module, meta)));
        }

        // Not supported by loop routes; let route chaining decline this shape.
        Ok(None)
    }

    /// Dev log helper for out-of-scope cases
    pub fn get_status_string(step_tree: &StepTree) -> String {
        format!(
            "shadow=skipped signature_basis={}",
            step_tree.signature_basis_string()
        )
    }
}
