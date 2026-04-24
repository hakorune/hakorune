/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::generic_method_route_facts::{
    classify_key_route, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::{
    build_value_def_map, BasicBlockId, Callee, CoreMethodLoweringTier, CoreMethodOp,
    CoreMethodOpCarrier, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteKind {
    RuntimeDataLoadAny,
    RuntimeDataContainsAny,
    MapContainsAny,
    MapContainsI64,
}

impl GenericMethodRouteKind {
    pub fn helper_symbol(self) -> &'static str {
        match self {
            Self::RuntimeDataLoadAny => "nyash.runtime_data.get_hh",
            Self::RuntimeDataContainsAny => "nyash.runtime_data.has_hh",
            Self::MapContainsAny => "nyash.map.probe_hh",
            Self::MapContainsI64 => "nyash.map.probe_hi",
        }
    }
}

impl std::fmt::Display for GenericMethodRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeDataLoadAny => f.write_str("runtime_data_load_any"),
            Self::RuntimeDataContainsAny => f.write_str("runtime_data_contains_any"),
            Self::MapContainsAny => f.write_str("map_contains_any"),
            Self::MapContainsI64 => f.write_str("map_contains_i64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteProof {
    GetSurfacePolicy,
    HasSurfacePolicy,
}

impl std::fmt::Display for GenericMethodRouteProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetSurfacePolicy => f.write_str("get_surface_policy"),
            Self::HasSurfacePolicy => f.write_str("has_surface_policy"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericMethodRoute {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub box_name: String,
    pub method: String,
    pub receiver_origin_box: Option<String>,
    pub key_route: GenericMethodKeyRoute,
    pub receiver_value: ValueId,
    pub key_value: ValueId,
    pub result_value: Option<ValueId>,
    pub route_kind: GenericMethodRouteKind,
    pub proof: GenericMethodRouteProof,
    pub core_method: Option<CoreMethodOpCarrier>,
    pub return_shape: Option<GenericMethodReturnShape>,
    pub value_demand: GenericMethodValueDemand,
    pub publication_policy: Option<GenericMethodPublicationPolicy>,
}

impl GenericMethodRoute {
    pub fn route_id(&self) -> &'static str {
        match self.method.as_str() {
            "get" => "generic_method.get",
            "has" => "generic_method.has",
            _ => "generic_method.unknown",
        }
    }

    pub fn emit_kind(&self) -> &'static str {
        match self.method.as_str() {
            "get" => "get",
            "has" => "has",
            _ => "unknown",
        }
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        match self.method.as_str() {
            "get" => &["read.key"],
            "has" => &["probe.key"],
            _ => &[],
        }
    }
}

pub fn refresh_module_generic_method_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_generic_method_routes(function);
    }
}

pub fn refresh_function_generic_method_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let def_map = build_value_def_map(function);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(route) =
                match_generic_has_route(function, &def_map, block_id, instruction_index, inst)
                    .or_else(|| {
                        match_generic_get_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
            {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.generic_method_routes = routes;
}

fn match_generic_has_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "has" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "MapBox").then(|| "MapBox".to_string()));
    let key_route = classify_key_route(function, def_map, args[0]);
    let (route_kind, core_method) = match box_name.as_str() {
        "MapBox" => (
            map_has_route_kind_for_key(key_route),
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
        "RuntimeDataBox"
            if receiver_origin_box.as_deref() == Some("MapBox")
                && key_route == GenericMethodKeyRoute::I64Const =>
        {
            (
                GenericMethodRouteKind::MapContainsI64,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapHas,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
            )
        }
        "ArrayBox" | "RuntimeDataBox" => (GenericMethodRouteKind::RuntimeDataContainsAny, None),
        _ => return None,
    };

    Some(GenericMethodRoute {
        block,
        instruction_index,
        box_name: box_name.clone(),
        method: method.clone(),
        receiver_origin_box,
        key_route,
        receiver_value: *receiver,
        key_value: args[0],
        result_value: *dst,
        route_kind,
        proof: GenericMethodRouteProof::HasSurfacePolicy,
        core_method,
        return_shape: None,
        value_demand: GenericMethodValueDemand::ReadRef,
        publication_policy: None,
    })
}

fn match_generic_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "get" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "MapBox").then(|| "MapBox".to_string()));
    if box_name != "RuntimeDataBox" || receiver_origin_box.as_deref() != Some("MapBox") {
        return None;
    }

    Some(GenericMethodRoute {
        block,
        instruction_index,
        box_name: box_name.clone(),
        method: method.clone(),
        receiver_origin_box,
        key_route: classify_key_route(function, def_map, args[0]),
        receiver_value: *receiver,
        key_value: args[0],
        result_value: *dst,
        route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
        proof: GenericMethodRouteProof::GetSurfacePolicy,
        core_method: Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapGet,
            CoreMethodLoweringTier::ColdFallback,
        )),
        return_shape: Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
        value_demand: GenericMethodValueDemand::RuntimeI64OrHandle,
        publication_policy: Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
    })
}

fn map_has_route_kind_for_key(key_route: GenericMethodKeyRoute) -> GenericMethodRouteKind {
    match key_route {
        GenericMethodKeyRoute::I64Const => GenericMethodRouteKind::MapContainsI64,
        GenericMethodKeyRoute::UnknownAny => GenericMethodRouteKind::MapContainsAny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

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
    fn detects_mapbox_has_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.block, BasicBlockId::new(0));
        assert_eq!(route.instruction_index, 0);
        assert_eq!(route.box_name, "MapBox");
        assert_eq!(route.method, "has");
        assert_eq!(route.receiver_origin_box.as_deref(), Some("MapBox"));
        assert_eq!(route.key_route, GenericMethodKeyRoute::UnknownAny);
        assert_eq!(route.receiver_value, ValueId::new(1));
        assert_eq!(route.key_value, ValueId::new(2));
        assert_eq!(route.result_value, Some(ValueId::new(3)));
        assert_eq!(route.route_kind, GenericMethodRouteKind::MapContainsAny);
        assert_eq!(route.proof, GenericMethodRouteProof::HasSurfacePolicy);
        let core_method = route.core_method.expect("MapBox.has core method op");
        assert_eq!(core_method.op, CoreMethodOp::MapHas);
        assert_eq!(
            core_method.proof.to_string(),
            "core_method_contract_manifest"
        );
        assert_eq!(core_method.lowering_tier.to_string(), "warm_direct_abi");
        assert_eq!(route.return_shape, None);
        assert_eq!(route.value_demand, GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy, None);
    }

    #[test]
    fn records_runtime_data_has_mapbox_receiver_origin_without_promotion() {
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
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 2, vec![3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name, "RuntimeDataBox");
        assert_eq!(route.receiver_origin_box.as_deref(), Some("MapBox"));
        assert_eq!(route.key_route, GenericMethodKeyRoute::UnknownAny);
        assert_eq!(
            route.route_kind,
            GenericMethodRouteKind::RuntimeDataContainsAny
        );
        assert!(route.core_method.is_none());
        assert_eq!(route.return_shape, None);
        assert_eq!(route.value_demand, GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy, None);
    }

    #[test]
    fn records_i64_const_key_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 3, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        assert_eq!(
            function.metadata.generic_method_routes[0].key_route,
            GenericMethodKeyRoute::I64Const
        );
    }

    #[test]
    fn promotes_runtime_data_mapbox_i64_has_to_map_contains_i64() {
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
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "has", 1, vec![3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name, "RuntimeDataBox");
        assert_eq!(route.receiver_origin_box.as_deref(), Some("MapBox"));
        assert_eq!(route.key_route, GenericMethodKeyRoute::I64Const);
        assert_eq!(route.route_kind, GenericMethodRouteKind::MapContainsI64);
        let core_method = route.core_method.expect("MapHas carrier");
        assert_eq!(core_method.op, CoreMethodOp::MapHas);
        assert_eq!(route.route_kind.helper_symbol(), "nyash.map.probe_hi");
        assert_eq!(route.return_shape, None);
        assert_eq!(route.publication_policy, None);
    }

    #[test]
    fn records_runtime_data_mapbox_get_as_cold_metadata_only() {
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
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name, "RuntimeDataBox");
        assert_eq!(route.method, "get");
        assert_eq!(route.receiver_origin_box.as_deref(), Some("MapBox"));
        assert_eq!(route.key_route, GenericMethodKeyRoute::I64Const);
        assert_eq!(route.route_kind, GenericMethodRouteKind::RuntimeDataLoadAny);
        assert_eq!(
            route.route_kind.helper_symbol(),
            "nyash.runtime_data.get_hh"
        );
        assert_eq!(route.proof, GenericMethodRouteProof::GetSurfacePolicy);
        let core_method = route.core_method.expect("MapGet carrier");
        assert_eq!(core_method.op, CoreMethodOp::MapGet);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::ColdFallback
        );
        assert_eq!(
            route.return_shape,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
        assert_eq!(
            route.value_demand,
            GenericMethodValueDemand::RuntimeI64OrHandle
        );
        assert_eq!(
            route.publication_policy,
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        );
    }

    #[test]
    fn detects_runtime_data_has_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "RuntimeDataBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        assert_eq!(
            function.metadata.generic_method_routes[0].route_kind,
            GenericMethodRouteKind::RuntimeDataContainsAny
        );
        assert!(function.metadata.generic_method_routes[0]
            .core_method
            .is_none());
        assert_eq!(
            function.metadata.generic_method_routes[0].return_shape,
            None
        );
        assert_eq!(
            function.metadata.generic_method_routes[0].publication_policy,
            None
        );
    }

    #[test]
    fn rejects_non_has_methods() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert!(function.metadata.generic_method_routes.is_empty());
    }
}
