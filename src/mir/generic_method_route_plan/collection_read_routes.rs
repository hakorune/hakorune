use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    classify_key_route, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};

use super::origin_inference::generic_collection_element_origin_box_name;
use super::{
    generic_array_flow_origin_box_name, generic_pure_string_value_origin_box_name,
    method_args_without_redundant_receiver, mir_json_routes, prove_scalar_i64_map_get_store_fact,
    string_corridor_method_origin_box_name, CollectionElementOriginMap, FieldHandleOriginMap,
    GenericMethodRoute, GenericMethodRouteDecision, GenericMethodRouteEvidence,
    GenericMethodRouteKind, GenericMethodRouteOperands, GenericMethodRouteProof,
    GenericMethodRouteSite, GenericMethodRouteSurface,
};

pub(super) fn match_generic_has_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
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
    if method != "has" {
        return None;
    }
    let args = method_args_without_redundant_receiver(function, def_map, *receiver, args, 1)?;

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, *receiver)
        })
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

pub(super) fn match_generic_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
    collection_element_origins: &CollectionElementOriginMap,
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
    if method != "get" {
        return None;
    }
    let args = method_args_without_redundant_receiver(function, def_map, *receiver, args, 1)?;

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, *receiver)
        })
        .or_else(|| matches!(box_name.as_str(), "ArrayBox" | "MapBox").then(|| box_name.clone()));
    let result_origin_box = generic_collection_element_origin_box_name(
        function,
        def_map,
        collection_element_origins,
        *receiver,
    );
    let key_route = classify_key_route(function, def_map, args[0]);
    if let Some(result) = *dst {
        if let Some(route) = mir_json_routes::match_mir_json_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            *receiver,
            args[0],
            result,
        ) {
            return Some(route);
        }
    }

    if box_name == "ArrayBox" && receiver_origin_box.as_deref() == Some("ArrayBox") {
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route))
                .with_result_origin_box(result_origin_box),
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
        let typed_object_result =
            map_get_result_origin_requires_runtime_data_load(result_origin_box.as_deref());
        let (route_kind, lowering_tier, return_shape, value_demand, publication_policy) =
            if typed_object_result {
                (
                    GenericMethodRouteKind::RuntimeDataLoadAny,
                    CoreMethodLoweringTier::ColdFallback,
                    Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
                    GenericMethodValueDemand::RuntimeI64OrHandle,
                    Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
                )
            } else {
                (
                    GenericMethodRouteKind::MapLoadAny,
                    CoreMethodLoweringTier::WarmDirectAbi,
                    None,
                    GenericMethodValueDemand::ReadRef,
                    None,
                )
            };
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route))
                .with_result_origin_box(result_origin_box),
            GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
            GenericMethodRouteDecision::new(
                route_kind,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapGet,
                    lowering_tier,
                )),
                return_shape,
                value_demand,
                publication_policy,
            ),
        ));
    }

    if box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("ArrayBox") {
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route))
                .with_result_origin_box(result_origin_box),
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
        GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route))
            .with_result_origin_box(result_origin_box),
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

fn map_get_result_origin_requires_runtime_data_load(result_origin_box: Option<&str>) -> bool {
    match result_origin_box {
        Some("StringBox" | "ArrayBox" | "MapBox") | None => false,
        Some(_) => true,
    }
}

pub(super) fn match_generic_len_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
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
    if !is_len_method(method)
        || !(args.is_empty()
            || generic_len_self_arg_is_supported(
                function,
                def_map,
                field_handle_origins,
                box_name,
                args,
            ))
    {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, *receiver)
        })
        .or_else(|| generic_pure_string_value_origin_box_name(function, def_map, *receiver))
        .or_else(|| {
            string_corridor_method_origin_box_name(function, *dst, StringCorridorOp::StrLen)
        })
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
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
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

pub(super) fn match_generic_keys_route(
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
    if method != "keys" {
        return None;
    }
    let args = method_args_without_redundant_receiver(function, def_map, *receiver, args, 0)?;
    if function.signature.name != "MirJsonEmitBox._emit_flags/1" || box_name != "RuntimeDataBox" {
        let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
            .or_else(|| (box_name == "MapBox").then(|| box_name.clone()));
        if !matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox")
            || receiver_origin_box.as_deref() != Some("MapBox")
        {
            return None;
        }
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
            GenericMethodRouteEvidence::new(receiver_origin_box, None)
                .with_result_origin_box(Some("ArrayBox".to_string())),
            GenericMethodRouteOperands::new(*receiver, None, *dst),
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::MapKeysArray,
                GenericMethodRouteProof::KeysSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapKeys,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
                GenericMethodValueDemand::RuntimeI64OrHandle,
                Some(GenericMethodPublicationPolicy::NoPublication),
            ),
        ));
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
        GenericMethodRouteEvidence::new(None, None),
        GenericMethodRouteOperands::new(*receiver, None, *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::MapKeysArray,
            GenericMethodRouteProof::MirJsonFlagsKeys,
            None,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn generic_len_self_arg_is_supported(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
    box_name: &str,
    args: &[ValueId],
) -> bool {
    if args.len() != 1 {
        return false;
    }

    match box_name {
        "StringBox" => {
            generic_pure_string_value_origin_box_name(function, def_map, args[0]).as_deref()
                == Some("StringBox")
        }
        "ArrayBox" => {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, args[0])
                .as_deref()
                == Some("ArrayBox")
        }
        _ => false,
    }
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

fn map_has_route_kind_for_key(key_route: GenericMethodKeyRoute) -> GenericMethodRouteKind {
    match key_route {
        GenericMethodKeyRoute::I64Const => GenericMethodRouteKind::MapContainsI64,
        GenericMethodKeyRoute::UnknownAny => GenericMethodRouteKind::MapContainsAny,
    }
}
