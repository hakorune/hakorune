use super::*;

fn requires_explicit_cold_publish(function: &MirFunction, value: ValueId) -> bool {
    matches!(
        function
            .metadata
            .string_kernel_plans
            .get(&value)
            .and_then(|plan| plan.text_consumer),
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    )
}

pub(super) fn collect_publication_return_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, PublicationReturnPlan> {
    let mut plans_by_block: BTreeMap<BasicBlockId, PublicationReturnPlan> = BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let Some((return_site, return_value)) = publication_return_site(block) else {
            continue;
        };
        let Some(return_chain) = resolve_single_use_copy_chain_in_block(
            function,
            *bbid,
            def_map,
            use_counts,
            return_value,
        ) else {
            continue;
        };
        if !requires_explicit_cold_publish(function, return_chain.root) {
            continue;
        }
        let Some(helper) = publication_helper_shape(function, def_map, return_chain.root) else {
            continue;
        };
        let Some((helper_bbid, helper_idx)) = def_map.get(&helper.dst).copied() else {
            continue;
        };
        if helper_bbid != *bbid {
            continue;
        }

        plans_by_block.insert(
            *bbid,
            PublicationReturnPlan {
                helper_idx,
                helper_dst: helper.dst,
                return_idx: match return_site {
                    ReturnSite::Instruction(idx) => Some(idx),
                    ReturnSite::Terminator => None,
                },
                left: helper.left,
                middle: helper.middle,
                right: helper.right,
                start: helper.start,
                end: helper.end,
                effects: helper.effects,
                copy_indices: return_chain.copy_indices,
            },
        );
    }

    plans_by_block
}

pub(super) fn publication_return_site(
    block: &crate::mir::BasicBlock,
) -> Option<(ReturnSite, ValueId)> {
    if let Some(MirInstruction::Return { value: Some(value) }) = block.terminator.as_ref() {
        return Some((ReturnSite::Terminator, *value));
    }

    block
        .instructions
        .iter()
        .enumerate()
        .rev()
        .find_map(|(idx, inst)| match inst {
            MirInstruction::Return { value: Some(value) } => {
                Some((ReturnSite::Instruction(idx), *value))
            }
            _ => None,
        })
}

pub(super) fn publication_write_boundary_value(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::Store { value, .. } | MirInstruction::FieldSet { value, .. } => {
            Some(*value)
        }
        _ => None,
    }
}

pub(super) fn rewrite_publication_write_boundary_value(
    inst: &MirInstruction,
    new_value: ValueId,
) -> Option<MirInstruction> {
    match inst {
        MirInstruction::Store { ptr, .. } => Some(MirInstruction::Store {
            value: new_value,
            ptr: *ptr,
        }),
        MirInstruction::FieldSet {
            base,
            field,
            declared_type,
            ..
        } => Some(MirInstruction::FieldSet {
            base: *base,
            field: field.clone(),
            value: new_value,
            declared_type: declared_type.clone(),
        }),
        _ => None,
    }
}

pub(super) fn can_sink_helper_past_intervening_instructions(
    block: &crate::mir::BasicBlock,
    helper_idx: usize,
    boundary_idx: usize,
    removable_indices: &BTreeSet<usize>,
) -> bool {
    if helper_idx >= boundary_idx {
        return false;
    }

    for (idx, inst) in block
        .instructions
        .iter()
        .enumerate()
        .skip(helper_idx + 1)
        .take(boundary_idx.saturating_sub(helper_idx + 1))
    {
        if removable_indices.contains(&idx) {
            continue;
        }
        if !inst.effects().is_pure() {
            return false;
        }
    }

    true
}

pub(super) fn collect_publication_write_boundary_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<PublicationWriteBoundaryPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<PublicationWriteBoundaryPlan>> =
        BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans = Vec::new();

        for (boundary_idx, inst) in block.instructions.iter().enumerate() {
            let Some(boundary_value) = publication_write_boundary_value(inst) else {
                continue;
            };
            let Some(boundary_chain) = resolve_single_use_copy_chain_in_block(
                function,
                *bbid,
                def_map,
                use_counts,
                boundary_value,
            ) else {
                continue;
            };
            if !requires_explicit_cold_publish(function, boundary_chain.root) {
                continue;
            }
            let Some(helper) = publication_helper_shape(function, def_map, boundary_chain.root)
            else {
                continue;
            };
            let Some((helper_bbid, helper_idx)) = def_map.get(&helper.dst).copied() else {
                continue;
            };
            if helper_bbid != *bbid {
                continue;
            }

            let removable_indices: BTreeSet<usize> =
                boundary_chain.copy_indices.iter().copied().collect();
            if !can_sink_helper_past_intervening_instructions(
                block,
                helper_idx,
                boundary_idx,
                &removable_indices,
            ) {
                continue;
            }

            plans.push(PublicationWriteBoundaryPlan {
                helper_idx,
                helper_dst: helper.dst,
                boundary_idx,
                left: helper.left,
                middle: helper.middle,
                right: helper.right,
                start: helper.start,
                end: helper.end,
                effects: helper.effects,
                copy_indices: boundary_chain.copy_indices,
            });
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

pub(super) fn collect_publication_host_boundary_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<PublicationHostBoundaryPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<PublicationHostBoundaryPlan>> =
        BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans = Vec::new();

        for (boundary_idx, inst) in block.instructions.iter().enumerate() {
            let Some(boundary) = publication_host_boundary_candidate(function, def_map, inst)
            else {
                continue;
            };
            let Some(boundary_chain) = resolve_single_use_copy_chain_in_block(
                function,
                *bbid,
                def_map,
                use_counts,
                boundary.value,
            ) else {
                continue;
            };
            if !requires_explicit_cold_publish(function, boundary_chain.root) {
                continue;
            }
            let Some(helper) = publication_helper_shape(function, def_map, boundary_chain.root)
            else {
                continue;
            };
            let Some((helper_bbid, helper_idx)) = def_map.get(&helper.dst).copied() else {
                continue;
            };
            if helper_bbid != *bbid {
                continue;
            }

            let removable_indices: BTreeSet<usize> =
                boundary_chain.copy_indices.iter().copied().collect();
            if !can_sink_helper_past_intervening_instructions(
                block,
                helper_idx,
                boundary_idx,
                &removable_indices,
            ) {
                continue;
            }

            plans.push(PublicationHostBoundaryPlan {
                helper_idx,
                helper_dst: helper.dst,
                boundary_idx,
                left: helper.left,
                middle: helper.middle,
                right: helper.right,
                start: helper.start,
                end: helper.end,
                effects: helper.effects,
                copy_indices: boundary_chain.copy_indices,
            });
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

pub(super) fn apply_publication_return_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, PublicationReturnPlan>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plan) in plans_by_block {
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len().saturating_sub(plan.copy_indices.len()));
        let mut new_spans = Vec::with_capacity(spans.len().saturating_sub(plan.copy_indices.len()));
        let mut helper_span = None;

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            if idx == plan.helper_idx {
                helper_span = Some(span);
                continue;
            }
            if plan.copy_indices.contains(&idx) {
                continue;
            }
            if plan.return_idx == Some(idx) {
                new_insts.push(MirInstruction::Call {
                    dst: Some(plan.helper_dst),
                    func: ValueId::INVALID,
                    callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                    args: vec![plan.left, plan.middle, plan.right, plan.start, plan.end],
                    effects: plan.effects,
                });
                new_spans.push(helper_span.clone().unwrap_or_else(|| span.clone()));
                new_insts.push(MirInstruction::Return {
                    value: Some(plan.helper_dst),
                });
                new_spans.push(span);
                continue;
            }
            new_insts.push(inst);
            new_spans.push(span);
        }

        let Some(helper_span) = helper_span else {
            block.instructions = new_insts;
            block.instruction_spans = new_spans;
            continue;
        };

        if plan.return_idx.is_none() {
            let Some(MirInstruction::Return { value }) = block.terminator.as_mut() else {
                block.instructions = new_insts;
                block.instruction_spans = new_spans;
                continue;
            };
            new_insts.push(MirInstruction::Call {
                dst: Some(plan.helper_dst),
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                args: vec![plan.left, plan.middle, plan.right, plan.start, plan.end],
                effects: plan.effects,
            });
            new_spans.push(helper_span);
            *value = Some(plan.helper_dst);
        }
        block.instructions = new_insts;
        block.instruction_spans = new_spans;
        function.metadata.optimization_hints.push(format!(
            "string_corridor_sink:publication_return:%{}",
            plan.helper_dst.0
        ));
        rewritten += 1;
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_folded_metadata(function);
    }

    rewritten
}

pub(super) fn apply_publication_write_boundary_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<PublicationWriteBoundaryPlan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let helper_spans: BTreeMap<usize, crate::ast::Span> = plans
            .iter()
            .filter_map(|plan| {
                spans
                    .get(plan.helper_idx)
                    .cloned()
                    .map(|span| (plan.boundary_idx, span))
            })
            .collect();
        let plans_by_boundary: BTreeMap<usize, PublicationWriteBoundaryPlan> = plans
            .into_iter()
            .map(|plan| (plan.boundary_idx, plan))
            .collect();
        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();

        for plan in plans_by_boundary.values() {
            remove_indices.insert(plan.helper_idx);
            remove_indices.extend(plan.copy_indices.iter().copied());
        }

        let mut new_insts = Vec::with_capacity(insts.len().saturating_sub(remove_indices.len()));
        let mut new_spans = Vec::with_capacity(spans.len().saturating_sub(remove_indices.len()));

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            let Some(plan) = plans_by_boundary.get(&idx) else {
                if remove_indices.contains(&idx) {
                    continue;
                }
                new_insts.push(inst);
                new_spans.push(span);
                continue;
            };

            let helper_span = helper_spans
                .get(&idx)
                .cloned()
                .unwrap_or_else(|| span.clone());
            new_insts.push(MirInstruction::Call {
                dst: Some(plan.helper_dst),
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                args: vec![plan.left, plan.middle, plan.right, plan.start, plan.end],
                effects: plan.effects,
            });
            new_spans.push(helper_span);
            new_insts.push(
                rewrite_publication_write_boundary_value(&inst, plan.helper_dst)
                    .expect("publication write-boundary rewrite should preserve store shape"),
            );
            new_spans.push(span);
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:publication_write_boundary:%{}",
                plan.helper_dst.0
            ));
            rewritten += 1;
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

pub(super) fn apply_publication_host_boundary_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<PublicationHostBoundaryPlan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let helper_spans: BTreeMap<usize, crate::ast::Span> = plans
            .iter()
            .filter_map(|plan| {
                spans
                    .get(plan.helper_idx)
                    .cloned()
                    .map(|span| (plan.boundary_idx, span))
            })
            .collect();
        let plans_by_boundary: BTreeMap<usize, PublicationHostBoundaryPlan> = plans
            .into_iter()
            .map(|plan| (plan.boundary_idx, plan))
            .collect();
        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();

        for plan in plans_by_boundary.values() {
            remove_indices.insert(plan.helper_idx);
            remove_indices.extend(plan.copy_indices.iter().copied());
        }

        let mut new_insts = Vec::with_capacity(insts.len().saturating_sub(remove_indices.len()));
        let mut new_spans = Vec::with_capacity(spans.len().saturating_sub(remove_indices.len()));

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            let Some(plan) = plans_by_boundary.get(&idx) else {
                if remove_indices.contains(&idx) {
                    continue;
                }
                new_insts.push(inst);
                new_spans.push(span);
                continue;
            };

            let helper_span = helper_spans
                .get(&idx)
                .cloned()
                .unwrap_or_else(|| span.clone());
            new_insts.push(MirInstruction::Call {
                dst: Some(plan.helper_dst),
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(SUBSTRING_CONCAT3_EXTERN.to_string())),
                args: vec![plan.left, plan.middle, plan.right, plan.start, plan.end],
                effects: plan.effects,
            });
            new_spans.push(helper_span);
            new_insts.push(
                rewrite_method_set_value(&inst, plan.helper_dst)
                    .expect("publication host-boundary rewrite should preserve set call shape"),
            );
            new_spans.push(span);
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:publication_host_boundary:%{}",
                plan.helper_dst.0
            ));
            rewritten += 1;
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
