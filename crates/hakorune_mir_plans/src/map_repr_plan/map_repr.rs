use super::kind::MapReprKind;
use hakorune_mir_core::{BasicBlockId, ValueId};

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
    key_route_tag: Option<&'static str>,
    return_shape_tag: Option<&'static str>,
    value_demand_tag: &'static str,
    publication_policy_tag: Option<&'static str>,
    proof_tag: &'static str,
    lowering_tier: Option<&'static str>,
}

impl MapReprPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
        key_route_tag: Option<&'static str>,
        return_shape_tag: Option<&'static str>,
        value_demand_tag: &'static str,
        publication_policy_tag: Option<&'static str>,
        proof_tag: &'static str,
        lowering_tier: Option<&'static str>,
    ) -> Self {
        Self {
            block,
            instruction_index,
            route_id,
            repr_kind,
            source_route_id,
            source_route_kind,
            source_helper_symbol,
            surface_box_name,
            receiver_origin_box,
            method,
            receiver_value,
            key_value,
            result_value,
            key_route_tag,
            return_shape_tag,
            value_demand_tag,
            publication_policy_tag,
            proof_tag,
            lowering_tier,
        }
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
        self.key_route_tag
    }

    pub fn return_shape_tag(&self) -> Option<&'static str> {
        self.return_shape_tag
    }

    pub fn value_demand_tag(&self) -> &'static str {
        self.value_demand_tag
    }

    pub fn publication_policy_tag(&self) -> Option<&'static str> {
        self.publication_policy_tag
    }

    pub fn proof_tag(&self) -> &'static str {
        self.proof_tag
    }

    pub fn lowering_tier_tag(&self) -> Option<&'static str> {
        self.lowering_tier
    }
}
