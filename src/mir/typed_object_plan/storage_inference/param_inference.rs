use std::collections::BTreeMap;

use crate::mir::value_origin::{build_value_def_map, ValueDefMap};
use crate::mir::{Callee, MirFunction, MirInstruction, MirModule};

use super::collection_storage::infer_collection_element_storages_with_def_maps;
use super::merge::{merge_box_origin_observation, merge_param_storage_observation};
use super::state::{
    CollectionElementStorageMap, FieldBoxOriginMap, FieldKey, FieldStorageInference,
    ParamBoxOriginMap, ParamKey,
};
use super::value_analysis::{BoxOriginQueryContext, StorageQueryContext};

type FunctionDefMaps = BTreeMap<String, ValueDefMap>;

fn build_function_def_maps(module: &MirModule) -> FunctionDefMaps {
    module
        .functions
        .iter()
        .map(|(name, function)| (name.clone(), build_value_def_map(function)))
        .collect()
}

fn def_map_for<'a>(
    function_def_maps: &'a FunctionDefMaps,
    function: &MirFunction,
) -> Option<&'a ValueDefMap> {
    function_def_maps.get(&function.signature.name)
}

pub(super) fn infer_param_box_origins(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
) -> ParamBoxOriginMap {
    let function_def_maps = build_function_def_maps(module);
    infer_param_box_origins_with_def_maps(module, field_box_origins, &function_def_maps)
}

fn infer_param_box_origins_with_def_maps(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
    function_def_maps: &FunctionDefMaps,
) -> ParamBoxOriginMap {
    let mut param_box_origins = ParamBoxOriginMap::new();
    for _ in 0..module.functions.len().max(1) {
        let current = param_box_origins.clone();
        let mut changed = false;
        changed |= infer_birth_param_box_origins(
            module,
            field_box_origins,
            function_def_maps,
            &current,
            &mut param_box_origins,
        );
        changed |= infer_call_param_box_origins(
            module,
            field_box_origins,
            function_def_maps,
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
    function_def_maps: &FunctionDefMaps,
    known_param_box_origins: &ParamBoxOriginMap,
    param_box_origins: &mut ParamBoxOriginMap,
) -> bool {
    let mut changed = false;
    let mut origin_queries =
        BoxOriginQueryContext::new(module, field_box_origins, known_param_box_origins);
    for function in module.functions.values() {
        let Some(def_map) = def_map_for(function_def_maps, function) else {
            continue;
        };
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
                    let Some(origin_box) =
                        origin_queries.box_origin_for_value(function, &def_map, *arg)
                    else {
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
    function_def_maps: &FunctionDefMaps,
    known_param_box_origins: &ParamBoxOriginMap,
    param_box_origins: &mut ParamBoxOriginMap,
) -> bool {
    let mut changed = false;
    let mut origin_queries =
        BoxOriginQueryContext::new(module, field_box_origins, known_param_box_origins);
    for function in module.functions.values() {
        let Some(def_map) = def_map_for(function_def_maps, function) else {
            continue;
        };
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
                            let Some(origin_box) =
                                origin_queries.box_origin_for_value(function, &def_map, *arg)
                            else {
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
                        let Some((target_box, target_symbol)) = origin_queries
                            .same_module_method_target(
                                function,
                                &def_map,
                                box_name,
                                method,
                                *receiver,
                                args.len(),
                            )
                        else {
                            continue;
                        };
                        if let Some(receiver) = receiver {
                            changed |= merge_box_origin_observation(
                                param_box_origins,
                                (target_symbol.clone(), 0),
                                target_box,
                            );
                            if let Some(receiver_box) =
                                origin_queries.box_origin_for_value(function, &def_map, *receiver)
                            {
                                changed |= merge_box_origin_observation(
                                    param_box_origins,
                                    (target_symbol.clone(), 0),
                                    receiver_box,
                                );
                            }
                        }
                        for (arg_index, arg) in args.iter().enumerate() {
                            let Some(origin_box) =
                                origin_queries.box_origin_for_value(function, &def_map, *arg)
                            else {
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

pub(super) fn infer_param_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    field_box_origins: &FieldBoxOriginMap,
) -> BTreeMap<ParamKey, FieldStorageInference> {
    let function_def_maps = build_function_def_maps(module);
    let param_box_origins =
        infer_param_box_origins_with_def_maps(module, field_box_origins, &function_def_maps);
    let mut param_storages = BTreeMap::new();
    for _ in 0..4 {
        let current = param_storages.clone();
        let collection_element_storages = infer_collection_element_storages_with_def_maps(
            module,
            inferred,
            field_box_origins,
            &current,
            &function_def_maps,
        );
        let mut changed = false;
        changed |= infer_birth_param_storages(
            module,
            inferred,
            field_box_origins,
            &function_def_maps,
            &collection_element_storages,
            &current,
            &mut param_storages,
        );
        changed |= infer_call_param_storages(
            module,
            inferred,
            field_box_origins,
            &param_box_origins,
            &function_def_maps,
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
    function_def_maps: &FunctionDefMaps,
    collection_element_storages: &CollectionElementStorageMap,
    known_param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    param_storages: &mut BTreeMap<ParamKey, FieldStorageInference>,
) -> bool {
    let mut changed = false;
    let mut storage_queries = StorageQueryContext::new(
        module,
        inferred,
        field_box_origins,
        known_param_storages,
        collection_element_storages,
    );
    for function in module.functions.values() {
        let Some(def_map) = def_map_for(function_def_maps, function) else {
            continue;
        };
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
                    let Some(storage) = storage_queries.storage_for_value(function, def_map, *arg)
                    else {
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
    function_def_maps: &FunctionDefMaps,
    collection_element_storages: &CollectionElementStorageMap,
    known_param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    param_storages: &mut BTreeMap<ParamKey, FieldStorageInference>,
) -> bool {
    let mut changed = false;
    let mut box_origin_context =
        BoxOriginQueryContext::new(module, field_box_origins, param_box_origins);
    let mut storage_queries = StorageQueryContext::new(
        module,
        inferred,
        field_box_origins,
        known_param_storages,
        collection_element_storages,
    );
    for function in module.functions.values() {
        let Some(def_map) = def_map_for(function_def_maps, function) else {
            continue;
        };
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
                            let Some(storage) =
                                storage_queries.storage_for_value(function, def_map, *arg)
                            else {
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
                        let Some((_, symbol)) = box_origin_context.same_module_method_target(
                            function,
                            &def_map,
                            box_name,
                            method,
                            *receiver,
                            args.len(),
                        ) else {
                            continue;
                        };
                        if let Some(receiver) = receiver {
                            if let Some(storage) =
                                storage_queries.storage_for_value(function, def_map, *receiver)
                            {
                                changed |= merge_param_storage_observation(
                                    param_storages,
                                    (symbol.clone(), 0),
                                    storage,
                                );
                            }
                        }
                        for (arg_index, arg) in args.iter().enumerate() {
                            let Some(storage) =
                                storage_queries.storage_for_value(function, def_map, *arg)
                            else {
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
