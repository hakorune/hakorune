#![cfg(feature = "normalized_dev")]

use super::super::convert_mir_like_inst;
use super::super::join_func_name;
use super::super::JoinIrVmBridgeError;
use crate::ast::Span;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::join_ir::normalized::{JpFuncId, JpFunction, JpInst, JpOp, NormalizedModule};
use crate::mir::join_ir::{JoinFuncId, JoinIrPhase, MirLikeInst};
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue as MirConstValue, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::BTreeMap;
use std::mem;

/// Normalized → MIR の最小 direct 変換（Pattern1/2 ミニ＋JP mini/atoi mini）。
pub(crate) fn lower_normalized_direct_minimal(
    norm: &NormalizedModule,
) -> Result<MirModule, JoinIrVmBridgeError> {
    if norm.phase != JoinIrPhase::Normalized {
        return Err(JoinIrVmBridgeError::new(
            "[joinir/normalized-bridge/direct] expected Normalized JoinIR module",
        ));
    }

    let debug_dump = crate::mir::join_ir::normalized::dev_env::normalized_dev_logs_enabled();
    super::log_dev(
        "direct",
        format!(
            "using direct normalized bridge (functions={}, env_layouts={})",
            norm.functions.len(),
            norm.env_layouts.len()
        ),
        false,
    );

    let mut mir_module = MirModule::new("joinir_normalized_direct".to_string());

    for func in norm.functions.values() {
        let mir_func = lower_normalized_function_direct(func, norm)?;
        mir_module.add_function(mir_func);
    }

    if debug_dump {
        super::log_debug(
            "direct",
            format!("produced MIR (debug dump): {:#?}", mir_module),
        );
    }

    Ok(mir_module)
}

fn lower_normalized_function_direct(
    func: &JpFunction,
    norm: &NormalizedModule,
) -> Result<MirFunction, JoinIrVmBridgeError> {
    let env_fields = func
        .env_layout
        .and_then(|id| norm.env_layouts.iter().find(|layout| layout.id == id));

    let params: Vec<ValueId> = env_fields
        .map(|layout| {
            layout
                .fields
                .iter()
                .enumerate()
                .map(|(idx, f)| f.value_id.unwrap_or(ValueId(idx as u32)))
                .collect()
        })
        .unwrap_or_default();

    // Build a dense ValueId mapping to ensure VM registersはVoidにならない。
    let mut value_map: BTreeMap<ValueId, ValueId> = BTreeMap::new();
    for id in &params {
        remap_value(*id, &mut value_map);
    }
    for inst in &func.body {
        for vid in value_ids_in_inst(inst) {
            remap_value(vid, &mut value_map);
        }
    }

    let remap = |id: ValueId, map: &mut BTreeMap<ValueId, ValueId>| remap_value(id, map);
    let remap_vec = |ids: &[ValueId], map: &mut BTreeMap<ValueId, ValueId>| {
        ids.iter()
            .map(|id| remap_value(*id, map))
            .collect::<Vec<_>>()
    };

    let remapped_params = remap_vec(&params, &mut value_map);
    let param_types = vec![MirType::Unknown; remapped_params.len()];
    let signature = FunctionSignature {
        name: join_func_name(JoinFuncId(func.id.0)),
        params: param_types,
        return_type: MirType::Unknown,
        effects: EffectMask::PURE,
    };

    let mut mir_func = MirFunction::new(signature, BasicBlockId(0));
    if !remapped_params.is_empty() {
        mir_func.params = remapped_params.clone();
    }

    mir_func.next_value_id = value_map.len() as u32;

    let mut current_block_id = BasicBlockId(0);
    let mut next_block_id = 1;
    let mut current_insts: Vec<MirInstruction> = Vec::new();
    let mut terminated = false;

    super::log_debug(
        "direct",
        format!(
            "lowering fn={} params={:?} remapped_params={:?} body_len={}",
            func.name,
            params,
            remapped_params,
            func.body.len()
        ),
    );

    for inst in &func.body {
        if terminated {
            break;
        }

        match inst {
            JpInst::Let { dst, op, args } => {
                if matches!(op, JpOp::Select) {
                    let cond = remap(*args.get(0).unwrap_or(&ValueId(0)), &mut value_map);
                    let then_val = remap(*args.get(1).unwrap_or(&ValueId(0)), &mut value_map);
                    let else_val = remap(*args.get(2).unwrap_or(&ValueId(0)), &mut value_map);
                    let remapped_dst = remap(*dst, &mut value_map);

                    let then_bb = BasicBlockId(next_block_id);
                    next_block_id += 1;
                    let else_bb = BasicBlockId(next_block_id);
                    next_block_id += 1;
                    let merge_bb = BasicBlockId(next_block_id);
                    next_block_id += 1;

                        finalize_block(
                            &mut mir_func,
                            current_block_id,
                            mem::take(&mut current_insts),
                            MirInstruction::Branch {
                                condition: cond,
                                then_bb,
                                else_bb,
                                then_edge_args: None,
                                else_edge_args: None,
                            },
                            None,
                        );

                        finalize_block(
                            &mut mir_func,
                            then_bb,
                            Vec::new(),
                            MirInstruction::Jump {
                                target: merge_bb,
                                edge_args: None,
                            },
                            None,
                        );
                        finalize_block(
                            &mut mir_func,
                            else_bb,
                            Vec::new(),
                            MirInstruction::Jump {
                                target: merge_bb,
                                edge_args: None,
                            },
                            None,
                        );

                    current_block_id = merge_bb;
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let caller = std::panic::Location::caller();
                        let loc =
                            format!("{}:{}:{}", caller.file(), caller.line(), caller.column());
                        mir_func.metadata.value_origin_callers.insert(remapped_dst, loc);
                    }
                    current_insts = vec![MirInstruction::Phi {
                        dst: remapped_dst,
                        inputs: vec![(then_bb, then_val), (else_bb, else_val)],
                        type_hint: None,
                    }];
                    mir_func.next_value_id = value_map.len() as u32;
                    continue;
                }

                let remapped_dst = remap(*dst, &mut value_map);
                let remapped_args = remap_vec(args, &mut value_map);
                let mir_like = jp_op_to_mir_like(remapped_dst, op, &remapped_args)?;
                let mir_inst = convert_mir_like_inst(&mir_like)?;
                current_insts.push(mir_inst);
            }
            JpInst::EnvLoad { .. } | JpInst::EnvStore { .. } => {
                return Err(JoinIrVmBridgeError::new(
                    "[joinir/normalized-bridge/direct] EnvLoad/EnvStore not supported in minimal direct bridge",
                ));
            }
            JpInst::TailCallFn { target, env } => {
                let env_remapped = remap_vec(env, &mut value_map);
                let (instructions, terminator) =
                    build_tail_call(&mut mir_func, target, &env_remapped);
                finalize_block(
                    &mut mir_func,
                    current_block_id,
                    {
                        let mut insts = mem::take(&mut current_insts);
                        insts.extend(instructions);
                        insts
                    },
                    terminator,
                    None,
                );
                terminated = true;
            }
            JpInst::TailCallKont { env, .. } => {
                let env_remapped = remap_vec(env, &mut value_map);
                let return_val = env_remapped.first().copied();
                finalize_block(
                    &mut mir_func,
                    current_block_id,
                    mem::take(&mut current_insts),
                    MirInstruction::Return { value: return_val },
                    Some(env_remapped),
                );
                terminated = true;
            }
            JpInst::If {
                cond,
                then_target,
                else_target,
                env,
            } => {
                let then_bb = BasicBlockId(next_block_id);
                next_block_id += 1;
                let else_bb = BasicBlockId(next_block_id);
                next_block_id += 1;

                let cond_remapped = remap(*cond, &mut value_map);
                let env_remapped = remap_vec(env, &mut value_map);

                // Branch from current block
                finalize_block(
                    &mut mir_func,
                    current_block_id,
                    mem::take(&mut current_insts),
                    MirInstruction::Branch {
                        condition: cond_remapped,
                        then_bb,
                        else_bb,
                        then_edge_args: None,
                        else_edge_args: None,
                    },
                    None,
                );

                // Decide which branch continues the loop (self) and which exits.
                let (exit_bb, exit_target, cont_bb) = if then_target.0 == func.id.0 {
                    (else_bb, else_target, then_bb)
                } else if else_target.0 == func.id.0 {
                    (then_bb, then_target, else_bb)
                } else {
                    // Both branches exit; build both and stop.
                    build_exit_or_tail_branch(
                        &mut mir_func,
                        then_bb,
                        then_target,
                        &env_remapped,
                        func.id,
                    )?;
                    build_exit_or_tail_branch(
                        &mut mir_func,
                        else_bb,
                        else_target,
                        &env_remapped,
                        func.id,
                    )?;
                    terminated = true;
                    continue;
                };

                build_exit_or_tail_branch(
                    &mut mir_func,
                    exit_bb,
                    exit_target,
                    &env_remapped,
                    func.id,
                )?;

                mir_func
                    .blocks
                    .entry(cont_bb)
                    .or_insert_with(|| BasicBlock::new(cont_bb));
                current_block_id = cont_bb;
                current_insts = Vec::new();
            }
        }
    }

    if !terminated {
        // Flush remaining instructions into the current block and end with Return None
        let block = mir_func
            .blocks
            .entry(current_block_id)
            .or_insert_with(|| BasicBlock::new(current_block_id));
        if !current_insts.is_empty() {
            block.instructions = current_insts;
            block.instruction_spans = vec![Span::unknown(); block.instructions.len()];
        }
        if block.terminator.is_none() {
            block.set_terminator(MirInstruction::Return { value: None });
        }
    }

    Ok(mir_func)
}

fn jp_op_to_mir_like(
    dst: ValueId,
    op: &JpOp,
    args: &[ValueId],
) -> Result<MirLikeInst, JoinIrVmBridgeError> {
    match op {
        JpOp::Const(v) => Ok(MirLikeInst::Const {
            dst,
            value: v.clone(),
        }),
        JpOp::BinOp(op) => Ok(MirLikeInst::BinOp {
            dst,
            op: *op,
            lhs: args.get(0).copied().unwrap_or(ValueId(0)),
            rhs: args.get(1).copied().unwrap_or(ValueId(0)),
        }),
        JpOp::Unary(op) => Ok(MirLikeInst::UnaryOp {
            dst,
            op: *op,
            operand: args.get(0).copied().unwrap_or(ValueId(0)),
        }),
        JpOp::Compare(op) => Ok(MirLikeInst::Compare {
            dst,
            op: *op,
            lhs: args.get(0).copied().unwrap_or(ValueId(0)),
            rhs: args.get(1).copied().unwrap_or(ValueId(0)),
        }),
        JpOp::Select => {
            let cond = args.get(0).copied().unwrap_or(ValueId(0));
            let then_val = args.get(1).copied().unwrap_or(ValueId(0));
            let else_val = args.get(2).copied().unwrap_or(ValueId(0));
            Ok(MirLikeInst::Select {
                dst,
                cond,
                then_val,
                else_val,
            })
        }
        JpOp::BoxCall { box_name, method } => Ok(MirLikeInst::BoxCall {
            dst: Some(dst),
            box_name: box_name.clone(),
            method: method.clone(),
            args: args.to_vec(),
        }),
    }
}

fn build_tail_call(
    mir_func: &mut MirFunction,
    target: &JpFuncId,
    env: &[ValueId],
) -> (Vec<MirInstruction>, MirInstruction) {
    let func_name_id = mir_func.next_value_id();
    let result_id = mir_func.next_value_id();
    let func_name = join_func_name(JoinFuncId(target.0));

    let mut instructions = Vec::new();
    instructions.push(MirInstruction::Const {
        dst: func_name_id,
        value: MirConstValue::String(func_name),
    });
    instructions.push(MirInstruction::Call {
        dst: Some(result_id),
        func: func_name_id,
        callee: None,
        args: env.to_vec(),
        effects: EffectMask::PURE,
    });

    (
        instructions,
        MirInstruction::Return {
            value: Some(result_id),
        },
    )
}

fn build_exit_or_tail_branch(
    mir_func: &mut MirFunction,
    block_id: BasicBlockId,
    target: &JpFuncId,
    env: &[ValueId],
    self_id: JpFuncId,
) -> Result<(), JoinIrVmBridgeError> {
    if target.0 == self_id.0 {
        // Continue the loop: tail call self with env, then return its result
        let (insts, term) = build_tail_call(mir_func, target, env);
        let block = mir_func
            .blocks
            .entry(block_id)
            .or_insert_with(|| BasicBlock::new(block_id));
        block.instructions = insts;
        block.instruction_spans = vec![Span::unknown(); block.instructions.len()];
        block.set_terminator(term);
        return Ok(());
    }

    // Exit: return first env arg (Pattern2 minis pass loop state in env[0])
    let ret_val = env.first().copied();
    let block = mir_func
        .blocks
        .entry(block_id)
        .or_insert_with(|| BasicBlock::new(block_id));
    block.instructions.clear();
    block.instruction_spans.clear();
    block.set_terminator(MirInstruction::Return { value: ret_val });
    block.set_return_env(crate::mir::EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: env.to_vec(),
    });
    Ok(())
}

fn value_ids_in_inst(inst: &JpInst) -> Vec<ValueId> {
    match inst {
        JpInst::Let { dst, args, .. } => {
            let mut ids = vec![*dst];
            ids.extend(args.iter().copied());
            ids
        }
        JpInst::EnvLoad { dst, env, .. } => vec![*dst, *env],
        JpInst::EnvStore { env, src, .. } => vec![*env, *src],
        JpInst::TailCallFn { env, .. } | JpInst::TailCallKont { env, .. } => env.clone(),
        JpInst::If { cond, env, .. } => {
            let mut ids = vec![*cond];
            ids.extend(env.iter().copied());
            ids
        }
    }
}

fn remap_value(id: ValueId, map: &mut BTreeMap<ValueId, ValueId>) -> ValueId {
    if let Some(mapped) = map.get(&id) {
        return *mapped;
    }
    let next = ValueId(map.len() as u32);
    map.insert(id, next);
    next
}

fn finalize_block(
    mir_func: &mut MirFunction,
    block_id: BasicBlockId,
    instructions: Vec<MirInstruction>,
    terminator: MirInstruction,
    jump_args: Option<Vec<ValueId>>,
) {
    let block = mir_func
        .blocks
        .entry(block_id)
        .or_insert_with(|| BasicBlock::new(block_id));
    block.instructions = instructions;
    block.instruction_spans = vec![Span::unknown(); block.instructions.len()];
    match terminator {
        MirInstruction::Jump { target, edge_args: None } => {
            if let Some(args) = jump_args {
                block.set_jump_with_edge_args(
                    target,
                    Some(crate::mir::EdgeArgs {
                        layout: JumpArgsLayout::CarriersOnly,
                        values: args,
                    }),
                );
            } else {
                block.set_terminator(MirInstruction::Jump {
                    target,
                    edge_args: None,
                });
            }
        }
        MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
        } => {
            block.set_branch_with_edge_args(
                condition,
                then_bb,
                then_edge_args,
                else_bb,
                else_edge_args,
            );
        }
        MirInstruction::Return { .. } => {
            block.set_terminator(terminator);
            if let Some(args) = jump_args {
                block.set_return_env(crate::mir::EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: args,
                });
            }
        }
        other => {
            block.set_terminator(other);
        }
    }
}
