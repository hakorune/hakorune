use std::collections::{BTreeMap, BTreeSet};

use super::TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0;
use crate::mir::function::{
    ModuleMetadata, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan,
};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{
    BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType,
    UserBoxFieldDecl, ValueId,
};

type FieldKey = (String, String);
type ParamKey = (String, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldStorageInference {
    Known(TypedObjectFieldStorage),
    Conflict,
}

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
    let mut inferred = BTreeMap::new();

    for _ in 0..4 {
        let mut changed = false;
        let param_storages = infer_param_storages(module, &inferred);
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
                                &param_storages,
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

fn infer_param_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
) -> BTreeMap<ParamKey, FieldStorageInference> {
    let mut param_storages = BTreeMap::new();
    for _ in 0..4 {
        let current = param_storages.clone();
        let mut changed = false;
        changed |= infer_birth_param_storages(module, inferred, &current, &mut param_storages);
        changed |= infer_call_param_storages(module, inferred, &current, &mut param_storages);
        if !changed {
            break;
        }
    }
    param_storages
}

fn infer_birth_param_storages(
    module: &MirModule,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
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
                        known_param_storages,
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
                                known_param_storages,
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
                        let symbol = format!("{box_name}.{method}/{}", args.len());
                        if !module.functions.contains_key(&symbol) {
                            continue;
                        }
                        if let Some(receiver) = receiver {
                            if let Some(storage) = storage_for_value(
                                module,
                                function,
                                &def_map,
                                *receiver,
                                inferred,
                                known_param_storages,
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
                                known_param_storages,
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

fn box_name_for_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    value_type_for(function, value)
        .or_else(|| value_type_for(function, origin))
        .and_then(box_name_from_mir_type)
        .map(str::to_string)
        .or_else(|| box_name_from_origin_instruction(function, def_map, origin))
}

fn box_name_from_origin_instruction(
    function: &MirFunction,
    def_map: &ValueDefMap,
    origin: ValueId,
) -> Option<String> {
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
        MirInstruction::Phi { type_hint, .. } => type_hint
            .as_ref()
            .and_then(box_name_from_mir_type)
            .map(str::to_string),
        _ => None,
    }
}

fn storage_for_value(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
) -> Option<TypedObjectFieldStorage> {
    let mut visiting_globals = BTreeSet::new();
    let mut visiting_values = BTreeSet::new();
    storage_for_value_inner(
        module,
        function,
        def_map,
        value,
        inferred,
        param_storages,
        &mut visiting_globals,
        &mut visiting_values,
    )
}

fn storage_for_value_inner(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    visiting_globals: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
) -> Option<TypedObjectFieldStorage> {
    let origin = resolve_value_origin(function, def_map, value);
    let value_key = (function.signature.name.clone(), origin);
    if !visiting_values.insert(value_key.clone()) {
        return None;
    }
    let origin_storage = storage_from_origin_instruction(
        module,
        function,
        def_map,
        origin,
        inferred,
        param_storages,
        visiting_globals,
        visiting_values,
    );
    visiting_values.remove(&value_key);
    origin_storage.or_else(|| {
        value_type_for(function, value)
            .or_else(|| value_type_for(function, origin))
            .and_then(storage_for_mir_type)
    })
}

fn storage_from_origin_instruction(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    origin: ValueId,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    visiting_globals: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
) -> Option<TypedObjectFieldStorage> {
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return storage_from_param(function, origin, param_storages);
    };
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::Const { value, .. } => storage_for_const(value),
        MirInstruction::BinOp { op, lhs, rhs, .. } => storage_for_binop(
            module,
            function,
            def_map,
            *op,
            *lhs,
            *rhs,
            inferred,
            param_storages,
            visiting_globals,
            visiting_values,
        ),
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } => storage_for_global_return(
            module,
            name,
            inferred,
            param_storages,
            visiting_globals,
            visiting_values,
        ),
        MirInstruction::Phi {
            inputs, type_hint, ..
        } => type_hint
            .as_ref()
            .and_then(storage_for_mir_type)
            .or_else(|| {
                let mut observed = None;
                for (_, input) in inputs {
                    let storage = storage_for_value_inner(
                        module,
                        function,
                        def_map,
                        *input,
                        inferred,
                        param_storages,
                        visiting_globals,
                        visiting_values,
                    )?;
                    match observed {
                        Some(existing) if existing != storage => return None,
                        Some(_) => {}
                        None => observed = Some(storage),
                    }
                }
                observed
            }),
        MirInstruction::NewBox { .. } | MirInstruction::NewClosure { .. } => {
            Some(TypedObjectFieldStorage::Handle)
        }
        MirInstruction::FieldGet {
            base,
            field,
            declared_type,
            ..
        } => declared_type
            .as_ref()
            .and_then(storage_for_mir_type)
            .or_else(|| field_storage_for_get(function, def_map, *base, field, inferred)),
        _ => None,
    }
}

fn storage_for_binop(
    module: &MirModule,
    function: &MirFunction,
    def_map: &ValueDefMap,
    op: BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    visiting_globals: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
) -> Option<TypedObjectFieldStorage> {
    let lhs_storage = storage_for_value_inner(
        module,
        function,
        def_map,
        lhs,
        inferred,
        param_storages,
        visiting_globals,
        visiting_values,
    );
    let rhs_storage = storage_for_value_inner(
        module,
        function,
        def_map,
        rhs,
        inferred,
        param_storages,
        visiting_globals,
        visiting_values,
    );
    match op {
        BinaryOp::Add
            if lhs_storage == Some(TypedObjectFieldStorage::Handle)
                || rhs_storage == Some(TypedObjectFieldStorage::Handle) =>
        {
            Some(TypedObjectFieldStorage::Handle)
        }
        BinaryOp::Add
        | BinaryOp::Sub
        | BinaryOp::Mul
        | BinaryOp::Div
        | BinaryOp::Mod
        | BinaryOp::BitAnd
        | BinaryOp::BitOr
        | BinaryOp::BitXor
        | BinaryOp::Shl
        | BinaryOp::Shr
        | BinaryOp::And
        | BinaryOp::Or
            if lhs_storage == Some(TypedObjectFieldStorage::I64)
                && rhs_storage == Some(TypedObjectFieldStorage::I64) =>
        {
            Some(TypedObjectFieldStorage::I64)
        }
        _ => None,
    }
}

fn storage_for_global_return(
    module: &MirModule,
    name: &str,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    visiting_globals: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
) -> Option<TypedObjectFieldStorage> {
    if !visiting_globals.insert(name.to_string()) {
        return None;
    }
    let storage = module.functions.get(name).and_then(|target| {
        storage_for_function_returns(
            module,
            target,
            inferred,
            param_storages,
            visiting_globals,
            visiting_values,
        )
    });
    visiting_globals.remove(name);
    storage
}

fn storage_for_function_returns(
    module: &MirModule,
    function: &MirFunction,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
    visiting_globals: &mut BTreeSet<String>,
    visiting_values: &mut BTreeSet<(String, ValueId)>,
) -> Option<TypedObjectFieldStorage> {
    let def_map = build_value_def_map(function);
    let mut observed = None;
    for block in function.blocks.values() {
        for inst in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value } = inst else {
                continue;
            };
            let value = (*value)?;
            let storage = storage_for_value_inner(
                module,
                function,
                &def_map,
                value,
                inferred,
                param_storages,
                visiting_globals,
                visiting_values,
            )?;
            match observed {
                Some(existing) if existing != storage => return None,
                Some(_) => {}
                None => observed = Some(storage),
            }
        }
    }
    observed
}

fn storage_from_param(
    function: &MirFunction,
    value: ValueId,
    param_storages: &BTreeMap<ParamKey, FieldStorageInference>,
) -> Option<TypedObjectFieldStorage> {
    let param_index = function.params.iter().position(|param| *param == value)?;
    match param_storages.get(&(function.signature.name.clone(), param_index)) {
        Some(FieldStorageInference::Known(storage)) => Some(*storage),
        Some(FieldStorageInference::Conflict) | None => None,
    }
}

fn field_storage_for_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    base: ValueId,
    field: &str,
    inferred: &BTreeMap<FieldKey, FieldStorageInference>,
) -> Option<TypedObjectFieldStorage> {
    let box_name = box_name_for_value(function, def_map, base)?;
    match inferred.get(&(box_name, field.to_string())) {
        Some(FieldStorageInference::Known(storage)) => Some(*storage),
        Some(FieldStorageInference::Conflict) | None => None,
    }
}

fn value_type_for(function: &MirFunction, value: ValueId) -> Option<&MirType> {
    function
        .metadata
        .value_types
        .get(&value)
        .or_else(|| function.signature.params.get(value.to_usize()))
}

fn storage_for_const(value: &ConstValue) -> Option<TypedObjectFieldStorage> {
    match value {
        ConstValue::Integer(_) | ConstValue::Bool(_) => Some(TypedObjectFieldStorage::I64),
        ConstValue::String(_) => Some(TypedObjectFieldStorage::Handle),
        ConstValue::Null | ConstValue::Void | ConstValue::Float(_) => None,
    }
}

fn storage_for_declared_type(
    metadata: &ModuleMetadata,
    type_name: Option<&str>,
) -> Option<TypedObjectFieldStorage> {
    match type_name {
        Some("IntegerBox") | Some("Integer") | Some("i64") | Some("BoolBox") | Some("Bool")
        | Some("bool") | Some("i1") => Some(TypedObjectFieldStorage::I64),
        Some("StringBox") | Some("String") | Some("str") | Some("ArrayBox") | Some("MapBox") => {
            Some(TypedObjectFieldStorage::Handle)
        }
        Some(name) if metadata.user_box_decls.contains_key(name) => {
            Some(TypedObjectFieldStorage::Handle)
        }
        Some(name) if metadata.user_box_field_decls.contains_key(name) => {
            Some(TypedObjectFieldStorage::Handle)
        }
        _ => None,
    }
}

fn storage_for_mir_type(ty: &MirType) -> Option<TypedObjectFieldStorage> {
    match ty {
        MirType::Integer | MirType::Bool => Some(TypedObjectFieldStorage::I64),
        MirType::Box(name) if matches!(name.as_str(), "IntegerBox" | "BoolBox") => {
            Some(TypedObjectFieldStorage::I64)
        }
        MirType::String | MirType::Box(_) | MirType::Array(_) | MirType::Future(_) => {
            Some(TypedObjectFieldStorage::Handle)
        }
        MirType::Float | MirType::WeakRef | MirType::Void | MirType::Unknown => None,
    }
}

fn box_name_from_mir_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}
