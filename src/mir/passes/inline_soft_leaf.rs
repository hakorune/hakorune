use crate::mir::{BasicBlock, Callee, EffectMask, MirFunction, MirInstruction, MirModule, ValueId};
use std::collections::{BTreeMap, BTreeSet};

const SOFT_LEAF_MAX_INSTRUCTIONS: usize = 8;

#[derive(Debug, Clone)]
struct LeafInlineBody {
    params: Vec<ValueId>,
    instructions: Vec<MirInstruction>,
    return_value: Option<ValueId>,
    value_types: BTreeMap<ValueId, crate::mir::MirType>,
}

/// Apply advisory `Hint(inline)` plans for narrow same-module leaf functions.
///
/// This is a best-effort transform. Any unsupported shape keeps the original
/// call intact; required inline and verifier-backed failures belong to a later
/// row.
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
        if !has_soft_inline_request(function) {
            continue;
        }
        if let Some(body) = leaf_inline_body(function) {
            candidates.insert(name.clone(), body);
        }
    }
    candidates
}

fn has_soft_inline_request(function: &MirFunction) -> bool {
    let mut has_prefer = false;
    for plan in &function.metadata.inline_plans {
        match plan.request {
            crate::mir::inline_plan::InlineRequest::Avoid => return false,
            crate::mir::inline_plan::InlineRequest::Prefer => has_prefer = true,
            crate::mir::inline_plan::InlineRequest::None => {}
        }
    }
    has_prefer
}

fn leaf_inline_body(function: &MirFunction) -> Option<LeafInlineBody> {
    if function.blocks.len() != 1 {
        return None;
    }
    let block = function.blocks.get(&function.entry_block)?;
    if block.return_env.is_some() || block.return_env_layout.is_some() {
        return None;
    }
    if block.instructions.len() > SOFT_LEAF_MAX_INSTRUCTIONS {
        return None;
    }
    if !block.instructions.iter().all(is_supported_leaf_instruction) {
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
    })
}

fn is_supported_leaf_instruction(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Const { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::StaticDataLoad { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::TypeOp { .. }
    ) && inst.effects().is_pure()
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
            if let Some((callee_name, dst, args)) = inlineable_call(inst) {
                if callee_name != function_name {
                    if let Some(body) = candidates.get(callee_name) {
                        if let Some(expanded) = expand_leaf_call(function, body, *dst, args) {
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

fn inlineable_call(inst: &MirInstruction) -> Option<(&str, &Option<ValueId>, &[ValueId])> {
    let MirInstruction::Call {
        dst,
        callee: Some(Callee::Global(name)),
        args,
        ..
    } = inst
    else {
        return None;
    };
    Some((name.as_str(), dst, args.as_slice()))
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
mod tests {
    use super::*;
    use crate::ast::RuneAttr;
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirType,
    };

    fn make_add1_inline_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.add1/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let one = function.next_value_id();
        let sum = function.next_value_id();
        let mut entry = BasicBlock::new(BasicBlockId(0));
        entry.add_instruction(MirInstruction::Const {
            dst: one,
            value: ConstValue::Integer(1),
        });
        entry.add_instruction(MirInstruction::BinOp {
            dst: sum,
            op: BinaryOp::Add,
            lhs: ValueId(0),
            rhs: one,
        });
        entry.add_instruction(MirInstruction::Return { value: Some(sum) });
        function.blocks.insert(BasicBlockId(0), entry);
        function.metadata.runes = vec![RuneAttr {
            name: "Hint".to_string(),
            args: vec!["inline".to_string()],
        }];
        crate::mir::inline_plan::refresh_function_inline_plans(&mut function);
        function
    }

    fn make_main_calling_add1() -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.main/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let arg = function.next_value_id();
        let result = function.next_value_id();
        let mut entry = BasicBlock::new(BasicBlockId(0));
        entry.add_instruction(MirInstruction::Const {
            dst: arg,
            value: ConstValue::Integer(41),
        });
        entry.add_instruction(MirInstruction::Call {
            dst: Some(result),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Main.add1/1".to_string())),
            args: vec![arg],
            effects: EffectMask::PURE,
        });
        entry.add_instruction(MirInstruction::Return {
            value: Some(result),
        });
        function.blocks.insert(BasicBlockId(0), entry);
        function
    }

    #[test]
    fn inline_soft_leaf_rewrites_same_module_prefer_global_call() {
        let mut module = MirModule::new("inline_soft_leaf_test".to_string());
        module.add_function(make_add1_inline_function());
        module.add_function(make_main_calling_add1());

        assert_eq!(apply(&mut module), 1);

        let main = module.get_function("Main.main/0").expect("main function");
        let entry = main.entry_block();
        assert!(!entry
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Call { .. })));
        assert!(entry.instructions.iter().any(|inst| matches!(
            inst,
            MirInstruction::Copy {
                dst: ValueId(2),
                ..
            }
        )));
    }

    #[test]
    fn inline_soft_leaf_keeps_call_without_prefer_plan() {
        let mut module = MirModule::new("inline_soft_leaf_no_plan_test".to_string());
        let mut callee = make_add1_inline_function();
        callee.metadata.runes.clear();
        crate::mir::inline_plan::refresh_function_inline_plans(&mut callee);
        module.add_function(callee);
        module.add_function(make_main_calling_add1());

        assert_eq!(apply(&mut module), 0);

        let main = module.get_function("Main.main/0").expect("main function");
        assert!(main
            .entry_block()
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Call { .. })));
    }

    #[test]
    fn inline_soft_leaf_keeps_recursive_call() {
        let mut module = MirModule::new("inline_soft_leaf_recursive_test".to_string());
        let mut function = make_add1_inline_function();
        function.signature.name = "Main.main/0".to_string();
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .expect("entry")
            .instructions
            .push(MirInstruction::Call {
                dst: None,
                func: ValueId::INVALID,
                callee: Some(Callee::Global("Main.main/0".to_string())),
                args: vec![],
                effects: EffectMask::PURE,
            });
        module.add_function(function);

        assert_eq!(apply(&mut module), 0);
    }
}
