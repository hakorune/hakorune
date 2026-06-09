/*!
 * MIR-owned Map representation plans.
 *
 * This module records proof-bearing map representation families without
 * changing lowering behavior. The v0 slice only records the current generic
 * hash runtime surface derived from generic method routes; later slices may
 * promote fixed / enum / interned subsets into the same plan family.
 */

use super::generic_method_route_facts::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use super::generic_method_route_plan::GenericMethodRoute;
use super::{BasicBlockId, MirFunction, MirModule, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapReprKind {
    GenericHashRuntime,
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
    fn generic_hash_runtime(route: &GenericMethodRoute) -> Option<Self> {
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

pub fn refresh_module_map_repr_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_map_repr_plans(function);
    }
}

pub fn refresh_function_map_repr_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();

    for route in &function.metadata.generic_method_routes {
        if let Some(plan) = MapReprPlan::generic_hash_runtime(route) {
            plans.push(plan);
        }
    }

    plans.sort_by_key(|plan| (plan.block().as_u32(), plan.instruction_index()));
    function.metadata.map_repr_plans = plans;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{Callee, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType};

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn refresh_function_map_repr_plans_emits_generic_hash_runtime_rows() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(
            &mut function,
        );
        refresh_function_map_repr_plans(&mut function);

        assert_eq!(function.metadata.map_repr_plans.len(), 1);
        let plan = &function.metadata.map_repr_plans[0];
        assert_eq!(plan.route_id(), "map_repr.generic_hash_runtime");
        assert_eq!(plan.repr_kind_tag(), "generic_hash_runtime");
        assert_eq!(plan.source_route_id(), "generic_method.set");
        assert_eq!(plan.surface_box_name(), "MapBox");
        assert_eq!(plan.receiver_origin_box(), Some("MapBox"));
        assert_eq!(plan.method(), "set");
        assert_eq!(plan.receiver_value(), ValueId::new(1));
        assert_eq!(plan.key_route_tag(), Some("i64_const"));
        assert_eq!(plan.value_demand_tag(), "write_any");
        assert_eq!(plan.proof_tag(), "set_surface_policy");
    }
}
