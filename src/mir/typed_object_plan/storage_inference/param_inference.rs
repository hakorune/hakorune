use std::collections::BTreeMap;

use crate::mir::value_origin::build_value_def_map;
use crate::mir::{Callee, MirInstruction, MirModule};

use super::collection_storage::infer_collection_element_storages;
use super::merge::{merge_box_origin_observation, merge_param_storage_observation};
use super::state::{
    CollectionElementStorageMap, FieldBoxOriginMap, FieldKey, FieldStorageInference,
    ParamBoxOriginMap, ParamKey,
};
use super::value_analysis::{box_origin_for_value, same_module_method_target, storage_for_value};

pub(super) fn infer_param_box_origins(
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

pub(super) fn infer_param_storages(
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
