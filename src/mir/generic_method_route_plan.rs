/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::generic_method_route_facts::{
    classify_key_route, const_i64_value, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::{
    build_value_def_map, resolve_value_origin, BasicBlockId, Callee, CoreMethodLoweringTier,
    CoreMethodOp, CoreMethodOpCarrier, MirFunction, MirInstruction, MirModule, ValueDefMap,
    ValueId,
};
use crate::mir::verification::utils::compute_dominators;

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
    MapSetScalarI64DominatesNoEscape,
    MapSetScalarI64SameKeyNoEscape,
}

impl std::fmt::Display for GenericMethodRouteProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetSurfacePolicy => f.write_str("get_surface_policy"),
            Self::HasSurfacePolicy => f.write_str("has_surface_policy"),
            Self::MapSetScalarI64DominatesNoEscape => {
                f.write_str("map_set_scalar_i64_dominates_no_escape")
            }
            Self::MapSetScalarI64SameKeyNoEscape => {
                f.write_str("map_set_scalar_i64_same_key_no_escape")
            }
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

    let key_route = classify_key_route(function, def_map, args[0]);
    let scalar_proof = prove_scalar_i64_map_get_store_fact(
        function,
        def_map,
        block,
        instruction_index,
        *receiver,
        args[0],
    );
    let (proof, return_shape, value_demand, publication_policy) = if let Some(proof) = scalar_proof
    {
        (
            proof.route_proof,
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        )
    } else {
        (
            GenericMethodRouteProof::GetSurfacePolicy,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
        )
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
        route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
        proof,
        core_method: Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapGet,
            CoreMethodLoweringTier::ColdFallback,
        )),
        return_shape,
        value_demand,
        publication_policy,
    })
}

fn map_has_route_kind_for_key(key_route: GenericMethodKeyRoute) -> GenericMethodRouteKind {
    match key_route {
        GenericMethodKeyRoute::I64Const => GenericMethodRouteKind::MapContainsI64,
        GenericMethodKeyRoute::UnknownAny => GenericMethodRouteKind::MapContainsAny,
    }
}

#[derive(Clone, Copy)]
struct MapSetCallShape {
    receiver: ValueId,
    key: ValueId,
    value: ValueId,
}

#[derive(Clone, Copy)]
struct MapSetCandidate {
    block: BasicBlockId,
    instruction_index: usize,
    stored_value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScalarI64MapGetStoreFact {
    pub route_proof: GenericMethodRouteProof,
    pub stored_value: i64,
}

pub(crate) fn prove_scalar_i64_map_get_store_fact(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<ScalarI64MapGetStoreFact> {
    if let Some(stored_value) = prove_same_block_scalar_i64_map_get(
        function,
        def_map,
        block_id,
        get_instruction_index,
        get_receiver,
        get_key,
    ) {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
            stored_value,
        });
    }
    if let Some(stored_value) =
        prove_dominating_scalar_i64_map_get(function, def_map, block_id, get_receiver, get_key)
    {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape,
            stored_value,
        });
    }
    None
}

fn prove_same_block_scalar_i64_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<i64> {
    let Some(get_key_const) = const_i64_value(function, def_map, get_key) else {
        return None;
    };
    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let Some(block) = function.blocks.get(&block_id) else {
        return None;
    };

    for inst in block.instructions.iter().take(get_instruction_index).rev() {
        if let Some(set_call) = map_set_call_shape(inst) {
            if same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                let Some(set_key_const) = const_i64_value(function, def_map, set_call.key) else {
                    return None;
                };
                if set_key_const != get_key_const {
                    return None;
                }
                return const_i64_value(function, def_map, set_call.value);
            }
        }

        if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
            return None;
        }
    }

    None
}

fn prove_dominating_scalar_i64_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    get_block_id: BasicBlockId,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<i64> {
    let Some(get_key_const) = const_i64_value(function, def_map, get_key) else {
        return None;
    };
    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let dominators = compute_dominators(function);
    let mut candidates = Vec::new();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if block_id == get_block_id || !dominators.dominates(block_id, get_block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some(set_call) = map_set_call_shape(inst) else {
                continue;
            };
            if !same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                continue;
            }
            if const_i64_value(function, def_map, set_call.key) != Some(get_key_const) {
                continue;
            }
            let Some(stored_value) = const_i64_value(function, def_map, set_call.value) else {
                continue;
            };
            candidates.push(MapSetCandidate {
                block: block_id,
                instruction_index,
                stored_value,
            });
        }
    }

    candidates.into_iter().rev().find_map(|candidate| {
        dominating_candidate_has_no_same_receiver_escape(
            function,
            def_map,
            &dominators,
            candidate,
            receiver_root,
        )
        .then_some(candidate.stored_value)
    })
}

fn dominating_candidate_has_no_same_receiver_escape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    dominators: &crate::mir::verification::utils::DominatorTree,
    candidate: MapSetCandidate,
    receiver_root: ValueId,
) -> bool {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if !dominators.dominates(candidate.block, block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        let start = if block_id == candidate.block {
            candidate.instruction_index + 1
        } else {
            0
        };
        for inst in block.instructions.iter().skip(start) {
            if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
                return false;
            }
        }
    }
    true
}

fn map_set_call_shape(inst: &MirInstruction) -> Option<MapSetCallShape> {
    let MirInstruction::Call {
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
    if method != "set" || !matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox") {
        return None;
    }

    let (key, value) = match args.as_slice() {
        [key, value] => (*key, *value),
        // Some source routes still carry the receiver as the first argument.
        [_receiver_arg, key, value] => (*key, *value),
        _ => return None,
    };
    Some(MapSetCallShape {
        receiver: *receiver,
        key,
        value,
    })
}

pub(crate) fn instruction_may_escape_or_mutate_receiver(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    receiver_root: ValueId,
) -> bool {
    if !instruction_uses_origin(function, def_map, inst, receiver_root) {
        return false;
    }

    match inst {
        MirInstruction::Copy { .. } | MirInstruction::KeepAlive { .. } => false,
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            ..
        } if same_value_origin(function, def_map, *receiver, receiver_root)
            && matches!(method.as_str(), "get" | "has") =>
        {
            false
        }
        _ => true,
    }
}

fn instruction_uses_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    origin: ValueId,
) -> bool {
    inst.used_values()
        .into_iter()
        .any(|value| same_value_origin(function, def_map, value, origin))
}

fn same_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    origin: ValueId,
) -> bool {
    resolve_value_origin(function, def_map, value) == origin
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirType};

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
    fn proves_same_block_runtime_data_get_scalar_i64_return_shape() {
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
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![1, 2, 3]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name, "RuntimeDataBox");
        assert_eq!(route.method, "get");
        assert_eq!(route.key_route, GenericMethodKeyRoute::I64Const);
        assert_eq!(
            route.proof,
            GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape
        );
        assert_eq!(
            route.return_shape,
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
        );
        assert_eq!(route.value_demand, GenericMethodValueDemand::ScalarI64);
        assert_eq!(
            route.publication_policy,
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
        assert_eq!(
            route.route_kind.helper_symbol(),
            "nyash.runtime_data.get_hh"
        );
    }

    #[test]
    fn rejects_same_block_get_scalar_shape_when_store_value_is_not_i64() {
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
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "StringBox".to_string(),
            args: vec![],
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.proof, GenericMethodRouteProof::GetSurfacePolicy);
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
    fn rejects_same_block_get_scalar_shape_after_unknown_same_receiver_mutation() {
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
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(None, "MapBox", "clear", 1, vec![]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.proof, GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
        assert_eq!(
            route.publication_policy,
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        );
    }

    #[test]
    fn rejects_same_block_get_scalar_shape_after_different_key_same_receiver_set() {
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
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(4),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![4, 3]));
        block.add_instruction(method_call(Some(7), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.proof, GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
    }

    #[test]
    fn proves_dominating_preheader_scalar_i64_map_get_return_shape() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
        function.add_block(body);

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(
            route.proof,
            GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape
        );
        assert_eq!(
            route.return_shape,
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
        );
        assert_eq!(route.value_demand, GenericMethodValueDemand::ScalarI64);
        assert_eq!(
            route.publication_policy,
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
        assert_eq!(
            route.route_kind.helper_symbol(),
            "nyash.runtime_data.get_hh"
        );
    }

    #[test]
    fn rejects_dominating_preheader_scalar_shape_after_body_mutation() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
        body.add_instruction(method_call(None, "MapBox", "clear", 1, vec![]));
        function.add_block(body);

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.proof, GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
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
