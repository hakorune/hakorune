//! Phase 132-R0 Task 3: Continuation Contract Tests
//!
//! Tests for the continuation contract enforcement in JoinIR merge.
//!
//! # Contract
//!
//! - Router/lowerer declares continuation functions in JoinInlineBoundary
//! - Merge uses structural checks only (no by-name/by-id inference)
//! - Skippable continuation: 1 block + empty instructions + Return only
//! - Non-skippable continuation: Contains other instructions (e.g., TailCall)

use crate::mir::builder::control_flow::joinir::merge::rewriter::helpers::is_skippable_continuation;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType, ValueId};

fn make_function(name: &str) -> MirFunction {
    let signature = FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    MirFunction::new(signature, BasicBlockId(0))
}

/// Case A: Pure k_exit with Return only is skippable
///
/// This is the typical continuation function pattern:
/// - 1 block
/// - No instructions
/// - Return terminator only
#[test]
fn case_a_pure_k_exit_return_is_skippable() {
    let mut func = make_function("join_func_2");
    let block = func.blocks.get_mut(&func.entry_block).unwrap();
    block.instructions.clear();
    block.set_terminator(MirInstruction::Return { value: None });
    assert!(is_skippable_continuation(&func));
}

/// Case B: k_exit with TailCall to post_k is NOT skippable
///
/// When k_exit calls another function (e.g., post_k for post-processing),
/// it is NOT a pure exit stub and must be merged as a real function.
#[test]
fn case_b_k_exit_tailcall_post_k_is_not_skippable() {
    let mut func = make_function("join_func_2");
    let block = func.blocks.get_mut(&func.entry_block).unwrap();
    block.instructions.push(MirInstruction::Call {
        dst: None,
        func: ValueId(0),
        callee: None,
        args: vec![],
        effects: EffectMask::CONTROL,
    });
    block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });
    assert!(!is_skippable_continuation(&func));
}

/// Case C: Multi-block continuation is NOT skippable
///
/// Even if empty, a continuation with multiple blocks is not a pure exit stub.
#[test]
fn case_c_multi_block_continuation_is_not_skippable() {
    let mut func = make_function("join_func_2");
    // Add a second block
    let second_block_id = BasicBlockId(1);
    let mut second_block = crate::mir::BasicBlock::new(second_block_id);
    second_block.set_terminator(MirInstruction::Return { value: None });
    func.blocks.insert(second_block_id, second_block);

    // Entry block jumps to second block
    let entry_block = func.blocks.get_mut(&func.entry_block).unwrap();
    entry_block.instructions.clear();
    entry_block.set_terminator(MirInstruction::Jump {
        target: second_block_id,
        edge_args: None,
    });

    assert!(!is_skippable_continuation(&func));
}

/// Case D: Continuation with instructions is NOT skippable
///
/// Even with Return terminator, any instruction makes it non-skippable.
#[test]
fn case_d_continuation_with_instructions_is_not_skippable() {
    let mut func = make_function("join_func_2");
    let block = func.blocks.get_mut(&func.entry_block).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(0),
        value: crate::mir::types::ConstValue::Integer(42),
    });
    block.set_terminator(MirInstruction::Return { value: None });
    assert!(!is_skippable_continuation(&func));
}
