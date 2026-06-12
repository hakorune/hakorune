use super::super::value_origin::build_value_def_map;
use super::super::{
    agg_local_scalarization::{AggLocalScalarizationKind, AggLocalScalarizationRoute},
    string_corridor::StringCorridorBorrowContract,
    string_corridor_placement::{
        StringCorridorCandidateKind, StringCorridorCandidateProof, StringCorridorCandidateState,
        StringCorridorPublicationBoundary,
    },
    sum_placement_selection::{SumPlacementPath, SumPlacementSelection},
    thin_entry::{ThinEntryDemand, ThinEntryPreferredEntry},
    thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState},
    MirFunction, MirModule,
};

use super::{
    PlacementEffectBorrowContract, PlacementEffectDecision, PlacementEffectDemand,
    PlacementEffectPublicationBoundary, PlacementEffectRoute, PlacementEffectSource,
    PlacementEffectState, PlacementEffectStringProof,
};

pub fn refresh_module_placement_effect_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_placement_effect_routes(function);
    }
}

pub fn refresh_function_placement_effect_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    collect_string_routes(function, &mut routes);
    collect_sum_routes(function, &mut routes);
    collect_agg_local_routes(function, &mut routes);
    collect_thin_entry_routes(function, &mut routes);
    routes.sort_by_key(route_sort_key);
    function.metadata.placement_effect_routes = routes;
}

fn collect_string_routes(function: &MirFunction, routes: &mut Vec<PlacementEffectRoute>) {
    let def_map = build_value_def_map(function);

    for (value, candidates) in &function.metadata.string_corridor_candidates {
        let location = def_map.get(value).copied();
        let publication_boundary =
            candidates
                .iter()
                .find_map(|candidate| match candidate.publication_boundary {
                    Some(StringCorridorPublicationBoundary::FirstExternalBoundary) => {
                        Some(PlacementEffectPublicationBoundary::FirstExternalBoundary)
                    }
                    None => None,
                });
        for candidate in candidates {
            routes.push(PlacementEffectRoute {
                block: location.map(|(block, _)| block),
                instruction_index: location.map(|(_, index)| index),
                value: Some(*value),
                source_value: None,
                window_start: candidate.plan.and_then(|plan| plan.start),
                window_end: candidate.plan.and_then(|plan| plan.end),
                borrow_contract: candidate.plan.and_then(|plan| match plan.borrow_contract {
                    Some(StringCorridorBorrowContract::BorrowTextFromObject) => {
                        Some(PlacementEffectBorrowContract::BorrowTextFromObject)
                    }
                    None => None,
                }),
                publish_reason: candidate.plan.and_then(|plan| plan.publish_reason),
                publish_repr_policy: candidate.plan.and_then(|plan| plan.publish_repr_policy),
                stable_view_provenance: candidate.plan.and_then(|plan| plan.stable_view_provenance),
                string_proof: candidate
                    .plan
                    .map(|plan| placement_effect_string_proof(plan.proof)),
                publication_boundary,
                source: PlacementEffectSource::StringCorridor,
                subject: format!("string.value%{}", value.as_u32()),
                decision: string_decision(candidate.kind),
                demand: string_demand(candidate.kind),
                state: string_state(candidate.state),
                detail: candidate.plan.map(|plan| plan.summary()),
                reason: candidate.reason.to_string(),
            });
        }
    }
}

fn placement_effect_string_proof(
    proof: StringCorridorCandidateProof,
) -> PlacementEffectStringProof {
    match proof {
        StringCorridorCandidateProof::BorrowedSlice { source, start, end } => {
            PlacementEffectStringProof::BorrowedSlice { source, start, end }
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
            shared_source,
        } => PlacementEffectStringProof::ConcatTriplet {
            left_value,
            left_source,
            left_start,
            left_end,
            middle,
            right_value,
            right_source,
            right_start,
            right_end,
            shared_source,
        },
    }
}

fn collect_sum_routes(function: &MirFunction, routes: &mut Vec<PlacementEffectRoute>) {
    routes.extend(
        function
            .metadata
            .sum_placement_selections
            .iter()
            .map(sum_route),
    );
}

fn collect_thin_entry_routes(function: &MirFunction, routes: &mut Vec<PlacementEffectRoute>) {
    routes.extend(
        function
            .metadata
            .thin_entry_selections
            .iter()
            .map(thin_entry_route),
    );
}

fn collect_agg_local_routes(function: &MirFunction, routes: &mut Vec<PlacementEffectRoute>) {
    routes.extend(
        function
            .metadata
            .agg_local_scalarization_routes
            .iter()
            .filter_map(agg_local_route),
    );
}

fn string_decision(kind: StringCorridorCandidateKind) -> PlacementEffectDecision {
    match kind {
        StringCorridorCandidateKind::BorrowCorridorFusion => PlacementEffectDecision::StayBorrowed,
        StringCorridorCandidateKind::PublicationSink => PlacementEffectDecision::PublishHandle,
        StringCorridorCandidateKind::MaterializationSink => {
            PlacementEffectDecision::MaterializeOwned
        }
        StringCorridorCandidateKind::DirectKernelEntry => {
            PlacementEffectDecision::DirectKernelEntry
        }
    }
}

fn string_demand(kind: StringCorridorCandidateKind) -> PlacementEffectDemand {
    match kind {
        StringCorridorCandidateKind::BorrowCorridorFusion => PlacementEffectDemand::ReadRef,
        StringCorridorCandidateKind::PublicationSink => PlacementEffectDemand::PublishHandle,
        StringCorridorCandidateKind::MaterializationSink => PlacementEffectDemand::OwnedPayload,
        StringCorridorCandidateKind::DirectKernelEntry => PlacementEffectDemand::CellResidence,
    }
}

fn string_state(state: StringCorridorCandidateState) -> PlacementEffectState {
    match state {
        StringCorridorCandidateState::Candidate => PlacementEffectState::Candidate,
        StringCorridorCandidateState::AlreadySatisfied => PlacementEffectState::AlreadySatisfied,
    }
}

fn thin_entry_demand(demand: ThinEntryDemand) -> PlacementEffectDemand {
    match demand {
        ThinEntryDemand::Unknown => PlacementEffectDemand::Unknown,
        ThinEntryDemand::InlineScalar => PlacementEffectDemand::Immediate,
        ThinEntryDemand::BorrowedText => PlacementEffectDemand::ReadRef,
        ThinEntryDemand::PublicHandle => PlacementEffectDemand::PublishHandle,
        ThinEntryDemand::LocalAggregate => PlacementEffectDemand::LocalAggregate,
    }
}

fn sum_route(selection: &SumPlacementSelection) -> PlacementEffectRoute {
    PlacementEffectRoute {
        block: Some(selection.block),
        instruction_index: Some(selection.instruction_index),
        value: selection.value,
        source_value: selection.source_sum,
        window_start: None,
        window_end: None,
        borrow_contract: None,
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        string_proof: None,
        publication_boundary: None,
        source: PlacementEffectSource::SumPlacement,
        subject: selection.subject.clone(),
        decision: match selection.selected_path {
            SumPlacementPath::LocalAggregate => PlacementEffectDecision::LocalAggregate,
            SumPlacementPath::CompatRuntimeBox => PlacementEffectDecision::CompatRuntimeBox,
        },
        demand: match selection.selected_path {
            SumPlacementPath::LocalAggregate => PlacementEffectDemand::LocalAggregate,
            SumPlacementPath::CompatRuntimeBox => PlacementEffectDemand::StableObject,
        },
        state: PlacementEffectState::Selected,
        detail: Some(selection.manifest_row.to_string()),
        reason: selection.reason.clone(),
    }
}

fn thin_entry_route(selection: &ThinEntrySelection) -> PlacementEffectRoute {
    PlacementEffectRoute {
        block: Some(selection.block),
        instruction_index: Some(selection.instruction_index),
        value: selection.value,
        source_value: None,
        window_start: None,
        window_end: None,
        borrow_contract: None,
        publish_reason: None,
        publish_repr_policy: None,
        stable_view_provenance: None,
        string_proof: None,
        publication_boundary: None,
        source: PlacementEffectSource::ThinEntry,
        subject: selection.subject.clone(),
        decision: match selection.selected_entry {
            ThinEntryPreferredEntry::PublicEntry => PlacementEffectDecision::PublicEntry,
            ThinEntryPreferredEntry::ThinInternalEntry => {
                PlacementEffectDecision::ThinInternalEntry
            }
        },
        demand: thin_entry_demand(selection.demand),
        state: match selection.state {
            ThinEntrySelectionState::Candidate => PlacementEffectState::Candidate,
            ThinEntrySelectionState::AlreadySatisfied => PlacementEffectState::AlreadySatisfied,
        },
        detail: Some(selection.manifest_row.to_string()),
        reason: selection.reason.clone(),
    }
}

fn agg_local_route(route: &AggLocalScalarizationRoute) -> Option<PlacementEffectRoute> {
    let detail = route.kind.to_string();
    match route.kind {
        AggLocalScalarizationKind::SumLocalLayout(_)
        | AggLocalScalarizationKind::UserBoxLocalBody(_)
        | AggLocalScalarizationKind::RecordLocalLayout(_) => Some(PlacementEffectRoute {
            block: route.block,
            instruction_index: route.instruction_index,
            value: route.value,
            source_value: None,
            window_start: None,
            window_end: None,
            borrow_contract: None,
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            string_proof: None,
            publication_boundary: None,
            source: PlacementEffectSource::AggLocalScalarization,
            subject: route.subject.clone(),
            decision: PlacementEffectDecision::LocalAggregate,
            demand: PlacementEffectDemand::LocalAggregate,
            state: PlacementEffectState::AlreadySatisfied,
            detail: Some(detail),
            reason: route.reason.clone(),
        }),
        AggLocalScalarizationKind::TypedSlotStorage(_) => None,
    }
}

fn source_rank(source: PlacementEffectSource) -> u8 {
    match source {
        PlacementEffectSource::StringCorridor => 0,
        PlacementEffectSource::SumPlacement => 1,
        PlacementEffectSource::AggLocalScalarization => 2,
        PlacementEffectSource::ThinEntry => 3,
    }
}

fn route_sort_key(route: &PlacementEffectRoute) -> (u8, u32, u32, u32, String) {
    let block_rank = route.block.map(|block| block.as_u32()).unwrap_or(u32::MAX);
    let instruction_rank = route
        .instruction_index
        .map(|index| index as u32)
        .unwrap_or(u32::MAX);
    let value_rank = route.value.map(|value| value.as_u32()).unwrap_or(u32::MAX);
    (
        source_rank(route.source),
        block_rank,
        instruction_rank,
        value_rank,
        route.subject.clone(),
    )
}
