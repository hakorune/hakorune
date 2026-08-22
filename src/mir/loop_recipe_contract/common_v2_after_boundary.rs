//! Source-backed After-boundary relation for common V2.
//!
//! This is a transport-only semantic receipt.  It names whether the admitted
//! boundary is a root After or a future parent resume; it never allocates a
//! block or chooses a physical successor.

use super::common_v2_layout_input::PreparedLoopV2PhysicalLayoutInputV1;
use super::ids::LoopNodeKeyV1;
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SourceStmtSiteV1,
};

impl<'a, 'rows, 'facts> S6CPrephysicalIngressRefV2<'a, 'rows, 'facts> {
    pub(crate) fn source_loop_membership(
        self,
    ) -> &'facts crate::mir::resolved_semantics::VerifiedCallableLoopMembershipV1 {
        self.source.facts().source().calls().typed().membership()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopV2AfterBoundaryRelationV1 {
    RootAfter,
    ParentResume { parent_loop: LoopNodeKeyV1 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AfterBoundaryIssueRejectV1 {
    ForeignOwner,
    MissingRoot,
    RootHasParent,
    AfterLoopMismatch,
}

/// One source/frame-branded After relation.  The relation is non-Clone so it
/// cannot be detached and re-paired with another common-V2 envelope.
#[derive(Debug)]
pub(crate) struct VerifiedLoopV2AfterBoundarySourceRelationV1 {
    owner: FunctionOwnerIdV1,
    loop_key: LoopNodeKeyV1,
    source_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    relation: LoopV2AfterBoundaryRelationV1,
}

impl VerifiedLoopV2AfterBoundarySourceRelationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn loop_key(&self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) fn source_site(&self) -> &SourceStmtSiteV1 {
        &self.source_site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) const fn relation(&self) -> LoopV2AfterBoundaryRelationV1 {
        self.relation
    }
}

pub(crate) fn issue_s6c_v2_after_boundary_source_relation_v1<'rows, 'facts>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'rows, 'facts>,
    layout: &PreparedLoopV2PhysicalLayoutInputV1<'rows>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<VerifiedLoopV2AfterBoundarySourceRelationV1, AfterBoundaryIssueRejectV1> {
    if ingress.source_owner() != expected_owner {
        return Err(AfterBoundaryIssueRejectV1::ForeignOwner);
    }
    let roots = layout
        .loops()
        .iter()
        .filter(|row| row.parent().is_none())
        .collect::<Vec<_>>();
    let Some(root) = (roots.len() == 1).then(|| roots[0]) else {
        return Err(AfterBoundaryIssueRejectV1::MissingRoot);
    };
    if root.parent().is_some() {
        return Err(AfterBoundaryIssueRejectV1::RootHasParent);
    }
    let after = layout.after();
    if after.0 != root.key() {
        return Err(AfterBoundaryIssueRejectV1::AfterLoopMismatch);
    }
    let membership = ingress.source_loop_membership();
    Ok(VerifiedLoopV2AfterBoundarySourceRelationV1 {
        owner: expected_owner,
        loop_key: root.key(),
        source_site: membership.source().site().clone(),
        frame: membership.frame().clone(),
        relation: LoopV2AfterBoundaryRelationV1::RootAfter,
    })
}
