use super::*;

fn preserve_emit_mir_concat_triplet_shape() -> bool {
    crate::config::env::stage1::emit_mir_json()
}

pub(crate) fn collect_concat_corridor_plans(
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
            if let Some(plan) = collect_store_shared_receiver_substring_plan(
                function, *bbid, def_map, use_counts, outer_idx, outer_dst, receiver,
            ) {
                plans.push(ConcatCorridorPlan::StoreSharedReceiverSubstring(plan));
                continue;
            }
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
                if !preserve_emit_mir_concat_triplet_shape() {
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
fn collect_store_shared_receiver_substring_plan(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    outer_idx: usize,
    outer_dst: ValueId,
    receiver: ValueId,
) -> Option<StoreSharedReceiverSubstringPlan> {
    let receiver_chain =
        resolve_copy_chain_in_block(function, bbid, def_map, use_counts, receiver)?;
    let duplicate_root = receiver_chain.root;
    if use_counts.get(&duplicate_root).copied().unwrap_or(0) != 1 {
        return None;
    }
    let duplicate_sig = const_suffix_add_signature(function, bbid, def_map, duplicate_root)?;

    let block = function.blocks.get(&bbid)?;
    let mut replacement_receiver = None;
    for inst in block.instructions.iter().take(outer_idx) {
        let Some(store) = array_store_candidate(function, def_map, inst) else {
            continue;
        };
        let Some(store_chain) =
            resolve_copy_chain_in_block(function, bbid, def_map, use_counts, store.value)
        else {
            continue;
        };
        if store_chain.root == duplicate_root {
            continue;
        }
        let Some(store_sig) = const_suffix_add_signature(function, bbid, def_map, store_chain.root)
        else {
            continue;
        };
        if store_sig == duplicate_sig {
            replacement_receiver = Some(store_chain.root);
        }
    }
    let replacement_receiver = replacement_receiver?;

    let mut remove_indices: BTreeSet<usize> = receiver_chain.copy_indices.iter().copied().collect();
    remove_indices.extend(
        removable_single_use_add_feeders(function, bbid, def_map, use_counts, duplicate_root)?
            .into_iter(),
    );

    Some(StoreSharedReceiverSubstringPlan {
        outer_idx,
        outer_dst,
        replacement_receiver,
        remove_indices: remove_indices.into_iter().collect(),
    })
}

fn const_suffix_add_signature(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<(ValueId, String)> {
    let add = match_add_in_block(function, bbid, def_map, value)?;
    let lhs_root = resolve_value_origin(function, def_map, add.lhs);
    let rhs_root = resolve_value_origin(function, def_map, add.rhs);
    let StringSourceIdentity::ConstString(text) =
        string_source_identity(function, def_map, rhs_root)?
    else {
        return None;
    };
    Some((lhs_root, text))
}

fn removable_single_use_add_feeders(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    add_root: ValueId,
) -> Option<Vec<usize>> {
    let add = match_add_in_block(function, bbid, def_map, add_root)?;
    let mut remove_indices = vec![add.idx];
    let block = function.blocks.get(&bbid)?;
    for operand in [add.lhs, add.rhs] {
        let mut current = operand;
        let mut visited = BTreeSet::new();
        while visited.insert(current) {
            let Some((inst_bbid, idx)) = def_map.get(&current).copied() else {
                break;
            };
            if inst_bbid != bbid {
                break;
            }
            match block.instructions.get(idx) {
                Some(MirInstruction::Copy { src, .. })
                    if use_counts.get(&current).copied().unwrap_or(0) == 1 =>
                {
                    remove_indices.push(idx);
                    current = *src;
                }
                Some(MirInstruction::Const { .. })
                    if use_counts.get(&current).copied().unwrap_or(0) == 1 =>
                {
                    remove_indices.push(idx);
                    break;
                }
                _ => break,
            }
        }
    }
    Some(remove_indices)
}
