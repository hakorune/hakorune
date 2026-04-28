//! Loop carrier helpers (phi info + bindings + EdgeArgs).

use crate::mir::builder::control_flow::plan::normalizer::helpers::create_phi_bindings;
use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

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

pub(in crate::mir::builder) fn build_loop_bindings(
    bindings: &[(&str, ValueId)],
) -> BTreeMap<String, ValueId> {
    create_phi_bindings(bindings)
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
