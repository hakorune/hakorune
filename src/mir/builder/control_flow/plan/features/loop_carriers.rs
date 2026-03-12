//! Loop carrier helpers (phi info + bindings + EdgeArgs).

use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::normalizer::helpers::create_phi_bindings;
use crate::mir::builder::control_flow::plan::{CoreLoopPlan, CorePhiInfo};
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct LoopCarrierSpec {
    pub dst: ValueId,
    pub init: ValueId,
    pub next: ValueId,
    pub tag: String,
}

pub(in crate::mir::builder) fn build_phi_info(
    block: BasicBlockId,
    dst: ValueId,
    inputs: Vec<(BasicBlockId, ValueId)>,
    tag: String,
) -> CorePhiInfo {
    CorePhiInfo {
        block,
        dst,
        inputs,
        tag,
    }
}

pub(in crate::mir::builder) fn build_loop_phi_info(
    header_bb: BasicBlockId,
    preheader_bb: BasicBlockId,
    step_bb: BasicBlockId,
    dst: ValueId,
    init: ValueId,
    next: ValueId,
    tag: String,
) -> CorePhiInfo {
    build_phi_info(
        header_bb,
        dst,
        vec![(preheader_bb, init), (step_bb, next)],
        tag,
    )
}

pub(in crate::mir::builder) fn build_loop_carriers(
    header_bb: BasicBlockId,
    preheader_bb: BasicBlockId,
    step_bb: BasicBlockId,
    carriers: Vec<LoopCarrierSpec>,
) -> Vec<CorePhiInfo> {
    carriers
        .into_iter()
        .map(|carrier| {
            build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                carrier.dst,
                carrier.init,
                carrier.next,
                carrier.tag,
            )
        })
        .collect()
}

pub(in crate::mir::builder) fn with_loop_carriers(
    mut plan: CoreLoopPlan,
    carriers: Vec<CorePhiInfo>,
) -> CoreLoopPlan {
    plan.phis = carriers;
    plan
}

pub(in crate::mir::builder) fn build_loop_bindings(
    bindings: &[(&str, ValueId)],
) -> BTreeMap<String, ValueId> {
    create_phi_bindings(bindings)
}

/// Build EdgeArgs for expr-result + carrier values.
pub(in crate::mir::builder) fn build_expr_carrier_join_args(values: Vec<ValueId>) -> EdgeArgs {
    EdgeArgs {
        layout: JumpArgsLayout::ExprResultPlusCarriers,
        values,
    }
}

/// Build empty-input step join PHI.
/// Inputs will be populated later by ContinueWithPhiArgs during lowering.
pub(in crate::mir::builder) fn build_step_join_phi_info(
    step_bb: BasicBlockId,
    dst: ValueId,
    tag: String,
) -> CorePhiInfo {
    build_phi_info(step_bb, dst, vec![], tag)
}

/// Build 1-input header PHI (preheader-only).
/// Additional inputs will be populated later by ContinueWithPhiArgs during lowering.
pub(in crate::mir::builder) fn build_preheader_only_phi_info(
    header_bb: BasicBlockId,
    preheader_bb: BasicBlockId,
    dst: ValueId,
    init: ValueId,
    tag: String,
) -> CorePhiInfo {
    build_phi_info(header_bb, dst, vec![(preheader_bb, init)], tag)
}

/// Build after-block merge PHI from predecessor list.
pub(in crate::mir::builder) fn build_after_merge_phi_info(
    after_bb: BasicBlockId,
    dst: ValueId,
    preds: impl IntoIterator<Item = BasicBlockId>,
    incoming: ValueId,
    tag: String,
) -> CorePhiInfo {
    build_phi_info(
        after_bb,
        dst,
        preds.into_iter().map(|pred| (pred, incoming)).collect(),
        tag,
    )
}
