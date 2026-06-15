//! Passive residence proof vocabulary for hot Array receivers.
//!
//! Developer-facing entry:
//! `ArrayReceiverResidenceProofChain::construct_input_source_from_representation_source`.
//!
//! Older staged types remain as internal compatibility gates so reports can
//! distinguish source construction from producer input. This module does not
//! inspect MIR, reinterpret public `ArrayBox` handles, or authorize backend
//! direct-handle bypass.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayReceiverRepresentationOwner {
    ArrayRepr,
    ObjectStoragePlan,
    RepresentationPlanner,
}

impl ArrayReceiverRepresentationOwner {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArrayRepr => "ArrayRepr",
            Self::ObjectStoragePlan => "ObjectStoragePlan",
            Self::RepresentationPlanner => "RepresentationPlanner",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayReceiverArrayRepr {
    DirectI64,
    PublicArrayBoxFallback,
}

impl ArrayReceiverArrayRepr {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectI64 => "DirectI64",
            Self::PublicArrayBoxFallback => "PublicArrayBoxFallback",
        }
    }

    pub const fn proves_direct_storage(self) -> bool {
        matches!(self, Self::DirectI64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayReceiverMaterializationRoute {
    PublicArrayBoxFallback,
    Snapshot,
}

impl ArrayReceiverMaterializationRoute {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicArrayBoxFallback => "public_arraybox_fallback",
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepresentationConfidence {
    Low,
    Medium,
    High,
}

impl RepresentationConfidence {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayReceiverRepresentationSource {
    pub owner: ArrayReceiverRepresentationOwner,
    pub route_kind: &'static str,
    pub receiver_box_name: &'static str,
    pub array_repr: ArrayReceiverArrayRepr,
    pub object_storage_plan_ref: Option<u32>,
    pub direct_array_access_plan_ref: Option<u32>,
    pub materialization_route: ArrayReceiverMaterializationRoute,
    pub confidence: RepresentationConfidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayReceiverConstructorHandoffKind {
    FallbackResidenceCandidate,
    DirectResidenceCandidate,
}

impl ArrayReceiverConstructorHandoffKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FallbackResidenceCandidate => "fallback_residence_candidate",
            Self::DirectResidenceCandidate => "direct_residence_candidate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayReceiverConstructorHandoff {
    pub input: ArrayReceiverRepresentationSource,
    pub kind: ArrayReceiverConstructorHandoffKind,
    pub materialization_route: ArrayReceiverMaterializationRoute,
    pub direct_storage_proof: bool,
    pub backend_bypass_authorized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayReceiverResidenceInputSourceKind {
    PublicArrayBoxFallback,
}

impl ArrayReceiverResidenceInputSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicArrayBoxFallback => "public_arraybox_fallback",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayReceiverResidenceInputSource {
    pub kind: ArrayReceiverResidenceInputSourceKind,
    pub representation_source: ArrayReceiverRepresentationSource,
    pub materialization_route: ArrayReceiverMaterializationRoute,
    pub direct_storage_proof: bool,
    pub backend_bypass_authorized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayReceiverResidenceCandidate {
    PublicArrayBoxFallback,
}

impl ArrayReceiverResidenceCandidate {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicArrayBoxFallback => "public_arraybox_fallback",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayReceiverResidenceInput {
    pub receiver_site_id: Option<u32>,
    pub route_kind: &'static str,
    pub receiver_box_name: &'static str,
    pub direct_array_plan_available: bool,
    pub object_storage_plan_available: bool,
    pub array_repr_available: bool,
    pub residence_candidate: ArrayReceiverResidenceCandidate,
    pub escape_facts_available: bool,
    pub host_handle_publication_before_read: bool,
    pub materialization_route_candidate: ArrayReceiverMaterializationRoute,
    pub direct_storage_proof: bool,
    pub backend_bypass_authorized: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ArrayReceiverResidenceSourceConstructor;

#[derive(Debug, Clone, Copy, Default)]
pub struct ArrayReceiverResidenceProofChain;

impl ArrayReceiverRepresentationSource {
    pub const fn public_arraybox_fallback() -> Self {
        Self {
            owner: ArrayReceiverRepresentationOwner::ArrayRepr,
            route_kind: "array_slot_len",
            receiver_box_name: "ArrayBox",
            array_repr: ArrayReceiverArrayRepr::PublicArrayBoxFallback,
            object_storage_plan_ref: None,
            direct_array_access_plan_ref: None,
            materialization_route: ArrayReceiverMaterializationRoute::PublicArrayBoxFallback,
            confidence: RepresentationConfidence::High,
        }
    }

    #[inline]
    pub const fn is_fallback_only(&self) -> bool {
        matches!(
            self.array_repr,
            ArrayReceiverArrayRepr::PublicArrayBoxFallback
        ) && self.object_storage_plan_ref.is_none()
            && self.direct_array_access_plan_ref.is_none()
    }

    #[inline]
    pub const fn proves_direct_storage(&self) -> bool {
        self.array_repr.proves_direct_storage() || self.object_storage_plan_ref.is_some()
    }

    #[inline]
    pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool {
        false
    }

    pub fn constructor_handoff(&self) -> ArrayReceiverConstructorHandoff {
        let direct_storage_proof = self.proves_direct_storage();
        let kind = if direct_storage_proof {
            ArrayReceiverConstructorHandoffKind::DirectResidenceCandidate
        } else {
            ArrayReceiverConstructorHandoffKind::FallbackResidenceCandidate
        };

        ArrayReceiverConstructorHandoff {
            input: self.clone(),
            kind,
            materialization_route: self.materialization_route,
            direct_storage_proof,
            backend_bypass_authorized: self.authorizes_backend_direct_handle_bypass(),
        }
    }
}

impl ArrayReceiverConstructorHandoff {
    #[inline]
    pub const fn is_fallback_residence_candidate(&self) -> bool {
        matches!(
            self.kind,
            ArrayReceiverConstructorHandoffKind::FallbackResidenceCandidate
        )
    }

    #[inline]
    pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool {
        self.backend_bypass_authorized
    }
}

impl ArrayReceiverResidenceInputSource {
    #[inline]
    pub const fn proves_direct_storage(&self) -> bool {
        self.direct_storage_proof
    }

    #[inline]
    pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool {
        self.backend_bypass_authorized
    }
}

impl ArrayReceiverResidenceInput {
    pub fn from_input_source(source: &ArrayReceiverResidenceInputSource) -> Option<Self> {
        if !matches!(
            source.kind,
            ArrayReceiverResidenceInputSourceKind::PublicArrayBoxFallback
        ) {
            return None;
        }
        if !matches!(
            source.materialization_route,
            ArrayReceiverMaterializationRoute::PublicArrayBoxFallback
        ) {
            return None;
        }
        if source.direct_storage_proof || source.backend_bypass_authorized {
            return None;
        }

        Some(Self {
            receiver_site_id: None,
            route_kind: source.representation_source.route_kind,
            receiver_box_name: source.representation_source.receiver_box_name,
            direct_array_plan_available: source
                .representation_source
                .direct_array_access_plan_ref
                .is_some(),
            object_storage_plan_available: source
                .representation_source
                .object_storage_plan_ref
                .is_some(),
            array_repr_available: true,
            residence_candidate: ArrayReceiverResidenceCandidate::PublicArrayBoxFallback,
            escape_facts_available: false,
            host_handle_publication_before_read: true,
            materialization_route_candidate: source.materialization_route,
            direct_storage_proof: false,
            backend_bypass_authorized: false,
        })
    }

    #[inline]
    pub const fn proves_direct_storage(&self) -> bool {
        self.direct_storage_proof
    }

    #[inline]
    pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool {
        self.backend_bypass_authorized
    }
}

impl ArrayReceiverResidenceSourceConstructor {
    pub fn construct(
        handoff: &ArrayReceiverConstructorHandoff,
    ) -> Option<ArrayReceiverResidenceInputSource> {
        if !handoff.is_fallback_residence_candidate() {
            return None;
        }
        if !matches!(
            handoff.materialization_route,
            ArrayReceiverMaterializationRoute::PublicArrayBoxFallback
        ) {
            return None;
        }
        if handoff.direct_storage_proof || handoff.backend_bypass_authorized {
            return None;
        }

        Some(ArrayReceiverResidenceInputSource {
            kind: ArrayReceiverResidenceInputSourceKind::PublicArrayBoxFallback,
            representation_source: handoff.input.clone(),
            materialization_route: handoff.materialization_route,
            direct_storage_proof: false,
            backend_bypass_authorized: false,
        })
    }
}

impl ArrayReceiverResidenceProofChain {
    pub fn construct_input_source_from_representation_source(
        source: &ArrayReceiverRepresentationSource,
    ) -> Option<ArrayReceiverResidenceInputSource> {
        let handoff = source.constructor_handoff();
        ArrayReceiverResidenceSourceConstructor::construct(&handoff)
    }
}

pub fn array_receiver_representation_source_report_fields(
) -> &'static [(&'static str, &'static str)] {
    &[
        (
            "output_contract",
            "hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-implementation-v0",
        ),
        ("array_receiver_representation_source_defined", "1"),
        ("representation_source_owner", "ArrayRepr"),
        (
            "representation_source_output",
            "ArrayReceiverRepresentationSource",
        ),
        (
            "representation_source_consumed_by",
            "ArrayReceiverResidenceSourceConstructor",
        ),
        ("representation_source_route_kind", "array_slot_len"),
        ("representation_source_receiver_box_name", "ArrayBox"),
        (
            "representation_source_array_repr",
            "PublicArrayBoxFallback",
        ),
        ("representation_source_object_storage_plan_ref", "none"),
        ("representation_source_direct_array_access_plan_ref", "none"),
        (
            "representation_source_materialization_route",
            "public_arraybox_fallback",
        ),
        ("representation_source_confidence", "high"),
        ("representation_source_is_fallback_only", "1"),
        ("representation_source_proves_direct_storage", "0"),
        ("backend_direct_handle_bypass_enabled", "0"),
        ("product_default_changed", "0"),
        ("mirbuilder_object_management_enabled", "0"),
    ]
}

pub fn array_receiver_constructor_handoff_report_fields() -> &'static [(&'static str, &'static str)]
{
    &[
        (
            "output_contract",
            "hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-implementation-v0",
        ),
        (
            "handoff_input",
            "ArrayReceiverRepresentationSource",
        ),
        (
            "handoff_consumer",
            "ArrayReceiverResidenceSourceConstructor",
        ),
        (
            "handoff_output_kind",
            "fallback_residence_candidate",
        ),
        ("handoff_input_array_repr", "PublicArrayBoxFallback"),
        ("handoff_input_is_fallback_only", "1"),
        ("handoff_output_direct_storage_proof", "0"),
        ("handoff_output_backend_bypass_authorized", "0"),
        (
            "handoff_materialization_route",
            "public_arraybox_fallback",
        ),
        ("handoff_preserves_public_arraybox_fallback", "1"),
        ("source_connected_to_constructor", "0"),
        ("source_consumed_by_backend", "0"),
        ("backend_direct_handle_bypass_enabled", "0"),
        ("product_default_changed", "0"),
        ("mirbuilder_object_management_enabled", "0"),
    ]
}

pub fn array_receiver_residence_source_constructor_report_fields(
) -> &'static [(&'static str, &'static str)] {
    &[
        (
            "output_contract",
            "hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-implementation-v0",
        ),
        ("constructor_input", "ArrayReceiverConstructorHandoff"),
        (
            "constructor_output",
            "ArrayReceiverResidenceInputSource|none",
        ),
        ("constructor_mode", "fallback_only"),
        ("constructor_accepts_fallback_residence_candidate", "1"),
        ("constructor_accepts_direct_residence_candidate", "0"),
        ("constructor_preserves_public_arraybox_fallback", "1"),
        ("constructor_output_direct_storage_proof", "0"),
        ("constructor_output_backend_bypass_authorized", "0"),
        ("source_connected_to_constructor", "1"),
        ("source_exported_to_mir_json", "0"),
        ("source_consumed_by_backend", "0"),
        ("backend_direct_handle_bypass_enabled", "0"),
        ("product_default_changed", "0"),
        ("mirbuilder_object_management_enabled", "0"),
    ]
}

pub fn array_receiver_residence_proof_chain_report_fields(
) -> &'static [(&'static str, &'static str)] {
    &[
        (
            "output_contract",
            "hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-implementation-v0",
        ),
        (
            "developer_facing_entry",
            "ArrayReceiverResidenceProofChain",
        ),
        ("facade_input", "ArrayReceiverRepresentationSource"),
        (
            "facade_output",
            "ArrayReceiverResidenceInputSource|none",
        ),
        (
            "facade_first_method",
            "construct_input_source_from_representation_source",
        ),
        ("facade_keeps_constructor_handoff_compat", "1"),
        ("facade_preserves_stage_reports", "1"),
        ("facade_preserves_stop_lines", "1"),
        ("facade_adds_direct_proof_power", "0"),
        ("facade_exports_to_mir_json", "0"),
        ("facade_consumed_by_backend", "0"),
        ("facade_accepts_public_arraybox_fallback", "1"),
        ("facade_accepts_direct_storage_source", "0"),
        ("facade_output_direct_storage_proof", "0"),
        ("facade_output_backend_bypass_authorized", "0"),
        ("backend_direct_handle_bypass_enabled", "0"),
        ("product_default_changed", "0"),
        ("mirbuilder_object_management_enabled", "0"),
    ]
}

pub fn array_receiver_residence_input_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        (
            "output_contract",
            "hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-implementation-v0",
        ),
        ("consumer_input", "ArrayReceiverResidenceInputSource"),
        (
            "consumer_input_entry",
            "ArrayReceiverResidenceProofChain",
        ),
        ("consumer_output", "ArrayReceiverResidenceInput|none"),
        ("consumer_mode", "fallback_only"),
        ("consumer_accepts_public_arraybox_fallback", "1"),
        ("consumer_accepts_direct_storage_source", "0"),
        ("input_field_receiver_site_id", "none"),
        ("input_field_route_kind", "array_slot_len"),
        ("input_field_receiver_box_name", "ArrayBox"),
        ("input_field_direct_array_plan_available", "0"),
        ("input_field_object_storage_plan_available", "0"),
        ("input_field_array_repr_available", "1"),
        (
            "input_field_residence_candidate",
            "public_arraybox_fallback",
        ),
        ("input_field_escape_facts_available", "0"),
        ("input_field_host_handle_publication_before_read", "1"),
        (
            "input_field_materialization_route_candidate",
            "public_arraybox_fallback",
        ),
        ("input_field_direct_storage_proof", "0"),
        ("input_field_backend_bypass_authorized", "0"),
        ("input_public_handle_reinterpretation", "0"),
        ("input_backend_raw_layout_inference", "0"),
        ("input_helper_name_inference", "0"),
        ("input_mirbuilder_owner", "0"),
        ("input_exported_to_mir_json", "0"),
        ("input_consumed_by_backend", "0"),
        ("backend_direct_handle_bypass_enabled", "0"),
        ("product_default_changed", "0"),
        ("mirbuilder_object_management_enabled", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_arraybox_fallback_source_is_not_direct_storage_proof() {
        let source = ArrayReceiverRepresentationSource::public_arraybox_fallback();

        assert_eq!(source.owner.as_str(), "ArrayRepr");
        assert_eq!(source.route_kind, "array_slot_len");
        assert_eq!(source.receiver_box_name, "ArrayBox");
        assert_eq!(source.array_repr.as_str(), "PublicArrayBoxFallback");
        assert_eq!(
            source.materialization_route.as_str(),
            "public_arraybox_fallback"
        );
        assert_eq!(source.confidence.as_str(), "high");
        assert!(source.is_fallback_only());
        assert!(!source.proves_direct_storage());
        assert!(!source.authorizes_backend_direct_handle_bypass());
    }

    #[test]
    fn report_fields_pin_passive_fallback_only_contract() {
        let fields = array_receiver_representation_source_report_fields();
        assert!(fields.contains(&("array_receiver_representation_source_defined", "1")));
        assert!(fields.contains(&("representation_source_array_repr", "PublicArrayBoxFallback")));
        assert!(fields.contains(&("representation_source_is_fallback_only", "1")));
        assert!(fields.contains(&("representation_source_proves_direct_storage", "0")));
        assert!(fields.contains(&("backend_direct_handle_bypass_enabled", "0")));
        assert!(fields.contains(&("mirbuilder_object_management_enabled", "0")));
    }

    #[test]
    fn fallback_source_handoff_does_not_authorize_direct_bypass() {
        let source = ArrayReceiverRepresentationSource::public_arraybox_fallback();
        let handoff = source.constructor_handoff();

        assert_eq!(handoff.kind.as_str(), "fallback_residence_candidate");
        assert!(handoff.is_fallback_residence_candidate());
        assert_eq!(
            handoff.materialization_route.as_str(),
            "public_arraybox_fallback"
        );
        assert!(!handoff.direct_storage_proof);
        assert!(!handoff.authorizes_backend_direct_handle_bypass());
        assert_eq!(handoff.input.array_repr.as_str(), "PublicArrayBoxFallback");
    }

    #[test]
    fn handoff_report_fields_pin_fallback_closed_contract() {
        let fields = array_receiver_constructor_handoff_report_fields();
        assert!(fields.contains(&("handoff_output_kind", "fallback_residence_candidate")));
        assert!(fields.contains(&("handoff_input_is_fallback_only", "1")));
        assert!(fields.contains(&("handoff_output_direct_storage_proof", "0")));
        assert!(fields.contains(&("handoff_output_backend_bypass_authorized", "0")));
        assert!(fields.contains(&("backend_direct_handle_bypass_enabled", "0")));
        assert!(fields.contains(&("mirbuilder_object_management_enabled", "0")));
    }

    #[test]
    fn constructor_consumes_fallback_handoff_without_direct_proof() {
        let handoff =
            ArrayReceiverRepresentationSource::public_arraybox_fallback().constructor_handoff();

        let input_source = ArrayReceiverResidenceSourceConstructor::construct(&handoff)
            .expect("fallback handoff should construct fallback input source");

        assert_eq!(
            input_source.kind,
            ArrayReceiverResidenceInputSourceKind::PublicArrayBoxFallback
        );
        assert_eq!(input_source.kind.as_str(), "public_arraybox_fallback");
        assert_eq!(
            input_source.materialization_route,
            ArrayReceiverMaterializationRoute::PublicArrayBoxFallback
        );
        assert!(!input_source.proves_direct_storage());
        assert!(!input_source.authorizes_backend_direct_handle_bypass());
    }

    #[test]
    fn constructor_rejects_direct_candidate_until_direct_row_exists() {
        let handoff = ArrayReceiverConstructorHandoff {
            input: ArrayReceiverRepresentationSource {
                owner: ArrayReceiverRepresentationOwner::ArrayRepr,
                route_kind: "array_slot_len",
                receiver_box_name: "ArrayBox",
                array_repr: ArrayReceiverArrayRepr::DirectI64,
                object_storage_plan_ref: None,
                direct_array_access_plan_ref: Some(1),
                materialization_route: ArrayReceiverMaterializationRoute::Snapshot,
                confidence: RepresentationConfidence::Medium,
            },
            kind: ArrayReceiverConstructorHandoffKind::DirectResidenceCandidate,
            materialization_route: ArrayReceiverMaterializationRoute::Snapshot,
            direct_storage_proof: true,
            backend_bypass_authorized: false,
        };

        assert!(ArrayReceiverResidenceSourceConstructor::construct(&handoff).is_none());
    }

    #[test]
    fn constructor_report_fields_keep_backend_closed() {
        let fields = array_receiver_residence_source_constructor_report_fields();

        assert!(fields.contains(&("constructor_mode", "fallback_only")));
        assert!(fields.contains(&("constructor_accepts_fallback_residence_candidate", "1")));
        assert!(fields.contains(&("constructor_accepts_direct_residence_candidate", "0")));
        assert!(fields.contains(&("constructor_output_direct_storage_proof", "0")));
        assert!(fields.contains(&("constructor_output_backend_bypass_authorized", "0")));
        assert!(fields.contains(&("source_connected_to_constructor", "1")));
        assert!(fields.contains(&("source_consumed_by_backend", "0")));
        assert!(fields.contains(&("backend_direct_handle_bypass_enabled", "0")));
    }

    #[test]
    fn proof_chain_facade_constructs_fallback_input_source() {
        let source = ArrayReceiverRepresentationSource::public_arraybox_fallback();

        let input_source =
            ArrayReceiverResidenceProofChain::construct_input_source_from_representation_source(
                &source,
            )
            .expect("fallback source should construct fallback input source");

        assert_eq!(
            input_source.kind,
            ArrayReceiverResidenceInputSourceKind::PublicArrayBoxFallback
        );
        assert!(!input_source.proves_direct_storage());
        assert!(!input_source.authorizes_backend_direct_handle_bypass());
    }

    #[test]
    fn proof_chain_report_fields_keep_stage_gates_and_backend_closed() {
        let fields = array_receiver_residence_proof_chain_report_fields();

        assert!(fields.contains(&("developer_facing_entry", "ArrayReceiverResidenceProofChain")));
        assert!(fields.contains(&(
            "facade_first_method",
            "construct_input_source_from_representation_source"
        )));
        assert!(fields.contains(&("facade_keeps_constructor_handoff_compat", "1")));
        assert!(fields.contains(&("facade_preserves_stage_reports", "1")));
        assert!(fields.contains(&("facade_adds_direct_proof_power", "0")));
        assert!(fields.contains(&("facade_consumed_by_backend", "0")));
        assert!(fields.contains(&("backend_direct_handle_bypass_enabled", "0")));
    }

    #[test]
    fn residence_input_constructs_fallback_without_direct_proof() {
        let source = ArrayReceiverRepresentationSource::public_arraybox_fallback();
        let input_source =
            ArrayReceiverResidenceProofChain::construct_input_source_from_representation_source(
                &source,
            )
            .expect("fallback source should construct input source");

        let input = ArrayReceiverResidenceInput::from_input_source(&input_source)
            .expect("fallback input source should construct residence input");

        assert_eq!(input.receiver_site_id, None);
        assert_eq!(input.route_kind, "array_slot_len");
        assert_eq!(input.receiver_box_name, "ArrayBox");
        assert!(!input.direct_array_plan_available);
        assert!(!input.object_storage_plan_available);
        assert!(input.array_repr_available);
        assert_eq!(
            input.residence_candidate.as_str(),
            "public_arraybox_fallback"
        );
        assert!(!input.escape_facts_available);
        assert!(input.host_handle_publication_before_read);
        assert_eq!(
            input.materialization_route_candidate.as_str(),
            "public_arraybox_fallback"
        );
        assert!(!input.proves_direct_storage());
        assert!(!input.authorizes_backend_direct_handle_bypass());
    }

    #[test]
    fn residence_input_report_fields_keep_backend_closed() {
        let fields = array_receiver_residence_input_report_fields();

        assert!(fields.contains(&("consumer_input", "ArrayReceiverResidenceInputSource")));
        assert!(fields.contains(&("consumer_input_entry", "ArrayReceiverResidenceProofChain")));
        assert!(fields.contains(&("consumer_output", "ArrayReceiverResidenceInput|none")));
        assert!(fields.contains(&(
            "input_field_residence_candidate",
            "public_arraybox_fallback"
        )));
        assert!(fields.contains(&("input_field_direct_storage_proof", "0")));
        assert!(fields.contains(&("input_field_backend_bypass_authorized", "0")));
        assert!(fields.contains(&("input_consumed_by_backend", "0")));
        assert!(fields.contains(&("backend_direct_handle_bypass_enabled", "0")));
        assert!(fields.contains(&("mirbuilder_object_management_enabled", "0")));
    }
}
