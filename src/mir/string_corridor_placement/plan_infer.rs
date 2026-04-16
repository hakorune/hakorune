use super::*;

fn active_publication_contract(
    start: Option<ValueId>,
    end: Option<ValueId>,
) -> Option<StringCorridorPublicationContract> {
    match (start, end) {
        (Some(_), Some(_)) => Some(
            StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
        ),
        _ => None,
    }
}

fn infer_borrowed_slice_plan(
    function: &MirFunction,
    value: ValueId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Option<StringCorridorCandidatePlan> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let (_, receiver, start, end, _) = match_substring_call(block.instructions.get(idx)?)?;
    let source = resolve_value_origin(function, def_map, receiver);
    let start = resolve_value_origin(function, def_map, start);
    let end = resolve_value_origin(function, def_map, end);
    Some(StringCorridorCandidatePlan {
        corridor_root: root,
        source_root: Some(source),
        start: Some(start),
        end: Some(end),
        known_length: None,
        publication_contract: None,
        proof: StringCorridorCandidateProof::BorrowedSlice { source, start, end },
    })
}

fn shared_source_root(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    lhs_source: ValueId,
    rhs_source: ValueId,
) -> (bool, Option<ValueId>) {
    let lhs_identity = string_source_identity(function, def_map, lhs_source);
    let rhs_identity = string_source_identity(function, def_map, rhs_source);
    match (lhs_identity, rhs_identity) {
        (Some(StringSourceIdentity::Value(lhs)), Some(StringSourceIdentity::Value(rhs)))
            if lhs == rhs =>
        {
            (true, Some(lhs))
        }
        (
            Some(StringSourceIdentity::ConstString(lhs)),
            Some(StringSourceIdentity::ConstString(rhs)),
        ) if lhs == rhs => (true, None),
        _ => (false, None),
    }
}

fn infer_concat_triplet_plan(
    function: &MirFunction,
    bbid: BasicBlockId,
    receiver: ValueId,
    outer_start: Option<ValueId>,
    outer_end: Option<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    require_shared_source: bool,
) -> Option<StringCorridorCandidatePlan> {
    let receiver_root = resolve_value_origin(function, def_map, receiver);
    let ConcatTripletShape {
        left: left_value,
        middle,
        right: right_value,
    } = match_concat_triplet(function, bbid, def_map, receiver_root)?;
    let Some(StringSourceIdentity::ConstString(text)) =
        string_source_identity(function, def_map, middle)
    else {
        return None;
    };
    let left = match_substring_call_shape(function, def_map, left_value)?;
    let right = match_substring_call_shape(function, def_map, right_value)?;
    let (shared_source, source_root) =
        shared_source_root(function, def_map, left.source, right.source);
    if require_shared_source && !shared_source {
        return None;
    }

    Some(StringCorridorCandidatePlan {
        corridor_root: receiver_root,
        source_root,
        start: outer_start.map(|value| resolve_value_origin(function, def_map, value)),
        end: outer_end.map(|value| resolve_value_origin(function, def_map, value)),
        known_length: Some(const_string_length(&text)),
        publication_contract: active_publication_contract(outer_start, outer_end),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(resolve_value_origin(function, def_map, left_value)),
            left_source: left.source,
            left_start: left.start,
            left_end: left.end,
            middle,
            right_value: Some(resolve_value_origin(function, def_map, right_value)),
            right_source: right.source,
            right_start: right.start,
            right_end: right.end,
            shared_source,
        },
    })
}

fn infer_concat_triplet_result_plan(
    function: &MirFunction,
    value: ValueId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Option<StringCorridorCandidatePlan> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let helper = match_substring_concat3_helper_call(block.instructions.get(idx)?)?;
    let (left, middle, right, start, end) = (
        helper.left,
        helper.middle,
        helper.right,
        helper.start,
        helper.end,
    );
    let Some(StringSourceIdentity::ConstString(text)) =
        string_source_identity(function, def_map, middle)
    else {
        return None;
    };
    let left = match_substring_call_shape(function, def_map, left)?;
    let right = match_substring_call_shape(function, def_map, right)?;
    let (shared_source, source_root) =
        shared_source_root(function, def_map, left.source, right.source);

    Some(StringCorridorCandidatePlan {
        corridor_root: root,
        source_root,
        start: Some(resolve_value_origin(function, def_map, start)),
        end: Some(resolve_value_origin(function, def_map, end)),
        known_length: Some(const_string_length(&text)),
        publication_contract: active_publication_contract(Some(start), Some(end)),
        proof: StringCorridorCandidateProof::ConcatTriplet {
            left_value: Some(resolve_value_origin(function, def_map, helper.left)),
            left_source: left.source,
            left_start: left.start,
            left_end: left.end,
            middle: resolve_value_origin(function, def_map, middle),
            right_value: Some(resolve_value_origin(function, def_map, helper.right)),
            right_source: right.source,
            right_start: right.start,
            right_end: right.end,
            shared_source,
        },
    })
}

pub(super) fn infer_plan(
    function: &MirFunction,
    value: ValueId,
    fact: &StringCorridorFact,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) -> Option<StringCorridorCandidatePlan> {
    let root = resolve_value_origin(function, def_map, value);
    let (bbid, idx) = def_map.get(&root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let inst = block.instructions.get(idx)?;

    match fact.op {
        StringCorridorOp::StrSlice => {
            if let Some((_, receiver, start, end, _)) = match_substring_call(inst) {
                infer_concat_triplet_plan(
                    function,
                    bbid,
                    receiver,
                    Some(start),
                    Some(end),
                    def_map,
                    true,
                )
                .or_else(|| infer_borrowed_slice_plan(function, value, def_map))
            } else {
                infer_concat_triplet_result_plan(function, value, def_map)
            }
        }
        StringCorridorOp::StrLen => {
            let (_, receiver, _) = match_len_call(inst)?;
            infer_concat_triplet_plan(function, bbid, receiver, None, None, def_map, false).or_else(
                || {
                    infer_borrowed_slice_plan(function, receiver, def_map).map(|plan| {
                        StringCorridorCandidatePlan {
                            corridor_root: plan.corridor_root,
                            source_root: plan.source_root,
                            start: plan.start,
                            end: plan.end,
                            known_length: plan.known_length,
                            publication_contract: plan.publication_contract,
                            proof: plan.proof,
                        }
                    })
                },
            )
        }
        StringCorridorOp::FreezeStr => None,
    }
}
