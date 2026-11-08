//! Loop carrier helpers (phi info + bindings + EdgeArgs).

use crate::mir::builder::control_flow::plan::CorePhiInfo;
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder) struct LoopPhiClosureV1 {
    phis: Vec<CorePhiInfo>,
    final_values: Vec<(String, ValueId)>,
}

impl LoopPhiClosureV1 {
    pub(in crate::mir::builder) fn new(
        phis: Vec<CorePhiInfo>,
        final_values: Vec<(String, ValueId)>,
    ) -> Self {
        Self { phis, final_values }
    }

    pub(in crate::mir::builder) fn phis(&self) -> &[CorePhiInfo] {
        &self.phis
    }

    pub(in crate::mir::builder) fn final_values(&self) -> &[(String, ValueId)] {
        &self.final_values
    }
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
        binding: None,
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
