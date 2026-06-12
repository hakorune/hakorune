use crate::mir::{BasicBlock, Callee, EffectMask, MirFunction, MirInstruction, MirModule, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
struct LeafInlineBody {
    params: Vec<ValueId>,
    instructions: Vec<MirInstruction>,
    return_value: Option<ValueId>,
    value_types: BTreeMap<ValueId, crate::mir::MirType>,
    allow_receiver_fieldset_leaf: bool,
}

#[derive(Debug, Clone)]
struct InlineCall {
    callee_name: String,
    dst: Option<ValueId>,
    args: Vec<ValueId>,
}

/// Apply inline plans for narrow same-module leaf functions.
///
/// Advisory `Hint(inline)` remains best-effort. Verified required inline plans
/// are also consumed here so allocator/substrate fast paths reach backends as
/// already-expanded MIR instead of asking a backend to inline.
pub fn apply(module: &mut MirModule) -> usize {
    let candidates = collect_leaf_candidates(module);
    if candidates.is_empty() {
        return 0;
    }

    let mut rewrites = 0;
    let function_names: Vec<String> = module.functions.keys().cloned().collect();
    for function_name in function_names {
        let Some(function) = module.functions.get_mut(&function_name) else {
            continue;
        };
        rewrites += inline_calls_in_function(&function_name, function, &candidates);
    }
    rewrites
}

fn collect_leaf_candidates(module: &MirModule) -> BTreeMap<String, LeafInlineBody> {
    let mut candidates = BTreeMap::new();
    for (name, function) in &module.functions {
        let Some(mode) = leaf_inline_mode(function) else {
            continue;
        };
        if let Some(body) = leaf_inline_body(function, mode.allow_receiver_fieldset_leaf) {
            candidates.insert(name.clone(), body);
        }
    }
    candidates
}

#[derive(Debug, Clone, Copy)]
struct LeafInlineMode {
    allow_receiver_fieldset_leaf: bool,
}

fn leaf_inline_mode(function: &MirFunction) -> Option<LeafInlineMode> {
    let mut has_prefer = false;
    let mut has_verified_required = false;
    for plan in &function.metadata.inline_plans {
        match plan.request {
            crate::mir::inline_plan::InlineRequest::Avoid => return None,
            crate::mir::inline_plan::InlineRequest::Prefer => has_prefer = true,
            crate::mir::inline_plan::InlineRequest::None => {}
            crate::mir::inline_plan::InlineRequest::Required if plan.verified => {
                has_verified_required = true
            }
            crate::mir::inline_plan::InlineRequest::Required => {}
        }
    }
    if has_verified_required {
        Some(LeafInlineMode {
            allow_receiver_fieldset_leaf: true,
        })
    } else if has_prefer {
        Some(LeafInlineMode {
            allow_receiver_fieldset_leaf: false,
        })
    } else {
        None
    }
}

fn leaf_inline_body(
    function: &MirFunction,
    allow_receiver_fieldset_leaf: bool,
) -> Option<LeafInlineBody> {
    if function.blocks.len() != 1 {
        return None;
    }
    let block = function.blocks.get(&function.entry_block)?;
    if block.return_env.is_some() || block.return_env_layout.is_some() {
        return None;
    }
    let budget = if allow_receiver_fieldset_leaf {
        crate::mir::inline_leaf::DEFAULT_REQUIRED_LEAF_INLINE_MAX_INSTRUCTIONS
    } else {
        crate::mir::inline_leaf::DEFAULT_LEAF_INLINE_MAX_INSTRUCTIONS
    };
    if block.instructions.len() > budget {
        return None;
    }
    if !allow_receiver_fieldset_leaf
        && !crate::mir::inline_leaf::check_leaf_inline_shape(function, None).is_empty()
    {
        return None;
    }
    if allow_receiver_fieldset_leaf
        && !crate::mir::inline_leaf::check_required_inline_shape(function, None).is_empty()
    {
        return None;
    }
    let Some(MirInstruction::Return { value }) = block.terminator else {
        return None;
    };
    Some(LeafInlineBody {
        params: function.params.clone(),
        instructions: block.instructions.clone(),
        return_value: value,
        value_types: function.metadata.value_types.clone(),
        allow_receiver_fieldset_leaf,
    })
}

fn inline_calls_in_function(
    function_name: &str,
    function: &mut MirFunction,
    candidates: &BTreeMap<String, LeafInlineBody>,
) -> usize {
    let block_ids = function.block_ids();
    let mut rewrites = 0;

    for block_id in block_ids {
        let Some(block_snapshot) = function.blocks.get(&block_id).cloned() else {
            continue;
        };
        let mut changed = false;
        let mut next_instructions = Vec::with_capacity(block_snapshot.instructions.len());
        let mut next_spans = Vec::with_capacity(block_snapshot.instruction_spans.len());

        for (idx, inst) in block_snapshot.instructions.iter().enumerate() {
            let span = block_snapshot
                .instruction_spans
                .get(idx)
                .copied()
                .unwrap_or_else(crate::ast::Span::unknown);
            if let Some(call) = inlineable_call(inst) {
                if call.callee_name != function_name {
                    if let Some(body) = candidates.get(&call.callee_name) {
                        if let Some(expanded) =
                            expand_leaf_call(function, body, call.dst, &call.args)
                        {
                            let expanded_len = expanded.len();
                            next_instructions.extend(expanded);
                            next_spans.extend(std::iter::repeat(span).take(expanded_len));
                            rewrites += 1;
                            changed = true;
                            continue;
                        }
                    }
                }
            }
            next_instructions.push(inst.clone());
            next_spans.push(span);
        }

        if changed {
            if let Some(block) = function.blocks.get_mut(&block_id) {
                block.instructions = next_instructions;
                block.instruction_spans = next_spans;
                recompute_block_effects(block);
            }
        }
    }

    rewrites
}

fn inlineable_call(inst: &MirInstruction) -> Option<InlineCall> {
    let MirInstruction::Call {
        dst,
        callee: Some(callee),
        args,
        ..
    } = inst
    else {
        return None;
    };
    match callee {
        Callee::Global(name) => Some(InlineCall {
            callee_name: name.clone(),
            dst: *dst,
            args: args.clone(),
        }),
        Callee::Method {
            box_name,
            method,
            receiver: Some(receiver),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        } => {
            let mut call_args = Vec::with_capacity(args.len() + 1);
            call_args.push(*receiver);
            call_args.extend(args.iter().copied());
            Some(InlineCall {
                callee_name: format!("{}.{}/{}", box_name, method, args.len()),
                dst: *dst,
                args: call_args,
            })
        }
        _ => None,
    }
}

fn expand_leaf_call(
    caller: &mut MirFunction,
    body: &LeafInlineBody,
    call_dst: Option<ValueId>,
    args: &[ValueId],
) -> Option<Vec<MirInstruction>> {
    if body.params.len() != args.len() {
        return None;
    }
    if call_dst.is_some() && body.return_value.is_none() {
        return None;
    }

    let mut value_map = BTreeMap::new();
    for (param, arg) in body.params.iter().copied().zip(args.iter().copied()) {
        value_map.insert(param, arg);
    }

    let mut expanded =
        Vec::with_capacity(body.instructions.len() + usize::from(call_dst.is_some()));
    let mut mapped_dsts = BTreeSet::new();
    for inst in &body.instructions {
        expanded.push(remap_leaf_instruction(
            caller,
            body,
            inst,
            &mut value_map,
            &mut mapped_dsts,
        )?);
    }

    if let (Some(dst), Some(ret)) = (call_dst, body.return_value) {
        let src = *value_map.get(&ret)?;
        caller.metadata.value_types.entry(dst).or_insert_with(|| {
            body.value_types
                .get(&ret)
                .cloned()
                .unwrap_or(crate::mir::MirType::Unknown)
        });
        expanded.push(MirInstruction::Copy { dst, src });
    }

    Some(expanded)
}

fn remap_leaf_instruction(
    caller: &mut MirFunction,
    body: &LeafInlineBody,
    inst: &MirInstruction,
    value_map: &mut BTreeMap<ValueId, ValueId>,
    mapped_dsts: &mut BTreeSet<ValueId>,
) -> Option<MirInstruction> {
    let mut alloc_dst = |old: ValueId,
                         value_map: &mut BTreeMap<ValueId, ValueId>,
                         mapped_dsts: &mut BTreeSet<ValueId>| {
        if mapped_dsts.contains(&old) {
            return None;
        }
        let new = caller.next_value_id();
        mapped_dsts.insert(old);
        value_map.insert(old, new);
        if let Some(ty) = body.value_types.get(&old).cloned() {
            caller.metadata.value_types.insert(new, ty);
        }
        Some(new)
    };
    let map = |old: ValueId, value_map: &BTreeMap<ValueId, ValueId>| value_map.get(&old).copied();

    match inst {
        MirInstruction::Const { dst, value } => Some(MirInstruction::Const {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            value: value.clone(),
        }),
        MirInstruction::UnaryOp { dst, op, operand } => Some(MirInstruction::UnaryOp {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            op: *op,
            operand: map(*operand, value_map)?,
        }),
        MirInstruction::BinOp { dst, op, lhs, rhs } => Some(MirInstruction::BinOp {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            op: *op,
            lhs: map(*lhs, value_map)?,
            rhs: map(*rhs, value_map)?,
        }),
        MirInstruction::Compare { dst, op, lhs, rhs } => Some(MirInstruction::Compare {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            op: *op,
            lhs: map(*lhs, value_map)?,
            rhs: map(*rhs, value_map)?,
        }),
        MirInstruction::StaticDataLoad {
            dst,
            source_name,
            symbol,
            element,
            len,
            align,
            index,
        } => Some(MirInstruction::StaticDataLoad {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            source_name: source_name.clone(),
            symbol: symbol.clone(),
            element: element.clone(),
            len: *len,
            align: *align,
            index: map(*index, value_map)?,
        }),
        MirInstruction::Copy { dst, src } => Some(MirInstruction::Copy {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            src: map(*src, value_map)?,
        }),
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => Some(MirInstruction::Select {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            cond: map(*cond, value_map)?,
            then_val: map(*then_val, value_map)?,
            else_val: map(*else_val, value_map)?,
        }),
        MirInstruction::TypeOp { dst, op, value, ty } => Some(MirInstruction::TypeOp {
            dst: alloc_dst(*dst, value_map, mapped_dsts)?,
            op: *op,
            value: map(*value, value_map)?,
            ty: ty.clone(),
        }),
        MirInstruction::FieldSet {
            base,
            field,
            value,
            declared_type,
        } if body.allow_receiver_fieldset_leaf => {
            let base = if *base == ValueId::INVALID {
                ValueId::INVALID
            } else {
                map(*base, value_map)?
            };
            Some(MirInstruction::FieldSet {
                base,
                field: field.clone(),
                value: map(*value, value_map)?,
                declared_type: declared_type.clone(),
            })
        }
        MirInstruction::FieldGet {
            dst,
            base,
            field,
            declared_type,
        } if body.allow_receiver_fieldset_leaf => {
            let base = if *base == ValueId::INVALID {
                ValueId::INVALID
            } else {
                map(*base, value_map)?
            };
            Some(MirInstruction::FieldGet {
                dst: alloc_dst(*dst, value_map, mapped_dsts)?,
                base,
                field: field.clone(),
                declared_type: declared_type.clone(),
            })
        }
        _ => None,
    }
}

fn recompute_block_effects(block: &mut BasicBlock) {
    let mut effects = EffectMask::PURE;
    for instruction in &block.instructions {
        effects = effects | instruction.effects();
    }
    if let Some(terminator) = &block.terminator {
        effects = effects | terminator.effects();
    }
    block.effects = effects;
}

#[cfg(test)]
mod tests;
