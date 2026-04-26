//! Shared loop_break prep types and debug helpers.
//!
//! Analysis-only facts gathering now lives in
//! `loop_break_steps::gather_facts_step_box`.
use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::{ConditionBinding, ConditionEnv};
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::ValueId;

use crate::mir::loop_route_detection::support::function_scope::CapturedEnv;

pub(crate) struct LoopBreakDebugLog {
    verbose: bool,
    debug: DebugOutputBox,
}

impl LoopBreakDebugLog {
    pub(crate) fn new(verbose: bool) -> Self {
        Self {
            verbose,
            debug: DebugOutputBox::new_with_enabled("joinir/loop_break", verbose),
        }
    }

    pub(crate) fn log(&self, tag: &str, message: impl AsRef<str>) {
        if self.verbose {
            self.debug.log(tag, message.as_ref());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyLocalHandlingPolicy {
    DefaultPromotion,
    SkipPromotion,
}

pub(in crate::mir::builder) struct LoopBreakPrepInputs {
    pub loop_var_name: String,
    pub loop_var_id: ValueId,
    pub carrier_info: CarrierInfo,
    pub scope: LoopScopeShape,
    pub captured_env: CapturedEnv,
    pub join_value_space: JoinValueSpace,
    pub env: ConditionEnv,
    pub condition_bindings: Vec<ConditionBinding>,
    pub body_local_env: LoopBodyLocalEnv,
    /// Phase 92 P3: Allow-list of LoopBodyLocal variable names permitted in conditions.
    /// This must stay minimal (1 variable) and is validated by ReadOnlyBodyLocalSlotBox.
    pub allowed_body_locals_for_conditions: Vec<String>,
    /// Phase 107: For some policy-routed families, loop_break route must not run promotion/slot heuristics.
    pub body_local_handling: BodyLocalHandlingPolicy,
    /// Phase 92 P3: Diagnostics / debug metadata for the allow-listed variable.
    pub read_only_body_local_slot: Option<crate::mir::join_ir::lowering::common::body_local_slot::ReadOnlyBodyLocalSlot>,
    /// Policy-routed "break when true" condition node.
    pub break_condition_node: ASTNode,
    /// loop(true) + break-only digits（read_digits_from family）
    pub is_loop_true_read_digits: bool,
    /// Phase 93 P0: ConditionOnly recipe for derived slot recalculation
    pub condition_only_recipe: Option<
        crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyRecipe,
    >,
    /// Phase 94: BodyLocalDerived recipe for P5b "ch" reassignment + escape counter.
    pub body_local_derived_recipe:
        Option<crate::mir::join_ir::lowering::common::body_local_derived_emitter::BodyLocalDerivedRecipe>,
    /// Phase 29ab P4: Derived slot recipe for seg-like conditional assignments.
    pub body_local_derived_slot_recipe: Option<
        crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::BodyLocalDerivedSlotRecipe,
    >,
    /// Phase 107: Balanced depth-scan (find_balanced_*) derived recipe.
    pub balanced_depth_scan_recipe:
        Option<crate::mir::join_ir::lowering::common::balanced_depth_scan_emitter::BalancedDepthScanRecipe>,
    /// Phase 107: Carrier updates override (policy SSOT).
    pub carrier_updates_override: Option<std::collections::BTreeMap<String, UpdateExpr>>,
    /// Phase 107: Post-loop early return plan for return-in-loop normalization.
    pub post_loop_early_return:
        Option<crate::mir::policies::post_loop_early_return_plan::PostLoopEarlyReturnPlan>,
    /// Phase 252: Name of the static box being lowered (for this.method(...) in break conditions).
    pub current_static_box_name: Option<String>,
}
