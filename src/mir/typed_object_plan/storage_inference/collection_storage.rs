use std::collections::BTreeMap;

use crate::mir::function::TypedObjectFieldStorage;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{Callee, MirFunction, MirInstruction, MirModule, ValueId};

use super::merge::merge_storage_inference;
use super::state::{
    BoxOriginInference, CollectionElementStorageMap, FieldBoxOriginMap, FieldKey,
    FieldStorageInference, ParamKey,
};
use super::value_analysis::{box_name_for_value, storage_for_value};

pub(super) fn infer_collection_element_storages(
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

pub(super) fn collection_element_storage_for_get(
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
