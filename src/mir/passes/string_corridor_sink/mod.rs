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

use crate::mir::{
    build_value_def_map, refresh_function_placement_effect_routes,
    refresh_function_string_corridor_metadata, refresh_function_string_kernel_plans,
    resolve_value_origin,
    string_corridor_recognizer::{
        const_string_length, extract_substring_args, match_add_in_block, match_concat_triplet,
        match_len_call, match_method_set_call, match_substring_call, match_substring_call_shape,
        match_substring_concat3_helper_call, match_substring_len_call, string_source_identity,
        ConcatTripletShape, MethodSetCallShape, StringSourceIdentity, SubstringCallProducerShape,
        SubstringConcat3HelperShape,
    },
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction, MirInstruction, MirModule,
    MirType, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorOp, StringKernelPlanTextConsumer, ValueId,
};

pub const SUBSTRING_LEN_EXTERN: &str = "nyash.string.substring_len_hii";
pub const SUBSTRING_CONCAT3_EXTERN: &str = "nyash.string.substring_concat3_hhhii";
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

fn apply_string_corridor_transforms(function: &mut MirFunction) -> usize {
    if !has_string_corridor_transform_sites(function) {
        return 0;
    }

    refresh_function_string_corridor_folded_metadata(function);

    let mut def_map = build_value_def_map(function);
    let mut use_counts = build_use_counts(function);
    let plans_by_block = collect_plans(function, &def_map, &use_counts);
    let mut rewritten = apply_plans(function, plans_by_block);
    if rewritten > 0 {
        def_map = build_value_def_map(function);
        use_counts = build_use_counts(function);
    }

    let retained_len_plans = collect_retained_len_plans(function, &def_map, &use_counts);
    let retained_len_rewritten = apply_retained_len_plans(function, retained_len_plans);
    rewritten += retained_len_rewritten;
    if retained_len_rewritten > 0 {
        def_map = build_value_def_map(function);
        use_counts = build_use_counts(function);
    }

    let concat_corridor_plans = collect_concat_corridor_plans(function, &def_map, &use_counts);
    let concat_corridor_rewritten = apply_concat_corridor_plans(function, concat_corridor_plans);
    rewritten += concat_corridor_rewritten;
    if concat_corridor_rewritten > 0 {
        def_map = build_value_def_map(function);
        use_counts = build_use_counts(function);
    }

    let publication_return_plans =
        collect_publication_return_plans(function, &def_map, &use_counts);
    let publication_return_rewritten =
        apply_publication_return_plans(function, publication_return_plans);
    rewritten += publication_return_rewritten;
    if publication_return_rewritten > 0 {
        def_map = build_value_def_map(function);
        use_counts = build_use_counts(function);
    }

    let publication_write_boundary_plans =
        collect_publication_write_boundary_plans(function, &def_map, &use_counts);
    let publication_write_boundary_rewritten =
        apply_publication_write_boundary_plans(function, publication_write_boundary_plans);
    rewritten += publication_write_boundary_rewritten;
    if publication_write_boundary_rewritten > 0 {
        def_map = build_value_def_map(function);
        use_counts = build_use_counts(function);
    }

    let publication_host_boundary_plans =
        collect_publication_host_boundary_plans(function, &def_map, &use_counts);
    let publication_host_boundary_rewritten =
        apply_publication_host_boundary_plans(function, publication_host_boundary_plans);
    rewritten += publication_host_boundary_rewritten;
    if publication_host_boundary_rewritten > 0 {
        def_map = build_value_def_map(function);
        use_counts = build_use_counts(function);
    }

    let fusion_plans = collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
    rewritten += apply_complementary_len_fusion_plans(function, fusion_plans);

    rewritten
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
