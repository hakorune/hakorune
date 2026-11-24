//! Exact LoopCond Phi completion at the sole physical writer boundary.
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirInstruction};
use std::collections::BTreeMap;
use std::rc::Rc;

/// Only the selected source entry can issue this exact completed physical role.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopCondAfterPhysicalRoleV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    block: BasicBlockId,
    proof: Rc<crate::mir::normal_callable_semantic_package::SourceLoopPhysicalProofV2>,
}
impl VerifiedLoopCondAfterPhysicalRoleV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 { &self.site }
    pub(in crate::mir) const fn block(&self) -> BasicBlockId { self.block }
    pub(in crate::mir) fn loop_phis(&self) -> &BTreeMap<BasicBlockId, Box<[MirInstruction]>> {
        self.proof.points().phis()
    }
    pub(in crate::mir) fn source_proof_v2(&self) -> &Rc<crate::mir::normal_callable_semantic_package::SourceLoopPhysicalProofV2> {
        &self.proof
    }
    pub(in crate::mir) fn from_source_v2(proof: Rc<crate::mir::normal_callable_semantic_package::SourceLoopPhysicalProofV2>) -> Self {
        Self { owner: proof.product().source.source.owner(), site: proof.product().source.source.site().clone(),
            block: proof.after(), proof }
    }
}

pub(super) fn freeze(reason: &str) -> String { format!("[freeze:contract][loop-cond-after-role/{reason}]") }
