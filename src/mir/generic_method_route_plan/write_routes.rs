use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    classify_key_route, receiver_origin_box_name, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction};

use super::{
    generic_array_flow_origin_box_name, method_args_without_redundant_receiver,
    FieldHandleOriginMap, GenericMethodRoute, GenericMethodRouteDecision,
    GenericMethodRouteEvidence, GenericMethodRouteKind, GenericMethodRouteOperands,
    GenericMethodRouteProof, GenericMethodRouteSite, GenericMethodRouteSurface,
};

pub(super) fn match_generic_push_route(
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
    if method != "push" || !matches!(args.len(), 1 | 2) {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, *receiver)
        })
        .or_else(|| (box_name == "ArrayBox").then(|| "ArrayBox".to_string()));
    if receiver_origin_box.as_deref() != Some("ArrayBox")
        || !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox")
    {
        return None;
    }
    if args.len() == 2 {
        let receiver_arg_origin_box =
            receiver_origin_box_name(function, def_map, args[0]).or_else(|| {
                generic_array_flow_origin_box_name(function, def_map, field_handle_origins, args[0])
            });
        if receiver_arg_origin_box.as_deref() != Some("ArrayBox")
            || resolve_value_origin(function, def_map, args[0])
                != resolve_value_origin(function, def_map, *receiver)
        {
            return None;
        }
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
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
            Some(crate::mir::generic_method_route_facts::GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_generic_set_route(
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
    if method != "set" {
        return None;
    }
    let args = method_args_without_redundant_receiver(function, def_map, *receiver, args, 2)?;

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, *receiver)
        })
        .or_else(|| matches!(box_name.as_str(), "ArrayBox" | "MapBox").then(|| box_name.clone()));
    let (route_kind, core_op) = match (box_name.as_str(), receiver_origin_box.as_deref()) {
        ("ArrayBox", _) | ("RuntimeDataBox", Some("ArrayBox")) => (
            GenericMethodRouteKind::ArrayStoreAny,
            CoreMethodOp::ArraySet,
        ),
        ("MapBox", _) | ("RuntimeDataBox", Some("MapBox")) => {
            (GenericMethodRouteKind::MapStoreAny, CoreMethodOp::MapSet)
        }
        _ => return None,
    };
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

pub(super) fn match_generic_delete_route(
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
    if method != "delete" {
        return None;
    }
    let args = method_args_without_redundant_receiver(function, def_map, *receiver, args, 1)?;
    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| {
            generic_array_flow_origin_box_name(function, def_map, field_handle_origins, *receiver)
        })
        .or_else(|| (box_name == "MapBox").then(|| "MapBox".to_string()));
    if receiver_origin_box.as_deref() != Some("MapBox")
        || !matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox")
    {
        return None;
    }
    let key_route = classify_key_route(function, def_map, args[0]);

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::MapDeleteAny,
            GenericMethodRouteProof::DeleteSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapDelete,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::WriteAny,
            None,
        ),
    ))
}
