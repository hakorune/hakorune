use super::*;

pub(super) fn collect_retained_len_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<RetainedSubstringLenPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<RetainedSubstringLenPlan>> = BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans: Vec<RetainedSubstringLenPlan> = Vec::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            let Some((outer_dst, receiver, effects)) = match_len_call(inst) else {
                continue;
            };
            let receiver_root = resolve_value_origin(function, def_map, receiver);
            let Some(inner_fact) = function.metadata.string_corridor_facts.get(&receiver_root)
            else {
                continue;
            };
            if inner_fact.op != StringCorridorOp::StrSlice {
                continue;
            }
            if use_counts.get(&receiver_root).copied().unwrap_or(0) != 1 {
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

pub(super) fn apply_retained_len_plans(
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
        refresh_function_string_corridor_metadata(function);
    }

    rewritten
}
