use super::*;
use crate::mir::placement_effect::{
    PlacementEffectBorrowContract, PlacementEffectDecision, PlacementEffectSource,
    PlacementEffectStringProof,
};

pub(super) fn build_use_counts(function: &MirFunction) -> HashMap<ValueId, usize> {
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

pub(super) fn placement_effect_string_window_for_value(
    function: &MirFunction,
    value: ValueId,
) -> Option<(ValueId, ValueId)> {
    function
        .metadata
        .placement_effect_routes
        .iter()
        .find_map(|route| {
            if route.source == PlacementEffectSource::StringCorridor && route.value == Some(value) {
                route.window_start.zip(route.window_end)
            } else {
                None
            }
        })
}

pub(super) fn sync_function_next_value_id(function: &mut MirFunction) {
    let max_used = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(MirInstruction::dst_value)
        .map(|vid| vid.0 + 1)
        .max()
        .unwrap_or(function.signature.params.len() as u32);
    if function.next_value_id < max_used {
        function.next_value_id = max_used;
    }
}

pub(super) fn collect_plans(
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

            let receiver_root = resolve_value_origin(function, def_map, receiver);
            if use_counts.get(&receiver_root).copied().unwrap_or(0) != 1 {
                continue;
            }

            let Some((inner_bbid, inner_idx)) = def_map.get(&receiver_root).copied() else {
                continue;
            };
            if inner_bbid != *bbid || inner_idx >= outer_idx {
                continue;
            }

            let Some((source, start, end)) = extract_substring_args(&block.instructions[inner_idx])
            else {
                continue;
            };
            let start_root = resolve_value_origin(function, def_map, start);
            let end_root = resolve_value_origin(function, def_map, end);

            match placement_effect_string_window_for_value(function, receiver_root) {
                Some((window_start, window_end))
                    if window_start == start_root && window_end == end_root => {}
                Some(_) => continue,
                None => {
                    let Some(inner_fact) =
                        function.metadata.string_corridor_facts.get(&receiver_root)
                    else {
                        continue;
                    };
                    if inner_fact.op != StringCorridorOp::StrSlice {
                        continue;
                    }
                }
            }

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

pub(super) fn resolve_copy_chain_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    mut value: ValueId,
) -> Option<SingleUseCopyChain> {
    let mut visited: BTreeSet<ValueId> = BTreeSet::new();
    let mut copy_indices = Vec::new();

    while visited.insert(value) {
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
                if use_counts.get(&value).copied().unwrap_or(0) != 1 {
                    return None;
                }
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

pub(super) fn resolve_single_use_copy_chain_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    value: ValueId,
) -> Option<SingleUseCopyChain> {
    let chain = resolve_copy_chain_in_block(function, bbid, def_map, use_counts, value)?;
    if use_counts.get(&chain.root).copied().unwrap_or(0) != 1 {
        return None;
    }
    Some(chain)
}

pub(super) fn find_trailing_len_observer_in_block(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    helper_root: ValueId,
    after_idx: usize,
) -> Option<TrailingLenObserverWindow> {
    let block = function.blocks.get(&bbid)?;

    for inst in block.instructions.iter().skip(after_idx + 1) {
        let Some((_dst, receiver, _effects)) = match_len_call(inst) else {
            continue;
        };
        let Some(receiver_chain) =
            resolve_copy_chain_in_block(function, bbid, def_map, use_counts, receiver)
        else {
            continue;
        };
        if receiver_chain.root != helper_root {
            continue;
        }
        return Some(TrailingLenObserverWindow {
            copy_indices: receiver_chain.copy_indices,
        });
    }

    None
}

pub(super) fn match_substring_len_call_in_block(
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

pub(super) fn helper_plan_for_kind(
    function: &MirFunction,
    root: ValueId,
    kind: StringCorridorCandidateKind,
) -> Option<StringCorridorCandidatePlan> {
    if let Some(plan) = placement_effect_helper_plan_for_kind(function, root, kind) {
        return Some(plan);
    }

    function
        .metadata
        .string_corridor_candidates
        .get(&root)?
        .iter()
        .find(|candidate| {
            candidate.kind == kind
                && matches!(
                    candidate.plan.map(|plan| plan.proof),
                    Some(StringCorridorCandidateProof::ConcatTriplet { .. })
                )
        })?
        .plan
}

fn placement_effect_helper_plan_for_kind(
    function: &MirFunction,
    root: ValueId,
    kind: StringCorridorCandidateKind,
) -> Option<StringCorridorCandidatePlan> {
    let decision = match kind {
        StringCorridorCandidateKind::BorrowCorridorFusion => PlacementEffectDecision::StayBorrowed,
        StringCorridorCandidateKind::PublicationSink => PlacementEffectDecision::PublishHandle,
        StringCorridorCandidateKind::MaterializationSink => {
            PlacementEffectDecision::MaterializeOwned
        }
        StringCorridorCandidateKind::DirectKernelEntry => {
            PlacementEffectDecision::DirectKernelEntry
        }
    };
    let route = function
        .metadata
        .placement_effect_routes
        .iter()
        .find(|route| {
            route.source == PlacementEffectSource::StringCorridor
                && route.value == Some(root)
                && route.decision == decision
        })?;
    let proof = match route.string_proof? {
        PlacementEffectStringProof::BorrowedSlice { source, start, end } => {
            StringCorridorCandidateProof::BorrowedSlice { source, start, end }
        }
        PlacementEffectStringProof::ConcatTriplet {
            left_value,
            left_source,
            left_start,
            left_end,
            middle,
            right_value,
            right_source,
            right_start,
            right_end,
            shared_source,
        } => StringCorridorCandidateProof::ConcatTriplet {
            left_value,
            left_source,
            left_start,
            left_end,
            middle,
            right_value,
            right_source,
            right_start,
            right_end,
            shared_source,
        },
    };
    if !matches!(proof, StringCorridorCandidateProof::ConcatTriplet { .. }) {
        return None;
    }
    Some(StringCorridorCandidatePlan {
        corridor_root: route.value.unwrap_or(root),
        source_root: route.source_value,
        borrow_contract: route.borrow_contract.map(|contract| match contract {
            PlacementEffectBorrowContract::BorrowTextFromObject => {
                crate::mir::StringCorridorBorrowContract::BorrowTextFromObject
            }
        }),
        publish_reason: route.publish_reason,
        publish_repr_policy: route.publish_repr_policy,
        stable_view_provenance: route.stable_view_provenance,
        start: route.window_start,
        end: route.window_end,
        known_length: None,
        publication_contract: None,
        proof,
    })
}

pub(super) fn corridor_helper_shape(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    kind: StringCorridorCandidateKind,
) -> Option<SubstringConcat3HelperShape> {
    let root = resolve_value_origin(function, def_map, value);
    let plan = helper_plan_for_kind(function, root, kind)?;
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let helper = match_substring_concat3_helper_call(block.instructions.get(idx)?)?;
    if helper.dst != plan.corridor_root {
        return None;
    }
    Some(helper)
}

pub(super) fn publication_helper_shape(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<SubstringConcat3HelperShape> {
    corridor_helper_shape(
        function,
        def_map,
        value,
        StringCorridorCandidateKind::PublicationSink,
    )
}

pub(super) fn materialization_helper_shape(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<SubstringConcat3HelperShape> {
    corridor_helper_shape(
        function,
        def_map,
        value,
        StringCorridorCandidateKind::MaterializationSink,
    )
}

pub(super) fn array_store_candidate(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    inst: &MirInstruction,
) -> Option<MethodSetCallShape> {
    let store = match_method_set_call(inst)?;
    if !matches!(store.box_name.as_str(), "ArrayBox" | "RuntimeDataBox") {
        return None;
    }
    let receiver_root = resolve_value_origin(function, def_map, store.receiver);
    match function.metadata.value_types.get(&receiver_root) {
        Some(MirType::Box(name)) if name == "ArrayBox" => Some(MethodSetCallShape {
            box_name: store.box_name,
            receiver: receiver_root,
            key: resolve_value_origin(function, def_map, store.key),
            value: store.value,
        }),
        _ => None,
    }
}

pub(super) fn publication_host_boundary_candidate(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    inst: &MirInstruction,
) -> Option<MethodSetCallShape> {
    let store = match_method_set_call(inst)?;
    if store.box_name != "RuntimeDataBox" {
        return None;
    }
    let receiver_root = resolve_value_origin(function, def_map, store.receiver);
    match function.metadata.value_types.get(&receiver_root) {
        Some(MirType::Box(name)) if name == "RuntimeDataBox" => Some(MethodSetCallShape {
            box_name: store.box_name,
            receiver: receiver_root,
            key: resolve_value_origin(function, def_map, store.key),
            value: store.value,
        }),
        _ => None,
    }
}

pub(super) fn rewrite_method_set_value(
    inst: &MirInstruction,
    new_value: ValueId,
) -> Option<MirInstruction> {
    match inst {
        MirInstruction::Call {
            dst,
            func,
            callee,
            args,
            effects,
        } => {
            if let Some(MethodSetCallShape { .. }) = match_method_set_call(inst) {
                let mut new_args = args.clone();
                if new_args.len() == 2 {
                    new_args[1] = new_value;
                    return Some(MirInstruction::Call {
                        dst: *dst,
                        func: *func,
                        callee: callee.clone(),
                        args: new_args,
                        effects: *effects,
                    });
                }
            }
            None
        }
        _ => None,
    }
}

pub(super) fn value_is_const_i64(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
    expected: i64,
) -> bool {
    let root = resolve_value_origin(function, def_map, value);
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

pub(super) fn match_source_length_value(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    source_identity: &StringSourceIdentity,
    candidate: ValueId,
) -> Option<ValueId> {
    let length_root = resolve_value_origin(function, def_map, candidate);
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

pub(super) fn stable_length_value_for_source(
    function: &MirFunction,
    source: ValueId,
) -> Option<ValueId> {
    function
        .metadata
        .string_corridor_relations
        .get(&source)?
        .iter()
        .find_map(|relation| {
            (relation.kind == crate::mir::StringCorridorRelationKind::StableLengthScalar)
                .then_some(relation.witness_value)
                .flatten()
        })
}

pub(super) fn apply_plans(
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
        refresh_function_string_corridor_folded_metadata(function);
    }
    rewritten
}
