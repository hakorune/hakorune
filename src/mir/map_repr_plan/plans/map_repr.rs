use super::kind::MapReprKind;
use crate::mir::generic_method_route_facts::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use crate::mir::generic_method_route_plan::GenericMethodRoute;
use crate::mir::{BasicBlockId, ValueId};

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
    pub(in crate::mir::map_repr_plan) fn generic_hash_runtime(
        route: &GenericMethodRoute,
    ) -> Option<Self> {
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

    pub(in crate::mir::map_repr_plan) fn local_i64_key_map_shadow(
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
