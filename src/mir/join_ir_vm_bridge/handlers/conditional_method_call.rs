//! Conditional Method Call Handler
//!
//! Phase 260 P0.3: Extracted from joinir_block_converter.rs (lines 243-327)
//! Handles JoinIR ConditionalMethodCall instruction conversion.
//!
//! ## Shape: if/phi expansion (3 blocks)
//!
//! ```text
//! ConditionalMethodCall:
//!   if cond:
//!     then_value = receiver.method(args)
//!   else:
//!     else_value = receiver
//!   dst = phi(then_value, else_value)
//! ```
//!
//! ## Control Flow Graph
//!
//! ```text
//! [current_block]
//!    |
//!    | Branch(cond)
//!    +-------------+
//!    |             |
//!    v             v
//! [then_block]  [else_block]
//!  Call(Method)  Copy receiver
//!    |             |
//!    +------+------+
//!           |
//!           v
//!      [merge_block]
//!        Phi(then, else)
//! ```

use crate::ast::Span;
use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::join_ir_vm_bridge::block_allocator::BlockAllocator;
use crate::mir::join_ir_vm_bridge::terminator_builder::{
    create_branch_terminator, create_jump_terminator,
};
use crate::mir::join_ir_vm_bridge::JoinIrVmBridgeError;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, MirFunction, MirInstruction, MirType, ValueId,
};

/// Handle JoinIR ConditionalMethodCall instruction
///
/// Phase 56: ConditionalMethodCall → if/phi expansion
///
/// Creates a 3-block control flow pattern:
/// 1. Current block: Branch on condition
/// 2. Then block: Execute method call
/// 3. Else block: Copy receiver (no-op path)
/// 4. Merge block: PHI to select result
///
/// # Arguments
///
/// * `mir_func` - Target MIR function to modify
/// * `allocator` - Block ID allocator
/// * `current_block_id` - Current block ID (becomes cond block)
/// * `current_instructions` - Instructions accumulated in current block
/// * `cond` - Condition ValueId (boolean)
/// * `dst` - Destination ValueId for result (receives PHI output)
/// * `receiver` - Receiver ValueId for method call
/// * `method` - Method name string
/// * `args` - Method arguments
/// * `type_hint` - Optional type hint for PHI instruction
/// * `finalize_fn` - Block finalization function (preserves PHI)
///
/// # Returns
///
/// * `Ok(BasicBlockId)` - New current block ID (merge_block)
/// * `Err(...)` - Conversion error
///
/// # Block Creation
///
/// - **then_block**: Call(Method) instruction + Jump to merge
/// - **else_block**: Copy receiver + Jump to merge
/// - **merge_block**: PHI(then_value, else_value) → dst (no terminator yet)
///
/// # Example
///
/// ```ignore
/// // JoinIR:
/// ConditionalMethodCall { cond: %1, dst: %2, receiver: %3, method: "foo", args: [%4] }
///
/// // MIR:
/// bb0:  // current_block
///   Branch(%1, then=bb1, else=bb2)
///
/// bb1:  // then_block
///   %5 = Call(Method{recv=%3,method="foo"}, [%4])
///   Jump(bb3)
///
/// bb2:  // else_block
///   %6 = Copy(%3)
///   Jump(bb3)
///
/// bb3:  // merge_block
///   %2 = Phi((bb1, %5), (bb2, %6))
///   // ... continues ...
/// ```
#[allow(clippy::too_many_arguments)]
pub fn handle_conditional_method_call<F>(
    mir_func: &mut MirFunction,
    allocator: &mut BlockAllocator,
    current_block_id: BasicBlockId,
    current_instructions: Vec<MirInstruction>,
    cond: &ValueId,
    dst: &ValueId,
    receiver: &ValueId,
    method: &str,
    args: &[ValueId],
    type_hint: &Option<MirType>,
    finalize_fn: F,
) -> Result<BasicBlockId, JoinIrVmBridgeError>
where
    F: FnOnce(&mut MirFunction, BasicBlockId, Vec<MirInstruction>, MirInstruction),
{
    // Phase 56: ConditionalMethodCall を if/phi に変換
    debug_log!(
        "[joinir_block] Converting ConditionalMethodCall: dst={:?}, cond={:?}",
        dst,
        cond
    );

    // Allocate 3 blocks: then, else, merge
    let (then_block, else_block, merge_block) = allocator.allocate_three();

    // Finalize current block with Branch terminator
    let branch_terminator = create_branch_terminator(*cond, then_block, else_block);
    finalize_fn(
        mir_func,
        current_block_id,
        current_instructions,
        branch_terminator,
    );

    // Allocate new ValueIds for then/else results
    let then_value = mir_func.next_value_id();
    let else_value = mir_func.next_value_id();

    // Then block: method call
    let mut then_block_obj = BasicBlock::new(then_block);
    then_block_obj.instructions.push(MirInstruction::Call {
        dst: Some(then_value),
        func: ValueId::INVALID,
        callee: Some(crate::mir::Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: method.to_string(),
            receiver: Some(*receiver),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: args.to_vec(),
        effects: EffectMask::WRITE,
    });
    then_block_obj.instruction_spans.push(Span::unknown());
    then_block_obj.set_terminator(create_jump_terminator(merge_block));
    mir_func.blocks.insert(then_block, then_block_obj);

    // Else block: copy receiver
    let mut else_block_obj = BasicBlock::new(else_block);
    else_block_obj.set_terminator(create_jump_terminator(merge_block));
    mir_func.blocks.insert(else_block, else_block_obj);

    // Merge block: phi for dst
    let mut merge_block_obj = BasicBlock::new(merge_block);
    if crate::config::env::joinir_dev::debug_enabled() {
        let caller = std::panic::Location::caller();
        let loc = format!("{}:{}:{}", caller.file(), caller.line(), caller.column());
        mir_func.metadata.value_origin_callers.insert(*dst, loc);
    }
    merge_block_obj.instructions.push(MirInstruction::Phi {
        dst: *dst,
        inputs: vec![(then_block, then_value), (else_block, else_value)],
        type_hint: type_hint.clone(),
    });
    merge_block_obj.instruction_spans.push(Span::unknown());
    mir_func.blocks.insert(merge_block, merge_block_obj);

    copy_emitter::emit_copy_in_block(
        mir_func,
        else_block,
        else_value,
        *receiver,
        CopyEmitReason::JoinIrBridgeConditionalMethodCall,
    )
    .map_err(JoinIrVmBridgeError::new)?;

    Ok(merge_block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{EffectMask, FunctionSignature, MirType};

    fn create_test_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        MirFunction::new(signature, BasicBlockId::new(0))
    }

    #[test]
    fn test_handle_conditional_method_call_basic() {
        let mut mir_func = create_test_function();
        let mut allocator = BlockAllocator::new(1); // start at 1 (0 is entry)

        let current_block_id = BasicBlockId::new(0);
        let current_instructions = vec![];
        let cond = ValueId(100);
        let dst = ValueId(200);
        let receiver = ValueId(300);
        let method = "test_method";
        let args = vec![];

        let result = handle_conditional_method_call(
            &mut mir_func,
            &mut allocator,
            current_block_id,
            current_instructions,
            &cond,
            &dst,
            &receiver,
            method,
            &args,
            &None,
            |func, block_id, insts, terminator| {
                // Finalize function mock
                let mut block = BasicBlock::new(block_id);
                block.instructions = insts;
                block.set_terminator(terminator);
                func.blocks.insert(block_id, block);
            },
        );

        assert!(result.is_ok());
        let merge_block = result.unwrap();

        // Verify block allocation
        assert_eq!(merge_block, BasicBlockId::new(3)); // then=1, else=2, merge=3

        // Verify current block has Branch terminator
        let current_block = mir_func.blocks.get(&current_block_id).unwrap();
        if let Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) = &current_block.terminator
        {
            assert_eq!(*condition, ValueId(100));
            assert_eq!(*then_bb, BasicBlockId::new(1));
            assert_eq!(*else_bb, BasicBlockId::new(2));
        } else {
            panic!("Expected Branch terminator");
        }

        // Verify then block has Call(Method)
        let then_block = mir_func.blocks.get(&BasicBlockId::new(1)).unwrap();
        assert_eq!(then_block.instructions.len(), 1);
        if let MirInstruction::Call {
            dst: Some(then_dst),
            callee:
                Some(crate::mir::Callee::Method {
                    method: method_name,
                    receiver: Some(box_val),
                    ..
                }),
            args: method_args,
            ..
        } = &then_block.instructions[0]
        {
            assert_eq!(*box_val, ValueId(300));
            assert_eq!(method_name, "test_method");
            assert_eq!(*method_args, vec![]);
            // then_dst is allocated dynamically
            assert!(then_dst.0 > 0);
        } else {
            panic!("Expected Call(Method) instruction");
        }

        // Verify else block has Copy
        let else_block = mir_func.blocks.get(&BasicBlockId::new(2)).unwrap();
        assert_eq!(else_block.instructions.len(), 1);
        if let MirInstruction::Copy { src, .. } = &else_block.instructions[0] {
            assert_eq!(*src, ValueId(300));
        } else {
            panic!("Expected Copy instruction");
        }

        // Verify merge block has Phi
        let merge_block_obj = mir_func.blocks.get(&merge_block).unwrap();
        assert_eq!(merge_block_obj.instructions.len(), 1);
        if let MirInstruction::Phi {
            dst: phi_dst,
            inputs,
            ..
        } = &merge_block_obj.instructions[0]
        {
            assert_eq!(*phi_dst, ValueId(200));
            assert_eq!(inputs.len(), 2);
            assert_eq!(inputs[0].0, BasicBlockId::new(1)); // then_block
            assert_eq!(inputs[1].0, BasicBlockId::new(2)); // else_block
        } else {
            panic!("Expected Phi instruction");
        }
    }

    #[test]
    fn test_handle_conditional_method_call_with_args() {
        let mut mir_func = create_test_function();
        let mut allocator = BlockAllocator::new(1);

        let current_block_id = BasicBlockId::new(0);
        let current_instructions = vec![];
        let cond = ValueId(100);
        let dst = ValueId(200);
        let receiver = ValueId(300);
        let method = "foo";
        let args = vec![ValueId(400), ValueId(500)];

        let result = handle_conditional_method_call(
            &mut mir_func,
            &mut allocator,
            current_block_id,
            current_instructions,
            &cond,
            &dst,
            &receiver,
            method,
            &args,
            &None,
            |func, block_id, insts, terminator| {
                let mut block = BasicBlock::new(block_id);
                block.instructions = insts;
                block.set_terminator(terminator);
                func.blocks.insert(block_id, block);
            },
        );

        assert!(result.is_ok());

        // Verify then block has Call(Method) with args
        let then_block = mir_func.blocks.get(&BasicBlockId::new(1)).unwrap();
        if let MirInstruction::Call {
            args: method_args, ..
        } = &then_block.instructions[0]
        {
            assert_eq!(*method_args, vec![ValueId(400), ValueId(500)]);
        } else {
            panic!("Expected Call instruction");
        }
    }

    #[test]
    fn test_handle_conditional_method_call_type_hint() {
        let mut mir_func = create_test_function();
        let mut allocator = BlockAllocator::new(1);

        let current_block_id = BasicBlockId::new(0);
        let current_instructions = vec![];
        let cond = ValueId(100);
        let dst = ValueId(200);
        let receiver = ValueId(300);
        let method = "bar";
        let args = vec![];
        let type_hint = Some(MirType::Integer);

        let result = handle_conditional_method_call(
            &mut mir_func,
            &mut allocator,
            current_block_id,
            current_instructions,
            &cond,
            &dst,
            &receiver,
            method,
            &args,
            &type_hint,
            |func, block_id, insts, terminator| {
                let mut block = BasicBlock::new(block_id);
                block.instructions = insts;
                block.set_terminator(terminator);
                func.blocks.insert(block_id, block);
            },
        );

        assert!(result.is_ok());
        let merge_block = result.unwrap();

        // Verify merge block Phi has type_hint
        let merge_block_obj = mir_func.blocks.get(&merge_block).unwrap();
        if let MirInstruction::Phi {
            type_hint: hint, ..
        } = &merge_block_obj.instructions[0]
        {
            assert_eq!(*hint, Some(MirType::Integer));
        } else {
            panic!("Expected Phi instruction");
        }
    }

    #[test]
    fn test_handle_conditional_method_call_current_instructions_preserved() {
        let mut mir_func = create_test_function();
        let mut allocator = BlockAllocator::new(1);

        let current_block_id = BasicBlockId::new(0);
        // Current block has some instructions before ConditionalMethodCall
        let current_instructions = vec![
            MirInstruction::Copy {
                dst: ValueId(10),
                src: ValueId(20),
            },
            MirInstruction::Copy {
                dst: ValueId(30),
                src: ValueId(40),
            },
        ];
        let cond = ValueId(100);
        let dst = ValueId(200);
        let receiver = ValueId(300);
        let method = "baz";
        let args = vec![];

        let result = handle_conditional_method_call(
            &mut mir_func,
            &mut allocator,
            current_block_id,
            current_instructions,
            &cond,
            &dst,
            &receiver,
            method,
            &args,
            &None,
            |func, block_id, insts, terminator| {
                let mut block = BasicBlock::new(block_id);
                block.instructions = insts;
                block.set_terminator(terminator);
                func.blocks.insert(block_id, block);
            },
        );

        assert!(result.is_ok());

        // Verify current block preserves instructions
        let current_block = mir_func.blocks.get(&current_block_id).unwrap();
        assert_eq!(current_block.instructions.len(), 2);
        if let MirInstruction::Copy { dst, src } = &current_block.instructions[0] {
            assert_eq!(*dst, ValueId(10));
            assert_eq!(*src, ValueId(20));
        } else {
            panic!("Expected Copy instruction");
        }
    }
}
