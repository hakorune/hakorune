/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use super::generic_method_route_facts::{
    classify_key_route, const_string_value, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::string_corridor::StringCorridorOp;
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType,
    ValueId,
};
use crate::mir::global_call_route_plan::GlobalCallRoute;
use std::collections::{BTreeMap, BTreeSet};

mod map_set_scalar_proof;
mod mir_json_routes;
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

type FieldHandleOriginKey = (String, String);
type FieldHandleOriginMap = BTreeMap<FieldHandleOriginKey, String>;
type MethodParamBoxOriginKey = (String, usize);
type MethodParamBoxOriginMap = BTreeMap<MethodParamBoxOriginKey, BoxOriginInference>;
type CollectionElementOriginMap = BTreeMap<FieldHandleOriginKey, String>;

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
            }) {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block().as_u32(), route.instruction_index()));
    function.metadata.generic_method_routes = routes;
}

fn infer_typed_object_field_handle_origins(module: &MirModule) -> FieldHandleOriginMap {
    let typed_object_fields = module
        .metadata
        .typed_object_plans
        .iter()
        .flat_map(|plan| {
            plan.fields.iter().map(move |field| {
                (
                    (plan.box_name.clone(), field.name.clone()),
                    field.storage == crate::mir::function::TypedObjectFieldStorage::Handle,
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    let mut origins = BTreeMap::<FieldHandleOriginKey, String>::new();
    let mut conflicts = BTreeSet::<FieldHandleOriginKey>::new();

    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::FieldSet {
                    base, field, value, ..
                } = inst
                else {
                    continue;
                };
                let Some(box_name) = typed_object_value_box_name(function, &def_map, *base) else {
                    continue;
                };
                let key = (box_name, field.clone());
                if typed_object_fields.get(&key) != Some(&true) {
                    continue;
                }
                let Some(origin_box) = handle_value_origin_box_name(function, &def_map, *value)
                else {
                    continue;
                };
                if conflicts.contains(&key) {
                    continue;
                }
                match origins.get(&key) {
                    Some(existing) if existing != &origin_box => {
                        origins.remove(&key);
                        conflicts.insert(key);
                    }
                    Some(_) => {}
                    None => {
                        origins.insert(key, origin_box);
                    }
                }
            }
        }
    }

    origins
}

fn infer_typed_object_collection_element_origins(
    module: &MirModule,
    field_handle_origins: &FieldHandleOriginMap,
) -> CollectionElementOriginMap {
    let param_box_origins = infer_same_module_method_param_box_origins(module);
    let mut origins = CollectionElementOriginMap::new();
    let mut conflicts = BTreeSet::<FieldHandleOriginKey>::new();

    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
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
                    continue;
                };
                let Some(field_key) =
                    typed_object_collection_field_key(function, &def_map, *receiver)
                else {
                    continue;
                };
                let Some(collection_box) = field_handle_origins.get(&field_key) else {
                    continue;
                };
                let semantic_args = match method.as_str() {
                    "push" if collection_box == "ArrayBox" => {
                        method_args_without_redundant_receiver(
                            function, &def_map, *receiver, args, 1,
                        )
                    }
                    "set" if matches!(collection_box.as_str(), "ArrayBox" | "MapBox") => {
                        method_args_without_redundant_receiver(
                            function, &def_map, *receiver, args, 2,
                        )
                    }
                    _ => None,
                };
                let Some(semantic_args) = semantic_args else {
                    continue;
                };
                let value = if method == "push" {
                    semantic_args[0]
                } else {
                    semantic_args[1]
                };
                let Some(origin_box) = handle_value_origin_box_name_with_context(
                    module,
                    function,
                    &def_map,
                    value,
                    &param_box_origins,
                ) else {
                    continue;
                };
                merge_collection_origin(&mut origins, &mut conflicts, field_key, origin_box);
                let _ = box_name;
            }
        }
    }

    origins
}

fn merge_collection_origin(
    origins: &mut CollectionElementOriginMap,
    conflicts: &mut BTreeSet<FieldHandleOriginKey>,
    key: FieldHandleOriginKey,
    origin_box: String,
) {
    if conflicts.contains(&key) {
        return;
    }
    match origins.get(&key) {
        Some(existing) if existing != &origin_box => {
            origins.remove(&key);
            conflicts.insert(key);
        }
        Some(_) => {}
        None => {
            origins.insert(key, origin_box);
        }
    }
}

fn infer_same_module_method_param_box_origins(module: &MirModule) -> MethodParamBoxOriginMap {
    let mut origins = MethodParamBoxOriginMap::new();
    for _ in 0..module.functions.len().max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for route in &function.metadata.user_box_method_routes {
                if !route.target_exists() || route.arity_matches() != Some(true) {
                    continue;
                }
                let Some(block) = function.blocks.get(&route.block()) else {
                    continue;
                };
                let Some(MirInstruction::Call { args, .. }) =
                    block.instructions.get(route.instruction_index())
                else {
                    continue;
                };
                changed |= merge_box_origin(
                    &mut origins,
                    (route.target_symbol().to_string(), 0),
                    route.box_name().to_string(),
                );
                for (arg_index, arg) in args.iter().enumerate() {
                    let Some(arg_box) = handle_value_origin_box_name_with_context(
                        module, function, &def_map, *arg, &current,
                    ) else {
                        continue;
                    };
                    changed |= merge_box_origin(
                        &mut origins,
                        (route.target_symbol().to_string(), arg_index + 1),
                        arg_box,
                    );
                }
            }
        }
        if !changed {
            break;
        }
    }
    origins
}

fn merge_box_origin(
    origins: &mut MethodParamBoxOriginMap,
    key: MethodParamBoxOriginKey,
    box_name: String,
) -> bool {
    use std::collections::btree_map::Entry;
    match origins.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(BoxOriginInference::Known(box_name));
            true
        }
        Entry::Occupied(mut slot) => {
            let next = match slot.get() {
                BoxOriginInference::Known(existing) if existing == &box_name => {
                    BoxOriginInference::Known(existing.clone())
                }
                BoxOriginInference::Known(_) | BoxOriginInference::Conflict => {
                    BoxOriginInference::Conflict
                }
            };
            let changed = slot.get() != &next;
            slot.insert(next);
            changed
        }
    }
}

fn typed_object_collection_field_key(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<FieldHandleOriginKey> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::FieldGet { base, field, .. } => {
            let box_name = typed_object_value_box_name(function, def_map, *base)?;
            Some((box_name, field.clone()))
        }
        MirInstruction::Phi { inputs, .. } if !inputs.is_empty() => {
            let mut field_key = None;
            for (_, input) in inputs {
                let next = typed_object_collection_field_key(function, def_map, *input)?;
                field_key = match field_key {
                    None => Some(next),
                    Some(existing) if existing == next => Some(existing),
                    _ => return None,
                };
            }
            field_key
        }
        _ => None,
    }
}

fn generic_collection_element_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    collection_element_origins: &CollectionElementOriginMap,
    receiver: ValueId,
) -> Option<String> {
    let field_key = typed_object_collection_field_key(function, def_map, receiver)?;
    collection_element_origins.get(&field_key).cloned()
}

fn typed_object_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    if let Some((block_id, instruction_index)) = def_map.get(&origin).copied() {
        let block = function.blocks.get(&block_id)?;
        match block.instructions.get(instruction_index)? {
            MirInstruction::NewBox { box_type, .. } => return Some(box_type.clone()),
            MirInstruction::Phi { type_hint, .. } => {
                if let Some(box_name) = type_hint.as_ref().and_then(box_name_from_mir_type) {
                    return Some(box_name.to_string());
                }
            }
            _ => {}
        }
    }
    if let Some(box_name) = function
        .metadata
        .value_types
        .get(&origin)
        .and_then(box_name_from_mir_type)
    {
        return Some(box_name.to_string());
    }
    function
        .params
        .iter()
        .position(|param| *param == origin)
        .and_then(|index| {
            function
                .signature
                .params
                .get(index)
                .and_then(|ty| match ty {
                    MirType::Box(name) => Some(name.clone()),
                    _ => None,
                })
                .or_else(|| {
                    (index == 0)
                        .then(|| method_receiver_box_name(&function.signature.name))
                        .flatten()
                })
        })
}

fn method_receiver_box_name(symbol: &str) -> Option<String> {
    let (owner_and_method, _arity) = symbol.rsplit_once('/')?;
    let (box_name, _method) = owner_and_method.rsplit_once('.')?;
    Some(box_name.to_string())
}

fn box_name_from_mir_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

fn handle_value_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    handle_value_origin_box_name_with_context(
        &MirModule::new(String::new()),
        function,
        def_map,
        value,
        &MethodParamBoxOriginMap::new(),
    )
}

fn handle_value_origin_box_name_with_context(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    param_box_origins: &MethodParamBoxOriginMap,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    if let Some(box_name) = value_box_name(function, value)
        .or_else(|| value_box_name(function, origin))
        .map(str::to_string)
    {
        return Some(box_name);
    }
    if let Some((block_id, instruction_index)) = def_map.get(&origin).copied() {
        let block = function.blocks.get(&block_id)?;
        match block.instructions.get(instruction_index)? {
            MirInstruction::Const {
                value: ConstValue::String(_),
                ..
            } => return Some("StringBox".to_string()),
            MirInstruction::NewBox { box_type, .. } => return Some(box_type.clone()),
            MirInstruction::Phi { inputs, .. } if !inputs.is_empty() => {
                let mut input_box = None;
                for (_, input) in inputs {
                    let next = handle_value_origin_box_name_with_context(
                        module,
                        function,
                        def_map,
                        *input,
                        param_box_origins,
                    )?;
                    input_box = match input_box {
                        None => Some(next),
                        Some(existing) if existing == next => Some(existing),
                        _ => return None,
                    };
                }
                return input_box;
            }
            MirInstruction::Call { dst, callee, .. } => {
                if *dst == Some(origin) {
                    match callee {
                        Some(Callee::Method { .. }) => {
                            if let Some(box_name) = user_box_method_call_result_origin_box_name(
                                module,
                                function,
                                block_id,
                                instruction_index,
                            ) {
                                return Some(box_name);
                            }
                        }
                        Some(Callee::Global(_)) => {
                            if let Some(box_name) = global_call_result_origin_box_name(
                                module,
                                function,
                                block_id,
                                instruction_index,
                            ) {
                                return Some(box_name);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    function
        .params
        .iter()
        .position(|param| *param == origin)
        .and_then(
            |index| match param_box_origins.get(&(function.signature.name.clone(), index)) {
                Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
                Some(BoxOriginInference::Conflict) | None => None,
            },
        )
}

fn user_box_method_call_result_origin_box_name(
    module: &MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
) -> Option<String> {
    let route = function
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.block() == block && route.instruction_index() == instruction_index)?;
    if let Some(box_name) = route.target_result_box_name() {
        return Some(box_name.to_string());
    }
    if route.return_shape() == Some("string_handle") {
        return Some("StringBox".to_string());
    }
    route
        .target_return_type()
        .as_deref()
        .and_then(|label| box_name_from_type_label(module, label))
}

fn global_call_result_origin_box_name(
    module: &MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
) -> Option<String> {
    let route =
        function.metadata.global_call_routes.iter().find(|route| {
            route.block() == block && route.instruction_index() == instruction_index
        })?;
    if matches!(
        route.return_shape(),
        Some("string_handle" | "string_handle_or_null")
    ) {
        return Some("StringBox".to_string());
    }
    route
        .target_return_type()
        .as_deref()
        .and_then(|label| box_name_from_type_label(module, label))
}

fn box_name_from_type_label(module: &MirModule, label: &str) -> Option<String> {
    match label {
        "StringBox" | "String" | "str" => Some("StringBox".to_string()),
        "ArrayBox" => Some("ArrayBox".to_string()),
        "MapBox" => Some("MapBox".to_string()),
        "i64" | "i1" | "void" | "unknown" | "WeakRef" => None,
        name if module.metadata.user_box_decls.contains_key(name)
            || module.metadata.user_box_field_decls.contains_key(name) =>
        {
            Some(name.to_string())
        }
        _ => None,
    }
}

fn value_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
    function
        .metadata
        .value_types
        .get(&value)
        .and_then(box_name_from_mir_type)
        .or_else(|| {
            function
                .params
                .iter()
                .position(|param| *param == value)
                .and_then(|index| function.signature.params.get(index))
                .and_then(box_name_from_mir_type)
        })
}

fn match_generic_has_route(
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

fn match_generic_get_route(
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
        return Some(GenericMethodRoute::new(
            GenericMethodRouteSite::new(block, instruction_index),
            GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route))
                .with_result_origin_box(result_origin_box),
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

fn match_generic_len_route(
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

fn match_generic_keys_route(
    function: &MirFunction,
    _def_map: &ValueDefMap,
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
    if method != "keys" || !args.is_empty() {
        return None;
    }
    if function.signature.name != "MirJsonEmitBox._emit_flags/1" || box_name != "RuntimeDataBox" {
        return None;
    }

    Some(GenericMethodRoute::new(
        GenericMethodRouteSite::new(block, instruction_index),
        GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 0),
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

fn generic_runtime_data_contains_param_text_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    box_name: &str,
    receiver: ValueId,
    needle: ValueId,
) -> Option<String> {
    if box_name != "RuntimeDataBox" {
        return None;
    }
    if generic_pure_string_value_origin_box_name(function, def_map, needle).as_deref()
        != Some("StringBox")
    {
        return None;
    }
    let mut visited = BTreeSet::new();
    generic_value_flows_from_text_param(function, def_map, receiver, &mut visited)
        .then(|| "StringBox".to_string())
}

fn generic_string_receiver_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    box_name: &str,
) -> Option<String> {
    receiver_origin_box_name(function, def_map, receiver)
        .or_else(|| generic_pure_string_value_origin_box_name(function, def_map, receiver))
        .or_else(|| {
            string_corridor_value_origin_box_name(
                function,
                def_map,
                receiver,
                StringCorridorOp::StrSlice,
            )
        })
        .or_else(|| (box_name == "StringBox").then(|| "StringBox".to_string()))
}

fn string_corridor_value_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    op: StringCorridorOp,
) -> Option<String> {
    let mut visited = BTreeSet::new();
    string_corridor_value_has_op_flow(function, def_map, value, op, &mut visited)
        .then(|| "StringBox".to_string())
}

fn string_corridor_value_has_op_flow(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    op: StringCorridorOp,
    visited: &mut BTreeSet<ValueId>,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    if !visited.insert(origin) {
        return false;
    }
    if function
        .metadata
        .string_corridor_facts
        .get(&origin)
        .is_some_and(|fact| fact.op == op)
    {
        return true;
    }
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Phi { inputs, .. }) if !inputs.is_empty() => {
            inputs.iter().all(|(_, input)| {
                let mut branch_visited = visited.clone();
                string_corridor_value_has_op_flow(
                    function,
                    def_map,
                    *input,
                    op,
                    &mut branch_visited,
                )
            })
        }
        _ => false,
    }
}

fn match_generic_push_route(
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
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    ))
}

fn match_generic_set_route(
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

fn string_corridor_method_origin_box_name(
    function: &MirFunction,
    dst: Option<ValueId>,
    op: StringCorridorOp,
) -> Option<String> {
    let dst = dst?;
    let fact = function.metadata.string_corridor_facts.get(&dst)?;
    (fact.op == op).then(|| "StringBox".to_string())
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
                && global_call_route_returns_string_like_handle(route)
        })
        .then(|| "StringBox".to_string())
}

fn global_call_route_returns_string_like_handle(route: &GlobalCallRoute) -> bool {
    matches!(
        route.proof(),
        "typed_global_call_generic_pure_string"
            | "typed_global_call_generic_string_or_void_sentinel"
    ) && matches!(
        route.return_shape(),
        Some("string_handle" | "string_handle_or_null")
    )
}

fn generic_pure_string_value_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    generic_pure_string_signature_param_origin_box_name(function, def_map, receiver)
        .or_else(|| generic_pure_string_global_call_origin_box_name(function, def_map, receiver))
        .or_else(|| generic_pure_string_flow_origin_box_name(function, receiver))
}

fn generic_pure_string_signature_param_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, receiver);
    function
        .params
        .iter()
        .position(|param| *param == origin)
        .and_then(|index| function.signature.params.get(index))
        .and_then(|ty| match ty {
            super::MirType::String => Some("StringBox".to_string()),
            super::MirType::Box(name) if name == "StringBox" => Some("StringBox".to_string()),
            _ => None,
        })
}

fn generic_value_flows_from_text_param(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visited: &mut BTreeSet<ValueId>,
) -> bool {
    let origin = resolve_value_origin(function, def_map, value);
    if !visited.insert(origin) {
        return false;
    }
    if generic_value_is_text_param(function, origin) {
        return true;
    }
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Phi { inputs, .. }) if !inputs.is_empty() => {
            inputs.iter().all(|(_, input)| {
                let mut branch_visited = visited.clone();
                generic_value_flows_from_text_param(function, def_map, *input, &mut branch_visited)
            })
        }
        _ => false,
    }
}

fn generic_value_is_text_param(function: &MirFunction, value: ValueId) -> bool {
    function
        .params
        .iter()
        .position(|param| *param == value)
        .and_then(|index| function.signature.params.get(index))
        .is_some_and(generic_param_type_can_flow_as_text)
}

fn generic_param_type_can_flow_as_text(ty: &super::MirType) -> bool {
    matches!(ty, super::MirType::Unknown | super::MirType::String)
        || matches!(ty, super::MirType::Box(name) if name == "StringBox")
}

fn generic_array_flow_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    field_handle_origins: &FieldHandleOriginMap,
    receiver: ValueId,
) -> Option<String> {
    let mut array_values = BTreeMap::<ValueId, &'static str>::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, inst) in block.instructions.iter().enumerate() {
                match inst {
                    MirInstruction::NewBox { dst, box_type, .. } => {
                        let Some(origin_box) = collection_origin_box_name(box_type) else {
                            continue;
                        };
                        if array_values.insert(*dst, origin_box) != Some(origin_box) {
                            changed = true;
                        }
                    }
                    MirInstruction::Copy { dst, src } => {
                        if let Some(origin) = array_values.get(src).copied() {
                            if array_values.insert(*dst, origin) != Some(origin) {
                                changed = true;
                            }
                        }
                    }
                    MirInstruction::FieldGet {
                        dst, base, field, ..
                    } => {
                        let Some(box_name) = typed_object_value_box_name(function, def_map, *base)
                        else {
                            continue;
                        };
                        let Some(origin_box) = field_handle_origins
                            .get(&(box_name, field.clone()))
                            .and_then(|origin| collection_origin_box_name(origin))
                        else {
                            continue;
                        };
                        if array_values.insert(*dst, origin_box) != Some(origin_box) {
                            changed = true;
                        }
                    }
                    MirInstruction::Phi { dst, inputs, .. } if !inputs.is_empty() => {
                        let mut origin = None;
                        let mut all_same = true;
                        for (_, value) in inputs {
                            let Some(input_origin) = array_values.get(value).copied() else {
                                all_same = false;
                                break;
                            };
                            if let Some(existing) = origin {
                                if existing != input_origin {
                                    all_same = false;
                                    break;
                                }
                            } else {
                                origin = Some(input_origin);
                            }
                        }
                        if all_same {
                            if let Some(origin) = origin {
                                if array_values.insert(*dst, origin) != Some(origin) {
                                    changed = true;
                                }
                            }
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee:
                            Some(Callee::Method {
                                box_name, method, ..
                            }),
                        args,
                        ..
                    } if function.signature.name == "MirJsonEmitBox._emit_flags/1"
                        && box_name == "RuntimeDataBox"
                        && method == "keys"
                        && args.is_empty() =>
                    {
                        if array_values.insert(*dst, "ArrayBox") != Some("ArrayBox") {
                            changed = true;
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee:
                            Some(Callee::Method {
                                box_name, method, ..
                            }),
                        args,
                        ..
                    } if function.signature.name == "MirJsonEmitBox._emit_function/1"
                        && box_name == "RuntimeDataBox"
                        && method == "get"
                        && args.len() == 1
                        && const_string_value(function, def_map, args[0])
                            .is_some_and(|key| key == "params" || key == "blocks") =>
                    {
                        if array_values.insert(*dst, "ArrayBox") != Some("ArrayBox") {
                            changed = true;
                        }
                    }
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Global(_)),
                        ..
                    } => {
                        let is_static_array =
                            function.metadata.global_call_routes.iter().any(|route| {
                                route.block() == *block_id
                                    && route.instruction_index() == instruction_index
                                    && route.result_value() == Some(*dst)
                                    && route.proof() == "typed_global_call_static_string_array"
                                    && route.return_shape() == Some("array_handle")
                            });
                        if is_static_array
                            && array_values.insert(*dst, "ArrayBox") != Some("ArrayBox")
                        {
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        if !changed {
            break;
        }
    }

    array_values.get(&receiver).map(|name| (*name).to_string())
}

fn collection_origin_box_name(box_name: &str) -> Option<&'static str> {
    match box_name {
        "ArrayBox" => Some("ArrayBox"),
        "MapBox" => Some("MapBox"),
        _ => None,
    }
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
                && global_call_route_returns_string_like_handle(route)
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
