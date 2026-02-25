//! Merge Variable Handler - Copy emission for if/else merge patterns
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Eliminates repeated merge copy emission patterns (4x duplication).
//!
//! ## Pattern Before
//!
//! ```ignore
//! for merge in merges {
//!     block_obj.instructions.push(MirInstruction::Copy {
//!         dst: merge.dst,
//!         src: merge.then_val, // or else_val
//!     });
//!     block_obj.instruction_spans.push(Span::unknown());
//! }
//! ```
//!
//! ## Pattern After
//!
//! ```ignore
//! emit_merge_copies_in_func(mir_func, block_id, merges, MergeBranch::Then, reason);
//! ```

use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::join_ir::MergePair;
use crate::mir::{BasicBlockId, MirFunction};

/// Which branch of an if/else to extract values from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeBranch {
    /// Use then_val from MergePair
    Then,
    /// Use else_val from MergePair
    Else,
}

/// Emit merge copies via MirFunction (CopyEmitter facade).
///
/// This provides a SSOT-friendly entrypoint so copy emission can be
/// centralized and fail-fast checks can be applied in one place.
pub fn emit_merge_copies_in_func(
    mir_func: &mut MirFunction,
    bb: BasicBlockId,
    merges: &[MergePair],
    branch: MergeBranch,
    reason: CopyEmitReason,
) -> Result<(), String> {
    for merge in merges {
        let src = match branch {
            MergeBranch::Then => merge.then_val,
            MergeBranch::Else => merge.else_val,
        };

        copy_emitter::emit_copy_in_block(mir_func, bb, merge.dst, src, reason)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::FunctionSignature;
    use crate::mir::{BasicBlockId, EffectMask, MirInstruction, MirType, ValueId};

    #[test]
    fn test_emit_merge_copies_then() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let block_id = BasicBlockId::new(0);
        let merges = vec![
            MergePair {
                dst: ValueId(10),
                then_val: ValueId(20),
                else_val: ValueId(30),
                type_hint: None,
            },
            MergePair {
                dst: ValueId(11),
                then_val: ValueId(21),
                else_val: ValueId(31),
                type_hint: None,
            },
        ];

        emit_merge_copies_in_func(
            &mut mir_func,
            block_id,
            &merges,
            MergeBranch::Then,
            CopyEmitReason::TestMergeVariableHandler,
        )
        .unwrap();

        let block = mir_func.blocks.get(&block_id).unwrap();
        assert_eq!(block.instructions.len(), 2);

        if let MirInstruction::Copy { dst, src } = &block.instructions[0] {
            assert_eq!(*dst, ValueId(10));
            assert_eq!(*src, ValueId(20));
        } else {
            panic!("Expected Copy instruction");
        }

        if let MirInstruction::Copy { dst, src } = &block.instructions[1] {
            assert_eq!(*dst, ValueId(11));
            assert_eq!(*src, ValueId(21));
        } else {
            panic!("Expected Copy instruction");
        }
    }

    #[test]
    fn test_emit_merge_copies_else() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let block_id = BasicBlockId::new(0);
        let merges = vec![MergePair {
            dst: ValueId(10),
            then_val: ValueId(20),
            else_val: ValueId(30),
            type_hint: None,
        }];

        emit_merge_copies_in_func(
            &mut mir_func,
            block_id,
            &merges,
            MergeBranch::Else,
            CopyEmitReason::TestMergeVariableHandler,
        )
        .unwrap();

        let block = mir_func.blocks.get(&block_id).unwrap();
        assert_eq!(block.instructions.len(), 1);

        if let MirInstruction::Copy { dst, src } = &block.instructions[0] {
            assert_eq!(*dst, ValueId(10));
            assert_eq!(*src, ValueId(30)); // else_val
        } else {
            panic!("Expected Copy instruction");
        }
    }

    #[test]
    fn test_emit_merge_copies_empty() {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut mir_func = MirFunction::new(signature, BasicBlockId::new(0));
        let block_id = BasicBlockId::new(0);
        let merges: Vec<MergePair> = vec![];

        emit_merge_copies_in_func(
            &mut mir_func,
            block_id,
            &merges,
            MergeBranch::Then,
            CopyEmitReason::TestMergeVariableHandler,
        )
        .unwrap();

        let block = mir_func.blocks.get(&block_id).unwrap();
        assert!(block.instructions.is_empty());
    }
}
