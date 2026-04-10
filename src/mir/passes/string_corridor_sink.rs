//! Borrowed string corridor sinking pilot.
//!
//! First real transforms for the string corridor lane:
//! `substring(...).length()` and retained-slice `length()` consumers are
//! rewritten into a single direct extern call so the corridor can stay
//! borrowed without forcing publication/materialization.
//! Complementary `substring_len_hii` pairs can then fuse back to one source
//! length add when the compiler can prove they partition the same source.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::mir::{
    refresh_function_string_corridor_candidates, refresh_function_string_corridor_facts,
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction, MirInstruction,
    MirModule, StringCorridorOp, ValueId,
};

pub const SUBSTRING_LEN_EXTERN: &str = "nyash.string.substring_len_hii";

pub fn sink_borrowed_string_corridors(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for (_name, function) in &mut module.functions {
        rewritten += sink_borrowed_string_corridors_in_function(function);
    }
    rewritten
}

fn sink_borrowed_string_corridors_in_function(function: &mut MirFunction) -> usize {
    refresh_function_string_corridor_facts(function);

    let def_map = build_def_map(function);
    let use_counts = build_use_counts(function);
    let plans_by_block = collect_plans(function, &def_map, &use_counts);
    let mut rewritten = apply_plans(function, plans_by_block);

    let def_map = build_def_map(function);
    let retained_len_plans = collect_retained_len_plans(function, &def_map);
    rewritten += apply_retained_len_plans(function, retained_len_plans);

    let def_map = build_def_map(function);
    let use_counts = build_use_counts(function);
    let fusion_plans =
        collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
    rewritten += apply_complementary_len_fusion_plans(function, fusion_plans);

    rewritten
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubstringLenPlan {
    inner_idx: usize,
    inner_dst: ValueId,
    outer_idx: usize,
    outer_dst: ValueId,
    source: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RetainedSubstringLenPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    source: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComplementarySubstringLenFusionPlan {
    remove_indices: Vec<usize>,
    outer_idx: usize,
    outer_dst: ValueId,
    acc: ValueId,
    source_len: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SingleUseCopyChain {
    root: ValueId,
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AddShape {
    idx: usize,
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubstringLenCallShape {
    idx: usize,
    dst: ValueId,
    source: ValueId,
    start: ValueId,
    end: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringSourceIdentity {
    Value(ValueId),
    ConstString(String),
}

fn build_def_map(function: &MirFunction) -> HashMap<ValueId, (BasicBlockId, usize)> {
    let mut defs: HashMap<ValueId, (BasicBlockId, usize)> = HashMap::new();
    for (bbid, block) in &function.blocks {
        for (idx, inst) in block.instructions.iter().enumerate() {
            if let Some(dst) = inst.dst_value() {
                defs.insert(dst, (*bbid, idx));
            }
        }
    }
    defs
}

fn build_use_counts(function: &MirFunction) -> HashMap<ValueId, usize> {
    let mut uses: HashMap<ValueId, usize> = HashMap::new();
    for block in function.blocks.values() {
        for inst in &block.instructions {
            for vid in inst.used_values() {
                *uses.entry(vid).or_insert(0) += 1;
            }
        }
        if let Some(term) = &block.terminator {
            for vid in term.used_values() {
                *uses.entry(vid).or_insert(0) += 1;
            }
        }
        for edge in block.out_edges() {
            if let Some(args) = edge.args {
                for vid in args.values {
                    *uses.entry(vid).or_insert(0) += 1;
                }
            }
        }
    }
    uses
}

fn collect_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<SubstringLenPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<SubstringLenPlan>> = BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans: Vec<SubstringLenPlan> = Vec::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            let Some((outer_dst, receiver, effects)) = match_len_call(inst) else {
                continue;
            };

            let receiver_root = resolve_copy_chain_source(function, def_map, receiver);
            if use_counts.get(&receiver_root).copied().unwrap_or(0) != 1 {
                continue;
            }

            let Some((inner_bbid, inner_idx)) = def_map.get(&receiver_root).copied() else {
                continue;
            };
            if inner_bbid != *bbid || inner_idx >= outer_idx {
                continue;
            }

            let Some(inner_fact) = function.metadata.string_corridor_facts.get(&receiver_root)
            else {
                continue;
            };
            if inner_fact.op != StringCorridorOp::StrSlice {
                continue;
            }

            let Some((source, start, end)) = extract_substring_args(&block.instructions[inner_idx])
            else {
                continue;
            };

            plans.push(SubstringLenPlan {
                inner_idx,
                inner_dst: receiver,
                outer_idx,
                outer_dst,
                source,
                start,
                end,
                effects,
            });
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

fn resolve_copy_chain_source(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    mut value: ValueId,
) -> ValueId {
    let mut visited: BTreeSet<ValueId> = BTreeSet::new();
    while visited.insert(value) {
        let Some((bbid, idx)) = def_map.get(&value).copied() else {
            break;
        };
        let Some(block) = function.blocks.get(&bbid) else {
            break;
        };
        let Some(inst) = block.instructions.get(idx) else {
            break;
        };
        match inst {
            MirInstruction::Copy { src, .. } => {
                value = *src;
            }
            _ => break,
        }
    }
    value
}

fn resolve_single_use_copy_chain_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    mut value: ValueId,
) -> Option<SingleUseCopyChain> {
    let mut visited: BTreeSet<ValueId> = BTreeSet::new();
    let mut copy_indices = Vec::new();

    while visited.insert(value) {
        if use_counts.get(&value).copied().unwrap_or(0) != 1 {
            return None;
        }
        let Some((inst_bbid, idx)) = def_map.get(&value).copied() else {
            break;
        };
        if inst_bbid != bbid {
            break;
        }
        let Some(block) = function.blocks.get(&inst_bbid) else {
            break;
        };
        let Some(inst) = block.instructions.get(idx) else {
            break;
        };
        match inst {
            MirInstruction::Copy { src, .. } => {
                copy_indices.push(idx);
                value = *src;
            }
            _ => break,
        }
    }

    Some(SingleUseCopyChain {
        root: value,
        copy_indices,
    })
}

fn match_add_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<AddShape> {
    let (inst_bbid, idx) = def_map.get(&value).copied()?;
    if inst_bbid != bbid {
        return None;
    }
    let block = function.blocks.get(&inst_bbid)?;
    match block.instructions.get(idx)? {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } => Some(AddShape {
            idx,
            dst: *dst,
            lhs: *lhs,
            rhs: *rhs,
        }),
        _ => None,
    }
}

fn match_substring_len_call_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<SubstringLenCallShape> {
    let (inst_bbid, idx) = def_map.get(&value).copied()?;
    if inst_bbid != bbid {
        return None;
    }
    let block = function.blocks.get(&inst_bbid)?;
    match_substring_len_call(block.instructions.get(idx)?).map(|(dst, source, start, end)| {
        SubstringLenCallShape {
            idx,
            dst,
            source,
            start,
            end,
        }
    })
}

fn match_len_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, EffectMask)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            effects,
            ..
        } if args.is_empty() && matches!(method.as_str(), "length" | "len") => {
            Some((*dst, *receiver, *effects))
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            effects,
            ..
        } if args.len() == 1 && name == "nyash.string.len_h" => Some((*dst, args[0], *effects)),
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            args,
            effects,
            ..
        } if args.len() == 1 && matches!(name.as_str(), "str.len" | "__str.len") => {
            Some((*dst, args[0], *effects))
        }
        _ => None,
    }
}

fn match_substring_len_call(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == SUBSTRING_LEN_EXTERN => Some((*dst, args[0], args[1], args[2])),
        _ => None,
    }
}

fn extract_substring_args(inst: &MirInstruction) -> Option<(ValueId, ValueId, ValueId)> {
    match inst {
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(source),
                    ..
                }),
            args,
            ..
        } if args.len() == 2 && matches!(method.as_str(), "substring" | "slice") => {
            Some((*source, args[0], args[1]))
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == "nyash.string.substring_hii" => {
            Some((args[0], args[1], args[2]))
        }
        _ => None,
    }
}

fn value_is_const_i64(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    expected: i64,
) -> bool {
    let root = resolve_copy_chain_source(function, def_map, value);
    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return false;
    };
    matches!(
        block.instructions.get(idx),
        Some(MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        }) if *actual == expected
    )
}

fn string_source_identity(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<StringSourceIdentity> {
    let root = resolve_copy_chain_source(function, def_map, value);
    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return Some(StringSourceIdentity::Value(root));
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return Some(StringSourceIdentity::Value(root));
    };
    match block.instructions.get(idx) {
        Some(MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        }) => Some(StringSourceIdentity::ConstString(text.clone())),
        _ => Some(StringSourceIdentity::Value(root)),
    }
}

fn match_source_length_value(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    source_identity: &StringSourceIdentity,
    candidate: ValueId,
) -> Option<ValueId> {
    let length_root = resolve_copy_chain_source(function, def_map, candidate);
    let (bbid, idx) = def_map.get(&length_root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let (_, receiver, _) = match_len_call(block.instructions.get(idx)?)?;
    let receiver_identity = string_source_identity(function, def_map, receiver)?;
    if &receiver_identity == source_identity {
        Some(length_root)
    } else {
        None
    }
}

fn complementary_pair_source_len(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    lhs: &SubstringLenCallShape,
    rhs: &SubstringLenCallShape,
) -> Option<ValueId> {
    let lhs_source = string_source_identity(function, def_map, lhs.source)?;
    let rhs_source = string_source_identity(function, def_map, rhs.source)?;
    if lhs_source != rhs_source {
        return None;
    }

    if value_is_const_i64(function, def_map, lhs.start, 0) {
        let mid = resolve_copy_chain_source(function, def_map, lhs.end);
        let rhs_start = resolve_copy_chain_source(function, def_map, rhs.start);
        if rhs_start != mid {
            return None;
        }
        return match_source_length_value(function, def_map, &lhs_source, rhs.end);
    }

    if value_is_const_i64(function, def_map, rhs.start, 0) {
        let mid = resolve_copy_chain_source(function, def_map, rhs.end);
        let lhs_start = resolve_copy_chain_source(function, def_map, lhs.start);
        if lhs_start != mid {
            return None;
        }
        return match_source_length_value(function, def_map, &lhs_source, lhs.end);
    }

    None
}

fn try_match_complementary_len_fusion_plan(
    function: &MirFunction,
    bbid: BasicBlockId,
    outer_idx: usize,
    outer_dst: ValueId,
    inner_leaf: ValueId,
    other_leaf: ValueId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> Option<ComplementarySubstringLenFusionPlan> {
    let inner_chain =
        resolve_single_use_copy_chain_in_block(function, bbid, def_map, use_counts, inner_leaf)?;
    let other_chain =
        resolve_single_use_copy_chain_in_block(function, bbid, def_map, use_counts, other_leaf)?;
    let inner_add = match_add_in_block(function, bbid, def_map, inner_chain.root)?;
    if inner_add.idx >= outer_idx || inner_add.dst != inner_chain.root {
        return None;
    }

    for (acc_leaf, first_leaf) in [(inner_add.lhs, inner_add.rhs), (inner_add.rhs, inner_add.lhs)] {
        let first_chain = match resolve_single_use_copy_chain_in_block(
            function, bbid, def_map, use_counts, first_leaf,
        ) {
            Some(chain) => chain,
            None => continue,
        };
        let first_call = match match_substring_len_call_in_block(function, bbid, def_map, first_chain.root) {
            Some(call) => call,
            None => continue,
        };
        let second_call =
            match match_substring_len_call_in_block(function, bbid, def_map, other_chain.root) {
                Some(call) => call,
                None => continue,
            };
        let Some(source_len) =
            complementary_pair_source_len(function, def_map, &first_call, &second_call)
        else {
            continue;
        };

        let acc = resolve_copy_chain_source(function, def_map, acc_leaf);
        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        remove_indices.insert(first_call.idx);
        remove_indices.insert(second_call.idx);
        remove_indices.insert(inner_add.idx);
        remove_indices.extend(first_chain.copy_indices.iter().copied());
        remove_indices.extend(other_chain.copy_indices.iter().copied());
        remove_indices.extend(inner_chain.copy_indices.iter().copied());

        return Some(ComplementarySubstringLenFusionPlan {
            remove_indices: remove_indices.into_iter().collect(),
            outer_idx,
            outer_dst,
            acc,
            source_len,
        });
    }

    None
}

fn collect_complementary_len_fusion_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<ComplementarySubstringLenFusionPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<ComplementarySubstringLenFusionPlan>> =
        BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans = Vec::new();
        let mut occupied: BTreeSet<usize> = BTreeSet::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            if occupied.contains(&outer_idx) {
                continue;
            }
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Add,
                lhs,
                rhs,
            } = inst
            else {
                continue;
            };

            let Some(plan) = try_match_complementary_len_fusion_plan(
                function,
                *bbid,
                outer_idx,
                *dst,
                *lhs,
                *rhs,
                def_map,
                use_counts,
            )
            .or_else(|| {
                try_match_complementary_len_fusion_plan(
                    function,
                    *bbid,
                    outer_idx,
                    *dst,
                    *rhs,
                    *lhs,
                    def_map,
                    use_counts,
                )
            }) else {
                continue;
            };

            if plan
                .remove_indices
                .iter()
                .any(|idx| occupied.contains(idx))
            {
                continue;
            }
            for idx in &plan.remove_indices {
                occupied.insert(*idx);
            }
            occupied.insert(plan.outer_idx);
            plans.push(plan);
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

fn apply_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<SubstringLenPlan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        let mut replacements: BTreeMap<usize, MirInstruction> = BTreeMap::new();
        let mut removed_values: Vec<ValueId> = Vec::new();

        for plan in plans {
            remove_indices.insert(plan.inner_idx);
            removed_values.push(plan.inner_dst);
            replacements.insert(
                plan.outer_idx,
                MirInstruction::Call {
                    dst: Some(plan.outer_dst),
                    func: ValueId::INVALID,
                    callee: Some(Callee::Extern(SUBSTRING_LEN_EXTERN.to_string())),
                    args: vec![plan.source, plan.start, plan.end],
                    effects: plan.effects,
                },
            );
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:substring_len_hii:%{}",
                plan.outer_dst.0
            ));
            rewritten += 1;
        }

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut new_spans = Vec::with_capacity(spans.len());

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            if remove_indices.contains(&idx) {
                continue;
            }
            if let Some(rewritten_inst) = replacements.get(&idx) {
                new_insts.push(rewritten_inst.clone());
                new_spans.push(span);
            } else {
                new_insts.push(inst);
                new_spans.push(span);
            }
        }

        block.instructions = new_insts;
        block.instruction_spans = new_spans;

        for vid in removed_values {
            function.metadata.value_types.remove(&vid);
            function.metadata.value_origin_callers.remove(&vid);
        }
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_facts(function);
        refresh_function_string_corridor_candidates(function);
    }
    rewritten
}

fn collect_retained_len_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> BTreeMap<BasicBlockId, Vec<RetainedSubstringLenPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<RetainedSubstringLenPlan>> =
        BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans: Vec<RetainedSubstringLenPlan> = Vec::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            let Some((outer_dst, receiver, effects)) = match_len_call(inst) else {
                continue;
            };
            let receiver_root = resolve_copy_chain_source(function, def_map, receiver);
            let Some(inner_fact) = function.metadata.string_corridor_facts.get(&receiver_root) else {
                continue;
            };
            if inner_fact.op != StringCorridorOp::StrSlice {
                continue;
            }

            let Some((inner_bbid, inner_idx)) = def_map.get(&receiver_root).copied() else {
                continue;
            };
            if inner_bbid == *bbid && inner_idx >= outer_idx {
                continue;
            }
            let Some(inner_block) = function.blocks.get(&inner_bbid) else {
                continue;
            };
            let Some((source, start, end)) = inner_block
                .instructions
                .get(inner_idx)
                .and_then(extract_substring_args)
            else {
                continue;
            };

            plans.push(RetainedSubstringLenPlan {
                outer_idx,
                outer_dst,
                source,
                start,
                end,
                effects,
            });
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

fn apply_retained_len_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<RetainedSubstringLenPlan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let mut replacements: BTreeMap<usize, MirInstruction> = BTreeMap::new();

        for plan in plans {
            replacements.insert(
                plan.outer_idx,
                MirInstruction::Call {
                    dst: Some(plan.outer_dst),
                    func: ValueId::INVALID,
                    callee: Some(Callee::Extern(SUBSTRING_LEN_EXTERN.to_string())),
                    args: vec![plan.source, plan.start, plan.end],
                    effects: plan.effects,
                },
            );
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:borrowed_slice_len:%{}",
                plan.outer_dst.0
            ));
            rewritten += 1;
        }

        for (idx, inst) in block.instructions.iter_mut().enumerate() {
            if let Some(rewritten_inst) = replacements.get(&idx) {
                *inst = rewritten_inst.clone();
            }
        }
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_facts(function);
        refresh_function_string_corridor_candidates(function);
    }

    rewritten
}

fn apply_complementary_len_fusion_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<ComplementarySubstringLenFusionPlan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        let mut replacements: BTreeMap<usize, MirInstruction> = BTreeMap::new();

        for plan in plans {
            remove_indices.extend(plan.remove_indices.iter().copied());
            replacements.insert(
                plan.outer_idx,
                MirInstruction::BinOp {
                    dst: plan.outer_dst,
                    op: BinaryOp::Add,
                    lhs: plan.acc,
                    rhs: plan.source_len,
                },
            );
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:complementary_substring_len_fusion:%{}",
                plan.outer_dst.0
            ));
            rewritten += 1;
        }

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut new_spans = Vec::with_capacity(spans.len());

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            if remove_indices.contains(&idx) {
                continue;
            }
            if let Some(rewritten_inst) = replacements.get(&idx) {
                new_insts.push(rewritten_inst.clone());
                new_spans.push(span);
            } else {
                new_insts.push(inst);
                new_spans.push(span);
            }
        }

        block.instructions = new_insts;
        block.instruction_spans = new_spans;
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_facts(function);
        refresh_function_string_corridor_candidates(function);
    }

    rewritten
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirCompiler, MirModule, MirType,
    };
    use crate::parser::NyashParser;

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
        ty: MirType,
    ) -> MirInstruction {
        let _ = ty;
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn extern_call(dst: ValueId, name: &str, args: Vec<ValueId>) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(name.to_string())),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

    #[test]
    fn rewrites_single_use_substring_length_chain_to_direct_extern() {
        let mut module = MirModule::new("substring_len".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("StringBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("StringBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(4),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert_eq!(block.instructions.len(), block.instruction_spans.len());
        assert_eq!(block.instructions.len(), 1);
        match &block.instructions[0] {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => {
                assert_eq!(*dst, Some(ValueId(4)));
                assert_eq!(name, SUBSTRING_LEN_EXTERN);
                assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
            }
            other => panic!("expected direct extern rewrite, got {other:?}"),
        }
    }

    #[test]
    fn rewrites_runtime_data_substring_length_chain_through_copy_chain() {
        let mut module = MirModule::new("substring_len_runtime_data".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("RuntimeDataBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(4),
            src: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(5),
            src: ValueId(4),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(5),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(6)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("RuntimeDataBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Box("RuntimeDataBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Box("RuntimeDataBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(6), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert_eq!(block.instructions.len(), 3);
        match &block.instructions[2] {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => {
                assert_eq!(*dst, Some(ValueId(6)));
                assert_eq!(name, SUBSTRING_LEN_EXTERN);
                assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
            }
            other => panic!("expected direct extern rewrite, got {other:?}"),
        }
    }

    #[test]
    fn keeps_substring_when_result_has_multiple_uses() {
        let mut module = MirModule::new("substring_len_multiuse".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("StringBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("StringBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(4),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 0);
    }

    #[test]
    fn fuses_complementary_substring_len_pair_back_to_source_length() {
        let mut module = MirModule::new("substring_len_fusion".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(5),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(4), ValueId(3)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(6),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(3), ValueId(1)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(7),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(9),
            rhs: ValueId(5),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(8),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(7),
            rhs: ValueId(6),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(8)),
        });

        function.metadata.value_types.insert(ValueId(1), MirType::Integer);
        function.metadata.value_types.insert(ValueId(2), MirType::Integer);
        function.metadata.value_types.insert(ValueId(3), MirType::Integer);
        function.metadata.value_types.insert(ValueId(4), MirType::Integer);
        function.metadata.value_types.insert(ValueId(5), MirType::Integer);
        function.metadata.value_types.insert(ValueId(6), MirType::Integer);
        function.metadata.value_types.insert(ValueId(7), MirType::Integer);
        function.metadata.value_types.insert(ValueId(8), MirType::Integer);
        function.metadata.value_types.insert(ValueId(9), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            }),
            "complementary substring_len_hii calls should be gone: {:?}",
            block.instructions
        );
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(8) && *lhs == ValueId(9) && *rhs == ValueId(1)
                )
            }),
            "outer add should fuse to acc + source_len: {:?}",
            block.instructions
        );
    }

    #[test]
    fn keeps_non_complementary_substring_len_pair() {
        let mut module = MirModule::new("substring_len_fusion_negative".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(6),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: crate::mir::ConstValue::Integer(9),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(4),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(2), ValueId(3)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(5),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(3), ValueId(2)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(6),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(4),
            rhs: ValueId(5),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(6)),
        });

        function.metadata.value_types.insert(ValueId(1), MirType::Integer);
        function.metadata.value_types.insert(ValueId(2), MirType::Integer);
        function.metadata.value_types.insert(ValueId(3), MirType::Integer);
        function.metadata.value_types.insert(ValueId(4), MirType::Integer);
        function.metadata.value_types.insert(ValueId(5), MirType::Integer);
        function.metadata.value_types.insert(ValueId(6), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 0);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        let substring_len_calls = block
            .instructions
            .iter()
            .filter(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            })
            .count();
        assert_eq!(substring_len_calls, 2);
    }

    #[test]
    fn fuses_complementary_substring_len_pair_with_entry_len_and_duplicated_const_source() {
        let mut module = MirModule::new("substring_len_fusion_cross_block".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::String("line-seed-abcdef".to_string()),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Copy {
            dst: ValueId(8),
            src: ValueId(9),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(method_call(
            ValueId(5),
            ValueId(8),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(25),
            value: crate::mir::ConstValue::Integer(7),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(19),
            edge_args: None,
        });

        let mut loop_block = BasicBlock::new(BasicBlockId(19));
        loop_block.instructions.push(MirInstruction::Const {
            dst: ValueId(41),
            value: crate::mir::ConstValue::Integer(0),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(46),
            src: ValueId(5),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(45),
            src: ValueId(46),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(44),
            src: ValueId(45),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Const {
            dst: ValueId(47),
            value: crate::mir::ConstValue::Integer(2),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(43),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(44),
            rhs: ValueId(47),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(42),
            src: ValueId(43),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Const {
            dst: ValueId(49),
            value: crate::mir::ConstValue::String("line-seed-abcdef".to_string()),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(48),
            src: ValueId(49),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(50),
            src: ValueId(46),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(extern_call(
            ValueId(30),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(48), ValueId(41), ValueId(42)],
        ));
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(53),
            src: ValueId(25),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(54),
            src: ValueId(30),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(extern_call(
            ValueId(32),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(48), ValueId(42), ValueId(50)],
        ));
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(58),
            src: ValueId(53),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(59),
            src: ValueId(54),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(57),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(58),
            rhs: ValueId(59),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(60),
            src: ValueId(32),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(33),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(57),
            rhs: ValueId(60),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(33)),
        });
        function.add_block(loop_block);

        for vid in [
            5u32, 25, 30, 32, 33, 41, 42, 43, 44, 45, 46, 47, 50, 53, 54, 57, 58, 59, 60,
        ] {
            function
                .metadata
                .value_types
                .insert(ValueId(vid), MirType::Integer);
        }
        module.add_function(function);

        let function = module.get_function("main").expect("main");
        let def_map = build_def_map(function);
        let use_counts = build_use_counts(function);
        let first_call =
            match_substring_len_call_in_block(function, BasicBlockId(19), &def_map, ValueId(30))
                .expect("first substring_len");
        let second_call =
            match_substring_len_call_in_block(function, BasicBlockId(19), &def_map, ValueId(32))
                .expect("second substring_len");
        assert_eq!(
            complementary_pair_source_len(function, &def_map, &first_call, &second_call),
            Some(ValueId(5))
        );
        let plans = collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
        assert_eq!(plans.get(&BasicBlockId(19)).map(Vec::len), Some(1));

        let function = module.get_function_mut("main").expect("main");
        let rewritten = sink_borrowed_string_corridors_in_function(function);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(19)).expect("loop");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            }),
            "cross-block complementary substring_len_hii calls should be gone: {:?}",
            block.instructions
        );
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(33) && *lhs == ValueId(25) && *rhs == ValueId(5)
                )
            }),
            "outer add should fuse to acc + source_len across blocks: {:?}",
            block.instructions
        );
    }

    #[test]
    fn rewrites_retained_slice_length_consumer_across_blocks() {
        let mut module = MirModule::new("substring_len_retained_cross_block".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
            MirType::Box("StringBox".to_string()),
        ));
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        let mut loop_block = BasicBlock::new(BasicBlockId(1));
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(6),
            src: ValueId(5),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(7),
            src: ValueId(6),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(method_call(
            ValueId(8),
            ValueId(7),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(8)),
        });
        function.add_block(loop_block);

        for (vid, ty) in [
            (ValueId(1), MirType::Integer),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Box("StringBox".to_string())),
            (ValueId(8), MirType::Integer),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(1)).expect("loop");
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(8)
                        && name == SUBSTRING_LEN_EXTERN
                        && args.as_slice() == [ValueId(0), ValueId(4), ValueId(3)]
                )
            }),
            "retained slice length should rewrite to substring_len_hii: {:?}",
            block.instructions
        );
    }

    #[test]
    fn benchmark_substring_only_compiles_without_substring_len_calls() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_substring_only.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared =
            crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
                .expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");
        let substring_len_calls: Vec<String> = result
            .module
            .functions
            .iter()
            .flat_map(|(name, function)| {
                function.blocks.iter().flat_map(move |(bbid, block)| {
                    block.instructions.iter().filter_map(move |inst| match inst {
                        MirInstruction::Call {
                            callee: Some(Callee::Extern(callee)),
                            ..
                        } if callee == SUBSTRING_LEN_EXTERN => {
                            Some(format!("fn={name} bb={} inst={inst:?}", bbid.0))
                        }
                        _ => None,
                    })
                })
            })
            .collect();
        assert!(
            substring_len_calls.is_empty(),
            "benchmark should fuse substring_len_hii away, found {:?}",
            substring_len_calls
        );
    }

    #[test]
    fn benchmark_len_substring_views_compiles_without_loop_string_consumers() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_len_substring_views.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared =
            crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
                .expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");

        let mut leftover_string_consumers = Vec::new();
        for (name, function) in &result.module.functions {
            for (bbid, block) in &function.blocks {
                for inst in &block.instructions {
                    match inst {
                        MirInstruction::Call {
                            callee: Some(Callee::Extern(callee)),
                            ..
                        } if callee == SUBSTRING_LEN_EXTERN => leftover_string_consumers
                            .push(format!("fn={name} bb={} extern={callee} inst={inst:?}", bbid.0)),
                        MirInstruction::Call {
                            callee:
                                Some(Callee::Method {
                                    box_name,
                                    method,
                                    receiver: Some(_),
                                    ..
                                }),
                            ..
                        } if method == "length" && box_name == "RuntimeDataBox" => {
                            leftover_string_consumers.push(format!(
                                "fn={name} bb={} runtime-data length inst={inst:?}",
                                bbid.0
                            ))
                        }
                        _ => {}
                    }
                }
            }
        }

        assert!(
            leftover_string_consumers.is_empty(),
            "len_substring_views should fuse away loop string consumers, found {:?}",
            leftover_string_consumers
        );
    }
}
