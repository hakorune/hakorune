use std::collections::BTreeSet;

use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    classify_key_route, const_i64_value, const_string_value,
};
use crate::mir::value_origin::{resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};

use super::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodRoute, GenericMethodRouteDecision, GenericMethodRouteEvidence,
    GenericMethodRouteKind, GenericMethodRouteOperands, GenericMethodRouteProof,
    GenericMethodRouteSite, GenericMethodRouteSurface, GenericMethodValueDemand,
};

fn mir_json_inst_field_result_origin_box(key: &str) -> Option<String> {
    match key {
        "op" | "operation" | "op_kind" | "cmp" | "value" => Some("StringBox".to_string()),
        "args" | "effects" => Some("ArrayBox".to_string()),
        _ => None,
    }
}

fn mir_json_function_field_result_origin_box(key: &str) -> Option<String> {
    match key {
        "name" => Some("StringBox".to_string()),
        "params" | "blocks" => Some("ArrayBox".to_string()),
        "flags" => Some("MapBox".to_string()),
        _ => None,
    }
}

fn mir_json_module_field_result_origin_box(key: &str) -> Option<String> {
    match key {
        "functions" => Some("ArrayBox".to_string()),
        "functions_0" => Some("MapBox".to_string()),
        _ => None,
    }
}

pub(super) fn match_mir_json_get_route(
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
    match_mir_json_numeric_value_field_get_route(
        function,
        def_map,
        block,
        instruction_index,
        box_name,
        method,
        receiver,
        key,
        result,
    )
    .or_else(|| {
        match_mir_json_const_value_field_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_phi_incoming_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_callee_field_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_vid_array_item_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_effects_array_item_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_block_inst_array_item_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_function_block_array_item_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_module_function_array_item_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_params_array_item_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_flags_rec_access_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_block_field_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_function_field_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_module_field_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
    .or_else(|| {
        match_mir_json_inst_field_get_route(
            function,
            def_map,
            block,
            instruction_index,
            box_name,
            method,
            receiver,
            key,
            result,
        )
    })
}

fn match_mir_json_numeric_value_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox._expect_i64/2" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if key_text != "value" {
        return None;
    }
    if !value_reaches_stringhelpers_to_i64(function, def_map, result) {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonNumericValueField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_const_value_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox._emit_box_value/1" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if !matches!(key_text.as_str(), "type" | "value") {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonConstValueField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_phi_incoming_get_route(
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
    if function.signature.name != "MirJsonEmitBox._emit_phi_incoming_rec/3" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }

    let key_i64 = const_i64_value(function, def_map, key);
    let (proof, return_shape, value_demand) = if matches!(key_i64, Some(0 | 1)) {
        (
            GenericMethodRouteProof::MirJsonPhiIncomingPairScalar,
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
        )
    } else {
        (
            GenericMethodRouteProof::MirJsonPhiIncomingArrayItem,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
        )
    };

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(classify_key_route(function, def_map, key))),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArraySlotLoadAny,
            proof,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            return_shape,
            value_demand,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_callee_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox._emit_callee/1" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if !matches!(
        key_text.as_str(),
        "type" | "name" | "box_name" | "method" | "receiver" | "box_type"
    ) {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonCalleeField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_vid_array_item_get_route(
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

fn match_mir_json_effects_array_item_get_route(
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

fn match_mir_json_block_inst_array_item_get_route(
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

fn match_mir_json_function_block_array_item_get_route(
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

fn match_mir_json_module_function_array_item_get_route(
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

fn match_mir_json_params_array_item_get_route(
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

fn match_mir_json_flags_rec_access_get_route(
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

fn match_mir_json_block_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox._emit_block/1" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if !matches!(key_text.as_str(), "instructions" | "id") {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonBlockField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_function_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox._emit_function/1" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if !matches!(key_text.as_str(), "name" | "params" | "flags" | "blocks") {
        return None;
    }
    let result_origin_box = mir_json_function_field_result_origin_box(&key_text);

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text)
            .with_result_origin_box(result_origin_box),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonFunctionField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_module_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox.to_json/1" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if !matches!(key_text.as_str(), "functions" | "functions_0") {
        return None;
    }
    let result_origin_box = mir_json_module_field_result_origin_box(&key_text);

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text)
            .with_result_origin_box(result_origin_box),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonModuleField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_mir_json_inst_field_get_route(
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
    if function.signature.name != "MirJsonEmitBox._emit_inst/1" {
        return None;
    }
    if box_name != "RuntimeDataBox" || method != "get" {
        return None;
    }
    let key_text = const_string_value(function, def_map, key)?;
    if !matches!(
        key_text.as_str(),
        "op" | "dst"
            | "value"
            | "operation"
            | "op_kind"
            | "lhs"
            | "rhs"
            | "cmp"
            | "cond"
            | "then"
            | "else"
            | "target"
            | "incoming"
            | "values"
            | "mir_call"
            | "callee"
            | "args"
            | "effects"
            | "func"
            | "name"
    ) {
        return None;
    }
    let result_origin_box = mir_json_inst_field_result_origin_box(&key_text);

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.to_string(), method.to_string(), 1),
        GenericMethodRouteEvidence::new(None, Some(GenericMethodKeyRoute::UnknownAny))
            .with_key_const_text(key_text)
            .with_result_origin_box(result_origin_box),
        GenericMethodRouteOperands::new(receiver, Some(key), Some(result)),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonInstField,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn value_reaches_stringhelpers_to_i64(
    function: &MirFunction,
    def_map: &ValueDefMap,
    source: ValueId,
) -> bool {
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            if name == "StringHelpers.to_i64/1"
                && args.len() == 1
                && value_depends_on(function, def_map, args[0], source)
            {
                return true;
            }
        }
    }
    false
}

fn value_depends_on(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    source: ValueId,
) -> bool {
    fn visit(
        function: &MirFunction,
        def_map: &ValueDefMap,
        value: ValueId,
        source: ValueId,
        seen: &mut BTreeSet<ValueId>,
    ) -> bool {
        if value == source {
            return true;
        }
        if !seen.insert(value) {
            return false;
        }
        let Some((block_id, instruction_index)) = def_map.get(&value).copied() else {
            return false;
        };
        let Some(block) = function.blocks.get(&block_id) else {
            return false;
        };
        match block.instructions.get(instruction_index) {
            Some(MirInstruction::Copy { src, .. }) => visit(function, def_map, *src, source, seen),
            Some(MirInstruction::Phi { inputs, .. }) => inputs
                .iter()
                .any(|(_, input)| visit(function, def_map, *input, source, seen)),
            _ => false,
        }
    }

    visit(function, def_map, value, source, &mut BTreeSet::new())
}
