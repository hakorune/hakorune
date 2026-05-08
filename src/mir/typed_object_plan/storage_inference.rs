use std::collections::{BTreeMap, BTreeSet};

mod value_analysis;

use super::TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0;
use crate::mir::function::{
    ModuleMetadata, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan,
};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{Callee, MirFunction, MirInstruction, MirModule, UserBoxFieldDecl, ValueId};
use value_analysis::{
    box_name_for_value, box_origin_for_value, same_module_method_target, storage_for_declared_type,
    storage_for_mir_type, storage_for_value,
};

type FieldKey = (String, String);
type ParamKey = (String, usize);
type FieldBoxOriginMap = BTreeMap<FieldKey, BoxOriginInference>;
type ParamBoxOriginMap = BTreeMap<ParamKey, BoxOriginInference>;
type CollectionElementStorageMap = BTreeMap<FieldKey, FieldStorageInference>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldStorageInference {
    Known(TypedObjectFieldStorage),
    Conflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoxOriginInference {
    Known(String),
    Conflict,
}

#[cfg(test)]
mod numeric_substrate_tests;
#[cfg(test)]
mod tests;

pub(super) fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan> {
    let inferred = infer_untyped_field_storages(module);
    let observed_newboxes = observed_user_newbox_names(module);
    build_typed_object_plans_from_metadata(&module.metadata, &inferred, &observed_newboxes)
}

fn build_typed_object_plans_from_metadata(
    metadata: &ModuleMetadata,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    observed_newboxes: &BTreeSet<String>,
) -> Vec<TypedObjectPlan> {
    let mut names = BTreeSet::new();
    names.extend(metadata.user_box_decls.keys().cloned());
    names.extend(metadata.user_box_field_decls.keys().cloned());
    names.extend(observed_newboxes.iter().cloned());

    let mut plans = Vec::new();
    for name in names {
        if !metadata.user_box_decls.contains_key(&name)
            && !metadata.user_box_field_decls.contains_key(&name)
        {
            continue;
        }
        let empty_field_decls = Vec::new();
        let field_decls = metadata
            .user_box_field_decls
            .get(&name)
            .unwrap_or(&empty_field_decls);
        if field_decls.is_empty() {
            if observed_newboxes.contains(&name) {
                let type_id = plans.len() as u32 + 1;
                plans.push(TypedObjectPlan {
                    box_name: name,
                    type_id,
                    layout_kind: TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0.to_string(),
                    field_count: 0,
                    fields: Vec::new(),
                });
            }
            continue;
        }
        let Some(fields) = build_field_plans(metadata, &name, field_decls, inferred) else {
            continue;
        };
        let type_id = plans.len() as u32 + 1;
        plans.push(TypedObjectPlan {
            box_name: name,
            type_id,
            layout_kind: TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0.to_string(),
            field_count: fields.len() as u32,
            fields,
        });
    }
    plans
}

fn observed_user_newbox_names(module: &MirModule) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                if let MirInstruction::NewBox { box_type, .. } = inst {
                    if module.metadata.user_box_decls.contains_key(box_type)
                        || module.metadata.user_box_field_decls.contains_key(box_type)
                    {
                        names.insert(box_type.clone());
                    }
                }
            }
        }
    }
    names
}

fn build_field_plans(
    metadata: &ModuleMetadata,
    box_name: &str,
    field_decls: &[UserBoxFieldDecl],
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
) -> Option<Vec<TypedObjectFieldPlan>> {
    let mut fields = Vec::new();
    for (slot, decl) in field_decls.iter().enumerate() {
        if decl.is_weak {
            return None;
        }
        let declared = storage_for_declared_type(metadata, decl.declared_type_name.as_deref());
        let observed = match inferred.get(&(box_name.to_string(), decl.name.clone())) {
            Some(FieldStorageInference::Known(storage)) => Some(*storage),
            Some(FieldStorageInference::Conflict) => return None,
            None => None,
        };
        let storage = match (declared, observed) {
            (Some(left), Some(right)) if left != right => return None,
            (Some(storage), _) | (None, Some(storage)) => storage,
            (None, None) => return None,
        };
        fields.push(TypedObjectFieldPlan {
            name: decl.name.clone(),
            slot: slot as u32,
            declared_type_name: decl.declared_type_name.clone(),
            storage,
            is_weak: decl.is_weak,
        });
    }
    Some(fields)
}

fn infer_untyped_field_storages(module: &MirModule) -> BTreeMap<FieldKey, FieldStorageInference> {
    let declared_fields = declared_field_sets(&module.metadata);
    let field_box_origins = infer_untyped_field_box_origins(module, &declared_fields);
    let mut inferred = BTreeMap::new();

    for _ in 0..4 {
        let mut changed = false;
        let param_storages = infer_param_storages(module, &inferred, &field_box_origins);
        let collection_element_storages = infer_collection_element_storages(
            module,
            &inferred,
            &field_box_origins,
            &param_storages,
        );
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for block in function.blocks.values() {
                for inst in &block.instructions {
                    let MirInstruction::FieldSet {
                        base,
                        field,
                        value,
                        declared_type,
                    } = inst
                    else {
                        continue;
                    };
                    let Some(box_name) = box_name_for_value(function, &def_map, *base) else {
                        continue;
                    };
                    if !declared_fields
                        .get(&box_name)
                        .is_some_and(|fields| fields.contains(field))
                    {
                        continue;
                    }
                    let storage = declared_type
                        .as_ref()
                        .and_then(storage_for_mir_type)
                        .or_else(|| {
                            storage_for_value(
                                module,
                                function,
                                &def_map,
                                *value,
                                &inferred,
                                &field_box_origins,
                                &param_storages,
                                &collection_element_storages,
                            )
                        });
                    if let Some(storage) = storage {
                        changed |= merge_storage_observation(
                            &mut inferred,
                            (box_name, field.clone()),
                            storage,
                        );
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    inferred
}

fn infer_untyped_field_box_origins(
    module: &MirModule,
    declared_fields: &BTreeMap<String, BTreeSet<String>>,
) -> FieldBoxOriginMap {
    let mut origins = FieldBoxOriginMap::new();
    for _ in 0..module.functions.len().max(1) {
        let current = origins.clone();
        let param_box_origins = infer_param_box_origins(module, &current);
        let mut changed = false;
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
                    let Some(box_name) = box_name_for_value(function, &def_map, *base) else {
                        continue;
                    };
                    if !declared_fields
                        .get(&box_name)
                        .is_some_and(|fields| fields.contains(field))
                    {
                        continue;
                    }
                    let Some(origin_box) = box_origin_for_value(
                        module,
                        function,
                        &def_map,
                        *value,
                        &current,
                        &param_box_origins,
                    ) else {
                        continue;
                    };
                    changed |= merge_box_origin_observation(
                        &mut origins,
                        (box_name, field.clone()),
                        origin_box,
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

fn infer_param_box_origins(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
) -> ParamBoxOriginMap {
    let mut param_box_origins = ParamBoxOriginMap::new();
    for _ in 0..module.functions.len().max(1) {
        let current = param_box_origins.clone();
        let mut changed = false;
        changed |= infer_birth_param_box_origins(
            module,
            field_box_origins,
            &current,
            &mut param_box_origins,
        );
        changed |= infer_call_param_box_origins(
            module,
            field_box_origins,
            &current,
            &mut param_box_origins,
        );
        if !changed {
            break;
        }
    }
    param_box_origins
}

fn infer_birth_param_box_origins(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
    known_param_box_origins: &ParamBoxOriginMap,
    param_box_origins: &mut ParamBoxOriginMap,
) -> bool {
    let mut changed = false;
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::NewBox { box_type, args, .. } = inst else {
                    continue;
                };
                if !module.metadata.user_box_decls.contains_key(box_type)
                    && !module.metadata.user_box_field_decls.contains_key(box_type)
                {
                    continue;
                }
                let birth_symbol = format!("{box_type}.birth/{}", args.len());
                if !module.functions.contains_key(&birth_symbol) {
                    continue;
                }
                for (arg_index, arg) in args.iter().enumerate() {
                    let Some(origin_box) = box_origin_for_value(
                        module,
                        function,
                        &def_map,
                        *arg,
                        field_box_origins,
                        known_param_box_origins,
                    ) else {
                        continue;
                    };
                    changed |= merge_box_origin_observation(
                        param_box_origins,
                        (birth_symbol.clone(), arg_index + 1),
                        origin_box,
                    );
                }
            }
        }
    }
    changed
}

fn infer_call_param_box_origins(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
    known_param_box_origins: &ParamBoxOriginMap,
    param_box_origins: &mut ParamBoxOriginMap,
) -> bool {
    let mut changed = false;
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    callee: Some(callee),
                    args,
                    ..
                } = inst
                else {
                    continue;
                };
                match callee {
                    Callee::Global(symbol) if module.functions.contains_key(symbol) => {
                        for (arg_index, arg) in args.iter().enumerate() {
                            let Some(origin_box) = box_origin_for_value(
                                module,
                                function,
                                &def_map,
                                *arg,
                                field_box_origins,
                                known_param_box_origins,
                            ) else {
                                continue;
                            };
                            changed |= merge_box_origin_observation(
                                param_box_origins,
                                (symbol.clone(), arg_index),
                                origin_box,
                            );
                        }
                    }
                    Callee::Method {
                        box_name,
                        method,
                        receiver,
                        ..
                    } => {
                        let Some((target_box, target_symbol)) = same_module_method_target(
                            module,
                            function,
                            &def_map,
                            box_name,
                            method,
                            *receiver,
                            args.len(),
                            field_box_origins,
                            known_param_box_origins,
                        ) else {
                            continue;
                        };
                        if let Some(receiver) = receiver {
                            changed |= merge_box_origin_observation(
                                param_box_origins,
                                (target_symbol.clone(), 0),
                                target_box,
                            );
                            if let Some(receiver_box) = box_origin_for_value(
                                module,
                                function,
                                &def_map,
                                *receiver,
                                field_box_origins,
                                known_param_box_origins,
                            ) {
                                changed |= merge_box_origin_observation(
                                    param_box_origins,
                                    (target_symbol.clone(), 0),
                                    receiver_box,
                                );
                            }
                        }
                        for (arg_index, arg) in args.iter().enumerate() {
                            let Some(origin_box) = box_origin_for_value(
                                module,
                                function,
                                &def_map,
                                *arg,
                                field_box_origins,
                                known_param_box_origins,
                            ) else {
                                continue;
                            };
                            changed |= merge_box_origin_observation(
                                param_box_origins,
                                (target_symbol.clone(), arg_index + 1),
                                origin_box,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    changed
}

fn infer_param_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    field_box_origins: &FieldBoxOriginMap,
) -> BTreeMap<ParamKey, FieldStorageInference> {
    let param_box_origins = infer_param_box_origins(module, field_box_origins);
    let mut param_storages = BTreeMap::new();
    for _ in 0..4 {
        let current = param_storages.clone();
        let collection_element_storages =
            infer_collection_element_storages(module, inferred, field_box_origins, &current);
        let mut changed = false;
        changed |= infer_birth_param_storages(
            module,
            inferred,
            field_box_origins,
            &collection_element_storages,
            &current,
            &mut param_storages,
        );
        changed |= infer_call_param_storages(
            module,
            inferred,
            field_box_origins,
            &param_box_origins,
            &collection_element_storages,
            &current,
            &mut param_storages,
        );
        if !changed {
            break;
        }
    }
    param_storages
}

fn infer_birth_param_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    field_box_origins: &FieldBoxOriginMap,
    collection_element_storages: &CollectionElementStorageMap,
    known_param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    param_storages: &mut BTreeMap<ParamKey, FieldStorageInference>,
) -> bool {
    let mut changed = false;
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::NewBox { box_type, args, .. } = inst else {
                    continue;
                };
                if !module.metadata.user_box_decls.contains_key(box_type)
                    && !module.metadata.user_box_field_decls.contains_key(box_type)
                {
                    continue;
                }
                let birth_symbol = format!("{box_type}.birth/{}", args.len());
                if !module.functions.contains_key(&birth_symbol) {
                    continue;
                }
                for (arg_index, arg) in args.iter().enumerate() {
                    let Some(storage) = storage_for_value(
                        module,
                        function,
                        &def_map,
                        *arg,
                        inferred,
                        field_box_origins,
                        known_param_storages,
                        collection_element_storages,
                    ) else {
                        continue;
                    };
                    changed |= merge_param_storage_observation(
                        param_storages,
                        (birth_symbol.clone(), arg_index + 1),
                        storage,
                    );
                }
            }
        }
    }
    changed
}

fn infer_call_param_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    field_box_origins: &FieldBoxOriginMap,
    param_box_origins: &ParamBoxOriginMap,
    collection_element_storages: &CollectionElementStorageMap,
    known_param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    param_storages: &mut BTreeMap<ParamKey, FieldStorageInference>,
) -> bool {
    let mut changed = false;
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    callee: Some(callee),
                    args,
                    ..
                } = inst
                else {
                    continue;
                };
                match callee {
                    Callee::Global(symbol) if module.functions.contains_key(symbol) => {
                        for (arg_index, arg) in args.iter().enumerate() {
                            let Some(storage) = storage_for_value(
                                module,
                                function,
                                &def_map,
                                *arg,
                                inferred,
                                field_box_origins,
                                known_param_storages,
                                collection_element_storages,
                            ) else {
                                continue;
                            };
                            changed |= merge_param_storage_observation(
                                param_storages,
                                (symbol.clone(), arg_index),
                                storage,
                            );
                        }
                    }
                    Callee::Method {
                        box_name,
                        method,
                        receiver,
                        ..
                    } => {
                        let Some((_, symbol)) = same_module_method_target(
                            module,
                            function,
                            &def_map,
                            box_name,
                            method,
                            *receiver,
                            args.len(),
                            field_box_origins,
                            param_box_origins,
                        ) else {
                            continue;
                        };
                        if let Some(receiver) = receiver {
                            if let Some(storage) = storage_for_value(
                                module,
                                function,
                                &def_map,
                                *receiver,
                                inferred,
                                field_box_origins,
                                known_param_storages,
                                collection_element_storages,
                            ) {
                                changed |= merge_param_storage_observation(
                                    param_storages,
                                    (symbol.clone(), 0),
                                    storage,
                                );
                            }
                        }
                        for (arg_index, arg) in args.iter().enumerate() {
                            let Some(storage) = storage_for_value(
                                module,
                                function,
                                &def_map,
                                *arg,
                                inferred,
                                field_box_origins,
                                known_param_storages,
                                collection_element_storages,
                            ) else {
                                continue;
                            };
                            changed |= merge_param_storage_observation(
                                param_storages,
                                (symbol.clone(), arg_index + 1),
                                storage,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    changed
}

fn infer_collection_element_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    field_box_origins: &FieldBoxOriginMap,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
) -> CollectionElementStorageMap {
    let mut element_storages = CollectionElementStorageMap::new();
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    callee:
                        Some(Callee::Method {
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
                let Some(collection_box) = field_box_origin(field_box_origins, &field_key) else {
                    continue;
                };
                let Some(value) = collection_write_value(
                    function,
                    &def_map,
                    *receiver,
                    method,
                    args,
                    collection_box.as_str(),
                ) else {
                    continue;
                };
                let Some(storage) = storage_for_value(
                    module,
                    function,
                    &def_map,
                    value,
                    inferred,
                    field_box_origins,
                    param_storages,
                    &CollectionElementStorageMap::new(),
                ) else {
                    continue;
                };
                merge_storage_inference(&mut element_storages, field_key, storage);
            }
        }
    }
    element_storages
}

fn collection_write_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    method: &str,
    args: &[ValueId],
    collection_box: &str,
) -> Option<ValueId> {
    match (collection_box, method, args.len()) {
        ("ArrayBox", "push", 1) => Some(args[0]),
        ("ArrayBox", "push", 2)
            if resolve_value_origin(function, def_map, args[0])
                == resolve_value_origin(function, def_map, receiver) =>
        {
            Some(args[1])
        }
        ("ArrayBox" | "MapBox", "set", 2) => Some(args[1]),
        ("ArrayBox" | "MapBox", "set", 3)
            if resolve_value_origin(function, def_map, args[0])
                == resolve_value_origin(function, def_map, receiver) =>
        {
            Some(args[2])
        }
        _ => None,
    }
}

fn collection_element_storage_for_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    callee: &Callee,
    args: &[ValueId],
    collection_element_storages: &CollectionElementStorageMap,
) -> Option<TypedObjectFieldStorage> {
    let Callee::Method {
        method,
        receiver: Some(receiver),
        ..
    } = callee
    else {
        return None;
    };
    if method != "get" {
        return None;
    }
    let field_key = typed_object_collection_field_key(function, def_map, *receiver)?;
    let _key = collection_read_key(function, def_map, *receiver, args)?;
    match collection_element_storages.get(&field_key) {
        Some(FieldStorageInference::Known(storage)) => Some(*storage),
        Some(FieldStorageInference::Conflict) | None => None,
    }
}

fn collection_read_key(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
    args: &[ValueId],
) -> Option<ValueId> {
    match args.len() {
        1 => Some(args[0]),
        2 if resolve_value_origin(function, def_map, args[0])
            == resolve_value_origin(function, def_map, receiver) =>
        {
            Some(args[1])
        }
        _ => None,
    }
}

fn typed_object_collection_field_key(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<FieldKey> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::FieldGet { base, field, .. } => {
            let box_name = box_name_for_value(function, def_map, *base)?;
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

fn field_box_origin(origins: &FieldBoxOriginMap, key: &FieldKey) -> Option<String> {
    match origins.get(key) {
        Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

fn declared_field_sets(metadata: &ModuleMetadata) -> BTreeMap<String, BTreeSet<String>> {
    metadata
        .user_box_field_decls
        .iter()
        .map(|(box_name, fields)| {
            (
                box_name.clone(),
                fields.iter().map(|field| field.name.clone()).collect(),
            )
        })
        .collect()
}

fn merge_storage_observation(
    inferred: &mut BTreeMap<FieldKey, FieldStorageInference>,
    key: FieldKey,
    storage: TypedObjectFieldStorage,
) -> bool {
    merge_storage_inference(inferred, key, storage)
}

fn merge_param_storage_observation(
    inferred: &mut BTreeMap<ParamKey, FieldStorageInference>,
    key: ParamKey,
    storage: TypedObjectFieldStorage,
) -> bool {
    merge_storage_inference(inferred, key, storage)
}

fn merge_storage_inference<K: Ord>(
    inferred: &mut BTreeMap<K, FieldStorageInference>,
    key: K,
    storage: TypedObjectFieldStorage,
) -> bool {
    use std::collections::btree_map::Entry;
    match inferred.entry(key) {
        Entry::Vacant(slot) => {
            slot.insert(FieldStorageInference::Known(storage));
            true
        }
        Entry::Occupied(mut slot) => {
            let next = match *slot.get() {
                FieldStorageInference::Known(existing) if existing == storage => {
                    FieldStorageInference::Known(existing)
                }
                FieldStorageInference::Known(_) | FieldStorageInference::Conflict => {
                    FieldStorageInference::Conflict
                }
            };
            let changed = *slot.get() != next;
            slot.insert(next);
            changed
        }
    }
}

fn merge_box_origin_observation<K: Ord>(
    inferred: &mut BTreeMap<K, BoxOriginInference>,
    key: K,
    box_name: String,
) -> bool {
    use std::collections::btree_map::Entry;
    match inferred.entry(key) {
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
