//! Terminator Builder - Unified terminator creation
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs
//! Eliminates repeated Branch/Jump/Return terminator shapes (4x duplication).
//!
//! ## Structure Before
//!
//! ```ignore
//! let branch_terminator = MirInstruction::Branch {
//!     condition: *cond,
//!     then_bb: then_block,
//!     else_bb: else_block,
//!     then_edge_args: None,
//!     else_edge_args: None,
//! };
//! Self::finalize_block(mir_func, block_id, instructions, branch_terminator);
//! ```
//!
//! ## Structure After
//!
//! ```ignore
//! emit_branch_and_finalize(mir_func, block_id, instructions, cond, then_bb, else_bb, finalize_fn);
//! ```

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

/// Create a Branch terminator instruction
///
/// Creates a Branch with empty edge args (most common pattern).
///
/// # Arguments
///
/// * `condition` - Condition ValueId (boolean)
/// * `then_bb` - Target block for true branch
/// * `else_bb` - Target block for false branch
///
/// # Returns
///
/// MirInstruction::Branch with None edge args
pub fn create_branch_terminator(
    condition: ValueId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
) -> MirInstruction {
    MirInstruction::Branch {
        condition,
        then_bb,
        else_bb,
        then_edge_args: None,
        else_edge_args: None,
    }
}

/// Create a Branch terminator and finalize block
///
/// Combines branch terminator creation with block finalization.
/// Eliminates 4x duplication pattern in joinir_block_converter.
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `block_id` - Block ID to finalize
/// * `instructions` - Instructions to add to block
/// * `condition` - Branch condition ValueId
/// * `then_bb` - Target block for true branch
/// * `else_bb` - Target block for false branch
/// * `finalize_fn` - Block finalization function (preserves PHI)
pub fn emit_branch_and_finalize<F>(
    mir_func: &mut MirFunction,
    block_id: BasicBlockId,
    instructions: Vec<MirInstruction>,
    condition: ValueId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
    finalize_fn: F,
) where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    let branch_terminator = create_branch_terminator(condition, then_bb, else_bb);
    finalize_fn(mir_func, block_id, instructions, branch_terminator);
}

/// Create a Jump terminator instruction
///
/// Creates a Jump with empty edge args (most common pattern).
///
/// # Arguments
///
/// * `target` - Target block ID to jump to
///
/// # Returns
///
/// MirInstruction::Jump with None edge args
pub fn create_jump_terminator(target: BasicBlockId) -> MirInstruction {
    MirInstruction::Jump {
        target,
        edge_args: None,
    }
}

/// Create a Return terminator instruction
///
/// Creates a Return terminator with optional value.
///
/// # Arguments
///
/// * `value` - Optional return value
///
/// # Returns
///
/// MirInstruction::Return
pub fn create_return_terminator(value: Option<ValueId>) -> MirInstruction {
    MirInstruction::Return { value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_branch_terminator() {
        let terminator =
            create_branch_terminator(ValueId(100), BasicBlockId::new(1), BasicBlockId::new(2));

        if let MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
        } = terminator
        {
            assert_eq!(condition, ValueId(100));
            assert_eq!(then_bb, BasicBlockId::new(1));
            assert_eq!(else_bb, BasicBlockId::new(2));
            assert_eq!(then_edge_args, None);
            assert_eq!(else_edge_args, None);
        } else {
            panic!("Expected Branch terminator");
        }
    }

    #[test]
    fn test_emit_branch_and_finalize() {
        use crate::mir::{EffectMask, FunctionSignature, MirType};

        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let instructions = vec![];

        let mut finalized = false;
        emit_branch_and_finalize(
            &mut mir_func,
            BasicBlockId::new(0),
            instructions,
            ValueId(200),
            BasicBlockId::new(1),
            BasicBlockId::new(2),
            |_func, block_id, _insts, terminator| {
                finalized = true;
                assert_eq!(block_id, BasicBlockId::new(0));
                if let MirInstruction::Branch { condition, .. } = terminator {
                    assert_eq!(condition, ValueId(200));
                } else {
                    panic!("Expected Branch terminator");
                }
            },
        );

        assert!(finalized);
    }

    #[test]
    fn test_create_jump_terminator() {
        let terminator = create_jump_terminator(BasicBlockId::new(5));

        if let MirInstruction::Jump { target, edge_args } = terminator {
            assert_eq!(target, BasicBlockId::new(5));
            assert_eq!(edge_args, None);
        } else {
            panic!("Expected Jump terminator");
        }
    }

    #[test]
    fn test_create_return_terminator_with_value() {
        let terminator = create_return_terminator(Some(ValueId(42)));

        if let MirInstruction::Return { value } = terminator {
            assert_eq!(value, Some(ValueId(42)));
        } else {
            panic!("Expected Return terminator");
        }
    }

    #[test]
    fn test_create_return_terminator_without_value() {
        let terminator = create_return_terminator(None);

        if let MirInstruction::Return { value } = terminator {
            assert_eq!(value, None);
        } else {
            panic!("Expected Return terminator");
        }
    }
}
