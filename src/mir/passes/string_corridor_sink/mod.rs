//! Borrowed string corridor sinking pilot.
//!
//! First real transforms for the string corridor lane:
//! `substring(...).length()`, retained-slice `length()` consumers, and the
//! narrow `concat(left_slice, const, right_slice)` observer/slice shape are
//! rewritten so the corridor can stay borrowed without forcing
//! publication/materialization.
//! Complementary `substring_len_hii` pairs can then fuse back to one source
//! length add when the compiler can prove they partition the same source.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin};
use crate::mir::{
    refresh_function_placement_effect_routes, refresh_function_string_corridor_metadata,
    refresh_function_string_kernel_plans, refresh_function_value_consumer_facts,
    string_corridor::{
        StringCorridorBorrowContract, StringCorridorOp, StringPublishReason,
        StringPublishReprPolicy,
    },
    string_corridor_placement::{
        StringCorridorCandidateKind, StringCorridorCandidatePlan, StringCorridorCandidateProof,
    },
    string_corridor_recognizer::{
        const_string_length, extract_substring_args, match_add_in_block, match_concat_triplet,
        match_len_call, match_method_set_call, match_substring_call, match_substring_call_shape,
        match_substring_concat3_helper_call, match_substring_len_call, string_source_identity,
        ConcatTripletShape, MethodSetCallShape, StringSourceIdentity, SubstringCallProducerShape,
        SubstringConcat3HelperShape,
    },
    string_kernel_plan::StringKernelPlanTextConsumer,
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction, MirInstruction, MirModule,
    MirType, ValueId,
};

pub const SUBSTRING_LEN_EXTERN: &str = "nyash.string.substring_len_hii";
pub const SUBSTRING_CONCAT3_EXTERN: &str = "nyash.string.substring_concat3_hhhii";
pub const SUBSTRING_CONCAT3_PUBLISH_EXPLICIT_API_OWNED_EXTERN: &str =
    "nyash.string.substring_concat3_publish_explicit_api_owned_hhhii";
pub const SUBSTRING_CONCAT3_PUBLISH_NEED_STABLE_OWNED_EXTERN: &str =
    "nyash.string.substring_concat3_publish_need_stable_owned_hhhii";
pub const INSERT_HSI_EXTERN: &str = "nyash.string.insert_hsi";

mod concat_corridor;
mod fusion;
mod publication;
mod retained_len;
mod shared;
#[cfg(test)]
mod tests;

use concat_corridor::*;
use fusion::*;
use publication::*;
use retained_len::*;
use shared::*;

pub fn sink_borrowed_string_corridors(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for (_name, function) in &mut module.functions {
        rewritten += apply_string_corridor_pre_dce_transforms(function);
    }
    rewritten
}

pub(crate) fn apply_string_corridor_pre_dce_transforms(function: &mut MirFunction) -> usize {
    apply_string_corridor_transforms(function)
}

pub(crate) fn apply_string_corridor_post_dce_transforms(function: &mut MirFunction) -> usize {
    apply_string_corridor_transforms(function)
}

/// Rebuild def_map and use_counts after the function was mutated.
fn refresh_analysis(
    function: &MirFunction,
    def_map: &mut HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &mut HashMap<ValueId, usize>,
    n: usize,
) {
    if n > 0 {
        *def_map = build_value_def_map(function);
        *use_counts = build_use_counts(function);
    }
}

fn apply_string_corridor_transforms(function: &mut MirFunction) -> usize {
    if !has_string_corridor_transform_sites(function) {
        return 0;
    }

    refresh_function_string_corridor_folded_metadata(function);

    let mut def_map = build_value_def_map(function);
    let mut use_counts = build_use_counts(function);

    // --- Phase 1: collect direct stable-length hints ---
    let stable_length_hints = collect_direct_stable_length_hints(function, &def_map);
    for hint in stable_length_hints {
        if !function.metadata.optimization_hints.contains(&hint) {
            function.metadata.optimization_hints.push(hint);
        }
    }

    // --- Phase 2: substring-length direct rewrite ---
    let plans_by_block = collect_plans(function, &def_map, &use_counts);
    let mut rewritten = apply_plans(function, plans_by_block);
    refresh_analysis(function, &mut def_map, &mut use_counts, rewritten);

    // --- Phase 3: retained-length rewrite ---
    let retained_len_plans = collect_retained_len_plans(function, &def_map, &use_counts);
    let n = apply_retained_len_plans(function, retained_len_plans);
    rewritten += n;
    refresh_analysis(function, &mut def_map, &mut use_counts, n);

    // --- Phase 4: first concat-corridor pass (immediate plans only) ---
    let concat_corridor_plans = collect_concat_corridor_plans(function, &def_map, &use_counts);
    let mut immediate_concat_plans_by_block = BTreeMap::new();
    for (bbid, plans) in concat_corridor_plans {
        let immediate_plans: Vec<_> = plans
            .into_iter()
            .filter(|plan| !matches!(plan, ConcatCorridorPlan::Substring(_)))
            .collect();
        if !immediate_plans.is_empty() {
            immediate_concat_plans_by_block.insert(bbid, immediate_plans);
        }
    }
    let n = apply_concat_corridor_plans(function, immediate_concat_plans_by_block);
    rewritten += n;
    refresh_analysis(function, &mut def_map, &mut use_counts, n);

    // --- Phase 5: publication — return boundary ---
    let publication_return_plans =
        collect_publication_return_plans(function, &def_map, &use_counts);
    let n = apply_publication_return_plans(function, publication_return_plans);
    rewritten += n;
    refresh_analysis(function, &mut def_map, &mut use_counts, n);

    // --- Phase 6: publication — write boundary ---
    let publication_write_boundary_plans =
        collect_publication_write_boundary_plans(function, &def_map, &use_counts);
    let n = apply_publication_write_boundary_plans(function, publication_write_boundary_plans);
    rewritten += n;
    refresh_analysis(function, &mut def_map, &mut use_counts, n);

    // --- Phase 7: publication — host boundary ---
    let publication_host_boundary_plans =
        collect_publication_host_boundary_plans(function, &def_map, &use_counts);
    let n = apply_publication_host_boundary_plans(function, publication_host_boundary_plans);
    rewritten += n;
    refresh_analysis(function, &mut def_map, &mut use_counts, n);

    // --- Phase 8: corridor-local DCE sweep ---
    // The complementary substring-len fusion owner needs dead intermediate
    // adds/copies to be removed before it can see the final single-use tree.
    let corridor_dce_rewritten =
        crate::mir::passes::dce::eliminate_dead_code_in_function(function);
    rewritten += corridor_dce_rewritten;
    refresh_analysis(function, &mut def_map, &mut use_counts, corridor_dce_rewritten);

    // --- Phase 9: complementary substring-length fusion + DCE ---
    let fusion_plans = collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
    let fusion_rewritten = apply_complementary_len_fusion_plans(function, fusion_plans);
    rewritten += fusion_rewritten;
    if fusion_rewritten > 0 {
        rewritten +=
            crate::mir::passes::dce::eliminate_dead_code_in_function(function);
    }

    // --- Phase 10: second concat-corridor pass + DCE ---
    def_map = build_value_def_map(function);
    use_counts = build_use_counts(function);
    let second_concat_corridor_plans = collect_concat_corridor_plans(function, &def_map, &use_counts);
    let second_concat_corridor_rewritten =
        apply_concat_corridor_plans(function, second_concat_corridor_plans);
    rewritten += second_concat_corridor_rewritten;
    if second_concat_corridor_rewritten > 0 {
        rewritten +=
            crate::mir::passes::dce::eliminate_dead_code_in_function(function);
        use_counts = build_use_counts(function);
    }

    // --- Phase 11: remove unused substring-view producers ---
    rewritten += remove_unused_substring_view_producers(function, &use_counts);

    rewritten
}

fn collect_direct_stable_length_hints(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Vec<String> {
    let mut hints = Vec::new();

    for block in function.blocks.values() {
        for inst in &block.instructions {
            let Some((dst, receiver, _effects)) = match_len_call(inst) else {
                continue;
            };
            let receiver_root = resolve_value_origin(function, def_map, receiver);
            let Some((bbid, idx)) = def_map.get(&receiver_root).copied() else {
                continue;
            };
            let Some(root_block) = function.blocks.get(&bbid) else {
                continue;
            };
            let Some(root_inst) = root_block.instructions.get(idx) else {
                continue;
            };
            if match_substring_call(root_inst).is_some()
                || match_substring_concat3_helper_call(root_inst).is_some()
                || match_substring_len_call(root_inst).is_some()
            {
                continue;
            }
            if !matches!(
                root_inst,
                MirInstruction::Const {
                    value: ConstValue::String(_),
                    ..
                } | MirInstruction::Copy { .. }
                    | MirInstruction::Phi { .. }
            ) {
                continue;
            }
            hints.push(format!(
                "string_corridor_sink:stable_length_scalar:%{}:%{}",
                receiver_root.0,
                resolve_value_origin(function, def_map, dst).0
            ));
        }
    }

    hints
}

fn remove_unused_substring_view_producers(
    function: &mut MirFunction,
    use_counts: &HashMap<ValueId, usize>,
) -> usize {
    let mut removed = 0usize;

    for block in function.blocks.values_mut() {
        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut new_spans = Vec::with_capacity(spans.len());

        for (inst, span) in insts.into_iter().zip(spans.into_iter()) {
            let remove = match_substring_call(&inst)
                .map(|(dst, _, _, _, _)| use_counts.get(&dst).copied().unwrap_or(0) == 0)
                .unwrap_or(false);
            if remove {
                removed += 1;
                continue;
            }
            new_insts.push(inst);
            new_spans.push(span);
        }

        block.instructions = new_insts;
        block.instruction_spans = new_spans;
    }

    if removed > 0 {
        function.update_cfg();
        refresh_function_string_corridor_folded_metadata(function);
        function.metadata.optimization_hints.push(format!(
            "string_corridor_sink:dead_substring_view_producers:{removed}"
        ));
    }

    removed
}

fn has_string_corridor_transform_sites(function: &MirFunction) -> bool {
    function.blocks.values().any(|block| {
        block.instructions.iter().any(|inst| {
            match_len_call(inst).is_some()
                || match_substring_call(inst).is_some()
                || match_substring_concat3_helper_call(inst).is_some()
                || match_substring_len_call(inst).is_some()
        }) || block.terminator.iter().any(|term| {
            match_len_call(term).is_some()
                || match_substring_call(term).is_some()
                || match_substring_concat3_helper_call(term).is_some()
                || match_substring_len_call(term).is_some()
        })
    })
}

pub(crate) fn refresh_function_string_corridor_folded_metadata(function: &mut MirFunction) {
    refresh_function_string_corridor_metadata(function);
    refresh_function_placement_effect_routes(function);
    refresh_function_value_consumer_facts(function);
    refresh_function_string_kernel_plans(function);
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
struct ConcatSubstringLenPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    left: SubstringCallProducerShape,
    right: SubstringCallProducerShape,
    middle_len: i64,
    effects: EffectMask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConcatSubstringPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    left: ValueId,
    middle: ValueId,
    right: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InsertMidSubstringPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    source: ValueId,
    middle: ValueId,
    split: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
    remove_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicationHelperLenPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    start: ValueId,
    end: ValueId,
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicationHelperSubstringPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    left: ValueId,
    middle: ValueId,
    right: ValueId,
    outer_start: ValueId,
    inner_start: ValueId,
    inner_end: ValueId,
    effects: EffectMask,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MaterializationStorePlan {
    helper_idx: usize,
    helper_dst: ValueId,
    store_idx: usize,
    left: ValueId,
    middle: ValueId,
    right: ValueId,
    start: ValueId,
    end: ValueId,
    helper_effects: EffectMask,
    copy_indices: Vec<usize>,
    observer_copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoreSharedReceiverSubstringPlan {
    outer_idx: usize,
    outer_dst: ValueId,
    replacement_receiver: ValueId,
    remove_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicationReturnPlan {
    helper_idx: usize,
    helper_dst: ValueId,
    return_idx: Option<usize>,
    publish_extern: &'static str,
    left: ValueId,
    middle: ValueId,
    right: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicationWriteBoundaryPlan {
    helper_idx: usize,
    helper_dst: ValueId,
    boundary_idx: usize,
    publish_extern: &'static str,
    left: ValueId,
    middle: ValueId,
    right: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PublicationHostBoundaryPlan {
    helper_idx: usize,
    helper_dst: ValueId,
    boundary_idx: usize,
    publish_extern: &'static str,
    left: ValueId,
    middle: ValueId,
    right: ValueId,
    start: ValueId,
    end: ValueId,
    effects: EffectMask,
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReturnSite {
    Instruction(usize),
    Terminator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConcatCorridorPlan {
    Len(ConcatSubstringLenPlan),
    Substring(ConcatSubstringPlan),
    InsertMidSubstring(InsertMidSubstringPlan),
    PublicationLen(PublicationHelperLenPlan),
    PublicationSubstring(PublicationHelperSubstringPlan),
    MaterializationStore(MaterializationStorePlan),
    StoreSharedReceiverSubstring(StoreSharedReceiverSubstringPlan),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComplementarySubstringLenFusionPlan {
    remove_indices: Vec<usize>,
    outer_idx: usize,
    outer_dst: ValueId,
    acc: ValueId,
    source_root: ValueId,
    source_len: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SingleUseCopyChain {
    root: ValueId,
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrailingLenObserverWindow {
    copy_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubstringLenCallShape {
    idx: usize,
    dst: ValueId,
    source: ValueId,
    start: ValueId,
    end: ValueId,
}
