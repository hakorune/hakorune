use super::*;

pub(super) fn collect_concat_corridor_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<ConcatCorridorPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<ConcatCorridorPlan>> = BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans = Vec::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            if let Some((outer_dst, receiver, effects)) = match_len_call(inst) {
                if let Some(ConcatTripletShape {
                    left,
                    middle,
                    right,
                }) = match_concat_triplet(function, *bbid, def_map, receiver)
                {
                    let Some(StringSourceIdentity::ConstString(text)) =
                        string_source_identity(function, def_map, middle)
                    else {
                        continue;
                    };
                    let Some(left) = match_substring_call_shape(function, def_map, left) else {
                        continue;
                    };
                    let Some(right) = match_substring_call_shape(function, def_map, right) else {
                        continue;
                    };
                    plans.push(ConcatCorridorPlan::Len(ConcatSubstringLenPlan {
                        outer_idx,
                        outer_dst,
                        left,
                        right,
                        middle_len: const_string_length(&text),
                        effects,
                    }));
                    continue;
                }

                if let Some(helper) = publication_helper_shape(function, def_map, receiver) {
                    let Some(receiver_chain) =
                        resolve_copy_chain_in_block(function, *bbid, def_map, use_counts, receiver)
                    else {
                        continue;
                    };
                    if receiver_chain.root != helper.dst {
                        continue;
                    }
                    plans.push(ConcatCorridorPlan::PublicationLen(
                        PublicationHelperLenPlan {
                            outer_idx,
                            outer_dst,
                            start: helper.start,
                            end: helper.end,
                            copy_indices: receiver_chain.copy_indices,
                        },
                    ));
                    continue;
                }

                continue;
            }

            if let Some(store) = array_store_candidate(function, def_map, inst) {
                let Some(value_chain) =
                    resolve_copy_chain_in_block(function, *bbid, def_map, use_counts, store.value)
                else {
                    continue;
                };
                let Some(helper) =
                    materialization_helper_shape(function, def_map, value_chain.root)
                else {
                    continue;
                };
                let Some((helper_bbid, helper_idx)) = def_map.get(&helper.dst).copied() else {
                    continue;
                };
                if helper_bbid != *bbid || helper_idx >= outer_idx {
                    continue;
                }
                let helper_uses = use_counts.get(&helper.dst).copied().unwrap_or(0);
                let observer_window = match helper_uses {
                    1 => None,
                    2 => {
                        let Some(window) = find_trailing_len_observer_in_block(
                            function, *bbid, def_map, use_counts, helper.dst, outer_idx,
                        ) else {
                            continue;
                        };
                        Some(window)
                    }
                    _ => continue,
                };
                plans.push(ConcatCorridorPlan::MaterializationStore(
                    MaterializationStorePlan {
                        helper_idx,
                        helper_dst: helper.dst,
                        store_idx: outer_idx,
                        left: helper.left,
                        middle: helper.middle,
                        right: helper.right,
                        start: helper.start,
                        end: helper.end,
                        helper_effects: helper.effects,
                        copy_indices: value_chain.copy_indices,
                        observer_copy_indices: observer_window
                            .map(|window| window.copy_indices)
                            .unwrap_or_default(),
                    },
                ));
                continue;
            }

            let Some((outer_dst, receiver, start, end, effects)) = match_substring_call(inst)
            else {
                continue;
            };
            if let Some(ConcatTripletShape {
                left,
                middle,
                right,
            }) = match_concat_triplet(function, *bbid, def_map, receiver)
            {
                let Some(StringSourceIdentity::ConstString(_)) =
                    string_source_identity(function, def_map, middle)
                else {
                    continue;
                };
                let Some(left_shape) = match_substring_call_shape(function, def_map, left) else {
                    continue;
                };
                let Some(right_shape) = match_substring_call_shape(function, def_map, right) else {
                    continue;
                };
                if let Some((source, split)) = ordered_complementary_substring_pair_source_split(
                    function,
                    def_map,
                    &left_shape,
                    &right_shape,
                ) {
                    let Some(left_chain) = resolve_single_use_copy_chain_in_block(
                        function, *bbid, def_map, use_counts, left,
                    ) else {
                        continue;
                    };
                    let Some(right_chain) = resolve_single_use_copy_chain_in_block(
                        function, *bbid, def_map, use_counts, right,
                    ) else {
                        continue;
                    };
                    let Some((left_root_bbid, left_root_idx)) =
                        def_map.get(&left_chain.root).copied()
                    else {
                        continue;
                    };
                    let Some((right_root_bbid, right_root_idx)) =
                        def_map.get(&right_chain.root).copied()
                    else {
                        continue;
                    };
                    if left_root_bbid != *bbid
                        || right_root_bbid != *bbid
                        || left_root_idx >= outer_idx
                        || right_root_idx >= outer_idx
                    {
                        continue;
                    }
                    if match_substring_call(&block.instructions[left_root_idx]).is_none()
                        || match_substring_call(&block.instructions[right_root_idx]).is_none()
                    {
                        continue;
                    }
                    let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
                    remove_indices.insert(left_root_idx);
                    remove_indices.insert(right_root_idx);
                    remove_indices.extend(left_chain.copy_indices.iter().copied());
                    remove_indices.extend(right_chain.copy_indices.iter().copied());
                    plans.push(ConcatCorridorPlan::InsertMidSubstring(
                        InsertMidSubstringPlan {
                            outer_idx,
                            outer_dst,
                            source,
                            middle,
                            split,
                            start: resolve_value_origin(function, def_map, start),
                            end: resolve_value_origin(function, def_map, end),
                            effects,
                            remove_indices: remove_indices.into_iter().collect(),
                        },
                    ));
                    continue;
                }
                if !substring_pair_shares_source(function, def_map, left, right) {
                    continue;
                }
                plans.push(ConcatCorridorPlan::Substring(ConcatSubstringPlan {
                    outer_idx,
                    outer_dst,
                    left,
                    middle,
                    right,
                    start: resolve_value_origin(function, def_map, start),
                    end: resolve_value_origin(function, def_map, end),
                    effects,
                }));
                continue;
            }

            if let Some(helper) = publication_helper_shape(function, def_map, receiver) {
                plans.push(ConcatCorridorPlan::PublicationSubstring(
                    PublicationHelperSubstringPlan {
                        outer_idx,
                        outer_dst,
                        left: helper.left,
                        middle: helper.middle,
                        right: helper.right,
                        outer_start: helper.start,
                        inner_start: resolve_value_origin(function, def_map, start),
                        inner_end: resolve_value_origin(function, def_map, end),
                        effects,
                    },
                ));
                continue;
            }
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedConcatCorridorPlan {
    Len {
        outer_idx: usize,
        outer_dst: ValueId,
        left_source: ValueId,
        left_start: ValueId,
        left_end: ValueId,
        right_source: ValueId,
        right_start: ValueId,
        right_end: ValueId,
        left_len_value: ValueId,
        right_len_value: ValueId,
        middle_len_value: ValueId,
        partial_sum_value: ValueId,
        middle_len: i64,
        effects: EffectMask,
    },
    Substring {
        outer_idx: usize,
        outer_dst: ValueId,
        left: ValueId,
        middle: ValueId,
        right: ValueId,
        start: ValueId,
        end: ValueId,
        effects: EffectMask,
    },
    InsertMidSubstring {
        outer_idx: usize,
        outer_dst: ValueId,
        insert_value: ValueId,
        source: ValueId,
        middle: ValueId,
        split: ValueId,
        start: ValueId,
        end: ValueId,
        effects: EffectMask,
    },
    PublicationLen {
        outer_idx: usize,
        outer_dst: ValueId,
        start: ValueId,
        end: ValueId,
        copy_indices: Vec<usize>,
    },
    PublicationSubstring {
        outer_idx: usize,
        outer_dst: ValueId,
        left: ValueId,
        middle: ValueId,
        right: ValueId,
        composed_start: ValueId,
        composed_end: ValueId,
        outer_start: ValueId,
        inner_start: ValueId,
        inner_end: ValueId,
        effects: EffectMask,
    },
    MaterializationStore {
        helper_dst: ValueId,
        left: ValueId,
        middle: ValueId,
        right: ValueId,
        start: ValueId,
        end: ValueId,
        helper_effects: EffectMask,
    },
}

pub(super) fn apply_concat_corridor_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<ConcatCorridorPlan>>,
) -> usize {
    let mut rewritten = 0usize;
    sync_function_next_value_id(function);

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }

        let mut resolved_by_idx: BTreeMap<usize, ResolvedConcatCorridorPlan> = BTreeMap::new();
        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        let mut hints = Vec::new();
        for plan in plans {
            match plan {
                ConcatCorridorPlan::Len(plan) => {
                    let left_len_value = function.next_value_id();
                    let right_len_value = function.next_value_id();
                    let middle_len_value = function.next_value_id();
                    let partial_sum_value = function.next_value_id();
                    for vid in [
                        left_len_value,
                        right_len_value,
                        middle_len_value,
                        partial_sum_value,
                    ] {
                        function.metadata.value_types.insert(vid, MirType::Integer);
                    }
                    function
                        .metadata
                        .value_types
                        .insert(plan.outer_dst, MirType::Integer);
                    hints.push(format!(
                        "string_corridor_sink:concat_slice_len:%{}",
                        plan.outer_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.outer_idx,
                        ResolvedConcatCorridorPlan::Len {
                            outer_idx: plan.outer_idx,
                            outer_dst: plan.outer_dst,
                            left_source: plan.left.source,
                            left_start: plan.left.start,
                            left_end: plan.left.end,
                            right_source: plan.right.source,
                            right_start: plan.right.start,
                            right_end: plan.right.end,
                            left_len_value,
                            right_len_value,
                            middle_len_value,
                            partial_sum_value,
                            middle_len: plan.middle_len,
                            effects: plan.effects,
                        },
                    );
                }
                ConcatCorridorPlan::Substring(plan) => {
                    hints.push(format!(
                        "string_corridor_sink:concat_slice_substring:%{}",
                        plan.outer_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.outer_idx,
                        ResolvedConcatCorridorPlan::Substring {
                            outer_idx: plan.outer_idx,
                            outer_dst: plan.outer_dst,
                            left: plan.left,
                            middle: plan.middle,
                            right: plan.right,
                            start: plan.start,
                            end: plan.end,
                            effects: plan.effects,
                        },
                    );
                }
                ConcatCorridorPlan::InsertMidSubstring(plan) => {
                    remove_indices.extend(plan.remove_indices.iter().copied());
                    let insert_value = function.next_value_id();
                    function.metadata.value_types.insert(
                        insert_value,
                        function
                            .metadata
                            .value_types
                            .get(&plan.outer_dst)
                            .cloned()
                            .unwrap_or_else(|| MirType::Box("RuntimeDataBox".to_string())),
                    );
                    hints.push(format!(
                        "string_corridor_sink:concat_slice_insert_mid_substring:%{}",
                        plan.outer_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.outer_idx,
                        ResolvedConcatCorridorPlan::InsertMidSubstring {
                            outer_idx: plan.outer_idx,
                            outer_dst: plan.outer_dst,
                            insert_value,
                            source: plan.source,
                            middle: plan.middle,
                            split: plan.split,
                            start: plan.start,
                            end: plan.end,
                            effects: plan.effects,
                        },
                    );
                }
                ConcatCorridorPlan::PublicationLen(plan) => {
                    remove_indices.extend(plan.copy_indices.iter().copied());
                    function
                        .metadata
                        .value_types
                        .insert(plan.outer_dst, MirType::Integer);
                    hints.push(format!(
                        "string_corridor_sink:publication_helper_len:%{}",
                        plan.outer_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.outer_idx,
                        ResolvedConcatCorridorPlan::PublicationLen {
                            outer_idx: plan.outer_idx,
                            outer_dst: plan.outer_dst,
                            start: plan.start,
                            end: plan.end,
                            copy_indices: plan.copy_indices,
                        },
                    );
                }
                ConcatCorridorPlan::PublicationSubstring(plan) => {
                    let composed_start = function.next_value_id();
                    let composed_end = function.next_value_id();
                    for vid in [composed_start, composed_end] {
                        function.metadata.value_types.insert(vid, MirType::Integer);
                    }
                    hints.push(format!(
                        "string_corridor_sink:publication_helper_substring:%{}",
                        plan.outer_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.outer_idx,
                        ResolvedConcatCorridorPlan::PublicationSubstring {
                            outer_idx: plan.outer_idx,
                            outer_dst: plan.outer_dst,
                            left: plan.left,
                            middle: plan.middle,
                            right: plan.right,
                            composed_start,
                            composed_end,
                            outer_start: plan.outer_start,
                            inner_start: plan.inner_start,
                            inner_end: plan.inner_end,
                            effects: plan.effects,
                        },
                    );
                }
                ConcatCorridorPlan::MaterializationStore(plan) => {
                    remove_indices.insert(plan.helper_idx);
                    remove_indices.extend(plan.copy_indices.iter().copied());
                    remove_indices.extend(plan.observer_copy_indices.iter().copied());
                    hints.push(format!(
                        "string_corridor_sink:materialization_store:%{}",
                        plan.helper_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.store_idx,
                        ResolvedConcatCorridorPlan::MaterializationStore {
                            helper_dst: plan.helper_dst,
                            left: plan.left,
                            middle: plan.middle,
                            right: plan.right,
                            start: plan.start,
                            end: plan.end,
                            helper_effects: plan.helper_effects,
                        },
                    );
                }
            }
        }

        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len() + resolved_by_idx.len());
        let mut new_spans = Vec::with_capacity(spans.len() + resolved_by_idx.len());

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            if remove_indices.contains(&idx) {
                continue;
            }
            let Some(plan) = resolved_by_idx.get(&idx) else {
                new_insts.push(inst);
                new_spans.push(span);
                continue;
            };

            match plan {
                ResolvedConcatCorridorPlan::Len {
                    outer_dst,
                    left_source,
                    left_start,
                    left_end,
                    right_source,
                    right_start,
                    right_end,
                    left_len_value,
                    right_len_value,
                    middle_len_value,
                    partial_sum_value,
                    middle_len,
                    effects,
                    ..
                } => {
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*left_len_value),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern(SUBSTRING_LEN_EXTERN.to_string())),
                        args: vec![*left_source, *left_start, *left_end],
                        effects: *effects,
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::Const {
                        dst: *middle_len_value,
                        value: ConstValue::Integer(*middle_len),
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::BinOp {
                        dst: *partial_sum_value,
                        op: BinaryOp::Add,
                        lhs: *left_len_value,
                        rhs: *middle_len_value,
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*right_len_value),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern(SUBSTRING_LEN_EXTERN.to_string())),
                        args: vec![*right_source, *right_start, *right_end],
                        effects: *effects,
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::BinOp {
                        dst: *outer_dst,
                        op: BinaryOp::Add,
                        lhs: *partial_sum_value,
                        rhs: *right_len_value,
                    });
                    new_spans.push(span);
                    rewritten += 1;
                }
                ResolvedConcatCorridorPlan::Substring {
                    outer_dst,
                    left,
                    middle,
                    right,
                    start,
                    end,
                    effects,
                    ..
                } => {
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*outer_dst),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                        args: vec![*left, *middle, *right, *start, *end],
                        effects: *effects,
                    });
                    new_spans.push(span);
                    rewritten += 1;
                }
                ResolvedConcatCorridorPlan::InsertMidSubstring {
                    outer_dst,
                    insert_value,
                    source,
                    middle,
                    split,
                    start,
                    end,
                    effects,
                    ..
                } => {
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*insert_value),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern(INSERT_HSI_EXTERN.to_string())),
                        args: vec![*source, *middle, *split],
                        effects: *effects,
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*outer_dst),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern("nyash.string.substring_hii".to_string())),
                        args: vec![*insert_value, *start, *end],
                        effects: *effects,
                    });
                    new_spans.push(span);
                    rewritten += 1;
                }
                ResolvedConcatCorridorPlan::PublicationLen {
                    outer_dst,
                    start,
                    end,
                    ..
                } => {
                    new_insts.push(MirInstruction::BinOp {
                        dst: *outer_dst,
                        op: BinaryOp::Sub,
                        lhs: *end,
                        rhs: *start,
                    });
                    new_spans.push(span);
                    rewritten += 1;
                }
                ResolvedConcatCorridorPlan::PublicationSubstring {
                    outer_dst,
                    left,
                    middle,
                    right,
                    composed_start,
                    composed_end,
                    outer_start,
                    inner_start,
                    inner_end,
                    effects,
                    ..
                } => {
                    new_insts.push(MirInstruction::BinOp {
                        dst: *composed_start,
                        op: BinaryOp::Add,
                        lhs: *outer_start,
                        rhs: *inner_start,
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::BinOp {
                        dst: *composed_end,
                        op: BinaryOp::Add,
                        lhs: *outer_start,
                        rhs: *inner_end,
                    });
                    new_spans.push(span.clone());
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*outer_dst),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                        args: vec![*left, *middle, *right, *composed_start, *composed_end],
                        effects: *effects,
                    });
                    new_spans.push(span);
                    rewritten += 1;
                }
                ResolvedConcatCorridorPlan::MaterializationStore {
                    helper_dst,
                    left,
                    middle,
                    right,
                    start,
                    end,
                    helper_effects,
                    ..
                } => {
                    new_insts.push(MirInstruction::Call {
                        dst: Some(*helper_dst),
                        func: ValueId::INVALID,
                        callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                        args: vec![*left, *middle, *right, *start, *end],
                        effects: *helper_effects,
                    });
                    new_spans.push(span.clone());
                    let rewritten_store = rewrite_method_set_value(&inst, *helper_dst)
                        .expect("materialization store rewrite should preserve set call shape");
                    new_insts.push(rewritten_store);
                    new_spans.push(span);
                    rewritten += 1;
                }
            }
        }

        block.instructions = new_insts;
        block.instruction_spans = new_spans;
        function.metadata.optimization_hints.extend(hints);
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_folded_metadata(function);
    }

    rewritten
}
