/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

#[cfg(test)]
use super::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use super::generic_method_route_facts::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
#[cfg(test)]
use super::{BasicBlockId, Callee, MirInstruction};
use super::{MirFunction, MirModule, ValueId};
use std::collections::BTreeMap;

mod collection_read_routes;
mod flow_origin;
mod map_set_scalar_proof;
mod mir_json_routes;
mod model;
mod origin_inference;
mod string_routes;
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
use string_routes::{
    match_generic_contains_route, match_generic_indexof_route, match_generic_lastindexof_route,
    match_generic_substring_route,
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
