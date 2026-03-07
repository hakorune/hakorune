//! Phase 273 P3: Core Orchestrator - Public API and dispatcher
//!
//! # Responsibilities
//!
//! - Provide public API (lower() entry point)
//! - Dispatch CorePlan variants to specialized modules
//! - Manage loop frame stack for Break/Continue resolution
//! - Handle Effect context (in-loop vs standalone)
//!
//! # Design
//!
//! - lower() is the single public entry point
//! - lower_with_stack() is the internal dispatcher
//! - LoopFrame tracks break/continue targets and PHI inputs

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// LoopFrame: Break/Continue target tracking with PHI input accumulation
#[derive(Debug, Clone)]
pub(in super::super) struct LoopFrame {
    pub(super) break_target: BasicBlockId,
    pub(super) continue_target: BasicBlockId,
    pub(super) step_phi_inputs: BTreeMap<ValueId, BTreeMap<BasicBlockId, ValueId>>,
    pub(super) break_phi_inputs: BTreeMap<ValueId, BTreeMap<BasicBlockId, ValueId>>,
}

impl super::PlanLowerer {
    /// CorePlan を受け取り、MIR を生成
    ///
    /// # Arguments
    ///
    /// * `builder` - MIR builder (mutable access for instruction emission)
    /// * `plan` - LoweredRecipe from Normalizer (pre-allocated ValueIds)
    /// * `ctx` - Loop pattern context for debug/func_name
    pub(in crate::mir::builder) fn lower(
        builder: &mut MirBuilder,
        plan: LoweredRecipe,
        ctx: &LoopRouteContext,
    ) -> Result<Option<ValueId>, String> {
        let mut loop_stack = Vec::new();
        Self::lower_with_stack(builder, plan, ctx, &mut loop_stack)
    }

    /// Internal dispatcher: routes CorePlan variants to specialized modules
    pub(super) fn lower_with_stack(
        builder: &mut MirBuilder,
        plan: LoweredRecipe,
        ctx: &LoopRouteContext,
        loop_stack: &mut Vec<LoopFrame>,
    ) -> Result<Option<ValueId>, String> {
        match plan {
            CorePlan::Seq(plans) => Self::lower_seq(builder, plans, ctx, loop_stack),
            CorePlan::Loop(loop_plan) => Self::lower_loop(builder, loop_plan, ctx, loop_stack),
            CorePlan::If(if_plan) => Self::lower_if(builder, if_plan, ctx, loop_stack),
            CorePlan::BranchN(branch_plan) => Self::lower_branchn(builder, branch_plan, ctx, loop_stack),
            CorePlan::Effect(effect) => {
                if loop_stack.is_empty() {
                    Self::emit_effect(builder, &effect)?;
                } else {
                    Self::emit_effect_in_loop(builder, &effect, loop_stack)?;
                }
                Ok(None)
            }
            CorePlan::Exit(exit) => Self::lower_exit(builder, exit, loop_stack),
        }
    }
}
