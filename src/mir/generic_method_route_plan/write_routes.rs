use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    classify_key_route, receiver_origin_box_name, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{ArrayElementWriteKind, BasicBlockId, Callee, MirFunction, MirInstruction};

use super::{
    generic_array_flow_origin_box_name, method_args_without_redundant_receiver,
    scalar_known_hako_shadow, FieldHandleOriginMap, GenericMethodRoute, GenericMethodRouteDecision,
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
    if let MirInstruction::ArrayElementWrite {
        site_id,
        dst,
        kind: ArrayElementWriteKind::LiteralAppend | ArrayElementWriteKind::Push,
        receiver,
        index: None,
        value,
        ..
    } = inst
    {
        let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
            .or_else(|| {
                generic_array_flow_origin_box_name(
                    function,
                    def_map,
                    field_handle_origins,
                    *receiver,
                )
            })
            .unwrap_or_else(|| "ArrayBox".to_string());
        return Some(
            GenericMethodRoute::new(
                GenericMethodRouteSite::new(block, instruction_index),
                GenericMethodRouteSurface::new(receiver_origin_box.clone(), "push", 1),
                GenericMethodRouteEvidence::new(Some(receiver_origin_box), None)
                    .with_value_origin_box(receiver_origin_box_name(function, def_map, *value)),
                GenericMethodRouteOperands::new(*receiver, None, *dst),
                scalar_known_hako_shadow::write_push_hako_route_authority_pilot_decision(),
            )
            .with_array_write_site_id(*site_id),
        );
    }
    None
}

pub(super) fn match_generic_set_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    if let MirInstruction::ArrayElementWrite {
        site_id,
        dst,
        kind: ArrayElementWriteKind::Set,
        receiver,
        index: Some(index),
        value,
        ..
    } = inst
    {
        let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
            .or_else(|| {
                generic_array_flow_origin_box_name(
                    function,
                    def_map,
                    field_handle_origins,
                    *receiver,
                )
            })
            .unwrap_or_else(|| "ArrayBox".to_string());
        return Some(
            GenericMethodRoute::new(
                GenericMethodRouteSite::new(block, instruction_index),
                GenericMethodRouteSurface::new(receiver_origin_box.clone(), "set", 2),
                GenericMethodRouteEvidence::new(
                    Some(receiver_origin_box),
                    Some(classify_key_route(function, def_map, *index)),
                )
                .with_value_origin_box(receiver_origin_box_name(function, def_map, *value)),
                GenericMethodRouteOperands::new(*receiver, Some(*index), *dst),
                GenericMethodRouteDecision::new(
                    GenericMethodRouteKind::ArrayStoreAny,
                    GenericMethodRouteProof::SetSurfacePolicy,
                    Some(CoreMethodOpCarrier::manifest(
                        CoreMethodOp::ArraySet,
                        CoreMethodLoweringTier::ColdFallback,
                    )),
                    None,
                    GenericMethodValueDemand::WriteAny,
                    None,
                ),
            )
            .with_array_write_site_id(*site_id),
        );
    }
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
        .or_else(|| {
            matches!(box_name.as_str(), "DirectArrayI64" | "MapBox").then(|| box_name.clone())
        });
    let key_route = classify_key_route(function, def_map, args[0]);
    let (route_kind, core_op) = match (box_name.as_str(), receiver_origin_box.as_deref()) {
        ("DirectArrayI64", Some("DirectArrayI64")) => (
            GenericMethodRouteKind::ArrayStoreAny,
            CoreMethodOp::ArraySet,
        ),
        ("MapBox", _) | ("Box", _) | ("RuntimeDataBox", Some("MapBox")) => (
            if key_route.is_i64() {
                GenericMethodRouteKind::MapStoreI64
            } else {
                GenericMethodRouteKind::MapStoreAny
            },
            CoreMethodOp::MapSet,
        ),
        _ => return None,
    };

    let value_origin_box = receiver_origin_box_name(function, def_map, args[1]);

    let decision = match route_kind {
        GenericMethodRouteKind::MapStoreI64 => {
            scalar_known_hako_shadow::mapstore_i64_hako_route_authority_pilot_decision()
        }
        GenericMethodRouteKind::MapStoreAny => {
            scalar_known_hako_shadow::mapstore_any_hako_route_authority_pilot_decision()
        }
        _ => GenericMethodRouteDecision::new(
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
    };

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 2),
        GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route))
            .with_value_origin_box(value_origin_box),
        GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
        decision,
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
