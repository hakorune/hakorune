use crate::ast::Span;
use super::super::call_generator::{emit_call_pair, emit_call_pair_with_spans};
use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::join_ir::{JoinContId, JoinFuncId, MergePair};
use crate::mir::{BasicBlockId, Callee, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

use super::super::block_allocator::BlockAllocator;
use super::super::{join_func_name, JoinIrVmBridgeError};
use super::utils::{finalize_block, log_dbg};

// Helper macro for logging, assumes `log_dbg` is available
macro_rules! debug_log {
    ($($arg:tt)*) => {
        log_dbg(format!($($arg)*))
    };
}

pub(crate) struct HandlerContext<'a> {
    pub(crate) mir_func: &'a mut MirFunction,
    pub(crate) current_block_id: &'a mut BasicBlockId,
    pub(crate) current_instructions: &'a mut Vec<MirInstruction>,
    pub(crate) next_block_id: &'a mut u32,
    pub(crate) func_name_map: &'a Option<BTreeMap<JoinFuncId, String>>,
}

pub(crate) fn handle_method_call(
    ctx: &mut HandlerContext,
    dst: &ValueId,
    receiver: &ValueId,
    method: &str,
    args: &[ValueId],
    type_hint: &Option<crate::mir::MirType>,
) -> Result<(), JoinIrVmBridgeError> {
    let mir_inst = MirInstruction::Call {
        dst: Some(*dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: method.to_string(),
            receiver: Some(*receiver),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: args.to_vec(),
        effects: EffectMask::PURE,
    };
    ctx.current_instructions.push(mir_inst);

    // Phase 65-2-A: TODO: type_hint を value_types に記録
    let _ = type_hint;
    Ok(())
}

pub(crate) fn handle_conditional_method_call(
    ctx: &mut HandlerContext,
    cond: &ValueId,
    dst: &ValueId,
    receiver: &ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 56: ConditionalMethodCall を if/phi に変換
    debug_log!(
        "[joinir_block] Converting ConditionalMethodCall: dst={:?}, cond={:?}",
        dst,
        cond
    );

    let cond_block = *ctx.current_block_id;
    // Phase 269 P1.2+: Use BlockAllocator to eliminate duplication (Site 1/4)
    let mut allocator = BlockAllocator::new(*ctx.next_block_id);
    let (then_block, else_block, merge_block) = allocator.allocate_three();
    *ctx.next_block_id = allocator.peek_next();

    // cond block: branch
    let branch_terminator = MirInstruction::Branch {
        condition: *cond,
        then_bb: then_block,
        else_bb: else_block,
        then_edge_args: None,
        else_edge_args: None,
    };
    finalize_block(
        ctx.mir_func,
        cond_block,
        std::mem::take(ctx.current_instructions),
        branch_terminator,
    );

    let then_value = ctx.mir_func.next_value_id();
    let else_value = ctx.mir_func.next_value_id();

    // then block: method call
    let mut then_block_obj = crate::mir::BasicBlock::new(then_block);
    then_block_obj.instructions.push(MirInstruction::Call {
        dst: Some(then_value),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
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
    then_block_obj.set_terminator(MirInstruction::Jump {
        target: merge_block,
        edge_args: None,
    });
    ctx.mir_func.blocks.insert(then_block, then_block_obj);

    // else block: copy receiver
    let mut else_block_obj = crate::mir::BasicBlock::new(else_block);
    else_block_obj.set_terminator(MirInstruction::Jump {
        target: merge_block,
        edge_args: None,
    });
    ctx.mir_func.blocks.insert(else_block, else_block_obj);

    // merge block: phi for dst
    let mut merge_block_obj = crate::mir::BasicBlock::new(merge_block);
    if crate::config::env::joinir_dev::debug_enabled() {
        let caller = std::panic::Location::caller();
        let loc = format!("{}:{}:{}", caller.file(), caller.line(), caller.column());
        ctx.mir_func.metadata.value_origin_callers.insert(*dst, loc);
    }
    merge_block_obj.instructions.push(MirInstruction::Phi {
        dst: *dst,
        inputs: vec![(then_block, then_value), (else_block, else_value)],
        type_hint: None,
    });
    merge_block_obj.instruction_spans.push(Span::unknown());
    ctx.mir_func.blocks.insert(merge_block, merge_block_obj);

    copy_emitter::emit_copy_in_block(
        ctx.mir_func,
        else_block,
        else_value,
        *receiver,
        CopyEmitReason::JoinIrBridgeJoinirBlockConverterConditionalMethodCall,
    )
    .map_err(JoinIrVmBridgeError::new)?;

    *ctx.current_block_id = merge_block;
    Ok(())
}

pub(crate) fn handle_field_access(
    ctx: &mut HandlerContext,
    dst: &ValueId,
    object: &ValueId,
    field: &str,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 51: FieldAccess → Call(Method) getter pattern
    let mir_inst = MirInstruction::Call {
        dst: Some(*dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: field.to_string(),
            receiver: Some(*object),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Union,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    };
    ctx.current_instructions.push(mir_inst);
    Ok(())
}

pub(crate) fn handle_new_box(
    ctx: &mut HandlerContext,
    dst: &ValueId,
    box_name: &str,
    args: &[ValueId],
    type_hint: &Option<crate::mir::MirType>,
) -> Result<(), JoinIrVmBridgeError> {
    let mir_inst = MirInstruction::NewBox {
        dst: *dst,
        box_type: box_name.to_string(),
        args: args.to_vec(),
    };
    ctx.current_instructions.push(mir_inst);

    // Phase 65-2-B: TODO: type_hint を value_types に記録
    let _ = type_hint;
    Ok(())
}

pub(crate) fn handle_call(
    ctx: &mut HandlerContext,
    func: &JoinFuncId,
    args: &[ValueId],
    dst: &Option<ValueId>,
    k_next: &Option<JoinContId>,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 30.x: Call conversion
    if k_next.is_some() {
        return Err(JoinIrVmBridgeError::new(
            "Call with k_next is not yet supported".to_string(),
        ));
    }

    // Phase 256 P1.8: Use actual function name if available
    let func_name = if let Some(ref map) = ctx.func_name_map {
        map.get(func).cloned().unwrap_or_else(|| join_func_name(*func))
    } else {
        join_func_name(*func)
    };

    // Phase 131 P2: Stable function name ValueId (module-global SSOT)
    //
    // The merge pipeline relies on `Const(String("join_func_N"))` to detect tail calls.
    // The ValueId used for that const MUST be stable across *all* functions in the module.
    //
    // IMPORTANT: avoid collisions with `call_result_id = ValueId(99991)`.
    const FUNC_NAME_ID_BASE: u32 = 90000;
    let func_name_id = ValueId(FUNC_NAME_ID_BASE + func.0);
    if func_name_id == ValueId(99991) {
        return Err(JoinIrVmBridgeError::new(
            "[joinir_block] func_name_id collided with call_result_id (99991)".to_string(),
        ));
    }

    match dst {
        Some(result_dst) => {
            // Non-tail call
            emit_call_pair(
                ctx.current_instructions,
                func_name_id,
                *result_dst,
                &func_name,
                args,
            );
        }
        None => {
            // Tail call
            let call_result_id = ValueId(99991);
            // Phase 188.3 P2: Use emit_call_pair with callee field
            emit_call_pair(
                ctx.current_instructions,
                func_name_id,
                call_result_id,
                &func_name,
                args,
            );

            // Phase 131 P2: Preserve tail-call args as legacy jump-args metadata (for exit wiring)
            //
            // The merge pipeline recovers carrier/env values from the legacy jump-args path
            // (via BasicBlock APIs) when converting Return → Jump to the exit block.
            // Without this, tail-call blocks look like "no args", forcing fallbacks that can
            // produce undefined ValueIds in DirectValue mode.
            let terminator = MirInstruction::Return {
                value: Some(call_result_id),
            };
            finalize_block(
                ctx.mir_func,
                *ctx.current_block_id,
                std::mem::take(ctx.current_instructions),
                terminator,
            );
            if let Some(block) = ctx.mir_func.blocks.get_mut(ctx.current_block_id) {
                if matches!(block.terminator, Some(MirInstruction::Return { .. })) && block.return_env().is_none() {
                    block.set_return_env(crate::mir::EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: args.to_vec(),
                    });
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn handle_jump(
    ctx: &mut HandlerContext,
    cont: &JoinContId,
    args: &[ValueId],
    cond: &Option<ValueId>,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 256 P1.9: Jump → tail call to continuation function
    // Previously was just `ret args[0]`, now generates `call cont(args...); ret result`
    debug_log!(
        "[joinir_block] Converting Jump to tail call: cont={:?}, args={:?}, cond={:?}",
        cont,
        args,
        cond
    );

    // Get continuation function name
    let cont_name = get_continuation_name(ctx.func_name_map, cont);

    // Phase 256 P1.9: Use distinct ValueIds for Jump tail call
    // FUNC_NAME_ID_BASE for call targets, 99992 for Jump result (distinct from 99991 in handle_call)
    const JUMP_FUNC_NAME_ID_BASE: u32 = 91000;  // Different from handle_call's 90000
    let func_name_id = ValueId(JUMP_FUNC_NAME_ID_BASE + cont.0);
    let call_result_id = ValueId(99992);  // Distinct from handle_call's 99991

    match cond {
        Some(cond_var) => {
            // Conditional jump → Branch + tail call to continuation
            // Phase 269 P1.2+: Use BlockAllocator (Site 2/4)
            let mut allocator = BlockAllocator::new(*ctx.next_block_id);
            let (exit_block_id, continue_block_id) = allocator.allocate_two();
            *ctx.next_block_id = allocator.peek_next();

            let branch_terminator = MirInstruction::Branch {
                condition: *cond_var,
                then_bb: exit_block_id,
                else_bb: continue_block_id,
                then_edge_args: None,
                else_edge_args: None,
            };

            finalize_block(
                ctx.mir_func,
                *ctx.current_block_id,
                std::mem::take(ctx.current_instructions),
                branch_terminator,
            );

            // Exit block: tail call to continuation function
            let mut exit_block = crate::mir::BasicBlock::new(exit_block_id);

            // Phase 256 P1.9: Generate tail call to continuation
            emit_call_pair_with_spans(
                &mut exit_block.instructions,
                &mut exit_block.instruction_spans,
                func_name_id,
                call_result_id,
                &cont_name,
                args,
            );
            exit_block.set_terminator(MirInstruction::Return { value: Some(call_result_id) });
            exit_block.set_return_env(crate::mir::EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: args.to_vec(),
            });
            ctx.mir_func.blocks.insert(exit_block_id, exit_block);

            // Continue block
            let continue_block = crate::mir::BasicBlock::new(continue_block_id);
            ctx.mir_func.blocks.insert(continue_block_id, continue_block);

            *ctx.current_block_id = continue_block_id;
        }
        None => {
            // Unconditional jump → tail call to continuation
            // Finalize current block with tail call
            emit_call_pair(
                ctx.current_instructions,
                func_name_id,
                call_result_id,
                &cont_name,
                args,
            );

            let return_terminator = MirInstruction::Return { value: Some(call_result_id) };

            finalize_block(
                ctx.mir_func,
                *ctx.current_block_id,
                std::mem::take(ctx.current_instructions),
                return_terminator,
            );
            if let Some(block) = ctx.mir_func.blocks.get_mut(ctx.current_block_id) {
                if matches!(block.terminator, Some(MirInstruction::Return { .. })) && block.return_env().is_none() {
                    block.set_return_env(crate::mir::EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: args.to_vec(),
                    });
                }
            }
        }
    }
    Ok(())
}

/// Phase 256 P1.9: Get continuation function name from func_name_map
fn get_continuation_name(
    func_name_map: &Option<BTreeMap<JoinFuncId, String>>,
    cont: &JoinContId,
) -> String {
    // JoinContId.0 == JoinFuncId.0 (same underlying ID via as_cont())
    if let Some(ref map) = func_name_map {
        if let Some(name) = map.get(&JoinFuncId(cont.0)) {
            return name.clone();
        }
    }
    // Fallback: use join_func_name()
    join_func_name(JoinFuncId(cont.0))
}

pub(crate) fn handle_select(
    ctx: &mut HandlerContext,
    dst: &ValueId,
    cond: &ValueId,
    then_val: &ValueId,
    else_val: &ValueId,
    _type_hint: &Option<crate::mir::MirType>,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 256 P1.5: Select → MirInstruction::Select (direct instruction, not control flow expansion)
    debug_log!(
        "[joinir_block] Converting Select: dst={:?}, cond={:?}, then_val={:?}, else_val={:?}",
        dst,
        cond,
        then_val,
        else_val
    );

    // Emit Select instruction directly (no branch/phi expansion)
    let select_inst = MirInstruction::Select {
        dst: *dst,
        cond: *cond,
        then_val: *then_val,
        else_val: *else_val,
    };

    ctx.current_instructions.push(select_inst);
    Ok(())
}

pub(crate) fn handle_if_merge(
    ctx: &mut HandlerContext,
    cond: &ValueId,
    merges: &[MergePair],
    k_next: &Option<JoinContId>,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 33-6: IfMerge → if/phi (multiple variables)
    if k_next.is_some() {
        return Err(JoinIrVmBridgeError::new(
            "IfMerge: k_next not yet supported".to_string(),
        ));
    }

    debug_log!(
        "[joinir_block] Converting IfMerge: merges.len()={}",
        merges.len()
    );

    let cond_block = *ctx.current_block_id;
    // Phase 269 P1.2+: Use BlockAllocator (Site 3/4)
    let mut allocator = BlockAllocator::new(*ctx.next_block_id);
    let (then_block, else_block, merge_block) = allocator.allocate_three();
    *ctx.next_block_id = allocator.peek_next();

    // cond block: branch
    let branch_terminator = MirInstruction::Branch {
        condition: *cond,
        then_bb: then_block,
        else_bb: else_block,
        then_edge_args: None,
        else_edge_args: None,
    };
    finalize_block(
        ctx.mir_func,
        cond_block,
        std::mem::take(ctx.current_instructions),
        branch_terminator,
    );

    // then block: jump to merge
    let then_block_obj = crate::mir::BasicBlock::new(then_block);
    ctx.mir_func.blocks.insert(then_block, then_block_obj);
    ctx.mir_func
        .get_block_mut(then_block)
        .ok_or_else(|| JoinIrVmBridgeError::new(format!("then block {:?} missing", then_block)))?
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });

    // else block: jump to merge
    let else_block_obj = crate::mir::BasicBlock::new(else_block);
    ctx.mir_func.blocks.insert(else_block, else_block_obj);
    ctx.mir_func
        .get_block_mut(else_block)
        .ok_or_else(|| JoinIrVmBridgeError::new(format!("else block {:?} missing", else_block)))?
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });

    // merge block: PHI nodes (SSA restore)
    ctx.mir_func.blocks.insert(merge_block, crate::mir::BasicBlock::new(merge_block));
    for merge in merges {
        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
            ctx.mir_func,
            merge_block,
            merge.dst,
            vec![(then_block, merge.then_val), (else_block, merge.else_val)],
            Span::unknown(),
        )
        .map_err(|e| JoinIrVmBridgeError::new(format!("insert_phi failed: {}", e)))?;
    }

    *ctx.current_block_id = merge_block;
    Ok(())
}

pub(crate) fn handle_ret(
    ctx: &mut HandlerContext,
    value: &Option<ValueId>,
) -> Result<(), JoinIrVmBridgeError> {
    let return_terminator = MirInstruction::Return { value: *value };
    finalize_block(
        ctx.mir_func,
        *ctx.current_block_id,
        std::mem::take(ctx.current_instructions),
        return_terminator,
    );
    Ok(())
}

pub(crate) fn handle_nested_if_merge(
    ctx: &mut HandlerContext,
    conds: &[ValueId],
    merges: &[MergePair],
    k_next: &Option<JoinContId>,
) -> Result<(), JoinIrVmBridgeError> {
    // Phase 41-4: NestedIfMerge → multi-level Branch + PHI
    if k_next.is_some() {
        return Err(JoinIrVmBridgeError::new(
            "NestedIfMerge: k_next not yet supported".to_string(),
        ));
    }

    if conds.is_empty() {
        return Err(JoinIrVmBridgeError::new(
            "NestedIfMerge: conds must not be empty".to_string(),
        ));
    }

    debug_log!(
        "[joinir_block] Converting NestedIfMerge: conds.len()={}",
        conds.len()
    );

    let num_conds = conds.len();
    // Phase 269 P1.2+: Use BlockAllocator (Site 4/4)
    let mut allocator = BlockAllocator::new(*ctx.next_block_id);

    let mut level_blocks: Vec<BasicBlockId> = Vec::with_capacity(num_conds);
    level_blocks.push(*ctx.current_block_id);
    // Allocate level 1..num_conds blocks
    level_blocks.extend(allocator.allocate_n(num_conds - 1));

    let (then_block, final_else_block, merge_block) = allocator.allocate_three();
    *ctx.next_block_id = allocator.peek_next();

    // Pre-create level 1+ blocks
    for level in 1..num_conds {
        ctx.mir_func.blocks.insert(
            level_blocks[level],
            crate::mir::BasicBlock::new(level_blocks[level]),
        );
    }

    // Create branches
    for (level, cond_var) in conds.iter().enumerate() {
        let this_block = level_blocks[level];
        let next_true_block = if level + 1 < num_conds {
            level_blocks[level + 1]
        } else {
            then_block
        };

        let branch_terminator = MirInstruction::Branch {
            condition: *cond_var,
            then_bb: next_true_block,
            else_bb: final_else_block,
            then_edge_args: None,
            else_edge_args: None,
        };

        if level == 0 {
            finalize_block(
                ctx.mir_func,
                this_block,
                std::mem::take(ctx.current_instructions),
                branch_terminator,
            );
        } else {
            finalize_block(ctx.mir_func, this_block, Vec::new(), branch_terminator);
        }
    }

    // then block: jump to merge
    let then_block_obj = crate::mir::BasicBlock::new(then_block);
    ctx.mir_func.blocks.insert(then_block, then_block_obj);
    ctx.mir_func
        .get_block_mut(then_block)
        .ok_or_else(|| JoinIrVmBridgeError::new(format!("then block {:?} missing", then_block)))?
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });

    // else block: jump to merge
    let else_block_obj = crate::mir::BasicBlock::new(final_else_block);
    ctx.mir_func.blocks.insert(final_else_block, else_block_obj);
    ctx.mir_func
        .get_block_mut(final_else_block)
        .ok_or_else(|| {
            JoinIrVmBridgeError::new(format!("else block {:?} missing", final_else_block))
        })?
        .set_terminator(MirInstruction::Jump {
            target: merge_block,
            edge_args: None,
        });

    // merge block: PHI nodes (SSA restore)
    ctx.mir_func.blocks.insert(merge_block, crate::mir::BasicBlock::new(merge_block));
    for merge in merges {
        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
            ctx.mir_func,
            merge_block,
            merge.dst,
            vec![(then_block, merge.then_val), (final_else_block, merge.else_val)],
            Span::unknown(),
        )
        .map_err(|e| JoinIrVmBridgeError::new(format!("insert_phi failed: {}", e)))?;
    }

    *ctx.current_block_id = merge_block;
    Ok(())
}
