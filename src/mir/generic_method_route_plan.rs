/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use super::generic_method_route_facts::{
    classify_key_route, const_i64_value, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};
use crate::mir::verification::utils::compute_dominators;
use std::collections::BTreeSet;

mod model;

pub use model::GenericMethodRoute;
pub(crate) use model::{
    GenericMethodRouteDecision, GenericMethodRouteEvidence, GenericMethodRouteKind,
    GenericMethodRouteOperands, GenericMethodRouteProof, GenericMethodRouteSite,
    GenericMethodRouteSurface,
};

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    fn site(block: u32, instruction_index: usize) -> GenericMethodRouteSite {
        GenericMethodRouteSite::new(BasicBlockId::new(block), instruction_index)
    }

    fn evidence(
        receiver_origin_box: Option<&str>,
        key_route: Option<GenericMethodKeyRoute>,
    ) -> GenericMethodRouteEvidence {
        GenericMethodRouteEvidence::new(receiver_origin_box.map(str::to_string), key_route)
    }

    fn operands(receiver: u32, key: Option<u32>, result: u32) -> GenericMethodRouteOperands {
        GenericMethodRouteOperands::new(
            ValueId::new(receiver),
            key.map(ValueId::new),
            Some(ValueId::new(result)),
        )
    }

    fn decision(
        route_kind: GenericMethodRouteKind,
        proof: GenericMethodRouteProof,
        core_op: CoreMethodOp,
        lowering_tier: CoreMethodLoweringTier,
        return_shape: Option<GenericMethodReturnShape>,
        value_demand: GenericMethodValueDemand,
        publication_policy: Option<GenericMethodPublicationPolicy>,
    ) -> GenericMethodRouteDecision {
        GenericMethodRouteDecision::new(
            route_kind,
            proof,
            Some(CoreMethodOpCarrier::manifest(core_op, lowering_tier)),
            return_shape,
            value_demand,
            publication_policy,
        )
    }

    struct FixtureSpec<'a> {
        block: u32,
        instruction_index: usize,
        box_name: &'a str,
        method: &'a str,
        arity: usize,
        receiver_origin_box: Option<&'a str>,
        key_route: Option<GenericMethodKeyRoute>,
        receiver: u32,
        key: Option<u32>,
        result: u32,
        route_kind: GenericMethodRouteKind,
        proof: GenericMethodRouteProof,
        core_op: CoreMethodOp,
        lowering_tier: CoreMethodLoweringTier,
        return_shape: Option<GenericMethodReturnShape>,
        value_demand: GenericMethodValueDemand,
        publication_policy: Option<GenericMethodPublicationPolicy>,
    }

    fn route(spec: FixtureSpec<'_>) -> GenericMethodRoute {
        GenericMethodRoute::new(
            site(spec.block, spec.instruction_index),
            GenericMethodRouteSurface::new(spec.box_name, spec.method, spec.arity),
            evidence(spec.receiver_origin_box, spec.key_route),
            operands(spec.receiver, spec.key, spec.result),
            decision(
                spec.route_kind,
                spec.proof,
                spec.core_op,
                spec.lowering_tier,
                spec.return_shape,
                spec.value_demand,
                spec.publication_policy,
            ),
        )
    }

    pub(crate) fn map_contains_i64(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "MapBox",
            method: "has",
            arity: 1,
            receiver_origin_box: Some("MapBox"),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver,
            key: Some(key),
            result,
            route_kind: GenericMethodRouteKind::MapContainsI64,
            proof: GenericMethodRouteProof::HasSurfacePolicy,
            core_op: CoreMethodOp::MapHas,
            lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
            return_shape: None,
            value_demand: GenericMethodValueDemand::ReadRef,
            publication_policy: None,
        })
    }

    pub(crate) fn runtime_data_map_get_mixed_i64_key(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "RuntimeDataBox",
            method: "get",
            arity: 1,
            receiver_origin_box: Some("MapBox"),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver,
            key: Some(key),
            result,
            route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
            proof: GenericMethodRouteProof::GetSurfacePolicy,
            core_op: CoreMethodOp::MapGet,
            lowering_tier: CoreMethodLoweringTier::ColdFallback,
            return_shape: Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            value_demand: GenericMethodValueDemand::RuntimeI64OrHandle,
            publication_policy: Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
        })
    }

    pub(crate) fn runtime_data_map_get_scalar_i64_same_key(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "RuntimeDataBox",
            method: "get",
            arity: 1,
            receiver_origin_box: Some("MapBox"),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver,
            key: Some(key),
            result,
            route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
            proof: GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
            core_op: CoreMethodOp::MapGet,
            lowering_tier: CoreMethodLoweringTier::ColdFallback,
            return_shape: Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            value_demand: GenericMethodValueDemand::ScalarI64,
            publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
        })
    }

    pub(crate) fn string_substring(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "StringBox",
            method: "substring",
            arity: 2,
            receiver_origin_box: Some("StringBox"),
            key_route: None,
            receiver,
            key: None,
            result,
            route_kind: GenericMethodRouteKind::StringSubstring,
            proof: GenericMethodRouteProof::SubstringSurfacePolicy,
            core_op: CoreMethodOp::StringSubstring,
            lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
            return_shape: None,
            value_demand: GenericMethodValueDemand::ReadRef,
            publication_policy: None,
        })
    }

    pub(crate) fn map_len(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "MapBox",
            method: "size",
            arity: 0,
            receiver_origin_box: Some("MapBox"),
            key_route: None,
            receiver,
            key: None,
            result,
            route_kind: GenericMethodRouteKind::MapEntryCount,
            proof: GenericMethodRouteProof::LenSurfacePolicy,
            core_op: CoreMethodOp::MapLen,
            lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
            return_shape: Some(GenericMethodReturnShape::ScalarI64),
            value_demand: GenericMethodValueDemand::ScalarI64,
            publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
        })
    }

    pub(crate) fn array_push(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "ArrayBox",
            method: "push",
            arity: 1,
            receiver_origin_box: Some("ArrayBox"),
            key_route: None,
            receiver,
            key: None,
            result,
            route_kind: GenericMethodRouteKind::ArrayAppendAny,
            proof: GenericMethodRouteProof::PushSurfacePolicy,
            core_op: CoreMethodOp::ArrayPush,
            lowering_tier: CoreMethodLoweringTier::ColdFallback,
            return_shape: Some(GenericMethodReturnShape::ScalarI64),
            value_demand: GenericMethodValueDemand::WriteAny,
            publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
        })
    }

    pub(crate) fn map_set_i64_key(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "MapBox",
            method: "set",
            arity: 2,
            receiver_origin_box: Some("MapBox"),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver,
            key: Some(key),
            result,
            route_kind: GenericMethodRouteKind::MapStoreAny,
            proof: GenericMethodRouteProof::SetSurfacePolicy,
            core_op: CoreMethodOp::MapSet,
            lowering_tier: CoreMethodLoweringTier::ColdFallback,
            return_shape: None,
            value_demand: GenericMethodValueDemand::WriteAny,
            publication_policy: None,
        })
    }

    pub(crate) fn map_get_unknown_key(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "MapBox",
            method: "get",
            arity: 1,
            receiver_origin_box: Some("MapBox"),
            key_route: Some(GenericMethodKeyRoute::UnknownAny),
            receiver,
            key: Some(key),
            result,
            route_kind: GenericMethodRouteKind::MapLoadAny,
            proof: GenericMethodRouteProof::GetSurfacePolicy,
            core_op: CoreMethodOp::MapGet,
            lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
            return_shape: None,
            value_demand: GenericMethodValueDemand::ReadRef,
            publication_policy: None,
        })
    }

    pub(crate) fn array_get_i64_key(
        block: u32,
        instruction_index: usize,
        receiver: u32,
        key: u32,
        result: u32,
    ) -> GenericMethodRoute {
        route(FixtureSpec {
            block,
            instruction_index,
            box_name: "ArrayBox",
            method: "get",
            arity: 1,
            receiver_origin_box: Some("ArrayBox"),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver,
            key: Some(key),
            result,
            route_kind: GenericMethodRouteKind::ArraySlotLoadAny,
            proof: GenericMethodRouteProof::GetSurfacePolicy,
            core_op: CoreMethodOp::ArrayGet,
            lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
            return_shape: None,
            value_demand: GenericMethodValueDemand::ReadRef,
            publication_policy: None,
        })
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
                    .or_else(|| {
                        match_generic_len_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_substring_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_indexof_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_push_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_set_route(
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

    routes.sort_by_key(|route| (route.block().as_u32(), route.instruction_index()));
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
        .or_else(|| matches!(box_name.as_str(), "ArrayBox" | "MapBox").then(|| box_name.clone()));
    let key_route = classify_key_route(function, def_map, args[0]);
    let (route_kind, core_method) = match box_name.as_str() {
        "ArrayBox" if receiver_origin_box.as_deref() == Some("ArrayBox") => (
            GenericMethodRouteKind::ArrayContainsAny,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
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
        "RuntimeDataBox" if receiver_origin_box.as_deref() == Some("ArrayBox") => (
            GenericMethodRouteKind::ArrayContainsAny,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
        "RuntimeDataBox" => (GenericMethodRouteKind::RuntimeDataContainsAny, None),
        _ => return None,
    };

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
        GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::HasSurfacePolicy,
            core_method,
            None,
            GenericMethodValueDemand::ReadRef,
            None,
        ),
    ))
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
        .or_else(|| matches!(box_name.as_str(), "ArrayBox" | "MapBox").then(|| box_name.clone()));
    let key_route = classify_key_route(function, def_map, args[0]);

    if box_name == "ArrayBox" && receiver_origin_box.as_deref() == Some("ArrayBox") {
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
            GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArraySlotLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::ArrayGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        ));
    }

    if box_name == "MapBox" && receiver_origin_box.as_deref() == Some("MapBox") {
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
            GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::MapLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        ));
    }

    if box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("ArrayBox") {
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
            GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArraySlotLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::ArrayGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        ));
    }

    if box_name != "RuntimeDataBox" || receiver_origin_box.as_deref() != Some("MapBox") {
        return None;
    }

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

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            proof,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            return_shape,
            value_demand,
            publication_policy,
        ),
    ))
}

fn match_generic_len_route(
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
    if !is_len_method(method) || !args.is_empty() {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| generic_pure_string_value_origin_box_name(function, def_map, *receiver))
        .or_else(|| len_surface_origin_box_name(box_name).map(str::to_string));
    let (route_kind, core_op) =
        match len_surface_origin_box_name(box_name).or(receiver_origin_box.as_deref()) {
            Some("MapBox") => (GenericMethodRouteKind::MapEntryCount, CoreMethodOp::MapLen),
            Some("ArrayBox") => (GenericMethodRouteKind::ArraySlotLen, CoreMethodOp::ArrayLen),
            Some("StringBox") => (GenericMethodRouteKind::StringLen, CoreMethodOp::StringLen),
            _ => return None,
        };

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 0),
        GenericMethodRouteEvidence::new(receiver_origin_box, None),
        GenericMethodRouteOperands::new(*receiver, None, *dst),
        GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::LenSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                core_op,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_generic_substring_route(
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
    if method != "substring" || !(args.len() == 1 || args.len() == 2) {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| generic_pure_string_value_origin_box_name(function, def_map, *receiver))
        .or_else(|| (box_name == "StringBox").then(|| "StringBox".to_string()));
    if box_name != "StringBox"
        && !(box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("StringBox"))
    {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
        GenericMethodRouteEvidence::new(receiver_origin_box, None),
        GenericMethodRouteOperands::new(*receiver, None, *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::StringSubstring,
            GenericMethodRouteProof::SubstringSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::StringSubstring,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            None,
            GenericMethodValueDemand::ReadRef,
            None,
        ),
    ))
}

fn match_generic_indexof_route(
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
    if method != "indexOf" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "StringBox").then(|| "StringBox".to_string()));
    if box_name != "StringBox"
        && !(box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("StringBox"))
    {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
        GenericMethodRouteEvidence::new(receiver_origin_box, None),
        GenericMethodRouteOperands::new(*receiver, None, *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::StringIndexOf,
            GenericMethodRouteProof::IndexOfSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::StringIndexOf,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_generic_push_route(
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
    if method != "push" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "ArrayBox").then(|| "ArrayBox".to_string()));
    if receiver_origin_box.as_deref() != Some("ArrayBox")
        || !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox")
    {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        GenericMethodRouteEvidence::new(receiver_origin_box, None),
        GenericMethodRouteOperands::new(*receiver, None, *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArrayAppendAny,
            GenericMethodRouteProof::PushSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayPush,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::WriteAny,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_generic_set_route(
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
    if method != "set" || args.len() != 2 {
        return None;
    }

    let (route_kind, core_op) = match box_name.as_str() {
        "ArrayBox" => (
            GenericMethodRouteKind::ArrayStoreAny,
            CoreMethodOp::ArraySet,
        ),
        "MapBox" => (GenericMethodRouteKind::MapStoreAny, CoreMethodOp::MapSet),
        _ => return None,
    };
    let receiver_origin_box =
        receiver_origin_box_name(function, def_map, *receiver).or_else(|| Some(box_name.clone()));
    let key_route = classify_key_route(function, def_map, args[0]);

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 2),
        GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
        GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::SetSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                core_op,
                CoreMethodLoweringTier::ColdFallback,
            )),
            None,
            GenericMethodValueDemand::WriteAny,
            None,
        ),
    ))
}

fn is_len_method(method: &str) -> bool {
    matches!(method, "len" | "length" | "size")
}

fn len_surface_origin_box_name(box_name: &str) -> Option<&'static str> {
    match box_name {
        "MapBox" => Some("MapBox"),
        "ArrayBox" => Some("ArrayBox"),
        "StringBox" => Some("StringBox"),
        _ => None,
    }
}

fn generic_pure_string_global_call_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, receiver);
    let (block, instruction_index) = def_map.get(&origin).copied()?;
    function
        .metadata
        .global_call_routes
        .iter()
        .any(|route| {
            route.block() == block
                && route.instruction_index() == instruction_index
                && route.result_value() == Some(origin)
                && matches!(
                    route.target_shape(),
                    Some("generic_pure_string_body" | "generic_string_or_void_sentinel_body")
                )
        })
        .then(|| "StringBox".to_string())
}

fn generic_pure_string_value_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    generic_pure_string_global_call_origin_box_name(function, def_map, receiver)
        .or_else(|| generic_pure_string_flow_origin_box_name(function, receiver))
}

fn generic_pure_string_flow_origin_box_name(
    function: &MirFunction,
    receiver: ValueId,
) -> Option<String> {
    let mut string_values = BTreeSet::<ValueId>::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, inst) in block.instructions.iter().enumerate() {
                if generic_pure_string_flow_marks_instruction(
                    function,
                    &mut string_values,
                    *block_id,
                    instruction_index,
                    inst,
                ) {
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    string_values
        .contains(&receiver)
        .then(|| "StringBox".to_string())
}

fn generic_pure_string_flow_marks_instruction(
    function: &MirFunction,
    string_values: &mut BTreeSet<ValueId>,
    block_id: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> bool {
    let mark = |string_values: &mut BTreeSet<ValueId>, value| string_values.insert(value);
    match inst {
        MirInstruction::Const {
            dst,
            value: ConstValue::String(_),
        } => mark(string_values, *dst),
        MirInstruction::NewBox { dst, box_type, .. } if box_type == "StringBox" => {
            mark(string_values, *dst)
        }
        MirInstruction::Copy { dst, src } if string_values.contains(src) => {
            mark(string_values, *dst)
        }
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
            ..
        } if string_values.contains(lhs) || string_values.contains(rhs) => {
            mark(string_values, *dst)
        }
        MirInstruction::Phi { dst, inputs, .. }
            if !inputs.is_empty()
                && inputs
                    .iter()
                    .all(|(_, value)| string_values.contains(value)) =>
        {
            mark(string_values, *dst)
        }
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(_)),
            ..
        } if function.metadata.global_call_routes.iter().any(|route| {
            route.block() == block_id
                && route.instruction_index() == instruction_index
                && route.result_value() == Some(*dst)
                && matches!(
                    route.target_shape(),
                    Some("generic_pure_string_body" | "generic_string_or_void_sentinel_body")
                )
        }) =>
        {
            mark(string_values, *dst)
        }
        _ => false,
    }
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
mod tests;
