/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::generic_method_route_facts::{
    classify_key_route, receiver_origin_box_name, GenericMethodKeyRoute, GenericMethodValueDemand,
};
use super::{
    build_value_def_map, BasicBlockId, Callee, CoreMethodLoweringTier, CoreMethodOp,
    CoreMethodOpCarrier, MirFunction, MirInstruction, MirModule, ValueDefMap, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteKind {
    RuntimeDataContainsAny,
    MapContainsAny,
}

impl GenericMethodRouteKind {
    pub fn helper_symbol(self) -> &'static str {
        match self {
            Self::RuntimeDataContainsAny => "nyash.runtime_data.has_hh",
            Self::MapContainsAny => "nyash.map.probe_hh",
        }
    }
}

impl std::fmt::Display for GenericMethodRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeDataContainsAny => f.write_str("runtime_data_contains_any"),
            Self::MapContainsAny => f.write_str("map_contains_any"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteProof {
    HasSurfacePolicy,
}

impl std::fmt::Display for GenericMethodRouteProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
    pub value_demand: GenericMethodValueDemand,
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
            GenericMethodRouteKind::MapContainsAny,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
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
        value_demand: GenericMethodValueDemand::ReadRef,
    })
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
        assert_eq!(route.value_demand, GenericMethodValueDemand::ReadRef);
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
        assert_eq!(route.value_demand, GenericMethodValueDemand::ReadRef);
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
