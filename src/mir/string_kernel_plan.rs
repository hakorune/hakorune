/*!
 * Backend-consumable string kernel plan seam.
 *
 * This module owns the thin derived view that MIR refresh now materializes
 * first-class. It is downstream of string corridor candidates and upstream of
 * JSON/shim transport. Placement remains the owner of candidate metadata
 * itself.
 */

use std::collections::BTreeMap;

use super::{
    build_value_def_map, resolve_value_origin,
    string_corridor_placement::{
        StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidateProof,
        StringCorridorCandidateState,
    },
    string_corridor_recognizer::{
        match_len_call, match_method_set_call, match_substring_call,
        match_substring_concat3_helper_call,
    },
    CompareOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

/// Backend-consumable family names derived from string corridor candidate plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanFamily {
    BorrowedSliceWindow,
    ConcatTripletWindow,
}

impl std::fmt::Display for StringKernelPlanFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowedSliceWindow => f.write_str("borrowed_slice_window"),
            Self::ConcatTripletWindow => f.write_str("concat_triplet_window"),
        }
    }
}

/// Current retained-form names exported to backend consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanRetainedForm {
    BorrowedText,
}

impl std::fmt::Display for StringKernelPlanRetainedForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowedText => f.write_str("borrowed_text"),
        }
    }
}

/// Backend consumer role selected from current candidate families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanConsumer {
    DirectKernelEntry,
}

impl std::fmt::Display for StringKernelPlanConsumer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
        }
    }
}

/// Direct-kernel text consumer rule derived from the current MIR uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanTextConsumer {
    SlotText,
    ExplicitColdPublish,
}

impl std::fmt::Display for StringKernelPlanTextConsumer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlotText => f.write_str("slot_text"),
            Self::ExplicitColdPublish => f.write_str("explicit_cold_publish"),
        }
    }
}

/// Runtime-private direct-kernel carrier selected by MIR/lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanCarrier {
    KernelTextSlot,
    RegistryBackedHandle,
}

impl std::fmt::Display for StringKernelPlanCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KernelTextSlot => f.write_str("kernel_text_slot"),
            Self::RegistryBackedHandle => f.write_str("registry_backed_handle"),
        }
    }
}

/// Backend-consumable borrow/provenance contract for object -> text entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanBorrowContract {
    BorrowTextFromObject,
}

impl std::fmt::Display for StringKernelPlanBorrowContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowTextFromObject => f.write_str("borrow_text_from_obj"),
        }
    }
}

/// Owner responsible for legality verification on the current direct-kernel lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanVerifierOwner {
    LoweringDirectKernelEntry,
}

impl std::fmt::Display for StringKernelPlanVerifierOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoweringDirectKernelEntry => f.write_str("lowering_direct_kernel_entry"),
        }
    }
}

/// Backend-consumable publication boundary for a string kernel plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanPublicationBoundary {
    FirstExternalBoundary,
}

impl std::fmt::Display for StringKernelPlanPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstExternalBoundary => f.write_str("first_external_boundary"),
        }
    }
}

/// Backend-consumable MIR proof that publication may stay deferred on this plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKernelPlanPublicationContract {
    PublishNowNotRequiredBeforeFirstExternalBoundary,
}

impl std::fmt::Display for StringKernelPlanPublicationContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PublishNowNotRequiredBeforeFirstExternalBoundary => {
                f.write_str("publish_now_not_required_before_first_external_boundary")
            }
        }
    }
}

/// Backend-consumable string kernel plan part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringKernelPlanPart {
    Slice {
        value: Option<ValueId>,
        source: ValueId,
        start: ValueId,
        end: ValueId,
    },
    Const {
        value: ValueId,
        known_length: Option<i64>,
        literal: Option<String>,
    },
}

/// Narrow scalar payload for the current substring-concat exact loop route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlanLoopPayload {
    pub seed_value: ValueId,
    pub seed_literal: String,
    pub seed_length: i64,
    pub loop_bound: i64,
    pub split_length: i64,
}

/// Thin legality facts that backend consumers may check before emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringKernelPlanLegality {
    pub byte_exact: bool,
    pub no_publish_inside: bool,
    pub requires_kernel_text_slot: bool,
    pub rejects_early_stable_box_now: bool,
    pub rejects_early_fresh_registry_handle: bool,
    pub rejects_registry_backed_carrier: bool,
}

/// Thin backend-consumable kernel plan derived from the current candidate set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringKernelPlan {
    pub plan_value: ValueId,
    pub version: u32,
    pub family: StringKernelPlanFamily,
    pub corridor_root: ValueId,
    pub source_root: Option<ValueId>,
    pub borrow_contract: Option<StringKernelPlanBorrowContract>,
    pub publish_reason: Option<crate::mir::StringPublishReason>,
    pub publish_repr_policy: Option<crate::mir::StringPublishReprPolicy>,
    pub stable_view_provenance: Option<crate::mir::StringStableViewProvenance>,
    pub known_length: Option<i64>,
    pub retained_form: StringKernelPlanRetainedForm,
    pub publication_boundary: Option<StringKernelPlanPublicationBoundary>,
    pub publication_contract: Option<StringKernelPlanPublicationContract>,
    pub publication: Option<StringCorridorCandidateState>,
    pub materialization: Option<StringCorridorCandidateState>,
    pub direct_kernel_entry: Option<StringCorridorCandidateState>,
    pub consumer: Option<StringKernelPlanConsumer>,
    pub text_consumer: Option<StringKernelPlanTextConsumer>,
    pub carrier: Option<StringKernelPlanCarrier>,
    pub verifier_owner: Option<StringKernelPlanVerifierOwner>,
    pub proof: StringCorridorCandidateProof,
    pub middle_literal: Option<String>,
    pub loop_payload: Option<StringKernelPlanLoopPayload>,
}

impl StringKernelPlan {
    pub fn parts(&self) -> Vec<StringKernelPlanPart> {
        match self.proof {
            StringCorridorCandidateProof::BorrowedSlice { source, start, end } => {
                vec![StringKernelPlanPart::Slice {
                    value: None,
                    source,
                    start,
                    end,
                }]
            }
            StringCorridorCandidateProof::ConcatTriplet {
                left_value,
                left_source,
                left_start,
                left_end,
                middle,
                right_value,
                right_source,
                right_start,
                right_end,
                shared_source: _,
            } => vec![
                StringKernelPlanPart::Slice {
                    value: left_value,
                    source: left_source,
                    start: left_start,
                    end: left_end,
                },
                StringKernelPlanPart::Const {
                    value: middle,
                    known_length: self.known_length,
                    literal: self.middle_literal.clone(),
                },
                StringKernelPlanPart::Slice {
                    value: right_value,
                    source: right_source,
                    start: right_start,
                    end: right_end,
                },
            ],
        }
    }

    pub fn legality(&self) -> StringKernelPlanLegality {
        let requires_kernel_text_slot = self.text_consumer.is_some();
        let reject_early_publish = self.publication_contract.is_some() && requires_kernel_text_slot;
        StringKernelPlanLegality {
            byte_exact: true,
            no_publish_inside: self.publication_contract.is_some(),
            requires_kernel_text_slot,
            rejects_early_stable_box_now: reject_early_publish,
            rejects_early_fresh_registry_handle: reject_early_publish,
            rejects_registry_backed_carrier: reject_early_publish,
        }
    }
}

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
            crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
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
        Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject) => {
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
                    Some(crate::mir::StringCorridorPublicationBoundary::FirstExternalBoundary)
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
    if matches!(
        text_consumer,
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    ) {
        publish_reason = Some(crate::mir::StringPublishReason::ExplicitApiReplay);
        publish_repr_policy.get_or_insert(crate::mir::StringPublishReprPolicy::StableOwned);
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
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, EffectMask, FunctionSignature, MirType,
        StringCorridorPublicationBoundary,
    };

    fn make_loop_function() -> MirFunction {
        let entry = BasicBlockId::new(0);
        let header = BasicBlockId::new(18);
        let body = BasicBlockId::new(19);
        let exit = BasicBlockId::new(21);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );

        function
            .blocks
            .get_mut(&entry)
            .unwrap()
            .instructions
            .extend([
                MirInstruction::Const {
                    dst: ValueId::new(3),
                    value: ConstValue::String("line-seed-abcdef".to_string()),
                },
                MirInstruction::Copy {
                    dst: ValueId::new(4),
                    src: ValueId::new(3),
                },
                MirInstruction::Const {
                    dst: ValueId::new(5),
                    value: ConstValue::Integer(16),
                },
            ]);

        let mut header_block = BasicBlock::new(header);
        header_block.instructions.extend([
            MirInstruction::Phi {
                dst: ValueId::new(15),
                inputs: vec![(entry, ValueId::new(12)), (body, ValueId::new(16))],
                type_hint: Some(MirType::Integer),
            },
            MirInstruction::Phi {
                dst: ValueId::new(21),
                inputs: vec![(entry, ValueId::new(4)), (body, ValueId::new(36))],
                type_hint: Some(MirType::String),
            },
            MirInstruction::Const {
                dst: ValueId::new(41),
                value: ConstValue::Integer(300000),
            },
            MirInstruction::Compare {
                dst: ValueId::new(37),
                op: CompareOp::Lt,
                lhs: ValueId::new(15),
                rhs: ValueId::new(41),
            },
            MirInstruction::Branch {
                condition: ValueId::new(37),
                then_bb: body,
                else_bb: exit,
                then_edge_args: None,
                else_edge_args: None,
            },
        ]);
        function.blocks.insert(header, header_block);

        let mut body_block = BasicBlock::new(body);
        body_block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(50),
                value: ConstValue::Integer(2),
            },
            MirInstruction::BinOp {
                dst: ValueId::new(47),
                op: BinaryOp::Div,
                lhs: ValueId::new(5),
                rhs: ValueId::new(50),
            },
            MirInstruction::Const {
                dst: ValueId::new(66),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Copy {
                dst: ValueId::new(36),
                src: ValueId::new(21),
            },
        ]);
        function.blocks.insert(body, body_block);
        function.blocks.insert(exit, BasicBlock::new(exit));
        function
    }

    #[test]
    fn derive_string_kernel_plan_prefers_direct_entry_and_collects_barriers() {
        let function = make_loop_function();
        let publication_plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: Some(crate::mir::StringPublishReason::StableObjectDemand),
            publish_repr_policy: Some(crate::mir::StringPublishReprPolicy::StableOwned),
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(4)),
                left_source: ValueId::new(1),
                left_start: ValueId::new(4),
                left_end: ValueId::new(5),
                middle: ValueId::new(6),
                right_value: Some(ValueId::new(8)),
                right_source: ValueId::new(1),
                right_start: ValueId::new(5),
                right_end: ValueId::new(9),
                shared_source: true,
            },
        };
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(4)),
                left_source: ValueId::new(1),
                left_start: ValueId::new(4),
                left_end: ValueId::new(5),
                middle: ValueId::new(6),
                right_value: Some(ValueId::new(8)),
                right_source: ValueId::new(1),
                right_start: ValueId::new(5),
                right_end: ValueId::new(9),
                shared_source: true,
            },
        };
        let candidates = vec![
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::PublicationSink,
                state: StringCorridorCandidateState::AlreadySatisfied,
                reason: "publish boundary is already sunk at the current corridor exit",
                plan: Some(publication_plan),
                publication_boundary: Some(
                    StringCorridorPublicationBoundary::FirstExternalBoundary,
                ),
            },
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::MaterializationSink,
                state: StringCorridorCandidateState::Candidate,
                reason: "slice result may stay borrowed until a later boundary",
                plan: Some(plan),
                publication_boundary: None,
            },
            StringCorridorCandidate {
                kind: StringCorridorCandidateKind::DirectKernelEntry,
                state: StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(plan),
                publication_boundary: Some(
                    StringCorridorPublicationBoundary::FirstExternalBoundary,
                ),
            },
        ];

        let kernel_plan = derive_string_kernel_plan(&function, ValueId::new(8), &candidates)
            .expect("kernel plan");

        assert_eq!(kernel_plan.plan_value, ValueId::new(8));
        assert_eq!(kernel_plan.version, 1);
        assert_eq!(
            kernel_plan.family,
            StringKernelPlanFamily::ConcatTripletWindow
        );
        assert_eq!(kernel_plan.corridor_root, ValueId::new(7));
        assert_eq!(kernel_plan.source_root, Some(ValueId::new(1)));
        assert_eq!(
            kernel_plan.borrow_contract,
            Some(StringKernelPlanBorrowContract::BorrowTextFromObject)
        );
        assert_eq!(
            kernel_plan.publish_reason,
            Some(crate::mir::StringPublishReason::StableObjectDemand)
        );
        assert_eq!(
            kernel_plan.publish_repr_policy,
            Some(crate::mir::StringPublishReprPolicy::StableOwned)
        );
        assert_eq!(kernel_plan.known_length, Some(2));
        assert_eq!(
            kernel_plan.retained_form,
            StringKernelPlanRetainedForm::BorrowedText
        );
        assert_eq!(
            kernel_plan.publication_boundary,
            Some(StringKernelPlanPublicationBoundary::FirstExternalBoundary)
        );
        assert_eq!(
            kernel_plan.publication_contract,
            Some(
                StringKernelPlanPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary
            )
        );
        assert_eq!(
            kernel_plan.publication,
            Some(StringCorridorCandidateState::AlreadySatisfied)
        );
        assert_eq!(
            kernel_plan.materialization,
            Some(StringCorridorCandidateState::Candidate)
        );
        assert_eq!(
            kernel_plan.direct_kernel_entry,
            Some(StringCorridorCandidateState::Candidate)
        );
        assert_eq!(
            kernel_plan.consumer,
            Some(StringKernelPlanConsumer::DirectKernelEntry)
        );
        assert_eq!(kernel_plan.text_consumer, None);
        assert_eq!(kernel_plan.carrier, None);
        assert_eq!(
            kernel_plan.verifier_owner,
            Some(StringKernelPlanVerifierOwner::LoweringDirectKernelEntry)
        );
        let parts = kernel_plan.parts();
        assert_eq!(parts.len(), 3);
        assert_eq!(
            kernel_plan.legality(),
            StringKernelPlanLegality {
                byte_exact: true,
                no_publish_inside: true,
                requires_kernel_text_slot: false,
                rejects_early_stable_box_now: false,
                rejects_early_fresh_registry_handle: false,
                rejects_registry_backed_carrier: false,
            }
        );
    }

    #[test]
    fn derive_string_kernel_plan_collects_concat_loop_payload() {
        let function = make_loop_function();
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(21),
            source_root: Some(ValueId::new(21)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(71)),
            end: Some(ValueId::new(72)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(26)),
                left_source: ValueId::new(21),
                left_start: ValueId::new(46),
                left_end: ValueId::new(47),
                middle: ValueId::new(66),
                right_value: Some(ValueId::new(27)),
                right_source: ValueId::new(21),
                right_start: ValueId::new(47),
                right_end: ValueId::new(42),
                shared_source: true,
            },
        };
        let candidates = vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "direct kernel entry candidate",
            plan: Some(plan),
            publication_boundary: None,
        }];

        let kernel_plan = derive_string_kernel_plan(&function, ValueId::new(21), &candidates)
            .expect("kernel plan");
        let payload = kernel_plan.loop_payload.expect("loop payload");

        assert_eq!(payload.seed_value, ValueId::new(3));
        assert_eq!(payload.seed_literal, "line-seed-abcdef");
        assert_eq!(payload.seed_length, 16);
        assert_eq!(payload.loop_bound, 300000);
        assert_eq!(payload.split_length, 8);
        assert_eq!(kernel_plan.middle_literal.as_deref(), Some("xx"));
    }

    #[test]
    fn derive_string_kernel_plan_marks_slot_text_consumer_for_same_corridor_substring() {
        use crate::ast::Span;
        use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

        fn method_call(
            dst: ValueId,
            receiver: ValueId,
            method: &str,
            args: Vec<ValueId>,
        ) -> MirInstruction {
            MirInstruction::Call {
                dst: Some(dst),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: method.to_string(),
                    receiver: Some(receiver),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args,
                effects: EffectMask::PURE,
            }
        }

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(6),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(1),
            },
            MirInstruction::Const {
                dst: ValueId::new(4),
                value: ConstValue::Integer(5),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Extern(
                    "nyash.string.substring_concat3_hhhii".to_string(),
                )),
                args: vec![
                    ValueId::new(0),
                    ValueId::new(1),
                    ValueId::new(0),
                    ValueId::new(3),
                    ValueId::new(4),
                ],
                effects: EffectMask::PURE,
            },
            method_call(
                ValueId::new(11),
                ValueId::new(10),
                "substring",
                vec![ValueId::new(3), ValueId::new(4)],
            ),
        ]);
        block.instruction_spans.extend(vec![Span::unknown(); 6]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(11)),
        });

        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(3)),
            end: Some(ValueId::new(4)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(0)),
                left_source: ValueId::new(0),
                left_start: ValueId::new(3),
                left_end: ValueId::new(2),
                middle: ValueId::new(1),
                right_value: Some(ValueId::new(0)),
                right_source: ValueId::new(0),
                right_start: ValueId::new(2),
                right_end: ValueId::new(4),
                shared_source: true,
            },
        };
        let candidates = vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "direct kernel entry candidate",
            plan: Some(plan),
            publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
        }];

        let kernel_plan = derive_string_kernel_plan(&function, ValueId::new(10), &candidates)
            .expect("kernel plan");

        assert_eq!(
            kernel_plan.text_consumer,
            Some(StringKernelPlanTextConsumer::SlotText)
        );
        assert_eq!(
            kernel_plan.carrier,
            Some(StringKernelPlanCarrier::KernelTextSlot)
        );
        assert_eq!(
            kernel_plan.legality(),
            StringKernelPlanLegality {
                byte_exact: true,
                no_publish_inside: true,
                requires_kernel_text_slot: true,
                rejects_early_stable_box_now: true,
                rejects_early_fresh_registry_handle: true,
                rejects_registry_backed_carrier: true,
            }
        );
    }

    #[test]
    fn infer_string_kernel_text_consumer_marks_return_boundary_as_explicit_cold_publish() {
        use crate::ast::Span;

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(3),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(8),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Extern(
                    "nyash.string.substring_concat3_hhhii".to_string(),
                )),
                args: vec![
                    ValueId::new(0),
                    ValueId::new(1),
                    ValueId::new(0),
                    ValueId::new(2),
                    ValueId::new(3),
                ],
                effects: EffectMask::PURE,
            },
        ]);
        block.instruction_spans.extend(vec![Span::unknown(); 4]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(10)),
        });

        assert_eq!(
            infer_string_kernel_text_consumer(&function, ValueId::new(10)),
            Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
        );
    }

    #[test]
    fn derive_string_kernel_plan_refines_explicit_cold_publish_reason() {
        use crate::ast::Span;

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("xx".to_string()),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(3),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(8),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Extern(
                    "nyash.string.substring_concat3_hhhii".to_string(),
                )),
                args: vec![
                    ValueId::new(0),
                    ValueId::new(1),
                    ValueId::new(0),
                    ValueId::new(2),
                    ValueId::new(3),
                ],
                effects: EffectMask::PURE,
            },
        ]);
        block.instruction_spans.extend(vec![Span::unknown(); 4]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(10)),
        });

        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(10),
            source_root: Some(ValueId::new(0)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(0)),
                left_source: ValueId::new(0),
                left_start: ValueId::new(2),
                left_end: ValueId::new(2),
                middle: ValueId::new(1),
                right_value: Some(ValueId::new(0)),
                right_source: ValueId::new(0),
                right_start: ValueId::new(2),
                right_end: ValueId::new(3),
                shared_source: true,
            },
        };
        let candidates = vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "direct kernel entry candidate",
            plan: Some(plan),
            publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
        }];

        let kernel_plan = derive_string_kernel_plan(&function, ValueId::new(10), &candidates)
            .expect("kernel plan");

        assert_eq!(
            kernel_plan.publish_reason,
            Some(crate::mir::StringPublishReason::ExplicitApiReplay)
        );
        assert_eq!(
            kernel_plan.publish_repr_policy,
            Some(crate::mir::StringPublishReprPolicy::StableOwned)
        );
    }

    #[test]
    fn refresh_function_collects_string_kernel_plans() {
        let mut function = make_loop_function();
        let plan = super::super::string_corridor_placement::StringCorridorCandidatePlan {
            corridor_root: ValueId::new(7),
            source_root: Some(ValueId::new(1)),
            borrow_contract: Some(crate::mir::StringCorridorBorrowContract::BorrowTextFromObject),
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            start: Some(ValueId::new(2)),
            end: Some(ValueId::new(3)),
            known_length: Some(2),
            publication_contract: Some(
                crate::mir::StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
            ),
            proof: StringCorridorCandidateProof::ConcatTriplet {
                left_value: Some(ValueId::new(4)),
                left_source: ValueId::new(1),
                left_start: ValueId::new(4),
                left_end: ValueId::new(5),
                middle: ValueId::new(6),
                right_value: Some(ValueId::new(8)),
                right_source: ValueId::new(1),
                right_start: ValueId::new(5),
                right_end: ValueId::new(9),
                shared_source: true,
            },
        };
        function.metadata.string_corridor_candidates.insert(
            ValueId::new(8),
            vec![StringCorridorCandidate {
                kind: StringCorridorCandidateKind::DirectKernelEntry,
                state: StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(plan),
                publication_boundary: None,
            }],
        );

        refresh_function_string_kernel_plans(&mut function);

        let kernel_plans = &function.metadata.string_kernel_plans;
        let kernel_plan = kernel_plans.get(&ValueId::new(8)).expect("kernel plan");
        assert_eq!(kernel_plan.version, 1);
        assert_eq!(
            kernel_plan.family,
            StringKernelPlanFamily::ConcatTripletWindow
        );
        assert_eq!(
            kernel_plan.consumer,
            Some(StringKernelPlanConsumer::DirectKernelEntry)
        );
    }
}
