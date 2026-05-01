/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use super::generic_method_route_facts::{
    classify_key_route, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};
use std::collections::BTreeSet;

mod map_set_scalar_proof;
mod model;

#[allow(unused_imports)]
pub(crate) use map_set_scalar_proof::ScalarI64MapGetStoreFact;
pub(crate) use map_set_scalar_proof::{
    instruction_may_escape_or_mutate_receiver, prove_scalar_i64_map_get_store_fact,
};
pub use model::GenericMethodRoute;
pub(crate) use model::{
    GenericMethodRouteDecision, GenericMethodRouteEvidence, GenericMethodRouteKind,
    GenericMethodRouteOperands, GenericMethodRouteProof, GenericMethodRouteSite,
    GenericMethodRouteSurface,
};

#[cfg(test)]
pub(crate) mod test_support;
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

#[cfg(test)]
mod tests;
