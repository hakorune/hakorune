//! JoinSig-owned logical transfer evidence for the V1 physical layout.
//!
//! This view copies no Recipe structure and owns no physical identifiers. It
//! only lends the already verified loop boundary edges and their predicate
//! condition relation so layout can bind them to placement exactly once.

use super::super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::model::{
    LoopJoinBranchArmV1, LoopJoinEdgeRoleV1, LoopJoinPayloadV1, LoopJoinPortV1, LoopJoinSigV1,
    VerifiedLoopJoinSigV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinBoundaryTransferRefV1<'sig> {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) from: LoopJoinPortV1,
    pub(crate) to: LoopJoinPortV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) condition: Option<(LoopBlockKeyV1, LoopValueKeyV1)>,
    pub(crate) payload: &'sig [LoopJoinPayloadV1],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchExitRefV1<'sig> {
    pub(crate) exit_item: LoopItemKeyV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) target_loop: LoopNodeKeyV1,
    pub(crate) payload: &'sig [LoopJoinPayloadV1],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopJoinBranchArmTransferRefV1<'sig> {
    Exit(LoopJoinBranchExitRefV1<'sig>),
    Fallthrough {
        continuation: super::model::LoopJoinNextItemV1,
        payload: &'sig [LoopJoinPayloadV1],
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchTransferRefV1<'sig> {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_arm: LoopJoinBranchArmTransferRefV1<'sig>,
    pub(crate) else_arm: LoopJoinBranchArmTransferRefV1<'sig>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopJoinLogicalTransferRejectV1 {
    DuplicateBoundary {
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
    },
    MissingBoundary {
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
    },
    DuplicateBranch {
        loop_key: LoopNodeKeyV1,
        if_item: LoopItemKeyV1,
    },
    MissingBranch {
        loop_key: LoopNodeKeyV1,
        if_item: LoopItemKeyV1,
    },
}

/// Borrowed boundary evidence issued by the verified JoinSig owner.
#[derive(Debug)]
pub(crate) struct LoopJoinLogicalTransferViewV1<'sig> {
    boundaries: Box<[LoopJoinBoundaryTransferRefV1<'sig>]>,
    branches: Box<[LoopJoinBranchTransferRefV1<'sig>]>,
}

impl LoopJoinLogicalTransferViewV1<'_> {
    pub(crate) fn require(
        &self,
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
    ) -> Result<LoopJoinBoundaryTransferRefV1<'_>, LoopJoinLogicalTransferRejectV1> {
        let mut found = None;
        for row in self
            .boundaries
            .iter()
            .filter(|row| row.loop_key == loop_key && row.role == role)
        {
            if found.is_some() {
                return Err(LoopJoinLogicalTransferRejectV1::DuplicateBoundary { loop_key, role });
            }
            found = Some(*row);
        }
        found.ok_or(LoopJoinLogicalTransferRejectV1::MissingBoundary { loop_key, role })
    }

    pub(crate) fn require_branch(
        &self,
        loop_key: LoopNodeKeyV1,
        if_item: LoopItemKeyV1,
    ) -> Result<LoopJoinBranchTransferRefV1<'_>, LoopJoinLogicalTransferRejectV1> {
        let mut found = None;
        for row in self
            .branches
            .iter()
            .filter(|row| row.owner_loop == loop_key && row.if_item == if_item)
        {
            if found.is_some() {
                return Err(LoopJoinLogicalTransferRejectV1::DuplicateBranch { loop_key, if_item });
            }
            found = Some(*row);
        }
        found.ok_or(LoopJoinLogicalTransferRejectV1::MissingBranch { loop_key, if_item })
    }

    pub(crate) fn branches(&self) -> &[LoopJoinBranchTransferRefV1<'_>] {
        &self.branches
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        boundaries: Vec<LoopJoinBoundaryTransferRefV1<'static>>,
        branches: Vec<LoopJoinBranchTransferRefV1<'static>>,
    ) -> Self {
        Self {
            boundaries: boundaries.into_boxed_slice(),
            branches: branches.into_boxed_slice(),
        }
    }
}

pub(super) fn issue(signature: &VerifiedLoopJoinSigV1) -> LoopJoinLogicalTransferViewV1<'_> {
    let signature: &LoopJoinSigV1 = signature.as_sig();
    let boundaries = signature
        .loops
        .iter()
        .flat_map(|row| {
            row.edges.iter().filter_map(|edge| {
                matches!(
                    edge.role,
                    LoopJoinEdgeRoleV1::Enter
                        | LoopJoinEdgeRoleV1::PredicateTrue
                        | LoopJoinEdgeRoleV1::PredicateFalse
                        | LoopJoinEdgeRoleV1::BodyEntry
                        | LoopJoinEdgeRoleV1::Backedge
                )
                .then_some(LoopJoinBoundaryTransferRefV1 {
                    loop_key: row.key,
                    from: edge.from,
                    to: edge.to,
                    role: edge.role,
                    condition: row.condition,
                    payload: edge.payload.as_slice(),
                })
            })
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let branches = signature
        .branches
        .iter()
        .map(|branch| LoopJoinBranchTransferRefV1 {
            owner_loop: branch.owner_loop,
            if_item: branch.if_item,
            condition: branch.condition,
            then_arm: branch_arm(&branch.then_arm),
            else_arm: branch_arm(&branch.else_arm),
        })
        .collect::<Vec<_>>();
    LoopJoinLogicalTransferViewV1 {
        boundaries,
        branches: branches.into_boxed_slice(),
    }
}

fn branch_arm(arm: &LoopJoinBranchArmV1) -> LoopJoinBranchArmTransferRefV1<'_> {
    match arm {
        LoopJoinBranchArmV1::Exit(exit) => {
            LoopJoinBranchArmTransferRefV1::Exit(LoopJoinBranchExitRefV1 {
                exit_item: exit.exit_item,
                role: exit.role,
                target_loop: exit.target,
                payload: exit.payload.as_slice(),
            })
        }
        LoopJoinBranchArmV1::Fallthrough {
            continuation,
            payload,
        } => LoopJoinBranchArmTransferRefV1::Fallthrough {
            continuation: *continuation,
            payload: payload.as_slice(),
        },
    }
}

impl VerifiedLoopJoinSigV1 {
    pub(crate) fn logical_transfer_view(&self) -> LoopJoinLogicalTransferViewV1<'_> {
        issue(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boundary(loop_key: LoopNodeKeyV1) -> LoopJoinBoundaryTransferRefV1<'static> {
        LoopJoinBoundaryTransferRefV1 {
            loop_key,
            from: LoopJoinPortV1::Preheader,
            to: LoopJoinPortV1::Header,
            role: LoopJoinEdgeRoleV1::Enter,
            condition: None,
            payload: &[],
        }
    }

    #[test]
    fn logical_transfer_view_rejects_missing_and_foreign_rows() {
        let view = LoopJoinLogicalTransferViewV1 {
            boundaries: vec![boundary(LoopNodeKeyV1::new(1))].into_boxed_slice(),
            branches: Box::new([]),
        };
        assert_eq!(
            view.require(LoopNodeKeyV1::new(0), LoopJoinEdgeRoleV1::Enter),
            Err(LoopJoinLogicalTransferRejectV1::MissingBoundary {
                loop_key: LoopNodeKeyV1::new(0),
                role: LoopJoinEdgeRoleV1::Enter,
            })
        );
        assert_eq!(
            view.require(LoopNodeKeyV1::new(1), LoopJoinEdgeRoleV1::Backedge),
            Err(LoopJoinLogicalTransferRejectV1::MissingBoundary {
                loop_key: LoopNodeKeyV1::new(1),
                role: LoopJoinEdgeRoleV1::Backedge,
            })
        );
    }

    #[test]
    fn logical_transfer_view_rejects_duplicate_rows_without_repair() {
        let row = boundary(LoopNodeKeyV1::new(0));
        let view = LoopJoinLogicalTransferViewV1 {
            boundaries: vec![row, row].into_boxed_slice(),
            branches: Box::new([]),
        };
        assert_eq!(
            view.require(LoopNodeKeyV1::new(0), LoopJoinEdgeRoleV1::Enter),
            Err(LoopJoinLogicalTransferRejectV1::DuplicateBoundary {
                loop_key: LoopNodeKeyV1::new(0),
                role: LoopJoinEdgeRoleV1::Enter,
            })
        );
    }

    fn branch(
        loop_key: LoopNodeKeyV1,
        if_item: LoopItemKeyV1,
    ) -> LoopJoinBranchTransferRefV1<'static> {
        LoopJoinBranchTransferRefV1 {
            owner_loop: loop_key,
            if_item,
            condition: LoopValueKeyV1::new(7),
            then_arm: LoopJoinBranchArmTransferRefV1::Exit(LoopJoinBranchExitRefV1 {
                exit_item: LoopItemKeyV1::new(8),
                role: LoopJoinEdgeRoleV1::Continue,
                target_loop: loop_key,
                payload: &[],
            }),
            else_arm: LoopJoinBranchArmTransferRefV1::Fallthrough {
                continuation: super::super::model::LoopJoinNextItemV1 {
                    block: LoopBlockKeyV1::new(9),
                    item: LoopItemKeyV1::new(10),
                },
                payload: &[],
            },
        }
    }

    #[test]
    fn logical_transfer_view_returns_the_exact_join_sig_branch() {
        let row = branch(LoopNodeKeyV1::new(2), LoopItemKeyV1::new(3));
        let view = LoopJoinLogicalTransferViewV1 {
            boundaries: Box::new([]),
            branches: vec![row].into_boxed_slice(),
        };
        let found = view
            .require_branch(LoopNodeKeyV1::new(2), LoopItemKeyV1::new(3))
            .expect("exact branch");
        assert_eq!(found, row);
        assert_eq!(
            view.require_branch(LoopNodeKeyV1::new(2), LoopItemKeyV1::new(4)),
            Err(LoopJoinLogicalTransferRejectV1::MissingBranch {
                loop_key: LoopNodeKeyV1::new(2),
                if_item: LoopItemKeyV1::new(4),
            })
        );
    }

    #[test]
    fn logical_transfer_view_rejects_duplicate_branch_consumption() {
        let row = branch(LoopNodeKeyV1::new(2), LoopItemKeyV1::new(3));
        let view = LoopJoinLogicalTransferViewV1 {
            boundaries: Box::new([]),
            branches: vec![row, row].into_boxed_slice(),
        };
        assert_eq!(
            view.require_branch(LoopNodeKeyV1::new(2), LoopItemKeyV1::new(3)),
            Err(LoopJoinLogicalTransferRejectV1::DuplicateBranch {
                loop_key: LoopNodeKeyV1::new(2),
                if_item: LoopItemKeyV1::new(3),
            })
        );
    }
}
