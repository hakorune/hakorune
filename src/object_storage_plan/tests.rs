use super::*;

#[test]
fn exact_candidates_are_separate_from_generic_or_escaped_routes() {
    assert!(ObjectStoragePlan::ExactStackObject {
        layout_id: LayoutId(7),
    }
    .is_exact_candidate());
    assert!(ObjectStoragePlan::ExactNativeStruct {
        layout_id: LayoutId(7),
    }
    .is_exact_candidate());
    assert!(ObjectStoragePlan::Scalarized {
        fields: vec![FieldScalarPlan {
            field_id: FieldId(1),
            layout_id: LayoutId(7),
            scalar_type: ScalarStorageType::I64,
        }],
    }
    .is_exact_candidate());
    assert!(ObjectStoragePlan::FlattenedNestedFields {
        owner_layout_id: LayoutId(8),
        fields: vec![FlattenedNestedFieldPlan {
            owner_field_id: FieldId(1),
            nested_field_id: FieldId(2),
            flattened_field_id: FieldId(3),
            nested_layout_id: LayoutId(7),
            scalar_type: ScalarStorageType::I64,
        }],
    }
    .is_exact_candidate());

    assert!(ObjectStoragePlan::GenericBox {
        reason: GenericBoxReason::MissingTypeProof,
    }
    .is_generic_or_escaped());
    assert!(ObjectStoragePlan::HostHandleEscaped {
        reason: EscapeReason::HostHandlePublicationRequired,
    }
    .is_generic_or_escaped());
    assert!(ObjectStoragePlan::ArcDynBox {
        reason: DynamicReason::ArcDynBoxCarrierRequired,
    }
    .is_generic_or_escaped());
}

#[test]
fn report_fields_keep_execution_disabled() {
    let fields = object_storage_plan_report_fields();
    assert!(fields.contains(&("mirbuilder_object_management_enabled", "0")));
    assert!(fields.contains(&("object_storage_plan_is_representation_truth", "1")));
    assert!(fields.contains(&("object_storage_plan_vocabulary_defined", "1")));
    assert!(fields.contains(&("objectplan_canonical_vocabulary_defined", "1")));
    assert!(fields.contains(&("objectplan_is_representation_truth", "1")));
    assert!(fields.contains(&("objectplan_is_publication_site_truth", "1")));
    assert!(fields.contains(&("local_first_object_plan_alias_retired", "1")));
    assert!(fields.contains(&("object_site_location_vocabulary_defined", "1")));
    assert!(fields.contains(&("object_site_location_field_migration_enabled", "1")));
    assert!(fields.contains(&(
        "object_publication_site_location_field_migrated",
        "1"
    )));
    assert!(fields.contains(&(
        "local_fastpath_fact_location_field_migrated",
        "1"
    )));
    assert!(fields.contains(&(
        "local_publication_inventory_location_field_migrated",
        "1"
    )));
    assert!(fields.contains(&("routeplan_objectplan_handoff_contract_defined", "1")));
    assert!(fields.contains(&("routeplan_owns_execution_not_representation", "1")));
    assert!(fields.contains(&("objectplan_owns_representation_not_execution", "1")));
    assert!(fields.contains(&("backend_requires_routeplan_for_direct_call", "1")));
    assert!(fields.contains(&("backend_requires_objectplan_for_representation_bypass", "1")));
    assert!(fields.contains(&("backend_plan_consumer_guard_enabled", "1")));
    assert!(fields.contains(&(
        "backend_plan_consumer_requires_routeplan_and_objectplan",
        "1"
    )));
    assert!(fields.contains(&("backend_existing_flattened_nested_consumer_allowed", "1")));
    assert!(fields.contains(&("backend_new_lowering_enabled", "0")));
    assert!(fields.contains(&("backend_helper_symbol_inference_enabled", "0")));
    assert!(fields.contains(&("backend_method_name_special_case_enabled", "0")));
    assert!(fields.contains(&("backend_variable_name_special_case_enabled", "0")));
    assert!(fields.contains(&("object_plan_local_first_vocabulary_defined", "1")));
    assert!(fields.contains(&("object_plan_publication_sites_defined", "1")));
    assert!(fields.contains(&("standalone_publication_plan_enabled", "0")));
    assert!(fields.contains(&("publication_state_vocabulary_defined", "1")));
    assert!(fields.contains(&("publication_state_unpublished_fastpath_allowed", "1")));
    assert!(fields.contains(&("publication_state_published_fastpath_allowed", "0")));
    assert!(fields.contains(&("publication_state_maybe_published_fastpath_allowed", "0")));
    assert!(fields.contains(&("local_fastpath_fallback_reason_vocabulary_defined", "1")));
    assert!(fields.contains(&("local_fastpath_fact_vocabulary_defined", "1")));
    assert!(fields.contains(&("fastpath_decision_vocabulary_defined", "1")));
    assert!(fields.contains(&("fastpath_decision_shape", "AllowFact_or_DenyReason")));
    assert!(fields.contains(&("fastpath_plan_epoch_vocabulary_defined", "1")));
    assert!(fields.contains(&("local_fastpath_fact_plan_epoch_required", "1")));
    assert!(fields.contains(&("local_fastpath_fact_backend_consumable", "1")));
    assert!(fields.contains(&("fallback_evidence_backend_consumable", "0")));
    assert!(fields.contains(&("fallback_fact_enabled", "0")));
    assert!(fields.contains(&("backend_reads_local_fastpath_fact_only", "1")));
    assert!(fields.contains(&("full_escape_engine_required_for_v0", "0")));
    assert!(fields.contains(&("interprocedural_fixedpoint_required_for_v0", "0")));
    assert!(fields.contains(&("local_alias_class_mvp_vocabulary_defined", "1")));
    assert!(fields.contains(&("local_alias_class_mvp_source_count", "5")));
    assert!(fields.contains(&("local_alias_class_heap_graph_enabled", "0")));
    assert!(fields.contains(&("local_alias_class_field_sensitive_points_to_enabled", "0")));
    assert!(fields.contains(&("local_publication_inventory_v2_vocabulary_defined", "1")));
    assert!(fields.contains(&("local_publication_inventory_v2_report_only", "1")));
    assert!(fields.contains(&("local_publication_inventory_v2_backend_consumable", "0")));
    assert!(fields.contains(&("local_publication_inventory_v2_unknown_alias_fallback", "1")));
    assert!(fields.contains(&(
        "local_publication_inventory_v2_maybe_published_fallback",
        "1"
    )));
    assert!(fields.contains(&("local_known_receiver_direct_call_shadow_defined", "1")));
    assert!(fields.contains(&(
        "local_known_receiver_direct_call_shadow_backend_consumable",
        "0"
    )));
    assert!(fields.contains(&("local_known_receiver_direct_call_shadow_fact_optional", "1")));
    assert!(fields.contains(&(
        "local_known_receiver_direct_call_shadow_requires_routeplan",
        "1"
    )));
    assert!(fields.contains(&(
        "local_known_receiver_direct_call_shadow_requires_objectstorageplan",
        "1"
    )));
    assert!(fields.contains(&("flattened_nested_field_layout_vocabulary_defined", "1")));
    assert!(fields.contains(&("object_storage_plan_execution_enabled", "0")));
    assert!(fields.contains(&("object_plan_execution_enabled", "0")));
    assert!(fields.contains(&("exact_object_shadow_ready", "1")));
}

#[test]
fn local_first_plan_tracks_publication_sites_without_enabling_execution() {
    let unpublished = ObjectPlan::new(
        ObjectValueId(1),
        ObjectStoragePlan::ExactNativeStruct {
            layout_id: LayoutId(7),
        },
        vec![],
    );
    assert!(unpublished.is_unpublished_local());
    assert!(!unpublished.requires_publication());

    let published = ObjectPlan::new(
        ObjectValueId(2),
        ObjectStoragePlan::ExactNativeStruct {
            layout_id: LayoutId(7),
        },
        vec![ObjectPublicationSite {
            value_id: ObjectValueId(2),
            reason: ObjectPublicationReason::PluginOrExternBoundary,
            location: ObjectSiteLocation::new(ObjectBasicBlockId(3), ObjectInstructionIndex(4)),
        }],
    );
    assert!(!published.is_unpublished_local());
    assert!(published.requires_publication());
    assert_eq!(
        published.publication_sites[0].location(),
        ObjectSiteLocation::new(ObjectBasicBlockId(3), ObjectInstructionIndex(4))
    );
    assert_eq!(published.publication_sites[0].block_id(), ObjectBasicBlockId(3));
    assert_eq!(
        published.publication_sites[0].instruction_index(),
        ObjectInstructionIndex(4)
    );
}

#[test]
fn publication_state_allows_only_unpublished_local_fast_path() {
    assert!(PublicationState::Unpublished.permits_local_fast_path());
    assert_eq!(PublicationState::Unpublished.fallback_reason(), None);

    assert!(!PublicationState::Published.permits_local_fast_path());
    assert_eq!(
        PublicationState::Published.fallback_reason(),
        Some(LocalFastPathFallbackReason::PublishedBeforeSite)
    );

    assert!(!PublicationState::MaybePublished.permits_local_fast_path());
    assert_eq!(
        PublicationState::MaybePublished.fallback_reason(),
        Some(LocalFastPathFallbackReason::MaybePublishedBeforeSite)
    );
}

#[test]
fn local_fastpath_fact_is_positive_permission_vocabulary() {
    let fact = LocalFastPathFact::known_receiver_direct_call(
        LocalFastPathSiteId(10),
        ObjectBasicBlockId(11),
        ObjectInstructionIndex(12),
        ObjectValueId(20),
        AliasClassId(30),
        RoutePlanId(40),
        ObjectStoragePlanId(50),
    );

    assert_eq!(fact.site_id, LocalFastPathSiteId(10));
    assert_eq!(
        fact.location(),
        ObjectSiteLocation::new(ObjectBasicBlockId(11), ObjectInstructionIndex(12))
    );
    assert_eq!(fact.block_id(), ObjectBasicBlockId(11));
    assert_eq!(fact.instruction_index(), ObjectInstructionIndex(12));
    assert_eq!(fact.object_id, ObjectValueId(20));
    assert_eq!(fact.alias_class, AliasClassId(30));
    assert_eq!(fact.route_plan, RoutePlanId(40));
    assert_eq!(fact.storage_plan, ObjectStoragePlanId(50));
    assert_eq!(fact.plan_epoch, PlanEpoch::INITIAL);
    assert!(fact.valid_until_publication);
    assert_eq!(
        fact.backend_kind,
        LocalFastPathKind::KnownReceiverDirectCall
    );
}

#[test]
fn fastpath_decision_is_allow_fact_or_deny_reason() {
    let fact = LocalFastPathFact::known_receiver_direct_call(
        LocalFastPathSiteId(10),
        ObjectBasicBlockId(11),
        ObjectInstructionIndex(12),
        ObjectValueId(20),
        AliasClassId(30),
        RoutePlanId(40),
        ObjectStoragePlanId(50),
    );

    let allow = FastPathDecision::allow(fact.clone());
    assert!(allow.is_allow());
    assert!(!allow.is_deny());
    assert_eq!(allow.fact(), Some(&fact));
    assert_eq!(allow.deny_reason(), None);

    let deny = FastPathDecision::deny(LocalFastPathFallbackReason::AliasUnknown);
    assert!(!deny.is_allow());
    assert!(deny.is_deny());
    assert_eq!(deny.fact(), None);
    assert_eq!(
        deny.deny_reason(),
        Some(LocalFastPathFallbackReason::AliasUnknown)
    );

    assert!(PlanEpoch::INITIAL.is_initial());
}

#[test]
fn local_alias_class_mvp_observation_is_passive_vocabulary() {
    let observation = LocalAliasClassObservation {
        value_id: ObjectValueId(7),
        alias_class: AliasClassId(3),
        source_kind: LocalAliasSourceKind::SsaCopy,
    };

    assert_eq!(observation.value_id, ObjectValueId(7));
    assert_eq!(observation.alias_class, AliasClassId(3));
    assert_eq!(observation.source_kind, LocalAliasSourceKind::SsaCopy);

    let allowed_sources = [
        LocalAliasSourceKind::LocalAssignment,
        LocalAliasSourceKind::SsaCopy,
        LocalAliasSourceKind::Phi,
        LocalAliasSourceKind::Select,
        LocalAliasSourceKind::SimpleReceiverAlias,
    ];
    assert_eq!(allowed_sources.len(), 5);
}

#[test]
fn local_publication_inventory_row_is_report_only_gate_input() {
    let eligible = LocalPublicationInventoryRow::new(
        LocalFastPathSiteId(1),
        ObjectBasicBlockId(10),
        ObjectInstructionIndex(11),
        ObjectValueId(2),
        Some(AliasClassId(3)),
        PublicationState::Unpublished,
    );
    assert!(eligible.can_feed_fastpath_eligibility());
    assert_eq!(eligible.fallback_reason, None);
    assert_eq!(
        eligible.location(),
        ObjectSiteLocation::new(ObjectBasicBlockId(10), ObjectInstructionIndex(11))
    );
    assert_eq!(eligible.block_id(), ObjectBasicBlockId(10));
    assert_eq!(eligible.instruction_index(), ObjectInstructionIndex(11));

    let unknown_alias = LocalPublicationInventoryRow::new(
        LocalFastPathSiteId(4),
        ObjectBasicBlockId(40),
        ObjectInstructionIndex(41),
        ObjectValueId(5),
        None,
        PublicationState::Unpublished,
    );
    assert!(!unknown_alias.can_feed_fastpath_eligibility());
    assert_eq!(
        unknown_alias.fallback_reason,
        Some(LocalFastPathFallbackReason::AliasUnknown)
    );

    let maybe_published = LocalPublicationInventoryRow::new(
        LocalFastPathSiteId(6),
        ObjectBasicBlockId(60),
        ObjectInstructionIndex(61),
        ObjectValueId(7),
        Some(AliasClassId(8)),
        PublicationState::MaybePublished,
    );
    assert!(!maybe_published.can_feed_fastpath_eligibility());
    assert_eq!(
        maybe_published.fallback_reason,
        Some(LocalFastPathFallbackReason::MaybePublishedBeforeSite)
    );
}

#[test]
fn local_known_receiver_direct_call_shadow_row_creates_fact_only_when_all_inputs_are_positive() {
    let eligible_inventory = LocalPublicationInventoryRow::new(
        LocalFastPathSiteId(1),
        ObjectBasicBlockId(10),
        ObjectInstructionIndex(11),
        ObjectValueId(2),
        Some(AliasClassId(3)),
        PublicationState::Unpublished,
    );
    let eligible = LocalKnownReceiverDirectCallShadowRow::new(
        eligible_inventory,
        Some(RoutePlanId(4)),
        Some(ObjectStoragePlanId(5)),
    );
    assert!(eligible.candidate_fact.is_some());
    assert_eq!(eligible.fallback_reason, None);

    let missing_route = LocalKnownReceiverDirectCallShadowRow::new(
        eligible_inventory,
        None,
        Some(ObjectStoragePlanId(5)),
    );
    assert!(missing_route.candidate_fact.is_none());
    assert_eq!(
        missing_route.fallback_reason,
        Some(LocalFastPathFallbackReason::DynamicRoute)
    );

    let missing_storage =
        LocalKnownReceiverDirectCallShadowRow::new(eligible_inventory, Some(RoutePlanId(4)), None);
    assert!(missing_storage.candidate_fact.is_none());
    assert_eq!(
        missing_storage.fallback_reason,
        Some(LocalFastPathFallbackReason::GenericStorage)
    );

    let maybe_published_inventory = LocalPublicationInventoryRow::new(
        LocalFastPathSiteId(6),
        ObjectBasicBlockId(60),
        ObjectInstructionIndex(61),
        ObjectValueId(7),
        Some(AliasClassId(8)),
        PublicationState::MaybePublished,
    );
    let maybe_published = LocalKnownReceiverDirectCallShadowRow::new(
        maybe_published_inventory,
        Some(RoutePlanId(9)),
        Some(ObjectStoragePlanId(10)),
    );
    assert!(maybe_published.candidate_fact.is_none());
    assert_eq!(
        maybe_published.fallback_reason,
        Some(LocalFastPathFallbackReason::MaybePublishedBeforeSite)
    );
}
