use std::collections::{BTreeMap, BTreeSet};

use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

use super::{
    method_args_without_redundant_receiver, BoxOriginInference, CollectionElementOriginMap,
    FieldHandleOriginKey, FieldHandleOriginMap, MethodParamBoxOriginKey, MethodParamBoxOriginMap,
};

pub(super) fn infer_typed_object_field_handle_origins(module: &MirModule) -> FieldHandleOriginMap {
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

pub(super) fn infer_typed_object_collection_element_origins(
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

pub(super) fn generic_collection_element_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    collection_element_origins: &CollectionElementOriginMap,
    receiver: ValueId,
) -> Option<String> {
    let field_key = typed_object_collection_field_key(function, def_map, receiver)?;
    collection_element_origins.get(&field_key).cloned()
}

pub(super) fn typed_object_value_box_name(
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
