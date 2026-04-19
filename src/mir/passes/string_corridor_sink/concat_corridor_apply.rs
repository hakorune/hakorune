use super::*;

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
    StoreSharedReceiverSubstring {
        replacement_receiver: ValueId,
    },
}

pub(crate) fn apply_concat_corridor_plans(
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
                ConcatCorridorPlan::StoreSharedReceiverSubstring(plan) => {
                    remove_indices.extend(plan.remove_indices.iter().copied());
                    hints.push(format!(
                        "string_corridor_sink:store_shared_receiver_substring:%{}",
                        plan.outer_dst.0
                    ));
                    resolved_by_idx.insert(
                        plan.outer_idx,
                        ResolvedConcatCorridorPlan::StoreSharedReceiverSubstring {
                            replacement_receiver: plan.replacement_receiver,
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
                ResolvedConcatCorridorPlan::StoreSharedReceiverSubstring {
                    replacement_receiver,
                } => {
                    let rewritten_substring =
                        rewrite_substring_receiver(&inst, *replacement_receiver).expect(
                            "store shared receiver rewrite should preserve substring call shape",
                        );
                    new_insts.push(rewritten_substring);
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
fn rewrite_substring_receiver(
    inst: &MirInstruction,
    new_receiver: ValueId,
) -> Option<MirInstruction> {
    match inst {
        MirInstruction::Call {
            dst,
            func,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(_),
                    certainty,
                    box_kind,
                }),
            args,
            effects,
        } if args.len() == 2 && matches!(method.as_str(), "substring" | "slice") => {
            Some(MirInstruction::Call {
                dst: *dst,
                func: *func,
                callee: Some(Callee::Method {
                    box_name: box_name.clone(),
                    method: method.clone(),
                    receiver: Some(new_receiver),
                    certainty: *certainty,
                    box_kind: *box_kind,
                }),
                args: args.clone(),
                effects: *effects,
            })
        }
        MirInstruction::Call {
            dst,
            func,
            callee: Some(Callee::Extern(name)),
            args,
            effects,
        } if args.len() == 3 && name == "nyash.string.substring_hii" => {
            let mut new_args = args.clone();
            new_args[0] = new_receiver;
            Some(MirInstruction::Call {
                dst: *dst,
                func: *func,
                callee: Some(Callee::Extern(name.clone())),
                args: new_args,
                effects: *effects,
            })
        }
        _ => None,
    }
}
