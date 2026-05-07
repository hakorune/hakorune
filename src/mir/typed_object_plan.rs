/*!
 * Typed object layout plans for EXE lowering.
 *
 * MIR owns the object layout truth. Backends consume these plans instead of
 * rediscovering user-box declarations or cloning VM InstanceBox semantics.
 */

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{
    function::{
        MirFunction, ModuleMetadata, TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan,
    },
    value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap},
    BinaryOp, Callee, ConstValue, MirInstruction, MirModule, MirType, UserBoxFieldDecl, ValueId,
};

pub const TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0: &str = "runtime_slot_object_v0";

pub fn refresh_module_typed_object_plans(module: &mut MirModule) {
    module.metadata.typed_object_plans = build_typed_object_plans(module);
}

pub fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan> {
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

type FieldKey = (String, String);
type ParamKey = (String, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldStorageInference {
    Known(TypedObjectFieldStorage),
    Conflict,
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
                    // Method parameter 0 is the receiver; explicit constructor
                    // arguments start at parameter 1.
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
        // Null is compatible with the first concrete storage observed for the field.
        ConstValue::Null | ConstValue::Void => None,
        ConstValue::Float(_) => None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, Callee, EffectMask, FunctionSignature, MirInstruction,
        MirType, UserBoxFieldDecl, ValueId,
    };

    fn module_with_metadata(metadata: ModuleMetadata) -> MirModule {
        let mut module = MirModule::new("typed_object_plan_test".to_string());
        module.metadata = metadata;
        module
    }

    #[test]
    fn build_typed_object_plans_accepts_nonweak_i64_fields() {
        let mut metadata = ModuleMetadata::default();
        metadata.user_box_field_decls.insert(
            "Pair".to_string(),
            vec![
                UserBoxFieldDecl {
                    name: "left".to_string(),
                    declared_type_name: Some("IntegerBox".to_string()),
                    is_weak: false,
                },
                UserBoxFieldDecl {
                    name: "right".to_string(),
                    declared_type_name: Some("Integer".to_string()),
                    is_weak: false,
                },
            ],
        );
        let module = module_with_metadata(metadata);

        let plans = build_typed_object_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].box_name, "Pair");
        assert_eq!(plans[0].type_id, 1);
        assert_eq!(
            plans[0].layout_kind,
            TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0
        );
        assert_eq!(plans[0].field_count, 2);
        assert_eq!(plans[0].fields[0].slot, 0);
        assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
        assert_eq!(plans[0].fields[1].slot, 1);
    }

    #[test]
    fn build_typed_object_plans_rejects_weak_or_unknown_storage() {
        let mut metadata = ModuleMetadata::default();
        metadata.user_box_field_decls.insert(
            "WeakBox".to_string(),
            vec![UserBoxFieldDecl {
                name: "next".to_string(),
                declared_type_name: Some("IntegerBox".to_string()),
                is_weak: true,
            }],
        );
        metadata.user_box_field_decls.insert(
            "AnyBox".to_string(),
            vec![UserBoxFieldDecl {
                name: "value".to_string(),
                declared_type_name: None,
                is_weak: false,
            }],
        );
        let module = module_with_metadata(metadata);

        let plans = build_typed_object_plans(&module);

        assert!(plans.is_empty());
    }

    #[test]
    fn build_typed_object_plans_infers_untyped_i64_and_handle_fields() {
        let mut module = MirModule::new("typed_object_infer".to_string());
        module.metadata.user_box_field_decls.insert(
            "Holder".to_string(),
            vec![
                UserBoxFieldDecl {
                    name: "count".to_string(),
                    declared_type_name: None,
                    is_weak: false,
                },
                UserBoxFieldDecl {
                    name: "items".to_string(),
                    declared_type_name: None,
                    is_weak: false,
                },
            ],
        );

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Holder".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(1),
            field: "count".to_string(),
            value: ValueId::new(2),
            declared_type: None,
        });
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(1),
            field: "items".to_string(),
            value: ValueId::new(3),
            declared_type: None,
        });
        function.add_block(block);
        module.add_function(function);

        let plans = build_typed_object_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].box_name, "Holder");
        assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
        assert_eq!(plans[0].fields[1].storage, TypedObjectFieldStorage::Handle);
    }

    #[test]
    fn build_typed_object_plans_infers_handle_from_same_module_string_like_global_return() {
        let mut module = MirModule::new("typed_object_global_return_infer".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Manifest".to_string(), vec!["root_id".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Manifest".to_string(),
            vec![UserBoxFieldDecl {
                name: "root_id".to_string(),
                declared_type_name: None,
                is_weak: false,
            }],
        );

        let mut digest = MirFunction::new(
            FunctionSignature {
                name: "Hasher.digest/1".to_string(),
                params: vec![MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        digest.params = vec![ValueId::new(0)];
        let mut digest_block = BasicBlock::new(BasicBlockId::new(0));
        digest_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("bt-".to_string()),
        });
        digest_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(7),
        });
        digest_block.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        });
        digest_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        digest.add_block(digest_block);

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Manifest.birth/0".to_string(),
                params: vec![MirType::Box("Manifest".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String(String::new()),
        });
        birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "root_id".to_string(),
            value: ValueId::new(1),
            declared_type: None,
        });
        birth.add_block(birth_block);

        let mut seal = MirFunction::new(
            FunctionSignature {
                name: "Manifest.seal/0".to_string(),
                params: vec![MirType::Box("Manifest".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        seal.params = vec![ValueId::new(0)];
        seal.metadata
            .value_types
            .insert(ValueId::new(2), MirType::Integer);
        let mut seal_block = BasicBlock::new(BasicBlockId::new(0));
        seal_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(13),
        });
        seal_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Hasher.digest/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
        seal_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "root_id".to_string(),
            value: ValueId::new(2),
            declared_type: None,
        });
        seal.add_block(seal_block);

        module.add_function(digest);
        module.add_function(birth);
        module.add_function(seal);

        let plans = build_typed_object_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].box_name, "Manifest");
        assert_eq!(plans[0].fields[0].name, "root_id");
        assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::Handle);
    }

    #[test]
    fn build_typed_object_plans_rejects_conflicting_untyped_storage() {
        let mut module = MirModule::new("typed_object_conflict".to_string());
        module.metadata.user_box_field_decls.insert(
            "Bad".to_string(),
            vec![UserBoxFieldDecl {
                name: "value".to_string(),
                declared_type_name: None,
                is_weak: false,
            }],
        );

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Bad".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(1),
            field: "value".to_string(),
            value: ValueId::new(2),
            declared_type: None,
        });
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(1),
            field: "value".to_string(),
            value: ValueId::new(3),
            declared_type: None,
        });
        function.add_block(block);
        module.add_function(function);

        let plans = build_typed_object_plans(&module);

        assert!(plans.is_empty());
    }

    #[test]
    fn build_typed_object_plans_infers_birth_param_field_storage_from_newbox_args() {
        let mut module = MirModule::new("typed_object_birth_param".to_string());
        module.metadata.user_box_decls.insert(
            "Page".to_string(),
            vec!["page_id".to_string(), "capacity".to_string()],
        );
        module.metadata.user_box_field_decls.insert(
            "Page".to_string(),
            vec![
                UserBoxFieldDecl {
                    name: "page_id".to_string(),
                    declared_type_name: None,
                    is_weak: false,
                },
                UserBoxFieldDecl {
                    name: "capacity".to_string(),
                    declared_type_name: None,
                    is_weak: false,
                },
            ],
        );

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Page.birth/2".to_string(),
                params: vec![
                    MirType::Box("Page".to_string()),
                    MirType::Unknown,
                    MirType::Unknown,
                ],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(0),
        });
        birth_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(1),
        });
        birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(3),
            field: "page_id".to_string(),
            value: ValueId::new(4),
            declared_type: None,
        });
        birth_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(5),
            src: ValueId::new(2),
        });
        birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(3),
            field: "capacity".to_string(),
            value: ValueId::new(5),
            declared_type: None,
        });
        birth.add_block(birth_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut main_block = BasicBlock::new(BasicBlockId::new(0));
        main_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        main_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(9),
        });
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "Page".to_string(),
            args: vec![ValueId::new(1), ValueId::new(2)],
        });
        main.add_block(main_block);
        module.add_function(birth);
        module.add_function(main);

        let plans = build_typed_object_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].box_name, "Page");
        assert_eq!(plans[0].fields[0].storage, TypedObjectFieldStorage::I64);
        assert_eq!(plans[0].fields[1].storage, TypedObjectFieldStorage::I64);
    }

    #[test]
    fn build_typed_object_plans_infers_birth_param_storage_through_same_module_method_call() {
        let mut module = MirModule::new("typed_object_method_param_flow".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Factory".to_string(), Vec::new());
        module
            .metadata
            .user_box_decls
            .insert("Item".to_string(), vec!["name".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Item".to_string(),
            vec![UserBoxFieldDecl {
                name: "name".to_string(),
                declared_type_name: None,
                is_weak: false,
            }],
        );

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Item.birth/1".to_string(),
                params: vec![MirType::Box("Item".to_string()), MirType::Unknown],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "name".to_string(),
            value: ValueId::new(1),
            declared_type: None,
        });
        birth.add_block(birth_block);

        let mut make = MirFunction::new(
            FunctionSignature {
                name: "Factory.make/1".to_string(),
                params: vec![MirType::Box("Factory".to_string()), MirType::Unknown],
                return_type: MirType::Box("Item".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        make.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut make_block = BasicBlock::new(BasicBlockId::new(0));
        make_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "Item".to_string(),
            args: vec![ValueId::new(1)],
        });
        make.add_block(make_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut main_block = BasicBlock::new(BasicBlockId::new(0));
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Factory".to_string(),
            args: vec![],
        });
        main_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("item-a".to_string()),
        });
        main_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Factory".to_string(),
                method: "make".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        });
        main.add_block(main_block);

        module.add_function(birth);
        module.add_function(make);
        module.add_function(main);

        let plans = build_typed_object_plans(&module);

        let item = plans
            .iter()
            .find(|plan| plan.box_name == "Item")
            .expect("Item typed object plan");
        assert_eq!(item.fields[0].storage, TypedObjectFieldStorage::Handle);
    }

    #[test]
    fn build_typed_object_plans_accepts_observed_empty_user_box() {
        let mut module = MirModule::new("typed_object_empty".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Worker".to_string(), Vec::new());
        module
            .metadata
            .user_box_field_decls
            .insert("Worker".to_string(), Vec::new());

        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Worker".to_string(),
            args: vec![],
        });
        function.add_block(block);
        module.add_function(function);

        let plans = build_typed_object_plans(&module);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].box_name, "Worker");
        assert_eq!(plans[0].field_count, 0);
        assert!(plans[0].fields.is_empty());
    }
}
