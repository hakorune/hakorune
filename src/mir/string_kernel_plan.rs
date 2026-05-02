/*!
 * Backend-consumable string kernel plan seam.
 *
 * This module owns the thin derived view that MIR refresh now materializes
 * first-class. It is downstream of string corridor candidates and upstream of
 * JSON/shim transport. Placement remains the owner of candidate metadata
 * itself.
 */

use std::collections::BTreeMap;

use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{
    string_corridor::{StringCorridorBorrowContract, StringPublishReason, StringPublishReprPolicy},
    string_corridor_placement::{
        StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidateProof,
        StringCorridorPublicationBoundary, StringCorridorPublicationContract,
    },
    string_corridor_recognizer::{
        match_len_call, match_method_set_call, match_substring_call,
        match_substring_concat3_helper_call,
    },
    CompareOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};

mod model;
pub use model::{
    StringKernelPlan, StringKernelPlanBorrowContract, StringKernelPlanCarrier,
    StringKernelPlanConsumer, StringKernelPlanFamily, StringKernelPlanLegality,
    StringKernelPlanLoopPayload, StringKernelPlanPart, StringKernelPlanPublicationBoundary,
    StringKernelPlanPublicationContract, StringKernelPlanReadAliasFacts,
    StringKernelPlanRetainedForm, StringKernelPlanSlotHopSubstring, StringKernelPlanTextConsumer,
    StringKernelPlanVerifierOwner,
};

fn candidate_priority(kind: StringCorridorCandidateKind) -> u8 {
    match kind {
        StringCorridorCandidateKind::DirectKernelEntry => 0,
        StringCorridorCandidateKind::PublicationSink => 1,
        StringCorridorCandidateKind::MaterializationSink => 2,
        StringCorridorCandidateKind::BorrowCorridorFusion => 3,
    }
}

fn publication_contract_from_plan(
    plan: crate::mir::string_corridor_placement::StringCorridorCandidatePlan,
) -> Option<StringKernelPlanPublicationContract> {
    match plan.publication_contract {
        Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ) => Some(
            StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        None => None,
    }
}

fn borrow_contract_from_plan(
    plan: crate::mir::string_corridor_placement::StringCorridorCandidatePlan,
) -> Option<StringKernelPlanBorrowContract> {
    match plan.borrow_contract {
        Some(StringCorridorBorrowContract::BorrowTextFromObject) => {
            Some(StringKernelPlanBorrowContract::BorrowTextFromObject)
        }
        None => None,
    }
}

fn const_string_literal(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<(ValueId, String)> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    match function.blocks.get(&bbid)?.instructions.get(idx)? {
        MirInstruction::Const {
            value: ConstValue::String(text),
            ..
        } => Some((root, text.clone())),
        _ => None,
    }
}

fn const_integer_literal(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<i64> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    match function.blocks.get(&bbid)?.instructions.get(idx)? {
        MirInstruction::Const {
            value: ConstValue::Integer(actual),
            ..
        } => Some(*actual),
        _ => None,
    }
}

fn find_loop_bound_for_corridor(function: &MirFunction, corridor_root: ValueId) -> Option<i64> {
    let def_map = build_value_def_map(function);
    let root = resolve_value_origin(function, &def_map, corridor_root);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    if !matches!(block.instructions.get(idx)?, MirInstruction::Phi { .. }) {
        return None;
    }
    let branch_condition = match block.terminator.as_ref() {
        Some(MirInstruction::Branch { condition, .. }) => Some(*condition),
        _ => block
            .instructions
            .iter()
            .find_map(|candidate| match candidate {
                MirInstruction::Branch { condition, .. } => Some(*condition),
                _ => None,
            }),
    };
    block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if branch_condition == Some(*dst) => const_integer_literal(function, &def_map, *lhs)
            .or_else(|| const_integer_literal(function, &def_map, *rhs)),
        _ => None,
    })
}

fn find_seed_input_for_corridor(
    function: &MirFunction,
    def_map: &ValueDefMap,
    corridor_root: ValueId,
) -> Option<(ValueId, String)> {
    let root = resolve_value_origin(function, def_map, corridor_root);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let inputs = match block.instructions.get(idx)? {
        MirInstruction::Phi { inputs, .. } => inputs,
        _ => return None,
    };
    inputs
        .iter()
        .find_map(|(_, value)| const_string_literal(function, def_map, *value))
}

fn derive_concat_triplet_loop_payload(
    function: &MirFunction,
    proof: &StringCorridorCandidateProof,
    corridor_root: ValueId,
) -> Option<StringKernelPlanLoopPayload> {
    let def_map = build_value_def_map(function);
    let (seed_value, seed_literal) =
        find_seed_input_for_corridor(function, &def_map, corridor_root)?;
    let seed_length = seed_literal.len() as i64;
    let loop_bound = find_loop_bound_for_corridor(function, corridor_root)?;
    let split_value = match proof {
        StringCorridorCandidateProof::ConcatTriplet {
            left_end,
            right_start,
            ..
        } if left_end == right_start => *left_end,
        _ => return None,
    };
    let split_root = resolve_value_origin(function, &def_map, split_value);
    let (bbid, idx) = def_map.get(&split_root).copied()?;
    let divisor = match function.blocks.get(&bbid)?.instructions.get(idx)? {
        MirInstruction::BinOp { lhs, rhs, .. } => const_integer_literal(function, &def_map, *rhs)
            .or_else(|| const_integer_literal(function, &def_map, *lhs)),
        _ => None,
    }?;
    if divisor <= 0 {
        return None;
    }
    let split_length = seed_length / divisor;
    if split_length <= 0 {
        return None;
    }
    Some(StringKernelPlanLoopPayload {
        seed_value,
        seed_literal,
        seed_length,
        loop_bound,
        split_length,
    })
}

fn inferred_text_output(
    function: &MirFunction,
    plan_value: ValueId,
    def_map: &ValueDefMap,
) -> bool {
    fn stringish_type(ty: Option<&crate::mir::MirType>) -> bool {
        match ty {
            Some(crate::mir::MirType::String) => true,
            Some(crate::mir::MirType::Box(name)) => {
                matches!(name.as_str(), "StringBox" | "RuntimeDataBox")
            }
            _ => false,
        }
    }

    let root = resolve_value_origin(function, def_map, plan_value);
    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return stringish_type(
            function
                .metadata
                .value_types
                .get(&root)
                .or_else(|| function.metadata.value_types.get(&plan_value)),
        );
    };
    let Some(inst) = function
        .blocks
        .get(&bbid)
        .and_then(|block| block.instructions.get(idx))
    else {
        return false;
    };
    if match_len_call(inst).is_some() {
        return false;
    }
    if match_substring_call(inst).is_some() || match_substring_concat3_helper_call(inst).is_some() {
        return true;
    }
    stringish_type(
        function
            .metadata
            .value_types
            .get(&root)
            .or_else(|| function.metadata.value_types.get(&plan_value)),
    )
}

#[derive(Default)]
struct TextConsumerScan {
    slot_text_uses: usize,
    non_slot_uses: usize,
}

#[derive(Default)]
struct ReadAliasConsumerScan {
    direct_set_uses: usize,
    substring_uses: usize,
    len_observer_uses: usize,
    other_uses: usize,
}

fn record_text_consumer_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    plan_root: ValueId,
    inst: &MirInstruction,
    scan: &mut TextConsumerScan,
) {
    if let Some((_, receiver, _, _, _)) = match_substring_call(inst) {
        if resolve_value_origin(function, def_map, receiver) == plan_root {
            scan.slot_text_uses += 1;
            return;
        }
    }

    if let Some(store) = match_method_set_call(inst) {
        if resolve_value_origin(function, def_map, store.value) == plan_root {
            scan.non_slot_uses += 1;
            return;
        }
    }

    match inst {
        MirInstruction::Return {
            value: Some(value), ..
        }
        | MirInstruction::Store { value, .. }
        | MirInstruction::FieldSet { value, .. } => {
            if resolve_value_origin(function, def_map, *value) == plan_root {
                scan.non_slot_uses += 1;
            }
            return;
        }
        MirInstruction::Call {
            callee:
                Some(crate::mir::Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            ..
        } => {
            if resolve_value_origin(function, def_map, *receiver) == plan_root
                && !matches!(method.as_str(), "length" | "size")
            {
                scan.non_slot_uses += 1;
                return;
            }
        }
        MirInstruction::Phi { .. } => return,
        _ => {}
    }

    if inst
        .used_values()
        .into_iter()
        .any(|value| resolve_value_origin(function, def_map, value) == plan_root)
    {
        scan.non_slot_uses += 1;
    }
}

fn record_read_alias_consumer_use(
    function: &MirFunction,
    def_map: &ValueDefMap,
    plan_root: ValueId,
    inst: &MirInstruction,
    scan: &mut ReadAliasConsumerScan,
) {
    if let MirInstruction::Copy { src, .. } = inst {
        if resolve_value_origin(function, def_map, *src) == plan_root {
            return;
        }
    }

    if let Some((_, receiver, _, _, _)) = match_substring_call(inst) {
        if resolve_value_origin(function, def_map, receiver) == plan_root {
            scan.substring_uses += 1;
            return;
        }
    }

    if let Some((_, receiver, _)) = match_len_call(inst) {
        if resolve_value_origin(function, def_map, receiver) == plan_root {
            scan.len_observer_uses += 1;
            return;
        }
    }

    if let Some(store) = match_method_set_call(inst) {
        if resolve_value_origin(function, def_map, store.value) == plan_root {
            scan.direct_set_uses += 1;
            return;
        }
    }

    if inst
        .used_values()
        .into_iter()
        .any(|value| resolve_value_origin(function, def_map, value) == plan_root)
    {
        scan.other_uses += 1;
    }
}

fn scan_read_alias_consumers(
    function: &MirFunction,
    plan_value: ValueId,
    def_map: &ValueDefMap,
) -> ReadAliasConsumerScan {
    let plan_root = resolve_value_origin(function, def_map, plan_value);
    let mut scan = ReadAliasConsumerScan::default();

    for block in function.blocks.values() {
        for inst in &block.instructions {
            record_read_alias_consumer_use(function, def_map, plan_root, inst, &mut scan);
        }
        if let Some(term) = &block.terminator {
            record_read_alias_consumer_use(function, def_map, plan_root, term, &mut scan);
        }
    }

    scan
}

fn derive_read_alias_facts(
    proof: &StringCorridorCandidateProof,
    source_root: Option<ValueId>,
    middle_literal: Option<&str>,
    scan: &ReadAliasConsumerScan,
) -> StringKernelPlanReadAliasFacts {
    let (same_receiver, source_window, contiguous_concat_window) = match *proof {
        StringCorridorCandidateProof::BorrowedSlice { source, .. } => {
            let source_window = source_root == Some(source);
            (source_window, source_window, false)
        }
        StringCorridorCandidateProof::ConcatTriplet {
            left_source,
            left_end,
            right_source,
            right_start,
            shared_source,
            ..
        } => {
            let same_receiver = shared_source && left_source == right_source;
            let source_window = same_receiver && source_root == Some(left_source);
            let contiguous_concat_window = source_window && left_end == right_start;
            (same_receiver, source_window, contiguous_concat_window)
        }
    };

    let piecewise_subrange = contiguous_concat_window && middle_literal.is_some();
    let len_observer_legal = scan.len_observer_uses <= 1;
    let followup_substring =
        scan.substring_uses > 0 && scan.direct_set_uses == 0 && scan.other_uses == 0;
    let direct_set_consumer = piecewise_subrange
        && scan.direct_set_uses == 1
        && scan.substring_uses == 0
        && scan.len_observer_uses == 0
        && scan.other_uses == 0;
    let shared_receiver = piecewise_subrange
        && scan.direct_set_uses == 1
        && scan.substring_uses == 1
        && scan.other_uses == 0
        && len_observer_legal;

    StringKernelPlanReadAliasFacts {
        same_receiver,
        source_window,
        followup_substring,
        piecewise_subrange,
        direct_set_consumer,
        shared_receiver,
    }
}

fn find_single_direct_use_index(
    instructions: &[MirInstruction],
    start_index: usize,
    value: ValueId,
) -> Option<usize> {
    let mut use_index = None;
    for (idx, inst) in instructions.iter().enumerate().skip(start_index) {
        if !inst.used_values().contains(&value) {
            continue;
        }
        if use_index.is_some() {
            return None;
        }
        use_index = Some(idx);
    }
    use_index
}

fn infer_slot_text_consumer_from_def_map(
    function: &MirFunction,
    def_map: &ValueDefMap,
    plan_value: ValueId,
) -> bool {
    if !inferred_text_output(function, plan_value, def_map) {
        return false;
    }

    let plan_root = resolve_value_origin(function, def_map, plan_value);
    let mut scan = TextConsumerScan::default();

    for block in function.blocks.values() {
        for inst in &block.instructions {
            record_text_consumer_use(function, def_map, plan_root, inst, &mut scan);
        }
        if let Some(term) = &block.terminator {
            record_text_consumer_use(function, def_map, plan_root, term, &mut scan);
        }
    }

    scan.non_slot_uses == 0 && scan.slot_text_uses == 1
}

fn derive_slot_hop_substring(
    function: &MirFunction,
    def_map: &ValueDefMap,
    plan_value: ValueId,
    text_consumer: Option<StringKernelPlanTextConsumer>,
) -> Option<StringKernelPlanSlotHopSubstring> {
    if !matches!(text_consumer, Some(StringKernelPlanTextConsumer::SlotText)) {
        return None;
    }

    let plan_root = resolve_value_origin(function, def_map, plan_value);
    let (bbid, def_idx) = def_map.get(&plan_root).copied()?;
    let instructions = function.blocks.get(&bbid)?.instructions.as_slice();
    let mut cursor = plan_root;
    let mut start_index = def_idx + 1;
    let mut copy_instruction_indices = Vec::new();

    loop {
        let use_idx = find_single_direct_use_index(instructions, start_index, cursor)?;
        match instructions.get(use_idx)? {
            MirInstruction::Copy { dst, src } if *src == cursor => {
                if copy_instruction_indices.len() >= 8 {
                    return None;
                }
                copy_instruction_indices.push(use_idx);
                cursor = *dst;
                start_index = use_idx + 1;
            }
            inst => {
                let (consumer_value, receiver, start, end, _) = match_substring_call(inst)?;
                if receiver != cursor {
                    return None;
                }
                if infer_slot_text_consumer_from_def_map(function, def_map, consumer_value) {
                    return None;
                }
                return Some(StringKernelPlanSlotHopSubstring {
                    consumer_value,
                    start,
                    end,
                    instruction_index: use_idx,
                    copy_instruction_indices,
                });
            }
        }
    }
}

pub fn infer_string_kernel_text_consumer(
    function: &MirFunction,
    plan_value: ValueId,
) -> Option<StringKernelPlanTextConsumer> {
    let def_map = build_value_def_map(function);
    if !inferred_text_output(function, plan_value, &def_map) {
        return None;
    }

    let plan_root = resolve_value_origin(function, &def_map, plan_value);
    let mut scan = TextConsumerScan::default();

    for block in function.blocks.values() {
        for inst in &block.instructions {
            record_text_consumer_use(function, &def_map, plan_root, inst, &mut scan);
        }
        if let Some(term) = &block.terminator {
            record_text_consumer_use(function, &def_map, plan_root, term, &mut scan);
        }
    }

    if scan.non_slot_uses > 0 || scan.slot_text_uses > 1 {
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    } else if scan.slot_text_uses == 1 {
        Some(StringKernelPlanTextConsumer::SlotText)
    } else {
        None
    }
}

/// Derive a backend-consumable string kernel plan from current candidate metadata.
pub fn derive_string_kernel_plan(
    function: &MirFunction,
    plan_value: ValueId,
    candidates: &[StringCorridorCandidate],
) -> Option<StringKernelPlan> {
    let mut representative: Option<StringCorridorCandidate> = None;
    let mut publication = None;
    let mut materialization = None;
    let mut direct_kernel_entry = None;
    let mut publication_boundary = None;
    let mut publish_reason = None;
    let mut publish_repr_policy = None;
    let mut stable_view_provenance = None;

    for candidate in candidates {
        match candidate.kind {
            StringCorridorCandidateKind::PublicationSink => {
                publication = Some(candidate.state);
                if let Some(plan) = candidate.plan {
                    publish_reason = plan.publish_reason.or(publish_reason);
                    publish_repr_policy = plan.publish_repr_policy.or(publish_repr_policy);
                    stable_view_provenance = plan.stable_view_provenance.or(stable_view_provenance);
                }
                if matches!(
                    candidate.publication_boundary,
                    Some(StringCorridorPublicationBoundary::FirstExternalBoundary)
                ) {
                    publication_boundary =
                        Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary);
                }
            }
            StringCorridorCandidateKind::MaterializationSink => {
                materialization = Some(candidate.state)
            }
            StringCorridorCandidateKind::DirectKernelEntry => {
                direct_kernel_entry = Some(candidate.state)
            }
            StringCorridorCandidateKind::BorrowCorridorFusion => {}
        }

        let Some(plan) = candidate.plan else {
            continue;
        };
        representative = match representative {
            Some(current)
                if current.plan.is_some()
                    && candidate_priority(current.kind) <= candidate_priority(candidate.kind) =>
            {
                Some(current)
            }
            _ => Some(StringCorridorCandidate {
                kind: candidate.kind,
                state: candidate.state,
                reason: candidate.reason,
                plan: Some(plan),
                publication_boundary: candidate.publication_boundary,
            }),
        };
    }

    let representative = representative?;
    let plan = representative.plan?;
    let borrow_contract = borrow_contract_from_plan(plan);
    let publication_contract = publication_contract_from_plan(plan);
    let stable_view_provenance = stable_view_provenance.or(plan.stable_view_provenance);
    let family = match plan.proof {
        StringCorridorCandidateProof::BorrowedSlice { .. } => {
            StringKernelPlanFamily::BorrowedSliceWindow
        }
        StringCorridorCandidateProof::ConcatTriplet { .. } => {
            StringKernelPlanFamily::ConcatTripletWindow
        }
    };

    let def_map = build_value_def_map(function);
    let middle_literal = match plan.proof {
        StringCorridorCandidateProof::ConcatTriplet { middle, .. } => {
            const_string_literal(function, &def_map, middle).map(|(_, text)| text)
        }
        _ => None,
    };
    let loop_payload = match plan.proof {
        StringCorridorCandidateProof::ConcatTriplet { .. } => derive_concat_triplet_loop_payload(
            function,
            &plan.proof,
            plan.source_root.unwrap_or(plan.corridor_root),
        ),
        _ => None,
    };
    let text_consumer = infer_string_kernel_text_consumer(function, plan_value);
    let slot_hop_substring =
        derive_slot_hop_substring(function, &def_map, plan_value, text_consumer);
    let read_alias_scan = scan_read_alias_consumers(function, plan_value, &def_map);
    let read_alias = derive_read_alias_facts(
        &plan.proof,
        plan.source_root,
        middle_literal.as_deref(),
        &read_alias_scan,
    );
    if matches!(
        text_consumer,
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    ) {
        publish_reason = Some(StringPublishReason::ExplicitApiReplay);
        publish_repr_policy.get_or_insert(StringPublishReprPolicy::StableOwned);
    }
    let carrier = text_consumer.map(|_| StringKernelPlanCarrier::KernelTextSlot);
    let verifier_owner =
        direct_kernel_entry.map(|_| StringKernelPlanVerifierOwner::LoweringDirectKernelEntry);

    Some(StringKernelPlan {
        plan_value,
        version: 1,
        family,
        corridor_root: plan.corridor_root,
        source_root: plan.source_root,
        borrow_contract,
        publish_reason,
        publish_repr_policy,
        stable_view_provenance,
        known_length: plan.known_length,
        retained_form: StringKernelPlanRetainedForm::BorrowedText,
        publication_boundary,
        publication_contract,
        publication,
        materialization,
        direct_kernel_entry,
        consumer: direct_kernel_entry.map(|_| StringKernelPlanConsumer::DirectKernelEntry),
        text_consumer,
        carrier,
        verifier_owner,
        read_alias,
        slot_hop_substring,
        proof: plan.proof,
        middle_literal,
        loop_payload,
    })
}

pub fn refresh_module_string_kernel_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_kernel_plans(function);
    }
}

pub fn refresh_function_string_kernel_plans(function: &mut MirFunction) {
    let mut plans = BTreeMap::new();
    for (plan_value, candidates) in &function.metadata.string_corridor_candidates {
        if let Some(plan) = derive_string_kernel_plan(function, *plan_value, candidates) {
            plans.insert(*plan_value, plan);
        }
    }
    function.metadata.string_kernel_plans = plans;
}

#[cfg(test)]
mod tests;
