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
    build_value_def_map, refresh_function_string_corridor_metadata, resolve_value_origin,
    string_corridor_recognizer::{
        const_string_length, extract_substring_args, match_add_in_block, match_concat_triplet,
        match_len_call, match_method_set_call, match_substring_call, match_substring_call_shape,
        match_substring_concat3_helper_call, match_substring_len_call, string_source_identity,
        ConcatTripletShape, MethodSetCallShape, StringSourceIdentity, SubstringCallProducerShape,
        SubstringConcat3HelperShape,
    },
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirFunction, MirInstruction, MirModule,
    MirType, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorOp, ValueId,
};

pub const SUBSTRING_LEN_EXTERN: &str = "nyash.string.substring_len_hii";
pub const SUBSTRING_CONCAT3_EXTERN: &str = "nyash.string.substring_concat3_hhhii";

pub fn sink_borrowed_string_corridors(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for (_name, function) in &mut module.functions {
        rewritten += sink_borrowed_string_corridors_in_function(function);
    }
    rewritten
}

fn sink_borrowed_string_corridors_in_function(function: &mut MirFunction) -> usize {
    refresh_function_string_corridor_metadata(function);

    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let plans_by_block = collect_plans(function, &def_map, &use_counts);
    let mut rewritten = apply_plans(function, plans_by_block);

    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let retained_len_plans = collect_retained_len_plans(function, &def_map, &use_counts);
    rewritten += apply_retained_len_plans(function, retained_len_plans);

    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let concat_corridor_plans = collect_concat_corridor_plans(function, &def_map, &use_counts);
    rewritten += apply_concat_corridor_plans(function, concat_corridor_plans);

    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let publication_return_plans =
        collect_publication_return_plans(function, &def_map, &use_counts);
    rewritten += apply_publication_return_plans(function, publication_return_plans);

    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let publication_write_boundary_plans =
        collect_publication_write_boundary_plans(function, &def_map, &use_counts);
    rewritten += apply_publication_write_boundary_plans(function, publication_write_boundary_plans);

    let def_map = build_value_def_map(function);
    let use_counts = build_use_counts(function);
    let fusion_plans = collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
    rewritten += apply_complementary_len_fusion_plans(function, fusion_plans);

    rewritten
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReturnSite {
    Instruction(usize),
    Terminator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConcatCorridorPlan {
    Len(ConcatSubstringLenPlan),
    Substring(ConcatSubstringPlan),
    PublicationLen(PublicationHelperLenPlan),
    PublicationSubstring(PublicationHelperSubstringPlan),
    MaterializationStore(MaterializationStorePlan),
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

fn build_use_counts(function: &MirFunction) -> HashMap<ValueId, usize> {
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

fn sync_function_next_value_id(function: &mut MirFunction) {
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

fn collect_plans(
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

            let Some(inner_fact) = function.metadata.string_corridor_facts.get(&receiver_root)
            else {
                continue;
            };
            if inner_fact.op != StringCorridorOp::StrSlice {
                continue;
            }

            let Some((source, start, end)) = extract_substring_args(&block.instructions[inner_idx])
            else {
                continue;
            };

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

fn resolve_copy_chain_in_block(
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

fn resolve_single_use_copy_chain_in_block(
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

fn find_trailing_len_observer_in_block(
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

fn match_substring_len_call_in_block(
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

fn helper_plan_for_kind(
    function: &MirFunction,
    root: ValueId,
    kind: StringCorridorCandidateKind,
) -> Option<StringCorridorCandidatePlan> {
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

fn corridor_helper_shape(
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

fn publication_helper_shape(
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

fn materialization_helper_shape(
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

fn array_store_candidate(
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

fn rewrite_array_store_value(inst: &MirInstruction, new_value: ValueId) -> Option<MirInstruction> {
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

fn value_is_const_i64(
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

fn match_source_length_value(
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

fn stable_length_value_for_source(function: &MirFunction, source: ValueId) -> Option<ValueId> {
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

fn complementary_pair_source_len(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    lhs: &SubstringLenCallShape,
    rhs: &SubstringLenCallShape,
) -> Option<ValueId> {
    let lhs_source = string_source_identity(function, def_map, lhs.source)?;
    let rhs_source = string_source_identity(function, def_map, rhs.source)?;
    if lhs_source != rhs_source {
        return None;
    }
    let shared_source_root = resolve_value_origin(function, def_map, lhs.source);

    if value_is_const_i64(function, def_map, lhs.start, 0) {
        let mid = resolve_value_origin(function, def_map, lhs.end);
        let rhs_start = resolve_value_origin(function, def_map, rhs.start);
        if rhs_start != mid {
            return None;
        }
        if let Some(source_len) = match_source_length_value(function, def_map, &lhs_source, rhs.end)
        {
            return Some(source_len);
        }
        let stable_len = stable_length_value_for_source(function, shared_source_root)?;
        let rhs_end = resolve_value_origin(function, def_map, rhs.end);
        if rhs_end == stable_len {
            return Some(stable_len);
        }
        return None;
    }

    if value_is_const_i64(function, def_map, rhs.start, 0) {
        let mid = resolve_value_origin(function, def_map, rhs.end);
        let lhs_start = resolve_value_origin(function, def_map, lhs.start);
        if lhs_start != mid {
            return None;
        }
        if let Some(source_len) = match_source_length_value(function, def_map, &lhs_source, lhs.end)
        {
            return Some(source_len);
        }
        let stable_len = stable_length_value_for_source(function, shared_source_root)?;
        let lhs_end = resolve_value_origin(function, def_map, lhs.end);
        if lhs_end == stable_len {
            return Some(stable_len);
        }
        return None;
    }

    None
}

fn substring_pair_shares_source(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    lhs_slice: ValueId,
    rhs_slice: ValueId,
) -> bool {
    let Some(lhs) = match_substring_call_shape(function, def_map, lhs_slice) else {
        return false;
    };
    let Some(rhs) = match_substring_call_shape(function, def_map, rhs_slice) else {
        return false;
    };
    string_source_identity(function, def_map, lhs.source)
        == string_source_identity(function, def_map, rhs.source)
}

fn try_match_complementary_len_fusion_plan(
    function: &MirFunction,
    bbid: BasicBlockId,
    outer_idx: usize,
    outer_dst: ValueId,
    inner_leaf: ValueId,
    other_leaf: ValueId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> Option<ComplementarySubstringLenFusionPlan> {
    let inner_chain =
        resolve_single_use_copy_chain_in_block(function, bbid, def_map, use_counts, inner_leaf)?;
    let other_chain =
        resolve_single_use_copy_chain_in_block(function, bbid, def_map, use_counts, other_leaf)?;
    let inner_add = match_add_in_block(function, bbid, def_map, inner_chain.root)?;
    if inner_add.idx >= outer_idx || inner_add.dst != inner_chain.root {
        return None;
    }

    for (acc_leaf, first_leaf) in [
        (inner_add.lhs, inner_add.rhs),
        (inner_add.rhs, inner_add.lhs),
    ] {
        let first_chain = match resolve_single_use_copy_chain_in_block(
            function, bbid, def_map, use_counts, first_leaf,
        ) {
            Some(chain) => chain,
            None => continue,
        };
        let first_call =
            match match_substring_len_call_in_block(function, bbid, def_map, first_chain.root) {
                Some(call) => call,
                None => continue,
            };
        let second_call =
            match match_substring_len_call_in_block(function, bbid, def_map, other_chain.root) {
                Some(call) => call,
                None => continue,
            };
        let Some(source_len) =
            complementary_pair_source_len(function, def_map, &first_call, &second_call)
        else {
            continue;
        };

        let acc = resolve_value_origin(function, def_map, acc_leaf);
        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        remove_indices.insert(first_call.idx);
        remove_indices.insert(second_call.idx);
        remove_indices.insert(inner_add.idx);
        remove_indices.extend(first_chain.copy_indices.iter().copied());
        remove_indices.extend(other_chain.copy_indices.iter().copied());
        remove_indices.extend(inner_chain.copy_indices.iter().copied());

        return Some(ComplementarySubstringLenFusionPlan {
            remove_indices: remove_indices.into_iter().collect(),
            outer_idx,
            outer_dst,
            acc,
            source_len,
        });
    }

    None
}

fn collect_complementary_len_fusion_plans(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<ComplementarySubstringLenFusionPlan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<ComplementarySubstringLenFusionPlan>> =
        BTreeMap::new();

    for (bbid, block) in &function.blocks {
        let mut plans = Vec::new();
        let mut occupied: BTreeSet<usize> = BTreeSet::new();

        for (outer_idx, inst) in block.instructions.iter().enumerate() {
            if occupied.contains(&outer_idx) {
                continue;
            }
            let MirInstruction::BinOp {
                dst,
                op: BinaryOp::Add,
                lhs,
                rhs,
            } = inst
            else {
                continue;
            };

            let Some(plan) = try_match_complementary_len_fusion_plan(
                function, *bbid, outer_idx, *dst, *lhs, *rhs, def_map, use_counts,
            )
            .or_else(|| {
                try_match_complementary_len_fusion_plan(
                    function, *bbid, outer_idx, *dst, *rhs, *lhs, def_map, use_counts,
                )
            }) else {
                continue;
            };

            if plan.remove_indices.iter().any(|idx| occupied.contains(idx)) {
                continue;
            }
            for idx in &plan.remove_indices {
                occupied.insert(*idx);
            }
            occupied.insert(plan.outer_idx);
            plans.push(plan);
        }

        if !plans.is_empty() {
            plans_by_block.insert(*bbid, plans);
        }
    }

    plans_by_block
}

fn apply_plans(
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
        refresh_function_string_corridor_metadata(function);
    }
    rewritten
}

fn collect_retained_len_plans(
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

fn apply_retained_len_plans(
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

fn collect_concat_corridor_plans(
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

fn collect_publication_return_plans(
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

fn publication_return_site(block: &crate::mir::BasicBlock) -> Option<(ReturnSite, ValueId)> {
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

fn publication_write_boundary_value(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::Store { value, .. } | MirInstruction::FieldSet { value, .. } => {
            Some(*value)
        }
        _ => None,
    }
}

fn rewrite_publication_write_boundary_value(
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

fn can_sink_helper_past_intervening_instructions(
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

fn collect_publication_write_boundary_plans(
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

fn apply_concat_corridor_plans(
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
                    let rewritten_store = rewrite_array_store_value(&inst, *helper_dst)
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
        refresh_function_string_corridor_metadata(function);
    }

    rewritten
}

fn apply_publication_return_plans(
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
        refresh_function_string_corridor_metadata(function);
    }

    rewritten
}

fn apply_publication_write_boundary_plans(
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
        refresh_function_string_corridor_metadata(function);
    }

    rewritten
}

fn apply_complementary_len_fusion_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<ComplementarySubstringLenFusionPlan>>,
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

        for plan in plans {
            remove_indices.extend(plan.remove_indices.iter().copied());
            replacements.insert(
                plan.outer_idx,
                MirInstruction::BinOp {
                    dst: plan.outer_dst,
                    op: BinaryOp::Add,
                    lhs: plan.acc,
                    rhs: plan.source_len,
                },
            );
            function.metadata.optimization_hints.push(format!(
                "string_corridor_sink:complementary_substring_len_fusion:%{}",
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
    }

    if rewritten > 0 {
        function.update_cfg();
        refresh_function_string_corridor_metadata(function);
    }

    rewritten
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirCompiler, MirModule, MirType,
    };
    use crate::parser::NyashParser;

    fn method_call(
        dst: ValueId,
        receiver: ValueId,
        box_name: &str,
        method: &str,
        args: Vec<ValueId>,
        ty: MirType,
    ) -> MirInstruction {
        let _ = ty;
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn extern_call(dst: ValueId, name: &str, args: Vec<ValueId>) -> MirInstruction {
        MirInstruction::Call {
            dst: Some(dst),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(name.to_string())),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        let _ = std::panic::catch_unwind(|| {
            init_global_ring0(default_ring0());
        });
    }

    #[test]
    fn rewrites_single_use_substring_length_chain_to_direct_extern() {
        let mut module = MirModule::new("substring_len".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("StringBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("StringBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(4),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert_eq!(block.instructions.len(), block.instruction_spans.len());
        assert_eq!(block.instructions.len(), 1);
        match &block.instructions[0] {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => {
                assert_eq!(*dst, Some(ValueId(4)));
                assert_eq!(name, SUBSTRING_LEN_EXTERN);
                assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
            }
            other => panic!("expected direct extern rewrite, got {other:?}"),
        }
    }

    #[test]
    fn rewrites_runtime_data_substring_length_chain_through_copy_chain() {
        let mut module = MirModule::new("substring_len_runtime_data".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("RuntimeDataBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(4),
            src: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(5),
            src: ValueId(4),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(5),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(6)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("RuntimeDataBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Box("RuntimeDataBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Box("RuntimeDataBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(6), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert_eq!(block.instructions.len(), 3);
        match &block.instructions[2] {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => {
                assert_eq!(*dst, Some(ValueId(6)));
                assert_eq!(name, SUBSTRING_LEN_EXTERN);
                assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
            }
            other => panic!("expected direct extern rewrite, got {other:?}"),
        }
    }

    #[test]
    fn keeps_substring_when_result_has_multiple_uses() {
        let mut module = MirModule::new("substring_len_multiuse".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("StringBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(3),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box("StringBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(4),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(3),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 0);
    }

    #[test]
    fn fuses_complementary_substring_len_pair_back_to_source_length() {
        let mut module = MirModule::new("substring_len_fusion".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(5),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(4), ValueId(3)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(6),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(3), ValueId(1)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(7),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(9),
            rhs: ValueId(5),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(8),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(7),
            rhs: ValueId(6),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(8)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(6), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(7), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(8), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(9), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            }),
            "complementary substring_len_hii calls should be gone: {:?}",
            block.instructions
        );
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(8) && *lhs == ValueId(9) && *rhs == ValueId(1)
                )
            }),
            "outer add should fuse to acc + source_len: {:?}",
            block.instructions
        );
    }

    #[test]
    fn keeps_non_complementary_substring_len_pair() {
        let mut module = MirModule::new("substring_len_fusion_negative".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(6),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: crate::mir::ConstValue::Integer(9),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(4),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(2), ValueId(3)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(5),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(0), ValueId(3), ValueId(2)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(6),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(4),
            rhs: ValueId(5),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(6)),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(6), MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 0);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        let substring_len_calls = block
            .instructions
            .iter()
            .filter(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            })
            .count();
        assert_eq!(substring_len_calls, 2);
    }

    #[test]
    fn fuses_complementary_substring_len_pair_with_entry_len_and_duplicated_const_source() {
        let mut module = MirModule::new("substring_len_fusion_cross_block".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::String("line-seed-abcdef".to_string()),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Copy {
            dst: ValueId(8),
            src: ValueId(9),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(method_call(
            ValueId(5),
            ValueId(8),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(25),
            value: crate::mir::ConstValue::Integer(7),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(19),
            edge_args: None,
        });

        let mut loop_block = BasicBlock::new(BasicBlockId(19));
        loop_block.instructions.push(MirInstruction::Const {
            dst: ValueId(41),
            value: crate::mir::ConstValue::Integer(0),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(46),
            src: ValueId(5),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(45),
            src: ValueId(46),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(44),
            src: ValueId(45),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Const {
            dst: ValueId(47),
            value: crate::mir::ConstValue::Integer(2),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(43),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(44),
            rhs: ValueId(47),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(42),
            src: ValueId(43),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Const {
            dst: ValueId(49),
            value: crate::mir::ConstValue::String("line-seed-abcdef".to_string()),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(48),
            src: ValueId(49),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(50),
            src: ValueId(46),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(extern_call(
            ValueId(30),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(48), ValueId(41), ValueId(42)],
        ));
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(53),
            src: ValueId(25),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(54),
            src: ValueId(30),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(extern_call(
            ValueId(32),
            SUBSTRING_LEN_EXTERN,
            vec![ValueId(48), ValueId(42), ValueId(50)],
        ));
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(58),
            src: ValueId(53),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(59),
            src: ValueId(54),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(57),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(58),
            rhs: ValueId(59),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(60),
            src: ValueId(32),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(33),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(57),
            rhs: ValueId(60),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(33)),
        });
        function.add_block(loop_block);

        for vid in [
            5u32, 25, 30, 32, 33, 41, 42, 43, 44, 45, 46, 47, 50, 53, 54, 57, 58, 59, 60,
        ] {
            function
                .metadata
                .value_types
                .insert(ValueId(vid), MirType::Integer);
        }
        module.add_function(function);

        let function = module.get_function("main").expect("main");
        let def_map = build_value_def_map(function);
        let use_counts = build_use_counts(function);
        let first_call =
            match_substring_len_call_in_block(function, BasicBlockId(19), &def_map, ValueId(30))
                .expect("first substring_len");
        let second_call =
            match_substring_len_call_in_block(function, BasicBlockId(19), &def_map, ValueId(32))
                .expect("second substring_len");
        assert_eq!(
            complementary_pair_source_len(function, &def_map, &first_call, &second_call),
            Some(ValueId(5))
        );
        let plans = collect_complementary_len_fusion_plans(function, &def_map, &use_counts);
        assert_eq!(plans.get(&BasicBlockId(19)).map(Vec::len), Some(1));

        let function = module.get_function_mut("main").expect("main");
        let rewritten = sink_borrowed_string_corridors_in_function(function);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(19)).expect("loop");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            }),
            "cross-block complementary substring_len_hii calls should be gone: {:?}",
            block.instructions
        );
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(33) && *lhs == ValueId(25) && *rhs == ValueId(5)
                )
            }),
            "outer add should fuse to acc + source_len across blocks: {:?}",
            block.instructions
        );
    }

    #[test]
    fn rewrites_retained_slice_length_consumer_across_blocks() {
        let mut module = MirModule::new("substring_len_retained_cross_block".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let entry = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        entry.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        entry.instruction_spans.push(Span::unknown());
        entry.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "StringBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
            MirType::Box("StringBox".to_string()),
        ));
        entry.instruction_spans.push(Span::unknown());
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });

        let mut loop_block = BasicBlock::new(BasicBlockId(1));
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(6),
            src: ValueId(5),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(7),
            src: ValueId(6),
        });
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.instructions.push(method_call(
            ValueId(8),
            ValueId(7),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        loop_block.instruction_spans.push(Span::unknown());
        loop_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(8)),
        });
        function.add_block(loop_block);

        for (vid, ty) in [
            (ValueId(1), MirType::Integer),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Box("StringBox".to_string())),
            (ValueId(8), MirType::Integer),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, 1);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(1)).expect("loop");
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(8)
                        && name == SUBSTRING_LEN_EXTERN
                        && args.as_slice() == [ValueId(0), ValueId(4), ValueId(3)]
                )
            }),
            "retained slice length should rewrite to substring_len_hii: {:?}",
            block.instructions
        );
    }

    #[test]
    fn rewrites_concat_slice_consumers_to_corridor_helpers() {
        let mut module = MirModule::new("substring_concat_corridor".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "StringBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(7),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(8),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(5),
            rhs: ValueId(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(9),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(8),
            rhs: ValueId(6),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(10),
            ValueId(9),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(11),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(12),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(13),
            ValueId(9),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(11), ValueId(12)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(10)),
        });

        for (vid, ty) in [
            (ValueId(1), MirType::Integer),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("StringBox".to_string())),
            (ValueId(10), MirType::Integer),
            (ValueId(11), MirType::Integer),
            (ValueId(12), MirType::Integer),
            (ValueId(13), MirType::Box("RuntimeDataBox".to_string())),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 2,
            "concat corridor should rewrite both consumers, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        let substring_len_calls: Vec<_> = block
            .instructions
            .iter()
            .filter(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == SUBSTRING_LEN_EXTERN
                )
            })
            .collect();
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(10)
                        && ((*lhs == ValueId(1) && *rhs != ValueId(1))
                            || (*rhs == ValueId(1) && *lhs != ValueId(1)))
                )
            }),
            "concat length should rewrite to source_len + const_len: {:?}",
            block.instructions
        );
        assert!(
            substring_len_calls.is_empty(),
            "complementary substring_len_hii calls should fuse away: {:?}",
            block.instructions
        );
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(13)
                        && name == SUBSTRING_CONCAT3_EXTERN
                        && args.as_slice()
                            == [ValueId(5), ValueId(7), ValueId(6), ValueId(11), ValueId(12)]
                )
            }),
            "concat substring should rewrite to substring_concat3 helper: {:?}",
            block.instructions
        );
    }

    #[test]
    fn rewrites_publication_helper_length_via_plan_metadata() {
        let mut module = MirModule::new("substring_concat_publication_len".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("RuntimeDataBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(7),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(9),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(8),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(10),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(11),
            src: ValueId(10),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(12),
            ValueId(11),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(12)),
        });

        for (vid, ty) in [
            (ValueId(1), MirType::Integer),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("StringBox".to_string())),
            (ValueId(8), MirType::Integer),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Integer),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 1,
            "publication helper length should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Sub,
                        lhs,
                        rhs,
                    } if *dst == ValueId(12) && *lhs == ValueId(9) && *rhs == ValueId(8)
                )
            }),
            "publication helper length should rewrite to end-start: {:?}",
            block.instructions
        );
    }

    #[test]
    fn rewrites_publication_helper_substring_via_plan_metadata() {
        let mut module = MirModule::new("substring_concat_publication_substring".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("RuntimeDataBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(7),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(9),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(8),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(10),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(11),
            src: ValueId(10),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(12),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(13),
            value: crate::mir::ConstValue::Integer(4),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(14),
            ValueId(11),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(12), ValueId(13)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(14)),
        });

        for (vid, ty) in [
            (ValueId(1), MirType::Integer),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("StringBox".to_string())),
            (ValueId(8), MirType::Integer),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Integer),
            (ValueId(13), MirType::Integer),
            (ValueId(14), MirType::Box("RuntimeDataBox".to_string())),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 1,
            "publication helper substring should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");

        let mut add_roots = std::collections::BTreeMap::new();
        for inst in &block.instructions {
            if let MirInstruction::BinOp {
                dst,
                op: crate::mir::BinaryOp::Add,
                lhs,
                rhs,
            } = inst
            {
                add_roots.insert(*dst, (*lhs, *rhs));
            }
        }

        let helper_call = block.instructions.iter().find_map(|inst| match inst {
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } if *dst == ValueId(14) && name == SUBSTRING_CONCAT3_EXTERN => Some(args.clone()),
            _ => None,
        });
        let helper_args = helper_call.expect("publication helper substring call");
        assert_eq!(&helper_args[..3], &[ValueId(5), ValueId(7), ValueId(6)]);
        let composed_start = helper_args[3];
        let composed_end = helper_args[4];
        assert_eq!(
            add_roots.get(&composed_start),
            Some(&(ValueId(8), ValueId(12)))
        );
        assert_eq!(
            add_roots.get(&composed_end),
            Some(&(ValueId(8), ValueId(13)))
        );
    }

    #[test]
    fn sinks_publication_helper_to_same_block_return_boundary() {
        let mut module = MirModule::new("substring_concat_publication_return".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("RuntimeDataBox".to_string())],
            return_type: MirType::Box("RuntimeDataBox".to_string()),
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(1),
            ValueId(0),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(4),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(5),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(3)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(0),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(3), ValueId(1)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(7),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(9),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(8),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(10),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(11),
            src: ValueId(10),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(12),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(13),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(12),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(14),
            src: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(14)),
        });

        for (vid, ty) in [
            (ValueId(1), MirType::Integer),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("StringBox".to_string())),
            (ValueId(8), MirType::Integer),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Integer),
            (ValueId(13), MirType::Integer),
            (ValueId(14), MirType::Box("RuntimeDataBox".to_string())),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 1,
            "publication helper return sink should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Copy { dst, .. } if *dst == ValueId(11) || *dst == ValueId(14)
                )
            }),
            "copy-only return chain should disappear: {:?}",
            block.instructions
        );

        let add_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(13) && *lhs == ValueId(1) && *rhs == ValueId(12)
                )
            })
            .expect("unrelated pure add");
        let helper_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(10)
                        && name == SUBSTRING_CONCAT3_EXTERN
                        && args.as_slice()
                            == [ValueId(5), ValueId(7), ValueId(6), ValueId(8), ValueId(9)]
                )
            })
            .expect("sunk helper call");
        assert!(
            helper_idx > add_idx,
            "helper should sink below unrelated pure work: {:?}",
            block.instructions
        );
        assert_eq!(
            helper_idx + 1,
            block.instructions.len(),
            "helper should end immediately before return: {:?}",
            block.instructions
        );
        assert!(matches!(
            block.terminator,
            Some(MirInstruction::Return {
                value: Some(ValueId(10))
            })
        ));
    }

    #[test]
    fn sinks_publication_helper_to_same_block_store_boundary() {
        let mut module = MirModule::new("substring_concat_publication_store".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Integer, MirType::Box("RuntimeDataBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(2),
            ValueId(1),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(2),
            rhs: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(5),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(5), ValueId(4)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(7),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(2)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(10),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(9),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(11),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(12),
            src: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(13),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(14),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(13),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(15),
            src: ValueId(12),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Store {
            value: ValueId(15),
            ptr: ValueId(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(14)),
        });

        for (vid, ty) in [
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Integer),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(8), MirType::Box("StringBox".to_string())),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Integer),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(13), MirType::Integer),
            (ValueId(14), MirType::Integer),
            (ValueId(15), MirType::Box("RuntimeDataBox".to_string())),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 1,
            "publication helper store sink should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(15)
                )
            }),
            "copy-only store chain should disappear: {:?}",
            block.instructions
        );

        let add_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(14) && *lhs == ValueId(2) && *rhs == ValueId(13)
                )
            })
            .expect("unrelated pure add");
        let helper_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(11)
                        && name == SUBSTRING_CONCAT3_EXTERN
                        && args.as_slice()
                            == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
                )
            })
            .expect("sunk helper call");
        let store_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Store { value, ptr } if *value == ValueId(11) && *ptr == ValueId(0)
                )
            })
            .expect("rewritten store");
        assert!(
            helper_idx > add_idx,
            "helper should sink below unrelated pure work: {:?}",
            block.instructions
        );
        assert_eq!(
            helper_idx + 1,
            store_idx,
            "helper should end immediately before store: {:?}",
            block.instructions
        );
    }

    #[test]
    fn sinks_publication_helper_to_same_block_fieldset_boundary() {
        let mut module = MirModule::new("substring_concat_publication_fieldset".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("RuntimeDataBox".to_string()),
                MirType::Box("RuntimeDataBox".to_string()),
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(2),
            ValueId(1),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(2),
            rhs: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(5),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(5), ValueId(4)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(7),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(2)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(10),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(9),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(11),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(12),
            src: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(13),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(14),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(13),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(15),
            src: ValueId(12),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::FieldSet {
            base: ValueId(0),
            field: "text".to_string(),
            value: ValueId(15),
            declared_type: Some(MirType::Box("RuntimeDataBox".to_string())),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(14)),
        });

        for (vid, ty) in [
            (ValueId(0), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Integer),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(8), MirType::Box("StringBox".to_string())),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Integer),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(13), MirType::Integer),
            (ValueId(14), MirType::Integer),
            (ValueId(15), MirType::Box("RuntimeDataBox".to_string())),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 1,
            "publication helper fieldset sink should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(15)
                )
            }),
            "copy-only fieldset chain should disappear: {:?}",
            block.instructions
        );

        let add_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        rhs,
                    } if *dst == ValueId(14) && *lhs == ValueId(2) && *rhs == ValueId(13)
                )
            })
            .expect("unrelated pure add");
        let helper_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(11)
                        && name == SUBSTRING_CONCAT3_EXTERN
                        && args.as_slice()
                            == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
                )
            })
            .expect("sunk helper call");
        let fieldset_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::FieldSet { base, field, value, .. }
                        if *base == ValueId(0) && field == "text" && *value == ValueId(11)
                )
            })
            .expect("rewritten fieldset");
        assert!(
            helper_idx > add_idx,
            "helper should sink below unrelated pure work: {:?}",
            block.instructions
        );
        assert_eq!(
            helper_idx + 1,
            fieldset_idx,
            "helper should end immediately before fieldset: {:?}",
            block.instructions
        );
    }

    #[test]
    fn sinks_materialization_helper_to_array_store_boundary() {
        let mut module = MirModule::new("substring_concat_materialization_store".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("ArrayBox".to_string()),
                MirType::Box("RuntimeDataBox".to_string()),
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(2),
            ValueId(1),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(2),
            rhs: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(5),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(5), ValueId(4)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(7),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(2)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(10),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(9),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(11),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(12),
            src: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(13),
            value: crate::mir::ConstValue::Integer(99),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(14),
            ValueId(0),
            "ArrayBox",
            "set",
            vec![ValueId(5), ValueId(12)],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(14)),
        });

        for (vid, ty) in [
            (ValueId(0), MirType::Box("ArrayBox".to_string())),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Integer),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(8), MirType::Box("StringBox".to_string())),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Integer),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(13), MirType::Integer),
            (ValueId(14), MirType::Integer),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 1,
            "materialization store sink should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(inst, MirInstruction::Copy { dst, .. } if *dst == ValueId(12))
            }),
            "copy-only store consumer should disappear: {:?}",
            block.instructions
        );

        let helper_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(11)
                        && name == SUBSTRING_CONCAT3_EXTERN
                        && args.as_slice()
                            == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
                )
            })
            .expect("materialization helper call");
        let const_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(99),
                    } if *dst == ValueId(13)
                )
            })
            .expect("unrelated const");
        let store_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Method { method, receiver: Some(receiver), .. }),
                        args,
                        ..
                    } if *dst == ValueId(14)
                        && method == "set"
                        && *receiver == ValueId(0)
                        && args.as_slice() == [ValueId(5), ValueId(11)]
                )
            })
            .expect("array store");
        assert!(
            helper_idx > const_idx,
            "helper should sink past unrelated pure instructions: {:?}",
            block.instructions
        );
        assert_eq!(
            helper_idx + 1,
            store_idx,
            "materialization helper should sit right before the store boundary: {:?}",
            block.instructions
        );
    }

    #[test]
    fn sinks_materialization_helper_with_trailing_length_observer() {
        let mut module =
            MirModule::new("substring_concat_materialization_store_len_observer".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("ArrayBox".to_string()),
                MirType::Box("RuntimeDataBox".to_string()),
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

        block.instructions.push(method_call(
            ValueId(2),
            ValueId(1),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(3),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: crate::mir::BinaryOp::Div,
            lhs: ValueId(2),
            rhs: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(5),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(6),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(5), ValueId(4)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(7),
            ValueId(1),
            "RuntimeDataBox",
            "substring",
            vec![ValueId(4), ValueId(2)],
            MirType::Box("RuntimeDataBox".to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(8),
            value: crate::mir::ConstValue::String("xx".to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(9),
            value: crate::mir::ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(10),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(9),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(extern_call(
            ValueId(11),
            SUBSTRING_CONCAT3_EXTERN,
            vec![ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)],
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(12),
            src: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(13),
            ValueId(0),
            "ArrayBox",
            "set",
            vec![ValueId(5), ValueId(12)],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Copy {
            dst: ValueId(14),
            src: ValueId(11),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(method_call(
            ValueId(15),
            ValueId(14),
            "RuntimeDataBox",
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(16),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(13),
            rhs: ValueId(15),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(16)),
        });

        for (vid, ty) in [
            (ValueId(0), MirType::Box("ArrayBox".to_string())),
            (ValueId(2), MirType::Integer),
            (ValueId(3), MirType::Integer),
            (ValueId(4), MirType::Integer),
            (ValueId(5), MirType::Integer),
            (ValueId(6), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(7), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(8), MirType::Box("StringBox".to_string())),
            (ValueId(9), MirType::Integer),
            (ValueId(10), MirType::Integer),
            (ValueId(11), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(12), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(13), MirType::Integer),
            (ValueId(14), MirType::Box("RuntimeDataBox".to_string())),
            (ValueId(15), MirType::Integer),
            (ValueId(16), MirType::Integer),
        ] {
            function.metadata.value_types.insert(vid, ty);
        }
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert!(
            rewritten >= 2,
            "materialization store plus observer length should rewrite, got {rewritten}"
        );

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(
            block.instructions.iter().all(|inst| {
                !matches!(
                    inst,
                    MirInstruction::Copy { dst, .. } if *dst == ValueId(12) || *dst == ValueId(14)
                )
            }),
            "copy-only store/observer chains should disappear: {:?}",
            block.instructions
        );
        assert!(
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Sub,
                        lhs,
                        rhs,
                    } if *dst == ValueId(15) && *lhs == ValueId(10) && *rhs == ValueId(9)
                )
            }),
            "trailing helper length should rewrite to end-start: {:?}",
            block.instructions
        );

        let helper_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name)),
                        args,
                        ..
                    } if *dst == ValueId(11)
                        && name == SUBSTRING_CONCAT3_EXTERN
                        && args.as_slice()
                            == [ValueId(6), ValueId(8), ValueId(7), ValueId(9), ValueId(10)]
                )
            })
            .expect("materialization helper call");
        let store_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Method { method, receiver: Some(receiver), .. }),
                        args,
                        ..
                    } if *dst == ValueId(13)
                        && method == "set"
                        && *receiver == ValueId(0)
                        && args.as_slice() == [ValueId(5), ValueId(11)]
                )
            })
            .expect("array store");
        let len_idx = block
            .instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::BinOp {
                        dst,
                        op: crate::mir::BinaryOp::Sub,
                        ..
                    } if *dst == ValueId(15)
                )
            })
            .expect("observer len rewrite");
        assert_eq!(
            helper_idx + 1,
            store_idx,
            "materialization helper should sit right before the store boundary: {:?}",
            block.instructions
        );
        assert!(
            len_idx > store_idx,
            "trailing len observer should stay after the store boundary: {:?}",
            block.instructions
        );
    }

    #[test]
    fn benchmark_substring_only_compiles_without_substring_len_calls() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_substring_only.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared =
            crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
                .expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");
        let substring_len_calls: Vec<String> = result
            .module
            .functions
            .iter()
            .flat_map(|(name, function)| {
                function.blocks.iter().flat_map(move |(bbid, block)| {
                    block
                        .instructions
                        .iter()
                        .filter_map(move |inst| match inst {
                            MirInstruction::Call {
                                callee: Some(Callee::Extern(callee)),
                                ..
                            } if callee == SUBSTRING_LEN_EXTERN => {
                                Some(format!("fn={name} bb={} inst={inst:?}", bbid.0))
                            }
                            _ => None,
                        })
                })
            })
            .collect();
        assert!(
            substring_len_calls.is_empty(),
            "benchmark should fuse substring_len_hii away, found {:?}",
            substring_len_calls
        );
    }

    #[test]
    fn benchmark_len_substring_views_compiles_without_loop_string_consumers() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_len_substring_views.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared =
            crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
                .expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");

        let mut leftover_string_consumers = Vec::new();
        for (name, function) in &result.module.functions {
            for (bbid, block) in &function.blocks {
                for inst in &block.instructions {
                    match inst {
                        MirInstruction::Call {
                            callee: Some(Callee::Extern(callee)),
                            ..
                        } if callee == SUBSTRING_LEN_EXTERN => leftover_string_consumers.push(
                            format!("fn={name} bb={} extern={callee} inst={inst:?}", bbid.0),
                        ),
                        MirInstruction::Call {
                            callee:
                                Some(Callee::Method {
                                    box_name,
                                    method,
                                    receiver: Some(_),
                                    ..
                                }),
                            ..
                        } if method == "length" && box_name == "RuntimeDataBox" => {
                            leftover_string_consumers.push(format!(
                                "fn={name} bb={} runtime-data length inst={inst:?}",
                                bbid.0
                            ))
                        }
                        _ => {}
                    }
                }
            }
        }

        assert!(
            leftover_string_consumers.is_empty(),
            "len_substring_views should fuse away loop string consumers, found {:?}",
            leftover_string_consumers
        );
    }

    #[test]
    fn benchmark_substring_concat_compiles_without_concat_string_consumers() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_substring_concat.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared =
            crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
                .expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");

        let mut saw_helper = false;
        let mut leftover_concat_consumers = Vec::new();
        let mut leftover_concat_lengths = Vec::new();
        let mut leftover_substring_len = Vec::new();
        for (name, function) in &result.module.functions {
            let def_map = build_value_def_map(function);
            for (bbid, block) in &function.blocks {
                for inst in &block.instructions {
                    match inst {
                        MirInstruction::Call {
                            callee: Some(Callee::Extern(callee)),
                            ..
                        } if callee == SUBSTRING_CONCAT3_EXTERN => {
                            saw_helper = true;
                        }
                        MirInstruction::Call {
                            callee:
                                Some(Callee::Method {
                                    box_name,
                                    method,
                                    receiver: Some(receiver),
                                    ..
                                }),
                            ..
                        } if box_name == "RuntimeDataBox"
                            && method == "substring"
                            && match_concat_triplet(function, *bbid, &def_map, *receiver)
                                .is_some() =>
                        {
                            leftover_concat_consumers.push(format!(
                                "fn={name} bb={} concat substring inst={inst:?}",
                                bbid.0
                            ));
                        }
                        MirInstruction::Call {
                            callee:
                                Some(Callee::Method {
                                    box_name,
                                    method,
                                    receiver: Some(receiver),
                                    ..
                                }),
                            ..
                        } if box_name == "RuntimeDataBox"
                            && method == "length"
                            && match_concat_triplet(function, *bbid, &def_map, *receiver)
                                .is_some() =>
                        {
                            leftover_concat_lengths.push(format!(
                                "fn={name} bb={} concat length inst={inst:?}",
                                bbid.0
                            ));
                        }
                        MirInstruction::Call {
                            callee: Some(Callee::Extern(callee)),
                            ..
                        } if callee == SUBSTRING_LEN_EXTERN => {
                            leftover_substring_len.push(format!(
                                "fn={name} bb={} substring_len inst={inst:?}",
                                bbid.0
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }

        assert!(saw_helper, "benchmark should emit substring_concat3 helper");
        assert!(
            leftover_concat_consumers.is_empty(),
            "substring_concat should sink concat substring consumers, found {:?}",
            leftover_concat_consumers
        );
        assert!(
            leftover_concat_lengths.is_empty(),
            "substring_concat should sink concat length consumers, found {:?}",
            leftover_concat_lengths
        );
        assert!(
            leftover_substring_len.is_empty(),
            "substring_concat should fuse loop substring_len_hii away, found {:?}",
            leftover_substring_len
        );
    }

    #[test]
    fn benchmark_substring_concat_array_set_compiles_without_helper_len_observers() {
        ensure_ring0_initialized();
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_meso_substring_concat_array_set.hako"
        );
        let source = std::fs::read_to_string(path).expect("benchmark source");
        let prepared =
            crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
                .expect("prepare benchmark source");
        let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
        let mut compiler = MirCompiler::with_options(true);
        let result = compiler
            .compile_with_source(ast, Some(path))
            .expect("compile benchmark");

        let mut leftover_helper_lengths = Vec::new();
        for (name, function) in &result.module.functions {
            let def_map = build_value_def_map(function);
            let use_counts = build_use_counts(function);
            for (bbid, block) in &function.blocks {
                for inst in &block.instructions {
                    let Some((_dst, receiver, _effects)) = match_len_call(inst) else {
                        continue;
                    };
                    let Some(receiver_chain) = resolve_copy_chain_in_block(
                        function,
                        *bbid,
                        &def_map,
                        &use_counts,
                        receiver,
                    ) else {
                        continue;
                    };
                    if publication_helper_shape(function, &def_map, receiver_chain.root).is_some() {
                        leftover_helper_lengths
                            .push(format!("fn={name} bb={} helper-len inst={inst:?}", bbid.0));
                    }
                }
            }
        }

        assert!(
            leftover_helper_lengths.is_empty(),
            "substring_concat_array_set should rewrite helper len observers, found {:?}",
            leftover_helper_lengths
        );
    }
}
