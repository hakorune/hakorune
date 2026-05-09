use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::classify_key_route;
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};

use super::{
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodRoute,
    GenericMethodRouteDecision, GenericMethodRouteEvidence, GenericMethodRouteKind,
    GenericMethodRouteOperands, GenericMethodRouteProof, GenericMethodRouteSite,
    GenericMethodRouteSurface, GenericMethodValueDemand,
};

pub(super) fn match_mir_json_vid_array_item_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_vid_array_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            GenericMethodRouteProof::MirJsonVidArrayItem,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_mir_json_effects_array_item_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_effects_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            GenericMethodRouteProof::MirJsonEffectsArrayItem,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_mir_json_block_inst_array_item_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_block_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            GenericMethodRouteProof::MirJsonBlockInstArrayItem,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_mir_json_function_block_array_item_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_function_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            GenericMethodRouteProof::MirJsonFunctionBlockArrayItem,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_mir_json_module_function_array_item_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_module_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            GenericMethodRouteProof::MirJsonModuleFunctionArrayItem,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_mir_json_params_array_item_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_params_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            GenericMethodRouteProof::MirJsonParamsArrayItem,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

pub(super) fn match_mir_json_flags_rec_access_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    key: ValueId,
    result: ValueId,
) -> Option<GenericMethodRoute> {
    if function.signature.name != "MirJsonEmitBox._emit_flags_rec/4" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    let key_origin_get = value_origin_is_runtime_data_get(function, def_map, key);
    let (route_kind, core_op, lowering_tier) = if key_origin_get {
        (
            GenericMethodRouteKind::RuntimeDataLoadAny,
            CoreMethodOp::MapGet,
            CoreMethodLoweringTier::ColdFallback,
        )
    } else {
        (
            GenericMethodRouteKind::ArraySlotLoadAny,
            CoreMethodOp::ArrayGet,
            CoreMethodLoweringTier::WarmDirectAbi,
        )
    };

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::MirJsonFlagsRecAccess,
            Some(CoreMethodOpCarrier::manifest(core_op, lowering_tier)),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn value_origin_is_runtime_data_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    let Some(MirInstruction::Call {
        callee: Some(Callee::Method {
            box_name, method, ..
        }),
        ..
    }) = block.instructions.get(instruction_index)
    else {
        return false;
    };
    box_name == "RuntimeDataBox" && method == "get"
}
