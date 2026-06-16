use super::super::generic_method_route_facts::{
    const_i64_value, GenericMethodKeyRoute, GenericMethodPublicationPolicy,
    GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::super::generic_method_route_plan::GenericMethodRoute;
use super::super::value_origin::ValueDefMap;
use super::super::{BasicBlockId, MirFunction, ValueId};
use super::candidates::LocalI64MapShadowCandidate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapReprKind {
    GenericHashRuntime,
    LocalI64KeyMapShadow,
    FixedStatic,
    FixedSmallLinear,
    FixedOpenAddress,
    EnumKeyDense,
    InternedKeyHash,
    InternedKeyFixed,
}

impl MapReprKind {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::GenericHashRuntime => "generic_hash_runtime",
            Self::LocalI64KeyMapShadow => "local_i64_key_map_shadow",
            Self::FixedStatic => "fixed_static",
            Self::FixedSmallLinear => "fixed_small_linear",
            Self::FixedOpenAddress => "fixed_open_address",
            Self::EnumKeyDense => "enum_key_dense",
            Self::InternedKeyHash => "interned_key_hash",
            Self::InternedKeyFixed => "interned_key_fixed",
        }
    }
}

impl std::fmt::Display for MapReprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapReprPlan {
    block: BasicBlockId,
    instruction_index: usize,
    route_id: &'static str,
    repr_kind: MapReprKind,
    source_route_id: &'static str,
    source_route_kind: &'static str,
    source_helper_symbol: &'static str,
    surface_box_name: String,
    receiver_origin_box: Option<String>,
    method: String,
    receiver_value: ValueId,
    key_value: Option<ValueId>,
    result_value: Option<ValueId>,
    key_route: Option<GenericMethodKeyRoute>,
    return_shape: Option<GenericMethodReturnShape>,
    value_demand: GenericMethodValueDemand,
    publication_policy: Option<GenericMethodPublicationPolicy>,
    proof_tag: &'static str,
    lowering_tier: Option<&'static str>,
}

impl MapReprPlan {
    pub(super) fn generic_hash_runtime(route: &GenericMethodRoute) -> Option<Self> {
        let receiver_origin_box = route.receiver_origin_box();
        if receiver_origin_box != Some("MapBox") {
            return None;
        }
        Some(Self {
            block: route.block(),
            instruction_index: route.instruction_index(),
            route_id: "map_repr.generic_hash_runtime",
            repr_kind: MapReprKind::GenericHashRuntime,
            source_route_id: route.route_id(),
            source_route_kind: route.route_kind_tag(),
            source_helper_symbol: route.helper_symbol(),
            surface_box_name: route.box_name().to_string(),
            receiver_origin_box: receiver_origin_box.map(str::to_string),
            method: route.method().to_string(),
            receiver_value: route.receiver_value(),
            key_value: route.key_value(),
            result_value: route.result_value(),
            key_route: route.key_route(),
            return_shape: route.return_shape(),
            value_demand: route.value_demand(),
            publication_policy: route.publication_policy(),
            proof_tag: route.proof_tag(),
            lowering_tier: route.lowering_tier().map(|tier| tier.as_json_name()),
        })
    }

    pub(super) fn local_i64_key_map_shadow(
        route: &GenericMethodRoute,
        receiver_value: ValueId,
    ) -> Option<Self> {
        let receiver_origin_box = route.receiver_origin_box();
        if receiver_origin_box != Some("MapBox") {
            return None;
        }
        Some(Self {
            block: route.block(),
            instruction_index: route.instruction_index(),
            route_id: "map_repr.local_i64_key_map_shadow",
            repr_kind: MapReprKind::LocalI64KeyMapShadow,
            source_route_id: route.route_id(),
            source_route_kind: route.route_kind_tag(),
            source_helper_symbol: route.helper_symbol(),
            surface_box_name: route.box_name().to_string(),
            receiver_origin_box: receiver_origin_box.map(str::to_string),
            method: route.method().to_string(),
            receiver_value,
            key_value: route.key_value(),
            result_value: route.result_value(),
            key_route: route.key_route(),
            return_shape: route.return_shape(),
            value_demand: route.value_demand(),
            publication_policy: route.publication_policy(),
            proof_tag: "local_i64_key_map_shadow",
            lowering_tier: None,
        })
    }

    pub fn route_id(&self) -> &'static str {
        self.route_id
    }

    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn repr_kind_tag(&self) -> &'static str {
        self.repr_kind.as_metadata_name()
    }

    pub fn source_route_id(&self) -> &'static str {
        self.source_route_id
    }

    pub fn source_route_kind(&self) -> &'static str {
        self.source_route_kind
    }

    pub fn source_helper_symbol(&self) -> &'static str {
        self.source_helper_symbol
    }

    pub fn surface_box_name(&self) -> &str {
        self.surface_box_name.as_str()
    }

    pub fn receiver_origin_box(&self) -> Option<&str> {
        self.receiver_origin_box.as_deref()
    }

    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn key_value(&self) -> Option<ValueId> {
        self.key_value
    }

    pub fn result_value(&self) -> Option<ValueId> {
        self.result_value
    }

    pub fn key_route_tag(&self) -> Option<&'static str> {
        self.key_route.map(|route| route.as_metadata_name())
    }

    pub fn return_shape_tag(&self) -> Option<&'static str> {
        self.return_shape.map(|shape| shape.as_metadata_name())
    }

    pub fn value_demand_tag(&self) -> &'static str {
        self.value_demand.as_metadata_name()
    }

    pub fn publication_policy_tag(&self) -> Option<&'static str> {
        self.publication_policy
            .map(|policy| policy.as_metadata_name())
    }

    pub fn proof_tag(&self) -> &'static str {
        self.proof_tag
    }

    pub fn lowering_tier_tag(&self) -> Option<&'static str> {
        self.lowering_tier
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalMapStorageRealizationPlan {
    receiver_value: ValueId,
    representation: &'static str,
    candidate_set_count: usize,
    candidate_scalar_get_count: usize,
    publication_materialization_required: bool,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalMapStorageRealizationPlan {
    pub(super) fn local_i64_key_map(
        receiver_value: ValueId,
        candidate: &LocalI64MapShadowCandidate,
    ) -> Self {
        Self {
            receiver_value,
            representation: "local_i64_key_map",
            candidate_set_count: candidate.i64_set_count,
            candidate_scalar_get_count: candidate.scalar_get_count,
            publication_materialization_required: true,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn representation(&self) -> &'static str {
        self.representation
    }

    pub fn candidate_set_count(&self) -> usize {
        self.candidate_set_count
    }

    pub fn candidate_scalar_get_count(&self) -> usize {
        self.candidate_scalar_get_count
    }

    pub fn publication_materialization_required(&self) -> bool {
        self.publication_materialization_required
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalI64MapDirectStoragePlan {
    receiver_value: ValueId,
    representation: &'static str,
    known_i64_key_set_count: usize,
    scalar_get_count: usize,
    entry_value_tracking_enabled: bool,
    publication_materialization_required: bool,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalI64MapDirectStoragePlan {
    pub(super) fn closed_world_i64_key_value_table(
        receiver_value: ValueId,
        candidate: &LocalI64MapShadowCandidate,
    ) -> Self {
        Self {
            receiver_value,
            representation: "closed_world_i64_key_value_table",
            known_i64_key_set_count: candidate.i64_set_count,
            scalar_get_count: candidate.scalar_get_count,
            entry_value_tracking_enabled: false,
            publication_materialization_required: true,
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn representation(&self) -> &'static str {
        self.representation
    }

    pub fn known_i64_key_set_count(&self) -> usize {
        self.known_i64_key_set_count
    }

    pub fn scalar_get_count(&self) -> usize {
        self.scalar_get_count
    }

    pub fn entry_value_tracking_enabled(&self) -> bool {
        self.entry_value_tracking_enabled
    }

    pub fn publication_materialization_required(&self) -> bool {
        self.publication_materialization_required
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalI64MapEntryValueTrackingPlan {
    receiver_value: ValueId,
    set_block: BasicBlockId,
    set_instruction_index: usize,
    key_value: ValueId,
    value_value: ValueId,
    key_const_if_known: Option<i64>,
    value_const_if_known: Option<i64>,
    backend_lowering_enabled: bool,
    runtime_helper_enabled: bool,
}

impl LocalI64MapEntryValueTrackingPlan {
    pub(super) fn from_set_site(
        function: &MirFunction,
        def_map: &ValueDefMap,
        route: &GenericMethodRoute,
        receiver_value: ValueId,
        key_value: ValueId,
        value_value: ValueId,
    ) -> Self {
        Self {
            receiver_value,
            set_block: route.block(),
            set_instruction_index: route.instruction_index(),
            key_value,
            value_value,
            key_const_if_known: const_i64_value(function, def_map, key_value),
            value_const_if_known: const_i64_value(function, def_map, value_value),
            backend_lowering_enabled: false,
            runtime_helper_enabled: false,
        }
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn set_block(&self) -> BasicBlockId {
        self.set_block
    }

    pub fn set_instruction_index(&self) -> usize {
        self.set_instruction_index
    }

    pub fn key_value(&self) -> ValueId {
        self.key_value
    }

    pub fn value_value(&self) -> ValueId {
        self.value_value
    }

    pub fn key_const_if_known(&self) -> Option<i64> {
        self.key_const_if_known
    }

    pub fn value_const_if_known(&self) -> Option<i64> {
        self.value_const_if_known
    }

    pub fn backend_lowering_enabled(&self) -> bool {
        self.backend_lowering_enabled
    }

    pub fn runtime_helper_enabled(&self) -> bool {
        self.runtime_helper_enabled
    }
}
