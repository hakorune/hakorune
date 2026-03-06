//! Phase 190: JoinIR Block Converter
//!
//! 責務: JoinIR のブロックを MIR ブロックに変換
//! - 命令列変換
//! - ターミネータ（Jump/Branch/Return）の処理
//! - ブロックID マッピング管理

use crate::mir::join_ir::{JoinFuncId, JoinInst, MirLikeInst};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::collections::BTreeMap;

use super::{convert_mir_like_inst, JoinIrVmBridgeError};

mod handlers;
mod utils;
mod tests;

use handlers::{HandlerContext, handle_method_call, handle_conditional_method_call, handle_field_access, handle_new_box, handle_call, handle_jump, handle_select, handle_if_merge, handle_ret, handle_nested_if_merge};
use utils::{finalize_remaining_instructions, annotate_value_types_for_inst, log_dbg};

pub struct JoinIrBlockConverter {
    pub(crate) current_block_id: BasicBlockId,
    pub(crate) current_instructions: Vec<MirInstruction>,
    pub(crate) next_block_id: u32,
    /// Phase 256 P1.8: Map from JoinFuncId to actual function name
    /// When set, handle_call uses this instead of join_func_name()
    pub(crate) func_name_map: Option<BTreeMap<JoinFuncId, String>>,
}

impl JoinIrBlockConverter {
    pub fn new() -> Self {
        Self {
            current_block_id: BasicBlockId(0), // entry block
            current_instructions: Vec::new(),
            next_block_id: 1, // start from 1 (0 is entry)
            func_name_map: None,
        }
    }

    /// Phase 256 P1.8: Create converter with function name map
    /// This ensures Call instructions use actual function names instead of "join_func_N"
    pub fn new_with_func_names(func_name_map: BTreeMap<JoinFuncId, String>) -> Self {
        Self {
            current_block_id: BasicBlockId(0),
            current_instructions: Vec::new(),
            next_block_id: 1,
            func_name_map: Some(func_name_map),
        }
    }

    /// JoinIR 関数本体を MIR ブロック群に変換
    ///
    /// # Phase 27-shortterm S-4.4: Multi-block conversion
    ///
    /// Strategy:
    /// - Accumulate Compute instructions in current block
    /// - On Jump: emit Branch + create exit block with Return
    /// - On Call: emit Call in current block
    pub fn convert_function_body(
        &mut self,
        mir_func: &mut MirFunction,
        join_body: &[JoinInst],
    ) -> Result<(), JoinIrVmBridgeError> {
        let mut ctx = HandlerContext {
            mir_func,
            current_block_id: &mut self.current_block_id,
            current_instructions: &mut self.current_instructions,
            next_block_id: &mut self.next_block_id,
            func_name_map: &self.func_name_map,
        };

        for join_inst in join_body {
            match join_inst {
                JoinInst::Compute(mir_like) => {
                    // Phase 189: Special handling for MirLikeInst::Select
                    // IfPhiJoin route uses JoinInst::Compute(MirLikeInst::Select {...})
                    // but Select needs control flow expansion (Branch + Phi), not single instruction
                    if let MirLikeInst::Select {
                        dst,
                        cond,
                        then_val,
                        else_val,
                    } = mir_like
                    {
                        log_dbg(format!(
                            "[joinir_block] ✅ Found Select! dst={:?}, calling handle_select",
                            dst
                        ));
                        handle_select(&mut ctx, dst, cond, then_val, else_val, &None)?;
                        continue;
                    }
                    // Debug: show what instruction we're processing
                    log_dbg(format!(
                        "[joinir_block] Compute instruction: {:?}",
                        mir_like
                    ));
                    let mir_inst = convert_mir_like_inst(mir_like)?;
                    annotate_value_types_for_inst(ctx.mir_func, &mir_inst);
                    ctx.current_instructions.push(mir_inst);
                }
                JoinInst::MethodCall {
                    dst,
                    receiver,
                    method,
                    args,
                    type_hint,
                } => {
                    handle_method_call(&mut ctx, dst, receiver, method, args, type_hint)?;
                }
                JoinInst::ConditionalMethodCall {
                    cond,
                    dst,
                    receiver,
                    method,
                    args,
                } => {
                    handle_conditional_method_call(
                        &mut ctx, cond, dst, receiver, method, args,
                    )?;
                }
                JoinInst::FieldAccess { dst, object, field } => {
                    handle_field_access(&mut ctx, dst, object, field)?;
                }
                JoinInst::NewBox {
                    dst,
                    box_name,
                    args,
                    type_hint,
                } => {
                    handle_new_box(&mut ctx, dst, box_name, args, type_hint)?;
                }
                JoinInst::Call {
                    func,
                    args,
                    dst,
                    k_next,
                } => {
                    handle_call(&mut ctx, func, args, dst, k_next)?;
                }
                JoinInst::Jump { cont, args, cond } => {
                    handle_jump(&mut ctx, cont, args, cond)?;
                }
                JoinInst::Select {
                    dst,
                    cond,
                    then_val,
                    else_val,
                    type_hint,
                } => {
                    handle_select(&mut ctx, dst, cond, then_val, else_val, type_hint)?;
                }
                JoinInst::IfMerge {
                    cond,
                    merges,
                    k_next,
                } => {
                    handle_if_merge(&mut ctx, cond, merges, k_next)?;
                }
                JoinInst::Ret { value } => {
                    handle_ret(&mut ctx, value)?;
                }
                JoinInst::NestedIfMerge {
                    conds,
                    merges,
                    k_next,
                } => {
                    handle_nested_if_merge(&mut ctx, conds, merges, k_next)?;
                }
            }
        }

        // Finalize any remaining instructions
        finalize_remaining_instructions(mir_func, self.current_block_id, &mut self.current_instructions);

        Ok(())
    }
}
