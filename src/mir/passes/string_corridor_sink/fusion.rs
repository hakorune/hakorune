use super::*;

pub(super) fn complementary_pair_source_len(
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
    let shared_source_root = resolve_value_origin(function, def_map, lhs.source);

    if value_is_const_i64(function, def_map, lhs.start, 0) {
        let mid = resolve_value_origin(function, def_map, lhs.end);
        let rhs_start = resolve_value_origin(function, def_map, rhs.start);
        if rhs_start != mid {
            return None;
        }
        if let Some(source_len) = match_source_length_value(function, def_map, &lhs_source, rhs.end)
        {
            return Some(source_len);
        }
        let stable_len = stable_length_value_for_source(function, shared_source_root)?;
        let rhs_end = resolve_value_origin(function, def_map, rhs.end);
        if rhs_end == stable_len {
            return Some(stable_len);
        }
        return None;
    }

    if value_is_const_i64(function, def_map, rhs.start, 0) {
        let mid = resolve_value_origin(function, def_map, rhs.end);
        let lhs_start = resolve_value_origin(function, def_map, lhs.start);
        if lhs_start != mid {
            return None;
        }
        if let Some(source_len) = match_source_length_value(function, def_map, &lhs_source, lhs.end)
        {
            return Some(source_len);
        }
        let stable_len = stable_length_value_for_source(function, shared_source_root)?;
        let lhs_end = resolve_value_origin(function, def_map, lhs.end);
        if lhs_end == stable_len {
            return Some(stable_len);
        }
        return None;
    }

    None
}

pub(super) fn substring_pair_shares_source(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    lhs_slice: ValueId,
    rhs_slice: ValueId,
) -> bool {
    let Some(lhs) = match_substring_call_shape(function, def_map, lhs_slice) else {
        return false;
    };
    let Some(rhs) = match_substring_call_shape(function, def_map, rhs_slice) else {
        return false;
    };
    string_source_identity(function, def_map, lhs.source)
        == string_source_identity(function, def_map, rhs.source)
}

pub(super) fn try_match_complementary_len_fusion_plan(
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

    for (acc_leaf, first_leaf) in [
        (inner_add.lhs, inner_add.rhs),
        (inner_add.rhs, inner_add.lhs),
    ] {
        let first_chain = match resolve_single_use_copy_chain_in_block(
            function, bbid, def_map, use_counts, first_leaf,
        ) {
            Some(chain) => chain,
            None => continue,
        };
        let first_call =
            match match_substring_len_call_in_block(function, bbid, def_map, first_chain.root) {
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

        let acc = resolve_value_origin(function, def_map, acc_leaf);
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

pub(super) fn collect_complementary_len_fusion_plans(
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
                function, *bbid, outer_idx, *dst, *lhs, *rhs, def_map, use_counts,
            )
            .or_else(|| {
                try_match_complementary_len_fusion_plan(
                    function, *bbid, outer_idx, *dst, *rhs, *lhs, def_map, use_counts,
                )
            }) else {
                continue;
            };

            if plan.remove_indices.iter().any(|idx| occupied.contains(idx)) {
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

pub(super) fn apply_complementary_len_fusion_plans(
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
        refresh_function_string_corridor_folded_metadata(function);
    }

    rewritten
}
