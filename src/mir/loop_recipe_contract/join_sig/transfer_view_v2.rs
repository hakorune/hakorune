//! Borrowed logical transfer view owned by the V2 JoinSig subtree.
//!
//! The view is deliberately narrower than both Recipe and physical lowering:
//! JoinSig owns logical ports, branch dispositions, and the exact After
//! relation. Recipe block placement and physical identifiers remain outside.

use super::super::ids::{LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::super::schema_v2::LoopValueClassV2;
use super::model::{
    LoopJoinBranchArmV2, LoopJoinBranchExitTargetV2, LoopJoinEdgeRoleV1, LoopJoinNextItemV1,
    LoopJoinPayloadV2, LoopJoinPortV1,
};
use super::v2::VerifiedLoopJoinClosureV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinBoundaryTransferRefV2<'program> {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) from: LoopJoinPortV1,
    pub(crate) to: LoopJoinPortV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) payload: &'program [LoopJoinPayloadV2],
}

/// A logical exit summary retained for branch-consistency checks.
///
/// This is not an executable physical transfer. Bounded Dynamic lowering
/// must consume the matching branch arm's exact exit item instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinSummaryTransferRefV2<'program> {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) from: LoopJoinPortV1,
    pub(crate) to: LoopJoinPortV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) payload: &'program [LoopJoinPayloadV2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchExitRefV2<'program> {
    pub(crate) exit_item: LoopItemKeyV1,
    pub(crate) role: LoopJoinEdgeRoleV1,
    pub(crate) target: LoopJoinBranchExitTargetV2,
    pub(crate) payload: &'program [LoopJoinPayloadV2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopJoinBranchArmTransferRefV2<'program> {
    Exit(LoopJoinBranchExitRefV2<'program>),
    Fallthrough {
        continuation: LoopJoinNextItemV1,
        payload: &'program [LoopJoinPayloadV2],
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopJoinBranchTransferRefV2<'program> {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) if_item: LoopItemKeyV1,
    pub(crate) condition: LoopValueKeyV1,
    pub(crate) then_arm: LoopJoinBranchArmTransferRefV2<'program>,
    pub(crate) else_arm: LoopJoinBranchArmTransferRefV2<'program>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopJoinAfterRefV2<'program> {
    closure: &'program VerifiedLoopJoinClosureV2,
}

impl LoopJoinAfterRefV2<'_> {
    pub(crate) fn loop_key(self) -> LoopNodeKeyV1 {
        self.closure.after_loop_key()
    }

    pub(crate) fn binding(self) -> LoopBindingKeyV1 {
        self.closure.after_binding()
    }

    pub(crate) fn class(self) -> LoopValueClassV2 {
        self.closure.after_class()
    }
}

/// One borrowed logical transfer product. It is not Clone and has no raw
/// JoinSig/After parts API; callers receive only the exact rows below.
#[derive(Debug)]
pub(crate) struct LoopJoinLogicalTransferViewV2<'program> {
    boundaries: Box<[LoopJoinBoundaryTransferRefV2<'program>]>,
    summaries: Box<[LoopJoinSummaryTransferRefV2<'program>]>,
    branches: Box<[LoopJoinBranchTransferRefV2<'program>]>,
    after: LoopJoinAfterRefV2<'program>,
}

impl LoopJoinLogicalTransferViewV2<'_> {
    pub(crate) fn boundaries(&self) -> &[LoopJoinBoundaryTransferRefV2<'_>] {
        &self.boundaries
    }

    pub(crate) fn summary_transfers(&self) -> &[LoopJoinSummaryTransferRefV2<'_>] {
        &self.summaries
    }

    pub(crate) fn branches(&self) -> &[LoopJoinBranchTransferRefV2<'_>] {
        &self.branches
    }

    pub(crate) fn after(&self) -> LoopJoinAfterRefV2<'_> {
        self.after
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopJoinLogicalTransferRejectV2 {
    SummaryCountMismatch {
        summaries: usize,
        branch_exits: usize,
    },
    SummaryDoesNotMatchBranch,
}

pub(super) fn issue<'program>(
    closure: &'program VerifiedLoopJoinClosureV2,
    signature: &'program super::model::VerifiedLoopJoinSigV2,
) -> Result<LoopJoinLogicalTransferViewV2<'program>, LoopJoinLogicalTransferRejectV2> {
    let signature = signature.as_sig();
    let mut boundaries = Vec::new();
    let mut summaries = Vec::new();

    for row in &signature.loops {
        for edge in &row.edges {
            let transfer = LoopJoinSummaryTransferRefV2 {
                loop_key: row.key,
                from: edge.from,
                to: edge.to,
                role: edge.role,
                payload: edge.payload.as_slice(),
            };
            match edge.role {
                LoopJoinEdgeRoleV1::Enter
                | LoopJoinEdgeRoleV1::PredicateTrue
                | LoopJoinEdgeRoleV1::PredicateFalse
                | LoopJoinEdgeRoleV1::BodyEntry
                | LoopJoinEdgeRoleV1::Backedge => boundaries.push(LoopJoinBoundaryTransferRefV2 {
                    loop_key: transfer.loop_key,
                    from: transfer.from,
                    to: transfer.to,
                    role: transfer.role,
                    payload: transfer.payload,
                }),
                LoopJoinEdgeRoleV1::Break
                | LoopJoinEdgeRoleV1::Continue
                | LoopJoinEdgeRoleV1::Return => summaries.push(transfer),
            }
        }
    }

    let branches = signature
        .branches
        .iter()
        .map(|branch| LoopJoinBranchTransferRefV2 {
            owner_loop: branch.owner_loop,
            if_item: branch.if_item,
            condition: branch.condition,
            then_arm: branch_arm(&branch.then_arm),
            else_arm: branch_arm(&branch.else_arm),
        })
        .collect::<Vec<_>>();
    let branch_exits = branches
        .iter()
        .flat_map(|branch| [branch.then_arm, branch.else_arm])
        .filter_map(|arm| match arm {
            LoopJoinBranchArmTransferRefV2::Exit(exit) => Some(exit),
            LoopJoinBranchArmTransferRefV2::Fallthrough { .. } => None,
        })
        .collect::<Vec<_>>();

    if summaries.len() != branch_exits.len() {
        return Err(LoopJoinLogicalTransferRejectV2::SummaryCountMismatch {
            summaries: summaries.len(),
            branch_exits: branch_exits.len(),
        });
    }
    let mut used = vec![false; branch_exits.len()];
    for summary in &summaries {
        let Some(index) = branches
            .iter()
            .flat_map(|branch| {
                [branch.then_arm, branch.else_arm]
                    .into_iter()
                    .filter_map(move |arm| match arm {
                        LoopJoinBranchArmTransferRefV2::Exit(exit) => {
                            Some((branch.owner_loop, exit))
                        }
                        LoopJoinBranchArmTransferRefV2::Fallthrough { .. } => None,
                    })
            })
            .enumerate()
            .position(|(index, (owner_loop, exit))| {
                !used[index]
                    && summary.loop_key == owner_loop
                    && summary.role == exit.role
                    && summary.payload == exit.payload
                    && summary.to == summary_port(exit.role)
                    && summary.from == LoopJoinPortV1::Body
            })
        else {
            return Err(LoopJoinLogicalTransferRejectV2::SummaryDoesNotMatchBranch);
        };
        used[index] = true;
    }

    Ok(LoopJoinLogicalTransferViewV2 {
        boundaries: boundaries.into_boxed_slice(),
        summaries: summaries.into_boxed_slice(),
        branches: branches.into_boxed_slice(),
        after: LoopJoinAfterRefV2 { closure },
    })
}

fn branch_arm(arm: &LoopJoinBranchArmV2) -> LoopJoinBranchArmTransferRefV2<'_> {
    match arm {
        LoopJoinBranchArmV2::Exit(exit) => {
            LoopJoinBranchArmTransferRefV2::Exit(LoopJoinBranchExitRefV2 {
                exit_item: exit.exit_item,
                role: exit.role,
                target: exit.target,
                payload: exit.payload.as_slice(),
            })
        }
        LoopJoinBranchArmV2::Fallthrough {
            continuation,
            payload,
        } => LoopJoinBranchArmTransferRefV2::Fallthrough {
            continuation: *continuation,
            payload: payload.as_slice(),
        },
    }
}

const fn summary_port(role: LoopJoinEdgeRoleV1) -> LoopJoinPortV1 {
    match role {
        LoopJoinEdgeRoleV1::Break => LoopJoinPortV1::After,
        LoopJoinEdgeRoleV1::Continue => LoopJoinPortV1::Header,
        LoopJoinEdgeRoleV1::Return => LoopJoinPortV1::FunctionExit,
        LoopJoinEdgeRoleV1::Enter
        | LoopJoinEdgeRoleV1::PredicateTrue
        | LoopJoinEdgeRoleV1::PredicateFalse
        | LoopJoinEdgeRoleV1::BodyEntry
        | LoopJoinEdgeRoleV1::Backedge => LoopJoinPortV1::FunctionExit,
    }
}
