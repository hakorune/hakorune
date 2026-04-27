use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::agg_local_scalarization::{AggLocalScalarizationKind, AggLocalScalarizationRoute};
use crate::mir::placement_effect::{
    PlacementEffectBorrowContract, PlacementEffectDecision, PlacementEffectDemand,
    PlacementEffectPublicationBoundary, PlacementEffectRoute, PlacementEffectSource,
    PlacementEffectState, PlacementEffectStringProof,
};
use crate::mir::storage_class::StorageClass;
use crate::mir::sum_placement::{SumObjectizationBarrier, SumPlacementFact, SumPlacementState};
use crate::mir::sum_placement_layout::{SumLocalAggregateLayout, SumPlacementLayout};
use crate::mir::sum_placement_selection::{SumPlacementPath, SumPlacementSelection};
use crate::mir::thin_entry::{ThinEntrySurface, ThinEntryValueClass};
use crate::mir::{BasicBlockId, MirModule};

#[test]
fn build_mir_json_root_emits_sum_placement_facts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .sum_placement_facts
        .push(SumPlacementFact {
            block: BasicBlockId::new(0),
            instruction_index: 4,
            value: Some(crate::mir::ValueId::new(9)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            value_class: ThinEntryValueClass::AggLocal,
            state: SumPlacementState::LocalAggregateCandidate,
            tag_reads: 1,
            project_reads: 1,
            barriers: vec![SumObjectizationBarrier::Call],
            reason: "variant value stays local until call boundary".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["sum_placement_facts"]
        .as_array()
        .expect("sum_placement_facts array");

    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0]["surface"], "variant_make");
    assert_eq!(facts[0]["state"], "local_agg_candidate");
    assert_eq!(facts[0]["barriers"][0], "call");
    assert_eq!(facts[0]["value"], 9);
}

#[test]
fn build_mir_json_root_emits_sum_placement_selections() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .sum_placement_selections
        .push(SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: 5,
            value: Some(crate::mir::ValueId::new(10)),
            surface: ThinEntrySurface::VariantProject,
            subject: "Option::Some".to_string(),
            source_sum: Some(crate::mir::ValueId::new(9)),
            manifest_row: "variant_project.local_aggregate",
            selected_path: SumPlacementPath::LocalAggregate,
            reason: "selected local aggregate projection".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let selections = root["functions"][0]["metadata"]["sum_placement_selections"]
        .as_array()
        .expect("sum_placement_selections array");

    assert_eq!(selections.len(), 1);
    assert_eq!(
        selections[0]["manifest_row"],
        "variant_project.local_aggregate"
    );
    assert_eq!(selections[0]["selected_path"], "local_aggregate");
    assert_eq!(selections[0]["source_sum"], 9);
    assert_eq!(selections[0]["value"], 10);
}

#[test]
fn build_mir_json_root_emits_sum_placement_layouts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .sum_placement_layouts
        .push(SumPlacementLayout {
            block: BasicBlockId::new(0),
            instruction_index: 6,
            value: Some(crate::mir::ValueId::new(11)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            layout: SumLocalAggregateLayout::TagI64Payload,
            reason: "selected local aggregate uses i64 payload lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let layouts = root["functions"][0]["metadata"]["sum_placement_layouts"]
        .as_array()
        .expect("sum_placement_layouts array");

    assert_eq!(layouts.len(), 1);
    assert_eq!(layouts[0]["layout"], "tag_i64_payload");
    assert_eq!(layouts[0]["surface"], "variant_make");
    assert_eq!(layouts[0]["value"], 11);
}

#[test]
fn build_mir_json_root_emits_agg_local_scalarization_routes() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .agg_local_scalarization_routes
        .push(AggLocalScalarizationRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(2),
            value: Some(crate::mir::ValueId::new(11)),
            subject: "Option::Some".to_string(),
            kind: AggLocalScalarizationKind::SumLocalLayout(SumLocalAggregateLayout::TagI64Payload),
            reason: "selected local aggregate uses i64 payload lane".to_string(),
        });
    function
        .metadata
        .agg_local_scalarization_routes
        .push(AggLocalScalarizationRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(3),
            value: Some(crate::mir::ValueId::new(12)),
            subject: "Point.x".to_string(),
            kind: AggLocalScalarizationKind::UserBoxLocalBody(ThinEntryValueClass::InlineI64),
            reason: "typed field read stays on thin internal scalar lane".to_string(),
        });
    function
        .metadata
        .agg_local_scalarization_routes
        .push(AggLocalScalarizationRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(4),
            value: Some(crate::mir::ValueId::new(13)),
            subject: "Point.flag".to_string(),
            kind: AggLocalScalarizationKind::TypedSlotStorage(StorageClass::InlineBool),
            reason: "typed slot stays inline on the scalar lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let routes = root["functions"][0]["metadata"]["agg_local_scalarization_routes"]
        .as_array()
        .expect("agg_local_scalarization_routes array");

    assert_eq!(routes.len(), 3);
    assert_eq!(routes[0]["kind"], "sum_local_layout");
    assert_eq!(routes[0]["layout"], "tag_i64_payload");
    assert_eq!(routes[1]["kind"], "user_box_local_body");
    assert_eq!(routes[1]["value_class"], "inline_i64");
    assert_eq!(routes[2]["kind"], "typed_slot_storage");
    assert_eq!(routes[2]["storage_class"], "inline_bool");
}

#[test]
fn build_mir_json_root_emits_placement_effect_routes() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .placement_effect_routes
        .push(PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(2),
            value: Some(crate::mir::ValueId::new(11)),
            source_value: None,
            window_start: Some(crate::mir::ValueId::new(2)),
            window_end: Some(crate::mir::ValueId::new(3)),
            borrow_contract: Some(PlacementEffectBorrowContract::BorrowTextFromObject),
            publish_reason: Some(crate::mir::StringPublishReason::StableObjectDemand),
            publish_repr_policy: Some(crate::mir::StringPublishReprPolicy::StableOwned),
            stable_view_provenance: None,
            string_proof: Some(PlacementEffectStringProof::BorrowedSlice {
                source: crate::mir::ValueId::new(1),
                start: crate::mir::ValueId::new(2),
                end: crate::mir::ValueId::new(3),
            }),
            publication_boundary: Some(PlacementEffectPublicationBoundary::FirstExternalBoundary),
            source: PlacementEffectSource::StringCorridor,
            subject: "string.value%11".to_string(),
            decision: PlacementEffectDecision::PublishHandle,
            demand: PlacementEffectDemand::PublishHandle,
            state: PlacementEffectState::Candidate,
            detail: Some("plan(root=%11 source=- outer=- known_len=- proof=borrowed_slice(src=%1 start=%2 end=%3))".to_string()),
            reason: "publish boundary can sink to the corridor exit".to_string(),
        });
    function
        .metadata
        .placement_effect_routes
        .push(PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(3),
            value: Some(crate::mir::ValueId::new(12)),
            source_value: Some(crate::mir::ValueId::new(9)),
            window_start: None,
            window_end: None,
            borrow_contract: None,
            publish_reason: None,
            publish_repr_policy: None,
            stable_view_provenance: None,
            string_proof: None,
            publication_boundary: None,
            source: PlacementEffectSource::SumPlacement,
            subject: "Option::Some".to_string(),
            decision: PlacementEffectDecision::LocalAggregate,
            demand: PlacementEffectDemand::LocalAggregate,
            state: PlacementEffectState::Selected,
            detail: Some("variant_make.local_aggregate".to_string()),
            reason: "selected local aggregate route".to_string(),
        });
    function
        .metadata
        .placement_effect_routes
        .push(PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(4),
            value: Some(crate::mir::ValueId::new(13)),
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
            subject: "Point.x".to_string(),
            decision: PlacementEffectDecision::LocalAggregate,
            demand: PlacementEffectDemand::LocalAggregate,
            state: PlacementEffectState::AlreadySatisfied,
            detail: Some("user_box_local_body(inline_i64)".to_string()),
            reason: "typed field body stays aggregate-local".to_string(),
        });
    function
        .metadata
        .placement_effect_routes
        .push(PlacementEffectRoute {
            block: Some(BasicBlockId::new(0)),
            instruction_index: Some(5),
            value: Some(crate::mir::ValueId::new(14)),
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
            subject: "Point.x".to_string(),
            decision: PlacementEffectDecision::ThinInternalEntry,
            demand: PlacementEffectDemand::Immediate,
            state: PlacementEffectState::AlreadySatisfied,
            detail: Some("user_box_field_get.inline_scalar".to_string()),
            reason: "typed field read stays on thin internal scalar lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let routes = root["functions"][0]["metadata"]["placement_effect_routes"]
        .as_array()
        .expect("placement_effect_routes array");

    assert_eq!(routes.len(), 4);
    assert_eq!(routes[0]["source"], "string_corridor");
    assert_eq!(routes[0]["decision"], "publish_handle");
    assert_eq!(routes[0]["demand"], "publish_handle");
    assert_eq!(routes[0]["window_start"], 2);
    assert_eq!(routes[0]["window_end"], 3);
    assert_eq!(routes[0]["borrow_contract"], "borrow_text_from_obj");
    assert_eq!(routes[0]["publish_reason"], "stable_object_demand");
    assert_eq!(routes[0]["publish_repr_policy"], "stable_owned");
    assert!(routes[0]["stable_view_provenance"].is_null());
    assert_eq!(routes[0]["publication_boundary"], "first_external_boundary");
    assert_eq!(routes[0]["string_proof"]["kind"], "borrowed_slice");
    assert_eq!(routes[0]["string_proof"]["source"], 1);
    assert_eq!(routes[1]["source"], "sum_placement");
    assert_eq!(routes[1]["state"], "selected");
    assert_eq!(routes[1]["source_value"], 9);
    assert!(routes[1]["window_start"].is_null());
    assert!(routes[1]["window_end"].is_null());
    assert!(routes[1]["string_proof"].is_null());
    assert_eq!(routes[2]["source"], "agg_local_scalarization");
    assert_eq!(routes[2]["decision"], "local_aggregate");
    assert_eq!(routes[2]["demand"], "local_aggregate");
    assert_eq!(routes[3]["source"], "thin_entry");
    assert_eq!(routes[3]["decision"], "thin_internal_entry");
    assert_eq!(routes[3]["demand"], "immediate");
}
