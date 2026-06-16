//! Object representation planning vocabulary.
//!
//! This module is intentionally passive. It names exact-AOT object storage
//! outcomes, but it does not choose them, does not mutate MIR, and is not wired
//! to lowering. MIRBuilder records object meaning; later analysis can produce
//! these plans for backend consumers.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldScalarPlan {
    pub field_id: FieldId,
    pub layout_id: LayoutId,
    pub scalar_type: ScalarStorageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectValueId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectBasicBlockId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectInstructionIndex(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalFastPathSiteId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AliasClassId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutePlanId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectStoragePlanId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlattenedNestedFieldPlan {
    pub owner_field_id: FieldId,
    pub nested_field_id: FieldId,
    pub flattened_field_id: FieldId,
    pub nested_layout_id: LayoutId,
    pub scalar_type: ScalarStorageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarStorageType {
    I64,
    U64,
    Usize,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericBoxReason {
    MissingTypeProof,
    MissingLayoutProof,
    DynamicNyashBoxApiRequired,
    UnknownDropOrFiniSemantics,
    UnsupportedBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EscapeReason {
    HostHandlePublicationRequired,
    PluginOrExternBoundary,
    ArrayOrMapDynamicStorage,
    ReturnEscapeUnplanned,
    SyncChannelFutureContextBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicReason {
    ArcDynBoxCarrierRequired,
    TraitObjectDowncastRequired,
    RuntimeTypeIdentityRequired,
    PluginLifecycleRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectPublicationReason {
    PluginOrExternBoundary,
    HostHandleRequired,
    DynamicArrayOrMapStorage,
    DynamicNyashBoxApi,
    ReturnAsDynamicBox,
    TaskFutureChannelBoundary,
    UnknownFiniOrDrop,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicationState {
    Unpublished,
    Published,
    MaybePublished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalFastPathFallbackReason {
    OpenWorld,
    AliasUnknown,
    PublishedBeforeSite,
    MaybePublishedBeforeSite,
    DynamicRoute,
    GenericStorage,
    BackendMissing,
    UnknownCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalFastPathKind {
    KnownReceiverDirectCall,
    LocalFieldAccess,
    LocalStorageAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalAliasSourceKind {
    LocalAssignment,
    SsaCopy,
    Phi,
    Select,
    SimpleReceiverAlias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalAliasClassObservation {
    pub value_id: ObjectValueId,
    pub alias_class: AliasClassId,
    pub source_kind: LocalAliasSourceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalPublicationInventoryRow {
    pub site_id: LocalFastPathSiteId,
    pub block_id: ObjectBasicBlockId,
    pub instruction_index: ObjectInstructionIndex,
    pub value_id: ObjectValueId,
    pub alias_class: Option<AliasClassId>,
    pub publication_state: PublicationState,
    pub fallback_reason: Option<LocalFastPathFallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalKnownReceiverDirectCallShadowRow {
    pub inventory: LocalPublicationInventoryRow,
    pub route_plan: Option<RoutePlanId>,
    pub storage_plan: Option<ObjectStoragePlanId>,
    pub candidate_fact: Option<LocalFastPathFact>,
    pub fallback_reason: Option<LocalFastPathFallbackReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectPublicationSite {
    pub value_id: ObjectValueId,
    pub reason: ObjectPublicationReason,
    pub block_id: ObjectBasicBlockId,
    pub instruction_index: ObjectInstructionIndex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFastPathFact {
    pub site_id: LocalFastPathSiteId,
    pub block_id: ObjectBasicBlockId,
    pub instruction_index: ObjectInstructionIndex,
    pub object_id: ObjectValueId,
    pub alias_class: AliasClassId,
    pub route_plan: RoutePlanId,
    pub storage_plan: ObjectStoragePlanId,
    pub valid_until_publication: bool,
    pub backend_kind: LocalFastPathKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectStoragePlan {
    GenericBox {
        reason: GenericBoxReason,
    },
    HostHandleEscaped {
        reason: EscapeReason,
    },
    ArcDynBox {
        reason: DynamicReason,
    },
    ExactStackObject {
        layout_id: LayoutId,
    },
    ExactNativeStruct {
        layout_id: LayoutId,
    },
    Scalarized {
        fields: Vec<FieldScalarPlan>,
    },
    FlattenedNestedFields {
        owner_layout_id: LayoutId,
        fields: Vec<FlattenedNestedFieldPlan>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectPlan {
    pub value_id: ObjectValueId,
    pub storage: ObjectStoragePlan,
    pub publication_sites: Vec<ObjectPublicationSite>,
}

/// Compatibility alias for older local-first rows.
///
/// The canonical vocabulary is `ObjectPlan`: representation and publication
/// sites are one passive planning artifact.  Older phase cards still mention
/// `LocalFirstObjectPlan`, so keep the alias until those rows are retired.
pub type LocalFirstObjectPlan = ObjectPlan;

impl ObjectStoragePlan {
    #[inline]
    pub fn is_exact_candidate(&self) -> bool {
        matches!(
            self,
            Self::ExactStackObject { .. }
                | Self::ExactNativeStruct { .. }
                | Self::Scalarized { .. }
                | Self::FlattenedNestedFields { .. }
        )
    }

    #[inline]
    pub fn is_generic_or_escaped(&self) -> bool {
        matches!(
            self,
            Self::GenericBox { .. } | Self::HostHandleEscaped { .. } | Self::ArcDynBox { .. }
        )
    }
}

impl PublicationState {
    #[inline]
    pub fn permits_local_fast_path(self) -> bool {
        matches!(self, Self::Unpublished)
    }

    #[inline]
    pub fn fallback_reason(self) -> Option<LocalFastPathFallbackReason> {
        match self {
            Self::Unpublished => None,
            Self::Published => Some(LocalFastPathFallbackReason::PublishedBeforeSite),
            Self::MaybePublished => Some(LocalFastPathFallbackReason::MaybePublishedBeforeSite),
        }
    }
}

impl LocalFastPathFact {
    pub fn known_receiver_direct_call(
        site_id: LocalFastPathSiteId,
        block_id: ObjectBasicBlockId,
        instruction_index: ObjectInstructionIndex,
        object_id: ObjectValueId,
        alias_class: AliasClassId,
        route_plan: RoutePlanId,
        storage_plan: ObjectStoragePlanId,
    ) -> Self {
        Self {
            site_id,
            block_id,
            instruction_index,
            object_id,
            alias_class,
            route_plan,
            storage_plan,
            valid_until_publication: true,
            backend_kind: LocalFastPathKind::KnownReceiverDirectCall,
        }
    }
}

impl LocalPublicationInventoryRow {
    pub fn new(
        site_id: LocalFastPathSiteId,
        block_id: ObjectBasicBlockId,
        instruction_index: ObjectInstructionIndex,
        value_id: ObjectValueId,
        alias_class: Option<AliasClassId>,
        publication_state: PublicationState,
    ) -> Self {
        let fallback_reason = match (alias_class, publication_state.fallback_reason()) {
            (None, _) => Some(LocalFastPathFallbackReason::AliasUnknown),
            (Some(_), reason) => reason,
        };
        Self {
            site_id,
            block_id,
            instruction_index,
            value_id,
            alias_class,
            publication_state,
            fallback_reason,
        }
    }

    #[inline]
    pub fn can_feed_fastpath_eligibility(&self) -> bool {
        self.alias_class.is_some()
            && self.publication_state.permits_local_fast_path()
            && self.fallback_reason.is_none()
    }
}

impl LocalKnownReceiverDirectCallShadowRow {
    pub fn new(
        inventory: LocalPublicationInventoryRow,
        route_plan: Option<RoutePlanId>,
        storage_plan: Option<ObjectStoragePlanId>,
    ) -> Self {
        let fallback_reason = inventory
            .fallback_reason
            .or_else(|| {
                route_plan
                    .is_none()
                    .then_some(LocalFastPathFallbackReason::DynamicRoute)
            })
            .or_else(|| {
                storage_plan
                    .is_none()
                    .then_some(LocalFastPathFallbackReason::GenericStorage)
            });

        let candidate_fact = match (
            inventory.can_feed_fastpath_eligibility(),
            inventory.alias_class,
            route_plan,
            storage_plan,
            fallback_reason,
        ) {
            (true, Some(alias_class), Some(route_plan), Some(storage_plan), None) => {
                Some(LocalFastPathFact::known_receiver_direct_call(
                    inventory.site_id,
                    inventory.block_id,
                    inventory.instruction_index,
                    inventory.value_id,
                    alias_class,
                    route_plan,
                    storage_plan,
                ))
            }
            _ => None,
        };

        Self {
            inventory,
            route_plan,
            storage_plan,
            candidate_fact,
            fallback_reason,
        }
    }
}

impl ObjectPlan {
    pub fn new(
        value_id: ObjectValueId,
        storage: ObjectStoragePlan,
        publication_sites: Vec<ObjectPublicationSite>,
    ) -> Self {
        Self {
            value_id,
            storage,
            publication_sites,
        }
    }

    #[inline]
    pub fn is_unpublished_local(&self) -> bool {
        self.publication_sites.is_empty() && self.storage.is_exact_candidate()
    }

    #[inline]
    pub fn requires_publication(&self) -> bool {
        !self.publication_sites.is_empty() || self.storage.is_generic_or_escaped()
    }
}

pub fn object_storage_plan_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("output_contract", "hako-object-storage-plan-ssot-v0"),
        ("mirbuilder_object_management_enabled", "0"),
        ("mirbuilder_records_object_meaning", "1"),
        ("box_callable_registry_is_callable_truth", "1"),
        ("routeplan_is_call_execution_truth", "1"),
        ("object_storage_plan_is_representation_truth", "1"),
        ("object_storage_plan_vocabulary_defined", "1"),
        ("objectplan_canonical_vocabulary_defined", "1"),
        ("objectplan_is_representation_truth", "1"),
        ("objectplan_is_publication_site_truth", "1"),
        ("local_first_object_plan_compat_alias_enabled", "1"),
        ("publication_site_generic_inventory_defined", "1"),
        ("publication_reason_vocabulary_count", "8"),
        ("unknown_publication_forces_generic_fallback", "1"),
        ("routeplan_objectplan_handoff_contract_defined", "1"),
        ("routeplan_owns_execution_not_representation", "1"),
        ("objectplan_owns_representation_not_execution", "1"),
        ("backend_requires_routeplan_for_direct_call", "1"),
        ("backend_requires_objectplan_for_representation_bypass", "1"),
        ("backend_plan_consumer_guard_enabled", "1"),
        (
            "backend_plan_consumer_requires_routeplan_and_objectplan",
            "1",
        ),
        ("backend_existing_flattened_nested_consumer_allowed", "1"),
        ("backend_new_lowering_enabled", "0"),
        ("backend_helper_symbol_inference_enabled", "0"),
        ("backend_method_name_special_case_enabled", "0"),
        ("backend_variable_name_special_case_enabled", "0"),
        ("object_plan_local_first_vocabulary_defined", "1"),
        ("object_plan_publication_sites_defined", "1"),
        ("standalone_publication_plan_enabled", "0"),
        ("unknown_publication_forces_generic_fallback", "1"),
        ("publication_state_vocabulary_defined", "1"),
        ("publication_state_unpublished_fastpath_allowed", "1"),
        ("publication_state_published_fastpath_allowed", "0"),
        ("publication_state_maybe_published_fastpath_allowed", "0"),
        ("local_fastpath_fallback_reason_vocabulary_defined", "1"),
        ("local_fastpath_fact_vocabulary_defined", "1"),
        ("local_fastpath_fact_backend_consumable", "1"),
        ("fallback_evidence_backend_consumable", "0"),
        ("fallback_fact_enabled", "0"),
        ("backend_reads_local_fastpath_fact_only", "1"),
        ("full_escape_engine_required_for_v0", "0"),
        ("interprocedural_fixedpoint_required_for_v0", "0"),
        ("local_alias_class_mvp_vocabulary_defined", "1"),
        ("local_alias_class_mvp_source_count", "5"),
        ("local_alias_class_heap_graph_enabled", "0"),
        ("local_alias_class_field_sensitive_points_to_enabled", "0"),
        ("local_publication_inventory_v2_vocabulary_defined", "1"),
        ("local_publication_inventory_v2_report_only", "1"),
        ("local_publication_inventory_v2_backend_consumable", "0"),
        ("local_publication_inventory_v2_unknown_alias_fallback", "1"),
        (
            "local_publication_inventory_v2_maybe_published_fallback",
            "1",
        ),
        ("local_known_receiver_direct_call_shadow_defined", "1"),
        (
            "local_known_receiver_direct_call_shadow_backend_consumable",
            "0",
        ),
        ("local_known_receiver_direct_call_shadow_fact_optional", "1"),
        (
            "local_known_receiver_direct_call_shadow_requires_routeplan",
            "1",
        ),
        (
            "local_known_receiver_direct_call_shadow_requires_objectstorageplan",
            "1",
        ),
        ("flattened_nested_field_layout_vocabulary_defined", "1"),
        ("object_storage_plan_execution_enabled", "0"),
        ("object_plan_execution_enabled", "0"),
        ("exact_object_shadow_ready", "1"),
        ("product_default_changed", "0"),
    ]
}

#[cfg(test)]
mod tests {
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
        assert!(fields.contains(&("local_first_object_plan_compat_alias_enabled", "1")));
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

        let published = LocalFirstObjectPlan::new(
            ObjectValueId(2),
            ObjectStoragePlan::ExactNativeStruct {
                layout_id: LayoutId(7),
            },
            vec![ObjectPublicationSite {
                value_id: ObjectValueId(2),
                reason: ObjectPublicationReason::PluginOrExternBoundary,
                block_id: ObjectBasicBlockId(3),
                instruction_index: ObjectInstructionIndex(4),
            }],
        );
        assert!(!published.is_unpublished_local());
        assert!(published.requires_publication());
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
        assert_eq!(fact.block_id, ObjectBasicBlockId(11));
        assert_eq!(fact.instruction_index, ObjectInstructionIndex(12));
        assert_eq!(fact.object_id, ObjectValueId(20));
        assert_eq!(fact.alias_class, AliasClassId(30));
        assert_eq!(fact.route_plan, RoutePlanId(40));
        assert_eq!(fact.storage_plan, ObjectStoragePlanId(50));
        assert!(fact.valid_until_publication);
        assert_eq!(
            fact.backend_kind,
            LocalFastPathKind::KnownReceiverDirectCall
        );
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
    fn local_known_receiver_direct_call_shadow_row_creates_fact_only_when_all_inputs_are_positive()
    {
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

        let missing_storage = LocalKnownReceiverDirectCallShadowRow::new(
            eligible_inventory,
            Some(RoutePlanId(4)),
            None,
        );
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
}
