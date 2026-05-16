/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use super::generic_method_route_facts::{
    receiver_origin_box_name, GenericMethodKeyRoute, GenericMethodPublicationPolicy,
    GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::string_corridor::StringCorridorOp;
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{BasicBlockId, Callee, MirFunction, MirInstruction, MirModule, ValueId};
use std::collections::BTreeMap;

mod collection_read_routes;
mod flow_origin;
mod map_set_scalar_proof;
mod mir_json_routes;
mod model;
mod origin_inference;
mod write_routes;

use collection_read_routes::{
    match_generic_get_route, match_generic_has_route, match_generic_keys_route,
    match_generic_len_route,
};
use flow_origin::{
    generic_array_flow_origin_box_name, generic_pure_string_value_origin_box_name,
    generic_runtime_data_contains_param_text_origin_box_name,
    generic_string_receiver_origin_box_name, string_corridor_method_origin_box_name,
};
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
use origin_inference::{
    infer_typed_object_collection_element_origins, infer_typed_object_field_handle_origins,
    typed_object_value_box_name,
};
use write_routes::{match_generic_delete_route, match_generic_push_route, match_generic_set_route};

#[cfg(test)]
pub(crate) mod test_support;

type FieldHandleOriginKey = (String, String);
type FieldHandleOriginMap = BTreeMap<FieldHandleOriginKey, String>;
type MethodParamBoxOriginKey = (String, usize);
type MethodParamBoxOriginMap = BTreeMap<MethodParamBoxOriginKey, BoxOriginInference>;
type CollectionElementOriginMap = BTreeMap<CollectionElementOriginKey, String>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CollectionElementOriginKey {
    Field(FieldHandleOriginKey),
    Local {
        function: String,
        receiver_origin: ValueId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoxOriginInference {
    Known(String),
    Conflict,
}

pub fn refresh_module_generic_method_routes(module: &mut MirModule) {
    let field_handle_origins = infer_typed_object_field_handle_origins(module);
    let collection_element_origins =
        infer_typed_object_collection_element_origins(module, &field_handle_origins);
    for function in module.functions.values_mut() {
        refresh_function_generic_method_routes_with_context(
            function,
            &field_handle_origins,
            &collection_element_origins,
        );
    }
}

pub fn refresh_function_generic_method_routes(function: &mut MirFunction) {
    refresh_function_generic_method_routes_with_context(
        function,
        &FieldHandleOriginMap::new(),
        &CollectionElementOriginMap::new(),
    );
}

fn refresh_function_generic_method_routes_with_context(
    function: &mut MirFunction,
    field_handle_origins: &FieldHandleOriginMap,
    collection_element_origins: &CollectionElementOriginMap,
) {
    let mut routes = Vec::new();
    let def_map = build_value_def_map(function);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(route) = match_generic_has_route(
                function,
                &def_map,
                field_handle_origins,
                block_id,
                instruction_index,
                inst,
            )
            .or_else(|| {
                match_generic_get_route(
                    function,
                    &def_map,
                    field_handle_origins,
                    collection_element_origins,
                    block_id,
                    instruction_index,
                    inst,
                )
            })
            .or_else(|| {
                match_generic_len_route(
                    function,
                    &def_map,
                    field_handle_origins,
                    block_id,
                    instruction_index,
                    inst,
                )
            })
            .or_else(|| {
                match_generic_keys_route(function, &def_map, block_id, instruction_index, inst)
            })
            .or_else(|| {
                match_generic_substring_route(function, &def_map, block_id, instruction_index, inst)
            })
            .or_else(|| {
                match_generic_indexof_route(function, &def_map, block_id, instruction_index, inst)
            })
            .or_else(|| {
                match_generic_lastindexof_route(
                    function,
                    &def_map,
                    block_id,
                    instruction_index,
                    inst,
                )
            })
            .or_else(|| {
                match_generic_contains_route(function, &def_map, block_id, instruction_index, inst)
            })
            .or_else(|| {
                match_generic_push_route(
                    function,
                    &def_map,
                    field_handle_origins,
                    block_id,
                    instruction_index,
                    inst,
                )
            })
            .or_else(|| {
                match_generic_set_route(
                    function,
                    &def_map,
                    field_handle_origins,
                    block_id,
                    instruction_index,
                    inst,
                )
            })
            .or_else(|| {
                match_generic_delete_route(
                    function,
                    &def_map,
                    field_handle_origins,
                    block_id,
                    instruction_index,
                    inst,
                )
            }) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block().as_u32(), route.instruction_index()));
    function.metadata.generic_method_routes = routes;
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
        .or_else(|| {
            string_corridor_method_origin_box_name(function, *dst, StringCorridorOp::StrSlice)
        })
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
    if method != "indexOf" || !matches!(args.len(), 1 | 2) {
        return None;
    }

    let receiver_origin_box =
        generic_string_receiver_origin_box_name(function, def_map, *receiver, box_name);
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

fn match_generic_lastindexof_route(
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
    if method != "lastIndexOf" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box =
        generic_string_receiver_origin_box_name(function, def_map, *receiver, box_name);
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
            GenericMethodRouteKind::StringLastIndexOf,
            GenericMethodRouteProof::LastIndexOfSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::StringLastIndexOf,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_generic_contains_route(
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
    if method != "contains" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = generic_string_receiver_origin_box_name(
        function, def_map, *receiver, box_name,
    )
    .or_else(|| {
        generic_runtime_data_contains_param_text_origin_box_name(
            function, def_map, box_name, *receiver, args[0],
        )
    });
    if box_name != "StringBox"
        && !(box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("StringBox"))
    {
        return None;
    }
    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        GenericMethodRouteEvidence::new(receiver_origin_box, None),
        GenericMethodRouteOperands::new(*receiver, Some(args[0]), *dst),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::StringContains,
            GenericMethodRouteProof::ContainsSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::StringContains,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn method_args_without_redundant_receiver<'a>(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    args: &'a [ValueId],
    semantic_arity: usize,
) -> Option<&'a [ValueId]> {
    if args.len() == semantic_arity {
        return Some(args);
    }
    if args.len() != semantic_arity + 1 {
        return None;
    }
    let receiver_origin = resolve_value_origin(function, def_map, receiver);
    let arg_receiver_origin = resolve_value_origin(function, def_map, args[0]);
    (receiver_origin == arg_receiver_origin).then_some(&args[1..])
}

#[cfg(test)]
mod tests;
