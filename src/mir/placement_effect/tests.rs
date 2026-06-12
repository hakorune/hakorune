use super::*;
use crate::mir::agg_local_scalarization::{AggLocalScalarizationKind, AggLocalScalarizationRoute};
use crate::mir::storage_class::StorageClass;
use crate::mir::string_corridor::{
    StringCorridorBorrowContract, StringPublishReason, StringPublishReprPolicy,
};
use crate::mir::string_corridor_placement::{
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorCandidateState, StringCorridorPublicationBoundary,
    StringCorridorPublicationContract,
};
use crate::mir::sum_placement_layout::SumLocalAggregateLayout;
use crate::mir::sum_placement_selection::{SumPlacementPath, SumPlacementSelection};
use crate::mir::thin_entry::{
    ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryPreferredEntry, ThinEntrySurface,
    ThinEntryValueClass,
};
use crate::mir::thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState};
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
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
            publication_boundary: Some(StringCorridorPublicationBoundary::FirstExternalBoundary),
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
            demand: ThinEntryDemand::InlineScalar,
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
            kind: AggLocalScalarizationKind::SumLocalLayout(SumLocalAggregateLayout::TagI64Payload),
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
            value: None,
            subject: "Meta".to_string(),
            kind: AggLocalScalarizationKind::RecordLocalLayout(7),
            reason: "record layout stays aggregate-local".to_string(),
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

    assert_eq!(function.metadata.placement_effect_routes.len(), 6);
    assert!(matches!(
        function.metadata.placement_effect_routes[0].decision,
        PlacementEffectDecision::PublishHandle
    ));
    assert_eq!(
        function.metadata.placement_effect_routes[0].publication_boundary,
        Some(PlacementEffectPublicationBoundary::FirstExternalBoundary)
    );
    assert_eq!(
        function.metadata.placement_effect_routes[0].demand,
        PlacementEffectDemand::PublishHandle
    );
    assert!(matches!(
        function.metadata.placement_effect_routes[1].decision,
        PlacementEffectDecision::LocalAggregate
    ));
    assert_eq!(
        function.metadata.placement_effect_routes[1].demand,
        PlacementEffectDemand::LocalAggregate
    );
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
        function.metadata.placement_effect_routes[4].source,
        PlacementEffectSource::AggLocalScalarization
    ));
    assert_eq!(
        function.metadata.placement_effect_routes[4]
            .detail
            .as_deref(),
        Some("record_local_layout(7)")
    );
    assert!(matches!(
        function.metadata.placement_effect_routes[5].decision,
        PlacementEffectDecision::ThinInternalEntry
    ));
    assert_eq!(
        function.metadata.placement_effect_routes[5].demand,
        PlacementEffectDemand::Immediate
    );
}

#[test]
fn refresh_function_collects_folded_string_concat_triplet_proof() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let entry = function.get_block_mut(BasicBlockId::new(0)).expect("entry");
    entry.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(21),
        src: ValueId::new(20),
    });
    function.metadata.string_corridor_candidates.insert(
        ValueId::new(21),
        vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::PublicationSink,
            state: StringCorridorCandidateState::Candidate,
            reason: "publish boundary can sink to the corridor exit",
            plan: Some(StringCorridorCandidatePlan {
                corridor_root: ValueId::new(21),
                source_root: Some(ValueId::new(1)),
                borrow_contract: Some(StringCorridorBorrowContract::BorrowTextFromObject),
                publish_reason: Some(StringPublishReason::StableObjectDemand),
                publish_repr_policy: Some(StringPublishReprPolicy::StableOwned),
                stable_view_provenance: None,
                start: Some(ValueId::new(8)),
                end: Some(ValueId::new(9)),
                known_length: Some(2),
                publication_contract: Some(
                    StringCorridorPublicationContract::PublishNowNotRequiredBeforeFirstExternalBoundary,
                ),
                proof: StringCorridorCandidateProof::ConcatTriplet {
                    left_value: Some(ValueId::new(3)),
                    left_source: ValueId::new(1),
                    left_start: ValueId::new(4),
                    left_end: ValueId::new(5),
                    middle: ValueId::new(6),
                    right_value: Some(ValueId::new(7)),
                    right_source: ValueId::new(1),
                    right_start: ValueId::new(8),
                    right_end: ValueId::new(9),
                    shared_source: true,
                },
            }),
            publication_boundary: Some(
                StringCorridorPublicationBoundary::FirstExternalBoundary,
            ),
        }],
    );

    refresh_function_placement_effect_routes(&mut function);

    let route = function
        .metadata
        .placement_effect_routes
        .first()
        .expect("string route");
    assert_eq!(
        route.string_proof,
        Some(PlacementEffectStringProof::ConcatTriplet {
            left_value: Some(ValueId::new(3)),
            left_source: ValueId::new(1),
            left_start: ValueId::new(4),
            left_end: ValueId::new(5),
            middle: ValueId::new(6),
            right_value: Some(ValueId::new(7)),
            right_source: ValueId::new(1),
            right_start: ValueId::new(8),
            right_end: ValueId::new(9),
            shared_source: true,
        })
    );
    assert_eq!(
        route.publication_boundary,
        Some(PlacementEffectPublicationBoundary::FirstExternalBoundary)
    );
    assert_eq!(
        route.borrow_contract,
        Some(PlacementEffectBorrowContract::BorrowTextFromObject)
    );
    assert_eq!(
        route.publish_reason,
        Some(StringPublishReason::StableObjectDemand)
    );
    assert_eq!(
        route.publish_repr_policy,
        Some(StringPublishReprPolicy::StableOwned)
    );
}
