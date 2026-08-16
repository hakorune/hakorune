//! Source-backed, physical-ID-free predicate branch plan for common V2.
//!
//! The plan joins the resolver loop condition with both logical predicate
//! transfers.  It is intentionally transport-only: no `ValueId`, block id,
//! CFG mutation, or operation receipt is issued here.

use super::common_v2_after_boundary::{
    LoopV2AfterBoundaryRelationV1, VerifiedLoopV2AfterBoundarySourceRelationV1,
};
use super::common_v2_layout_input::PreparedLoopV2PhysicalLayoutInputV1;
use super::join_sig::{
    LoopJoinBoundaryTransferRefV2, LoopJoinEdgeRoleV1, LoopJoinLogicalTransferViewV2,
    LoopJoinPortV1,
};
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use super::schema_v2::LoopValueClassV2;
use crate::mir::loop_recipe_contract::ids::{LoopBlockKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PredicateBranchPlanRejectV1 {
    ForeignOwner,
    MissingLoop,
    RootHasParent,
    ConditionValueMissing,
    ConditionClassMismatch,
    LayoutLoopMismatch,
    ConditionSegmentMissing,
    BodySegmentMissing,
    AfterRelationMismatch,
    MissingPredicateTrue,
    MissingPredicateFalse,
    DuplicatePredicateTrue,
    DuplicatePredicateFalse,
    PredicateTruePortMismatch,
    PredicateFalsePortMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLoopV2ConditionCarrierRequirementV1 {
    owner: FunctionOwnerIdV1,
    loop_key: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    value: LoopValueKeyV1,
    class: LoopValueClassV2,
}

impl PreparedLoopV2ConditionCarrierRequirementV1 {
    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn loop_key(self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn value(self) -> LoopValueKeyV1 {
        self.value
    }

    pub(crate) const fn class(self) -> LoopValueClassV2 {
        self.class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreparedLoopV2PredicateFalseTargetV1 {
    RootAfter,
}

/// Complete logical branch shape consumed by a later physical edge slice.
/// The condition is a requirement, not a physical value receipt.
#[derive(Debug)]
pub(crate) struct PreparedLoopV2PredicateBranchPlanV1 {
    owner: FunctionOwnerIdV1,
    loop_key: LoopNodeKeyV1,
    condition: PreparedLoopV2ConditionCarrierRequirementV1,
    true_target: LoopBlockKeyV1,
    false_target: PreparedLoopV2PredicateFalseTargetV1,
}

impl PreparedLoopV2PredicateBranchPlanV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn loop_key(&self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) const fn condition(&self) -> PreparedLoopV2ConditionCarrierRequirementV1 {
        self.condition
    }

    pub(crate) const fn true_target(&self) -> LoopBlockKeyV1 {
        self.true_target
    }

    pub(crate) const fn false_target(&self) -> PreparedLoopV2PredicateFalseTargetV1 {
        self.false_target
    }
}

pub(crate) fn issue_s6c_v2_predicate_branch_plan_v1<'a, 'rows, 'facts>(
    ingress: S6CPrephysicalIngressRefV2<'a, 'rows, 'facts>,
    layout: &PreparedLoopV2PhysicalLayoutInputV1<'rows>,
    transfer: &LoopJoinLogicalTransferViewV2<'facts>,
    after: &VerifiedLoopV2AfterBoundarySourceRelationV1,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2PredicateBranchPlanV1, PredicateBranchPlanRejectV1> {
    if ingress.source_owner() != expected_owner || after.owner() != expected_owner {
        return Err(PredicateBranchPlanRejectV1::ForeignOwner);
    }
    if after.relation() != LoopV2AfterBoundaryRelationV1::RootAfter {
        return Err(PredicateBranchPlanRejectV1::AfterRelationMismatch);
    }

    let loops = ingress.logical_loops();
    let Some(root) = loops.iter().find(|row| row.key == after.loop_key()) else {
        return Err(PredicateBranchPlanRejectV1::MissingLoop);
    };
    if root.parent.is_some() {
        return Err(PredicateBranchPlanRejectV1::RootHasParent);
    }

    let class = ingress
        .source
        .logical()
        .rows()
        .values()
        .iter()
        .find(|value| value.key == root.condition_value)
        .map(|value| value.class)
        .ok_or(PredicateBranchPlanRejectV1::ConditionValueMissing)?;
    if class != LoopValueClassV2::Bool {
        return Err(PredicateBranchPlanRejectV1::ConditionClassMismatch);
    }

    let Some(layout_loop) = layout.loops().iter().find(|row| row.key() == root.key) else {
        return Err(PredicateBranchPlanRejectV1::LayoutLoopMismatch);
    };
    if layout_loop.condition_block() != root.condition_block || layout_loop.body() != root.body {
        return Err(PredicateBranchPlanRejectV1::LayoutLoopMismatch);
    }
    let Some(condition_segment) = layout.segment_for_block(root.condition_block) else {
        return Err(PredicateBranchPlanRejectV1::ConditionSegmentMissing);
    };
    let Some(body_segment) = layout.segment_for_block(root.body) else {
        return Err(PredicateBranchPlanRejectV1::BodySegmentMissing);
    };
    if condition_segment.loop_key() != root.key || body_segment.loop_key() != root.key {
        return Err(PredicateBranchPlanRejectV1::LayoutLoopMismatch);
    }

    collect_predicate_boundaries(transfer.boundaries(), root.key)?;

    Ok(PreparedLoopV2PredicateBranchPlanV1 {
        owner: expected_owner,
        loop_key: root.key,
        condition: PreparedLoopV2ConditionCarrierRequirementV1 {
            owner: expected_owner,
            loop_key: root.key,
            block: root.condition_block,
            value: root.condition_value,
            class,
        },
        true_target: root.body,
        false_target: PreparedLoopV2PredicateFalseTargetV1::RootAfter,
    })
}

fn collect_predicate_boundaries(
    boundaries: &[LoopJoinBoundaryTransferRefV2<'_>],
    loop_key: LoopNodeKeyV1,
) -> Result<(), PredicateBranchPlanRejectV1> {
    let mut true_seen = false;
    let mut false_seen = false;
    for row in boundaries.iter().filter(|row| row.loop_key == loop_key) {
        match row.role {
            LoopJoinEdgeRoleV1::PredicateTrue => {
                if row.from != LoopJoinPortV1::Header || row.to != LoopJoinPortV1::Body {
                    return Err(PredicateBranchPlanRejectV1::PredicateTruePortMismatch);
                }
                if true_seen {
                    return Err(PredicateBranchPlanRejectV1::DuplicatePredicateTrue);
                }
                true_seen = true;
            }
            LoopJoinEdgeRoleV1::PredicateFalse => {
                if row.from != LoopJoinPortV1::Header || row.to != LoopJoinPortV1::After {
                    return Err(PredicateBranchPlanRejectV1::PredicateFalsePortMismatch);
                }
                if false_seen {
                    return Err(PredicateBranchPlanRejectV1::DuplicatePredicateFalse);
                }
                false_seen = true;
            }
            _ => {}
        }
    }
    if !true_seen {
        return Err(PredicateBranchPlanRejectV1::MissingPredicateTrue);
    }
    if !false_seen {
        return Err(PredicateBranchPlanRejectV1::MissingPredicateFalse);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::loop_recipe_contract::join_sig::LoopJoinPayloadV2;

    fn row(role: LoopJoinEdgeRoleV1, to: LoopJoinPortV1) -> LoopJoinBoundaryTransferRefV2<'static> {
        LoopJoinBoundaryTransferRefV2 {
            loop_key: LoopNodeKeyV1::new(1),
            from: LoopJoinPortV1::Header,
            to,
            role,
            payload: &[] as &[LoopJoinPayloadV2],
        }
    }

    #[test]
    fn predicate_boundary_requires_both_successors() {
        let rows = [row(LoopJoinEdgeRoleV1::PredicateTrue, LoopJoinPortV1::Body)];
        assert_eq!(
            collect_predicate_boundaries(&rows, LoopNodeKeyV1::new(1)),
            Err(PredicateBranchPlanRejectV1::MissingPredicateFalse)
        );
    }

    #[test]
    fn predicate_boundary_rejects_duplicate_false_row() {
        let rows = [
            row(LoopJoinEdgeRoleV1::PredicateTrue, LoopJoinPortV1::Body),
            row(LoopJoinEdgeRoleV1::PredicateFalse, LoopJoinPortV1::After),
            row(LoopJoinEdgeRoleV1::PredicateFalse, LoopJoinPortV1::After),
        ];
        assert_eq!(
            collect_predicate_boundaries(&rows, LoopNodeKeyV1::new(1)),
            Err(PredicateBranchPlanRejectV1::DuplicatePredicateFalse)
        );
    }
}
