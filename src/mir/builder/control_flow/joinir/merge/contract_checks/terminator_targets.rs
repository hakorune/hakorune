use crate::mir::{MirFunction, MirInstruction};

use super::super::merge_result::MergeContracts;

/// Contract check (Fail-Fast): every Branch/Jump target must exist in the function.
///
/// This prevents latent runtime failures like:
/// - "Invalid basic block: bb <id> not found"
///
/// Typical root causes:
/// - Jumping to a continuation function entry block whose blocks were intentionally skipped
/// - Allocating a block ID but forgetting to insert the block
///
/// Phase 131 P1 Task 1: Now accepts MergeContracts instead of raw slice for SSOT visibility.
pub(in crate::mir::builder::control_flow::joinir::merge) fn verify_all_terminator_targets_exist(
    func: &MirFunction,
    contracts: &MergeContracts,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::error_tags;

    for (block_id, block) in &func.blocks {
        let Some(term) = &block.terminator else { continue };

        match term {
            MirInstruction::Jump { target, .. } => {
                if !func.blocks.contains_key(target)
                    && !contracts.allowed_missing_jump_targets.contains(target)
                {
                    return Err(error_tags::freeze_with_hint(
                        "joinir/merge/contract/missing_jump_target",
                        &format!(
                            "Jump target {:?} not found in function '{}' (from block {:?})",
                            target, func.signature.name, block_id
                        ),
                        "ensure merge inserts all remapped blocks and does not Jump to skipped continuation blocks (k_exit must Jump to exit_block_id)",
                    ));
                }
            }
            MirInstruction::Branch {
                then_bb, else_bb, ..
            } => {
                if !func.blocks.contains_key(then_bb)
                    && !contracts.allowed_missing_jump_targets.contains(then_bb)
                {
                    return Err(error_tags::freeze_with_hint(
                        "joinir/merge/contract/missing_branch_target",
                        &format!(
                            "Branch then_bb {:?} not found in function '{}' (from block {:?})",
                            then_bb, func.signature.name, block_id
                        ),
                        "ensure all remapped blocks are inserted and Branch targets are block-remapped consistently",
                    ));
                }
                if !func.blocks.contains_key(else_bb)
                    && !contracts.allowed_missing_jump_targets.contains(else_bb)
                {
                    return Err(error_tags::freeze_with_hint(
                        "joinir/merge/contract/missing_branch_target",
                        &format!(
                            "Branch else_bb {:?} not found in function '{}' (from block {:?})",
                            else_bb, func.signature.name, block_id
                        ),
                        "ensure all remapped blocks are inserted and Branch targets are block-remapped consistently",
                    ));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
    };

    /// Helper: Create a minimal test function with given blocks
    fn create_test_function(blocks: Vec<(BasicBlockId, Option<MirInstruction>)>) -> MirFunction {
        use crate::mir::EffectMask;

        let entry_block = blocks.first().map(|(id, _)| *id).unwrap_or(BasicBlockId(0));

        let mut func = MirFunction::new(
            FunctionSignature {
                name: "test_func".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::default(),
            },
            entry_block,
        );

        // Remove the entry block that was auto-created
        func.blocks.clear();

        for (block_id, terminator) in blocks {
            let mut block = BasicBlock::new(block_id);
            block.terminator = terminator;
            func.add_block(block);
        }

        func
    }

    #[test]
    fn test_verify_all_terminator_targets_exist_all_present() {
        // case 1: すべてのターゲットが存在 → OK
        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);

        let func = create_test_function(vec![
            (
                bb0,
                Some(MirInstruction::Jump {
                    target: bb1,
                    edge_args: None,
                }),
            ),
            (
                bb1,
                Some(MirInstruction::Branch {
                    condition: ValueId(0),
                    then_bb: bb2,
                    else_bb: bb2,
                    then_edge_args: None,
                    else_edge_args: None,
                }),
            ),
            (bb2, Some(MirInstruction::Return { value: None })),
        ]);

        let contracts = MergeContracts {
            allowed_missing_jump_targets: vec![],
        };

        let result = verify_all_terminator_targets_exist(&func, &contracts);
        assert!(result.is_ok(), "All targets exist, should pass");
    }

    #[test]
    fn test_verify_all_terminator_targets_exist_missing_disallowed() {
        // case 2: 許可ターゲット以外が missing → FAIL
        let bb0 = BasicBlockId(0);
        let bb99 = BasicBlockId(99); // Missing block

        let func = create_test_function(vec![(
            bb0,
            Some(MirInstruction::Jump {
                target: bb99,
                edge_args: None,
            }),
        )]);

        let contracts = MergeContracts {
            allowed_missing_jump_targets: vec![],
        };

        let result = verify_all_terminator_targets_exist(&func, &contracts);
        assert!(result.is_err(), "Missing disallowed target should fail");
        assert!(
            result.unwrap_err().contains("Jump target"),
            "Error should mention Jump target"
        );
    }

    #[test]
    fn test_verify_all_terminator_targets_exist_missing_allowed() {
        // case 3: 許可ターゲットが missing → OK（許可）
        let bb0 = BasicBlockId(0);
        let bb_exit = BasicBlockId(100); // Missing but allowed

        let func = create_test_function(vec![(
            bb0,
            Some(MirInstruction::Jump {
                target: bb_exit,
                edge_args: None,
            }),
        )]);

        let contracts = MergeContracts {
            allowed_missing_jump_targets: vec![bb_exit],
        };

        let result = verify_all_terminator_targets_exist(&func, &contracts);
        assert!(
            result.is_ok(),
            "Missing allowed target should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_merge_contracts_creation() {
        // MergeContracts の生成と値確認
        let exit_block = BasicBlockId(42);
        let contracts = MergeContracts {
            allowed_missing_jump_targets: vec![exit_block],
        };

        assert_eq!(contracts.allowed_missing_jump_targets.len(), 1);
        assert_eq!(contracts.allowed_missing_jump_targets[0], exit_block);
    }
}
