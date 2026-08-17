//! Caller-zero validation of the JoinSig-owned If continuation relation.
//!
//! This module checks only source block/item placement before Layout. It does
//! not issue a physical edge, split a block, or infer a target from layout.

use std::collections::{BTreeMap, BTreeSet};

use super::common_v2_issuers::CommonV2IssuerRejectV1;
use super::join_sig::{LoopJoinBranchArmTransferRefV2, LoopJoinBranchTransferRefV2};
use super::s6c_scan_with_init_joinir_output_rows::{S6CLogicalBlockV1, S6CLogicalItemV1};

pub(crate) fn validate_continuation_relation(
    branches: &[LoopJoinBranchTransferRefV2<'_>],
    items: &[S6CLogicalItemV1],
    blocks: &[S6CLogicalBlockV1],
) -> Result<(), CommonV2IssuerRejectV1> {
    let if_blocks = items
        .iter()
        .filter_map(|item| match item {
            S6CLogicalItemV1::If { item, block, .. } => Some((*item, *block)),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let mut seen_targets = BTreeSet::new();
    for branch in branches {
        let Some(parent_block) = if_blocks.get(&branch.if_item).copied() else {
            return Err(CommonV2IssuerRejectV1::ContinuationRelation);
        };
        for arm in [branch.then_arm, branch.else_arm] {
            let LoopJoinBranchArmTransferRefV2::Fallthrough { continuation, .. } = arm else {
                continue;
            };
            if continuation.block != parent_block
                || !seen_targets.insert((continuation.block, continuation.item))
            {
                return Err(CommonV2IssuerRejectV1::ContinuationRelation);
            }
            let Some(block) = blocks.iter().find(|block| block.key == parent_block) else {
                return Err(CommonV2IssuerRejectV1::ContinuationRelation);
            };
            let Some(if_index) = block.items.iter().position(|item| *item == branch.if_item) else {
                return Err(CommonV2IssuerRejectV1::ContinuationRelation);
            };
            let Some(target_index) = block
                .items
                .iter()
                .position(|item| *item == continuation.item)
            else {
                return Err(CommonV2IssuerRejectV1::ContinuationRelation);
            };
            if target_index <= if_index {
                return Err(CommonV2IssuerRejectV1::ContinuationRelation);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
    use super::super::join_sig::LoopJoinBranchTransferRefV2;
    use super::super::join_sig::{
        LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitRefV2, LoopJoinBranchExitTargetV2,
        LoopJoinEdgeRoleV1, LoopJoinNextItemV1,
    };
    use super::super::s6c_scan_with_init_joinir_output_rows::{
        S6CLogicalBlockV1, S6CLogicalItemV1,
    };
    use super::{validate_continuation_relation, CommonV2IssuerRejectV1};

    fn test_branch(continuation: LoopJoinNextItemV1) -> LoopJoinBranchTransferRefV2<'static> {
        LoopJoinBranchTransferRefV2 {
            owner_loop: LoopNodeKeyV1::new(0),
            if_item: LoopItemKeyV1::new(8),
            condition: LoopValueKeyV1::new(0),
            then_arm: LoopJoinBranchArmTransferRefV2::Exit(LoopJoinBranchExitRefV2 {
                exit_item: LoopItemKeyV1::new(9),
                role: LoopJoinEdgeRoleV1::Return,
                target: LoopJoinBranchExitTargetV2::FunctionExit,
                payload: &[],
            }),
            else_arm: LoopJoinBranchArmTransferRefV2::Fallthrough {
                continuation,
                payload: &[],
            },
        }
    }

    fn test_continuation_rows() -> (Vec<S6CLogicalItemV1>, Vec<S6CLogicalBlockV1>) {
        (
            vec![S6CLogicalItemV1::If {
                item: LoopItemKeyV1::new(8),
                block: LoopBlockKeyV1::new(1),
                condition: LoopValueKeyV1::new(0),
                then_block: LoopBlockKeyV1::new(2),
                else_block: None,
            }],
            vec![S6CLogicalBlockV1 {
                key: LoopBlockKeyV1::new(1),
                owner_loop: LoopNodeKeyV1::new(0),
                items: Box::new([LoopItemKeyV1::new(8), LoopItemKeyV1::new(11)]),
            }],
        )
    }

    #[test]
    fn rejects_foreign_non_strict_and_duplicate_targets() {
        let (items, blocks) = test_continuation_rows();
        let foreign = test_branch(LoopJoinNextItemV1 {
            block: LoopBlockKeyV1::new(99),
            item: LoopItemKeyV1::new(11),
        });
        assert!(matches!(
            validate_continuation_relation(&[foreign], &items, &blocks),
            Err(CommonV2IssuerRejectV1::ContinuationRelation)
        ));

        let non_strict = test_branch(LoopJoinNextItemV1 {
            block: LoopBlockKeyV1::new(1),
            item: LoopItemKeyV1::new(8),
        });
        assert!(matches!(
            validate_continuation_relation(&[non_strict], &items, &blocks),
            Err(CommonV2IssuerRejectV1::ContinuationRelation)
        ));

        let valid = test_branch(LoopJoinNextItemV1 {
            block: LoopBlockKeyV1::new(1),
            item: LoopItemKeyV1::new(11),
        });
        assert!(matches!(
            validate_continuation_relation(&[valid, valid], &items, &blocks),
            Err(CommonV2IssuerRejectV1::ContinuationRelation)
        ));
    }
}
