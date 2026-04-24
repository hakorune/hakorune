/*!
 * MIR-owned MapGet/MapHas same-key fusion preflight metadata.
 *
 * This module only derives metadata from already-owned generic method routes.
 * It does not lower, call backend helpers, or classify method strings for
 * codegen. Later lowering may consume these records as a typed plan.
 */

use super::generic_method_route_facts::const_i64_value;
use super::generic_method_route_plan::{
    instruction_may_escape_or_mutate_receiver, prove_scalar_i64_map_get_store_fact,
};
use super::{
    build_value_def_map, resolve_value_origin, BasicBlockId, CoreMethodLoweringTier, CoreMethodOp,
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodRoute, GenericMethodValueDemand, MirFunction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapLookupFusionOp {
    MapLookupSameKey,
}

impl std::fmt::Display for MapLookupFusionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MapLookupSameKey => f.write_str("MapLookupSameKey"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapLookupFusionProof {
    SameReceiverSameI64KeyScalarGetHas,
}

impl std::fmt::Display for MapLookupFusionProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SameReceiverSameI64KeyScalarGetHas => {
                f.write_str("same_receiver_same_i64_key_scalar_get_has")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapLookupStoredValueProof {
    UnknownScalar,
    ScalarI64Const,
    ScalarI64NonZero,
}

impl std::fmt::Display for MapLookupStoredValueProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownScalar => f.write_str("unknown_scalar"),
            Self::ScalarI64Const => f.write_str("scalar_i64_const"),
            Self::ScalarI64NonZero => f.write_str("scalar_i64_nonzero"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapLookupFusionRoute {
    pub block: BasicBlockId,
    pub get_instruction_index: usize,
    pub has_instruction_index: usize,
    pub fusion_op: MapLookupFusionOp,
    pub receiver_origin_box: Option<String>,
    pub receiver_value: ValueId,
    pub key_value: ValueId,
    pub key_const: i64,
    pub key_route: GenericMethodKeyRoute,
    pub get_result_value: ValueId,
    pub has_result_value: ValueId,
    pub get_return_shape: GenericMethodReturnShape,
    pub get_value_demand: GenericMethodValueDemand,
    pub get_publication_policy: GenericMethodPublicationPolicy,
    pub has_result_shape: &'static str,
    pub stored_value_proof: MapLookupStoredValueProof,
    pub stored_value_const: Option<i64>,
    pub stored_value_known_nonzero: Option<bool>,
    pub proof: MapLookupFusionProof,
    pub lowering_tier: CoreMethodLoweringTier,
}

impl MapLookupFusionRoute {
    pub fn route_id(&self) -> &'static str {
        "map_lookup.same_key"
    }
}

pub fn refresh_module_map_lookup_fusion_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_map_lookup_fusion_routes(function);
    }
}

pub fn refresh_function_map_lookup_fusion_routes(function: &mut MirFunction) {
    let def_map = build_value_def_map(function);
    let mut routes = Vec::new();

    for get_route in function
        .metadata
        .generic_method_routes
        .iter()
        .filter(|route| is_scalar_map_get_route(route))
    {
        for has_route in function
            .metadata
            .generic_method_routes
            .iter()
            .filter(|route| is_i64_map_has_route(route))
        {
            if let Some(route) =
                match_same_key_get_has_pair(function, &def_map, get_route, has_route)
            {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.get_instruction_index));
    function.metadata.map_lookup_fusion_routes = routes;
}

fn is_scalar_map_get_route(route: &GenericMethodRoute) -> bool {
    route
        .core_method
        .is_some_and(|carrier| carrier.op == CoreMethodOp::MapGet)
        && route.receiver_origin_box.as_deref() == Some("MapBox")
        && route.key_route == Some(GenericMethodKeyRoute::I64Const)
        && route.return_shape == Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
        && route.value_demand == GenericMethodValueDemand::ScalarI64
        && route.publication_policy == Some(GenericMethodPublicationPolicy::NoPublication)
}

fn is_i64_map_has_route(route: &GenericMethodRoute) -> bool {
    route
        .core_method
        .is_some_and(|carrier| carrier.op == CoreMethodOp::MapHas)
        && route.receiver_origin_box.as_deref() == Some("MapBox")
        && route.key_route == Some(GenericMethodKeyRoute::I64Const)
}

fn match_same_key_get_has_pair(
    function: &MirFunction,
    def_map: &ValueDefMap,
    get_route: &GenericMethodRoute,
    has_route: &GenericMethodRoute,
) -> Option<MapLookupFusionRoute> {
    if get_route.block != has_route.block
        || get_route.instruction_index >= has_route.instruction_index
    {
        return None;
    }
    let get_result_value = get_route.result_value?;
    let has_result_value = has_route.result_value?;
    let get_key_value = get_route.key_value?;
    let has_key_value = has_route.key_value?;
    let get_key_route = get_route.key_route?;
    let receiver_root = resolve_value_origin(function, def_map, get_route.receiver_value);
    if receiver_root != resolve_value_origin(function, def_map, has_route.receiver_value) {
        return None;
    }
    let key_const = const_i64_value(function, def_map, get_key_value)?;
    if Some(key_const) != const_i64_value(function, def_map, has_key_value) {
        return None;
    }
    if !same_block_window_keeps_receiver_stable(
        function,
        def_map,
        get_route.block,
        get_route.instruction_index,
        has_route.instruction_index,
        receiver_root,
    ) {
        return None;
    }

    let stored_value = prove_scalar_i64_map_get_store_fact(
        function,
        def_map,
        get_route.block,
        get_route.instruction_index,
        get_route.receiver_value,
        get_key_value,
    )
    .map(|fact| fact.stored_value);
    let stored_value_proof = match stored_value {
        Some(value) if value != 0 => MapLookupStoredValueProof::ScalarI64NonZero,
        Some(_) => MapLookupStoredValueProof::ScalarI64Const,
        None => MapLookupStoredValueProof::UnknownScalar,
    };

    Some(MapLookupFusionRoute {
        block: get_route.block,
        get_instruction_index: get_route.instruction_index,
        has_instruction_index: has_route.instruction_index,
        fusion_op: MapLookupFusionOp::MapLookupSameKey,
        receiver_origin_box: get_route.receiver_origin_box.clone(),
        receiver_value: get_route.receiver_value,
        key_value: get_key_value,
        key_const,
        key_route: get_key_route,
        get_result_value,
        has_result_value,
        get_return_shape: GenericMethodReturnShape::ScalarI64OrMissingZero,
        get_value_demand: GenericMethodValueDemand::ScalarI64,
        get_publication_policy: GenericMethodPublicationPolicy::NoPublication,
        has_result_shape: "presence_bool",
        stored_value_proof,
        stored_value_const: stored_value,
        stored_value_known_nonzero: stored_value.map(|value| value != 0),
        proof: MapLookupFusionProof::SameReceiverSameI64KeyScalarGetHas,
        lowering_tier: CoreMethodLoweringTier::ColdFallback,
    })
}

fn same_block_window_keeps_receiver_stable(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    has_instruction_index: usize,
    receiver_root: ValueId,
) -> bool {
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    block
        .instructions
        .iter()
        .take(has_instruction_index)
        .skip(get_instruction_index + 1)
        .all(|inst| {
            !instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        Callee, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId,
    };

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
    fn detects_same_block_scalar_get_has_fusion_metadata() {
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
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
        block.add_instruction(method_call(Some(6), "RuntimeDataBox", "has", 1, vec![2]));

        crate::mir::refresh_function_generic_method_routes(&mut function);
        refresh_function_map_lookup_fusion_routes(&mut function);

        assert_eq!(function.metadata.map_lookup_fusion_routes.len(), 1);
        let route = &function.metadata.map_lookup_fusion_routes[0];
        assert_eq!(route.route_id(), "map_lookup.same_key");
        assert_eq!(route.fusion_op, MapLookupFusionOp::MapLookupSameKey);
        assert_eq!(route.block, BasicBlockId::new(0));
        assert_eq!(route.get_instruction_index, 4);
        assert_eq!(route.has_instruction_index, 5);
        assert_eq!(route.receiver_origin_box.as_deref(), Some("MapBox"));
        assert_eq!(route.receiver_value, ValueId::new(1));
        assert_eq!(route.key_value, ValueId::new(2));
        assert_eq!(route.key_const, -1);
        assert_eq!(route.key_route, GenericMethodKeyRoute::I64Const);
        assert_eq!(route.get_result_value, ValueId::new(5));
        assert_eq!(route.has_result_value, ValueId::new(6));
        assert_eq!(
            route.get_return_shape,
            GenericMethodReturnShape::ScalarI64OrMissingZero
        );
        assert_eq!(route.get_value_demand, GenericMethodValueDemand::ScalarI64);
        assert_eq!(
            route.get_publication_policy,
            GenericMethodPublicationPolicy::NoPublication
        );
        assert_eq!(route.has_result_shape, "presence_bool");
        assert_eq!(
            route.stored_value_proof,
            MapLookupStoredValueProof::ScalarI64NonZero
        );
        assert_eq!(route.stored_value_const, Some(7));
        assert_eq!(route.stored_value_known_nonzero, Some(true));
        assert_eq!(
            route.proof,
            MapLookupFusionProof::SameReceiverSameI64KeyScalarGetHas
        );
        assert_eq!(route.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    }

    #[test]
    fn rejects_same_block_fusion_when_has_uses_different_i64_key() {
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
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(2),
        });
        block.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(Some(6), "RuntimeDataBox", "get", 1, vec![2]));
        block.add_instruction(method_call(Some(7), "RuntimeDataBox", "has", 1, vec![4]));

        crate::mir::refresh_function_generic_method_routes(&mut function);
        refresh_function_map_lookup_fusion_routes(&mut function);

        assert!(function.metadata.map_lookup_fusion_routes.is_empty());
    }
}
