//! JoinSig-owned logical transfer evidence for the V1 physical layout.
//!
//! This view copies no Recipe structure and owns no physical identifiers. It
//! only lends the already verified loop boundary edges and their predicate
//! condition relation so layout can bind them to placement exactly once.

use super::super::ids::{LoopBlockKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::model::{
    LoopJoinEdgeRoleV1, LoopJoinPayloadV1, LoopJoinPortV1, LoopJoinSigV1, VerifiedLoopJoinSigV1,
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
pub(crate) enum LoopJoinLogicalTransferRejectV1 {
    DuplicateBoundary {
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
    },
    MissingBoundary {
        loop_key: LoopNodeKeyV1,
        role: LoopJoinEdgeRoleV1,
    },
}

/// Borrowed boundary evidence issued by the verified JoinSig owner.
#[derive(Debug)]
pub(crate) struct LoopJoinLogicalTransferViewV1<'sig> {
    boundaries: Box<[LoopJoinBoundaryTransferRefV1<'sig>]>,
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
    LoopJoinLogicalTransferViewV1 { boundaries }
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
        };
        assert_eq!(
            view.require(LoopNodeKeyV1::new(0), LoopJoinEdgeRoleV1::Enter),
            Err(LoopJoinLogicalTransferRejectV1::DuplicateBoundary {
                loop_key: LoopNodeKeyV1::new(0),
                role: LoopJoinEdgeRoleV1::Enter,
            })
        );
    }
}
