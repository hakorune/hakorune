/*!
 * Folded generic placement/effect owner seam.
 *
 * This module reads the landed string corridor, sum placement, and thin-entry
 * pilot metadata together and exposes one generic route inventory. It does not
 * rewrite MIR or change lowering behavior in this wave.
 */

use super::{
    agg_local_scalarization::AggLocalScalarizationKind,
    build_value_def_map,
    string_corridor_placement::{StringCorridorCandidateKind, StringCorridorCandidateState},
    sum_placement_selection::{SumPlacementPath, SumPlacementSelection},
    thin_entry::ThinEntryPreferredEntry,
    thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState},
    BasicBlockId, MirFunction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectSource {
    StringCorridor,
    SumPlacement,
    AggLocalScalarization,
    ThinEntry,
}

impl std::fmt::Display for PlacementEffectSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringCorridor => f.write_str("string_corridor"),
            Self::SumPlacement => f.write_str("sum_placement"),
            Self::AggLocalScalarization => f.write_str("agg_local_scalarization"),
            Self::ThinEntry => f.write_str("thin_entry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectDecision {
    StayBorrowed,
    PublishHandle,
    MaterializeOwned,
    DirectKernelEntry,
    LocalAggregate,
    CompatRuntimeBox,
    PublicEntry,
    ThinInternalEntry,
}

impl std::fmt::Display for PlacementEffectDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StayBorrowed => f.write_str("stay_borrowed"),
            Self::PublishHandle => f.write_str("publish_handle"),
            Self::MaterializeOwned => f.write_str("materialize_owned"),
            Self::DirectKernelEntry => f.write_str("direct_kernel_entry"),
            Self::LocalAggregate => f.write_str("local_aggregate"),
            Self::CompatRuntimeBox => f.write_str("compat_runtime_box"),
            Self::PublicEntry => f.write_str("public_entry"),
            Self::ThinInternalEntry => f.write_str("thin_internal_entry"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEffectState {
    Candidate,
    Selected,
    AlreadySatisfied,
}

impl std::fmt::Display for PlacementEffectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Candidate => f.write_str("candidate"),
            Self::Selected => f.write_str("selected"),
            Self::AlreadySatisfied => f.write_str("already_satisfied"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementEffectRoute {
    pub block: Option<BasicBlockId>,
    pub instruction_index: Option<usize>,
    pub value: Option<ValueId>,
    pub source_value: Option<ValueId>,
    pub window_start: Option<ValueId>,
    pub window_end: Option<ValueId>,
    pub source: PlacementEffectSource,
    pub subject: String,
    pub decision: PlacementEffectDecision,
    pub state: PlacementEffectState,
    pub detail: Option<String>,
    pub reason: String,
}

impl PlacementEffectRoute {
    pub fn summary(&self) -> String {
        let block_suffix = self
            .block
            .map(|block| format!(" bb{}", block.as_u32()))
            .unwrap_or_else(|| " module".to_string());
        let instruction_suffix = self
            .instruction_index
            .map(|index| format!("#{index}"))
            .unwrap_or_default();
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        let source_value_suffix = self
            .source_value
            .map(|value| format!(" source_value=%{}", value.as_u32()))
            .unwrap_or_default();
        let window_suffix = match (self.window_start, self.window_end) {
            (Some(start), Some(end)) => {
                format!(" window=[%{}, %{}]", start.as_u32(), end.as_u32())
            }
            _ => String::new(),
        };
        let detail_suffix = self
            .detail
            .as_ref()
            .map(|detail| format!(" detail={detail}"))
            .unwrap_or_default();
        format!(
            "{}{} {} {} {} [{}]{}{}{}{} reason={}",
            block_suffix,
            instruction_suffix,
            self.source,
            self.subject,
            self.decision,
            self.state,
            value_suffix,
            source_value_suffix,
            window_suffix,
            detail_suffix,
            self.reason
        )
    }
}

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
        for candidate in candidates {
            routes.push(PlacementEffectRoute {
                block: location.map(|(block, _)| block),
                instruction_index: location.map(|(_, index)| index),
                value: Some(*value),
                source_value: None,
                window_start: candidate.plan.and_then(|plan| plan.start),
                window_end: candidate.plan.and_then(|plan| plan.end),
                source: PlacementEffectSource::StringCorridor,
                subject: format!("string.value%{}", value.as_u32()),
                decision: string_decision(candidate.kind),
                state: string_state(candidate.state),
                detail: candidate.plan.map(|plan| plan.summary()),
                reason: candidate.reason.to_string(),
            });
        }
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

fn string_state(state: StringCorridorCandidateState) -> PlacementEffectState {
    match state {
        StringCorridorCandidateState::Candidate => PlacementEffectState::Candidate,
        StringCorridorCandidateState::AlreadySatisfied => PlacementEffectState::AlreadySatisfied,
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
        source: PlacementEffectSource::SumPlacement,
        subject: selection.subject.clone(),
        decision: match selection.selected_path {
            SumPlacementPath::LocalAggregate => PlacementEffectDecision::LocalAggregate,
            SumPlacementPath::CompatRuntimeBox => PlacementEffectDecision::CompatRuntimeBox,
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
        source: PlacementEffectSource::ThinEntry,
        subject: selection.subject.clone(),
        decision: match selection.selected_entry {
            ThinEntryPreferredEntry::PublicEntry => PlacementEffectDecision::PublicEntry,
            ThinEntryPreferredEntry::ThinInternalEntry => {
                PlacementEffectDecision::ThinInternalEntry
            }
        },
        state: match selection.state {
            ThinEntrySelectionState::Candidate => PlacementEffectState::Candidate,
            ThinEntrySelectionState::AlreadySatisfied => PlacementEffectState::AlreadySatisfied,
        },
        detail: Some(selection.manifest_row.to_string()),
        reason: selection.reason.clone(),
    }
}

fn agg_local_route(route: &crate::mir::AggLocalScalarizationRoute) -> Option<PlacementEffectRoute> {
    let detail = route.kind.to_string();
    match route.kind {
        AggLocalScalarizationKind::SumLocalLayout(_)
        | AggLocalScalarizationKind::UserBoxLocalBody(_) => Some(PlacementEffectRoute {
            block: route.block,
            instruction_index: route.instruction_index,
            value: route.value,
            source_value: None,
            window_start: None,
            window_end: None,
            source: PlacementEffectSource::AggLocalScalarization,
            subject: route.subject.clone(),
            decision: PlacementEffectDecision::LocalAggregate,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        AggLocalScalarizationKind, AggLocalScalarizationRoute, BasicBlockId, EffectMask,
        FunctionSignature, MirInstruction, MirType, StorageClass, StringCorridorCandidate,
        StringCorridorCandidateKind, StringCorridorCandidateState, SumLocalAggregateLayout,
        SumPlacementPath, SumPlacementSelection, ThinEntryCurrentCarrier, ThinEntryPreferredEntry,
        ThinEntrySelection, ThinEntrySelectionState, ThinEntrySurface, ThinEntryValueClass,
        ValueId,
    };

    #[test]
    fn refresh_function_collects_folded_placement_effect_routes() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let entry = function.get_block_mut(BasicBlockId::new(0)).expect("entry");
        entry.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        });
        function.metadata.string_corridor_candidates.insert(
            ValueId::new(1),
            vec![StringCorridorCandidate {
                kind: StringCorridorCandidateKind::PublicationSink,
                state: StringCorridorCandidateState::Candidate,
                reason: "publish boundary can sink to the corridor exit",
                plan: None,
            }],
        );
        function
            .metadata
            .sum_placement_selections
            .push(SumPlacementSelection {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                value: Some(ValueId::new(2)),
                surface: ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                source_sum: Some(ValueId::new(9)),
                manifest_row: "variant_make.local_aggregate",
                selected_path: SumPlacementPath::LocalAggregate,
                reason: "selected local aggregate route".to_string(),
            });
        function
            .metadata
            .thin_entry_selections
            .push(ThinEntrySelection {
                block: BasicBlockId::new(0),
                instruction_index: 2,
                value: Some(ValueId::new(3)),
                surface: ThinEntrySurface::UserBoxFieldGet,
                subject: "Point.x".to_string(),
                manifest_row: "user_box_field_get.inline_scalar",
                selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                state: ThinEntrySelectionState::AlreadySatisfied,
                current_carrier: ThinEntryCurrentCarrier::BackendTyped,
                value_class: ThinEntryValueClass::InlineI64,
                reason: "typed field read stays on thin internal scalar lane".to_string(),
            });
        function
            .metadata
            .agg_local_scalarization_routes
            .push(AggLocalScalarizationRoute {
                block: Some(BasicBlockId::new(0)),
                instruction_index: Some(3),
                value: Some(ValueId::new(4)),
                subject: "Option::Some layout".to_string(),
                kind: AggLocalScalarizationKind::SumLocalLayout(
                    SumLocalAggregateLayout::TagI64Payload,
                ),
                reason: "selected sum local layout stays aggregate-local".to_string(),
            });
        function
            .metadata
            .agg_local_scalarization_routes
            .push(AggLocalScalarizationRoute {
                block: Some(BasicBlockId::new(0)),
                instruction_index: Some(4),
                value: Some(ValueId::new(5)),
                subject: "Point.x".to_string(),
                kind: AggLocalScalarizationKind::UserBoxLocalBody(ThinEntryValueClass::InlineI64),
                reason: "typed field body stays aggregate-local".to_string(),
            });
        function
            .metadata
            .agg_local_scalarization_routes
            .push(AggLocalScalarizationRoute {
            block: None,
            instruction_index: None,
            value: Some(ValueId::new(6)),
            subject: "value%6".to_string(),
            kind: AggLocalScalarizationKind::TypedSlotStorage(StorageClass::InlineBool),
            reason:
                "typed slot storage stays agg_local-only and should not fold into placement/effect"
                    .to_string(),
        });

        refresh_function_placement_effect_routes(&mut function);

        assert_eq!(function.metadata.placement_effect_routes.len(), 5);
        assert!(matches!(
            function.metadata.placement_effect_routes[0].decision,
            PlacementEffectDecision::PublishHandle
        ));
        assert!(matches!(
            function.metadata.placement_effect_routes[1].decision,
            PlacementEffectDecision::LocalAggregate
        ));
        assert_eq!(
            function.metadata.placement_effect_routes[1].source_value,
            Some(ValueId::new(9))
        );
        assert!(matches!(
            function.metadata.placement_effect_routes[2].source,
            PlacementEffectSource::AggLocalScalarization
        ));
        assert!(matches!(
            function.metadata.placement_effect_routes[2].decision,
            PlacementEffectDecision::LocalAggregate
        ));
        assert!(matches!(
            function.metadata.placement_effect_routes[3].source,
            PlacementEffectSource::AggLocalScalarization
        ));
        assert!(matches!(
            function.metadata.placement_effect_routes[4].decision,
            PlacementEffectDecision::ThinInternalEntry
        ));
    }
}
