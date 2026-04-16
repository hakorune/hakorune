use super::*;

pub(super) fn refresh_function_string_corridor_relation_candidates(
    function: &mut MirFunction,
    _def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
) {
    let mut phi_updates: Vec<(ValueId, Vec<StringCorridorCandidate>)> = Vec::new();

    for (value, relations) in &function.metadata.string_corridor_relations {
        let carried_candidates = carried_candidates_from_relations(function, relations);
        if !carried_candidates.is_empty() {
            phi_updates.push((*value, carried_candidates));
        }
    }

    for (value, candidates) in phi_updates {
        function
            .metadata
            .string_corridor_candidates
            .entry(value)
            .or_default()
            .extend(candidates);
    }
}

fn carried_candidates_from_relations(
    function: &MirFunction,
    relations: &[StringCorridorRelation],
) -> Vec<StringCorridorCandidate> {
    let mut out = Vec::new();
    for relation in relations {
        if relation.kind != StringCorridorRelationKind::PhiCarryBase {
            continue;
        }
        let Some(base_candidates) = base_phi_carry_candidates(function, relation.base_value) else {
            continue;
        };
        out.extend(
            base_candidates
                .into_iter()
                .map(|candidate| StringCorridorCandidate {
                    kind: candidate.kind,
                    state: candidate.state,
                    reason: relation.reason,
                    plan: if relation.window_contract.preserves_plan_window() {
                        candidate.plan
                    } else {
                        None
                    },
                    publication_boundary: candidate.publication_boundary,
                }),
        );
    }
    out
}

fn base_phi_carry_candidates(
    function: &MirFunction,
    value: ValueId,
) -> Option<Vec<StringCorridorCandidate>> {
    let candidates = function.metadata.string_corridor_candidates.get(&value)?;
    let carried: Vec<_> = candidates
        .iter()
        .filter(|candidate| {
            matches!(
                candidate.kind,
                StringCorridorCandidateKind::PublicationSink
                    | StringCorridorCandidateKind::MaterializationSink
                    | StringCorridorCandidateKind::DirectKernelEntry
            ) && matches!(
                candidate.plan.map(|plan| plan.proof),
                Some(StringCorridorCandidateProof::ConcatTriplet { .. })
            )
        })
        .copied()
        .collect();
    if carried.is_empty() {
        None
    } else {
        Some(carried)
    }
}
