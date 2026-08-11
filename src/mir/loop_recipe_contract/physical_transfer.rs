//! Placement binder for JoinSig-owned logical Loop transfers.
//!
//! This is the only owner allowed to turn logical transfer evidence into the
//! private physical-layout transfer enum. It receives already verified
//! JoinSig rows and placement targets; it never reads Recipe conditions or
//! invents a fallback transfer.

use super::ids::LoopNodeKeyV1;
use super::join_sig::{LoopJoinBoundaryTransferRefV1, LoopJoinEdgeRoleV1, LoopJoinPortV1};
use super::physical_layout::{LoopPhysicalTargetV1, LoopPhysicalTransferV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalTransferBindingRejectV1 {
    PortMismatch {
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
        expected_from: LoopJoinPortV1,
        expected_to: LoopJoinPortV1,
        found_from: LoopJoinPortV1,
        found_to: LoopJoinPortV1,
    },
    RoleMismatch {
        loop_key: LoopNodeKeyV1,
        expected: LoopJoinEdgeRoleV1,
        found: LoopJoinEdgeRoleV1,
    },
    LoopMismatch {
        expected: LoopNodeKeyV1,
        found: LoopNodeKeyV1,
    },
    ConditionMismatch {
        loop_key: LoopNodeKeyV1,
    },
}

pub(super) fn bind_predicate(
    on_true: LoopJoinBoundaryTransferRefV1<'_>,
    on_false: LoopJoinBoundaryTransferRefV1<'_>,
    true_target: super::physical_layout::LoopPhysicalSegmentKeyV1,
    false_target: LoopPhysicalTargetV1,
) -> Result<LoopPhysicalTransferV1, LoopPhysicalTransferBindingRejectV1> {
    require_role(&on_true, LoopJoinEdgeRoleV1::PredicateTrue)?;
    require_role(&on_false, LoopJoinEdgeRoleV1::PredicateFalse)?;
    require_port(&on_true, LoopJoinPortV1::Header, LoopJoinPortV1::Body)?;
    require_port(&on_false, LoopJoinPortV1::Header, LoopJoinPortV1::After)?;
    if on_true.loop_key != on_false.loop_key {
        return Err(LoopPhysicalTransferBindingRejectV1::LoopMismatch {
            expected: on_true.loop_key,
            found: on_false.loop_key,
        });
    }
    if on_true.condition != on_false.condition {
        return Err(LoopPhysicalTransferBindingRejectV1::ConditionMismatch {
            loop_key: on_true.loop_key,
        });
    }
    let Some((_, condition)) = on_true.condition else {
        return Err(LoopPhysicalTransferBindingRejectV1::ConditionMismatch {
            loop_key: on_true.loop_key,
        });
    };
    Ok(LoopPhysicalTransferV1::Predicate {
        condition,
        on_true: true_target,
        on_false: false_target,
    })
}

pub(super) fn bind_backedge(
    edge: LoopJoinBoundaryTransferRefV1<'_>,
    target: LoopPhysicalTargetV1,
) -> Result<LoopPhysicalTransferV1, LoopPhysicalTransferBindingRejectV1> {
    require_role(&edge, LoopJoinEdgeRoleV1::Backedge)?;
    require_port(&edge, LoopJoinPortV1::Body, LoopJoinPortV1::Header)?;
    Ok(LoopPhysicalTransferV1::Jump { target })
}

pub(super) fn bind_nested_loop(
    edge: LoopJoinBoundaryTransferRefV1<'_>,
    loop_key: LoopNodeKeyV1,
    entry: super::physical_layout::LoopPhysicalSegmentKeyV1,
) -> Result<LoopPhysicalTransferV1, LoopPhysicalTransferBindingRejectV1> {
    require_role(&edge, LoopJoinEdgeRoleV1::Enter)?;
    require_port(&edge, LoopJoinPortV1::Preheader, LoopJoinPortV1::Header)?;
    if edge.loop_key != loop_key {
        return Err(LoopPhysicalTransferBindingRejectV1::LoopMismatch {
            expected: loop_key,
            found: edge.loop_key,
        });
    }
    Ok(LoopPhysicalTransferV1::OpenNestedLoop { loop_key, entry })
}

fn require_role(
    edge: &LoopJoinBoundaryTransferRefV1<'_>,
    role: LoopJoinEdgeRoleV1,
) -> Result<(), LoopPhysicalTransferBindingRejectV1> {
    if edge.role == role {
        Ok(())
    } else {
        Err(LoopPhysicalTransferBindingRejectV1::RoleMismatch {
            loop_key: edge.loop_key,
            expected: role,
            found: edge.role,
        })
    }
}

fn require_port(
    edge: &LoopJoinBoundaryTransferRefV1<'_>,
    expected_from: LoopJoinPortV1,
    expected_to: LoopJoinPortV1,
) -> Result<(), LoopPhysicalTransferBindingRejectV1> {
    if edge.from == expected_from && edge.to == expected_to {
        return Ok(());
    }
    Err(LoopPhysicalTransferBindingRejectV1::PortMismatch {
        loop_key: edge.loop_key,
        role: edge.role,
        expected_from,
        expected_to,
        found_from: edge.from,
        found_to: edge.to,
    })
}
