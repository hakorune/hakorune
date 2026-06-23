/*!
 * Typed object layout plans for EXE lowering.
 *
 * MIR owns the object layout truth. Backends consume these plans instead of
 * rediscovering user-box declarations or cloning VM InstanceBox semantics.
 */

mod storage_inference;

use crate::mir::function::{TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, ValueId};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub const TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0: &str = "runtime_slot_object_v0";

pub fn refresh_module_typed_object_plans(module: &mut MirModule) {
    module.metadata.typed_object_plans = build_typed_object_plans(module);
}

pub fn refresh_module_typed_object_field_value_types(module: &mut MirModule) {
    let fields = typed_object_field_storage_map(module);
    for function in module.functions.values_mut() {
        refresh_function_typed_object_field_value_types(function, &fields);
    }
}

pub fn refresh_module_typed_object_collection_field_element_value_types(module: &mut MirModule) {
    let array_fields = typed_object_array_field_set(module);
    let element_types = typed_object_collection_field_element_type_map(module, &array_fields);
    if element_types.is_empty() {
        return;
    }
    for function in module.functions.values_mut() {
        refresh_function_typed_object_collection_field_element_value_types(
            function,
            &array_fields,
            &element_types,
        );
    }
}

pub fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan> {
    storage_inference::build_typed_object_plans(module)
}

type TypedObjectFieldStorageMap = BTreeMap<(String, String), TypedObjectFieldStorage>;
type TypedObjectFieldElementTypeMap = BTreeMap<(String, String), Option<MirType>>;
type TypedObjectArrayFieldSet = BTreeSet<(String, String)>;
type ObservedMethodParamTypeMap = BTreeMap<(String, usize), Option<MirType>>;

fn typed_object_field_storage_map(module: &MirModule) -> TypedObjectFieldStorageMap {
    let mut fields = BTreeMap::new();
    for plan in &module.metadata.typed_object_plans {
        for field in &plan.fields {
            fields.insert((plan.box_name.clone(), field.name.clone()), field.storage);
        }
    }
    fields
}

fn typed_object_array_field_set(module: &MirModule) -> TypedObjectArrayFieldSet {
    let mut fields = BTreeSet::new();
    for (box_name, decls) in &module.metadata.user_box_field_decls {
        for decl in decls {
            if decl.declared_type_name.as_deref() == Some("ArrayBox") {
                fields.insert((box_name.clone(), decl.name.clone()));
            }
        }
    }
    fields
}

fn typed_object_collection_field_element_type_map(
    module: &MirModule,
    array_fields: &TypedObjectArrayFieldSet,
) -> TypedObjectFieldElementTypeMap {
    let mut fields = BTreeMap::new();
    let observed_param_types = observed_method_param_type_map(module);
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                let MirInstruction::Call {
                    callee:
                        Some(crate::mir::Callee::Method {
                            box_name,
                            method,
                            receiver: Some(receiver),
                            ..
                        }),
                    args,
                    ..
                } = instruction
                else {
                    continue;
                };
                if !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") {
                    continue;
                }
                let Some(value_arg) = array_write_value_arg(method, args, *receiver) else {
                    continue;
                };
                let Some((owner_box, field)) =
                    typed_object_field_array_origin(function, &def_map, array_fields, *receiver)
                else {
                    continue;
                };
                if typed_object_field_array_origin(function, &def_map, array_fields, value_arg)
                    .or_else(|| {
                        typed_object_field_array_read_origin(
                            function,
                            &def_map,
                            array_fields,
                            value_arg,
                        )
                    })
                    == Some((owner_box.clone(), field.clone()))
                {
                    continue;
                }
                let value_type =
                    publishable_value_type(function, &def_map, &observed_param_types, value_arg);
                merge_field_element_type(&mut fields, (owner_box, field), value_type);
            }
        }
    }
    fields
}

fn typed_object_field_array_read_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    array_fields: &TypedObjectArrayFieldSet,
    value: ValueId,
) -> Option<(String, String)> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    let MirInstruction::Call {
        dst,
        callee:
            Some(crate::mir::Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = block.instructions.get(instruction_index)?
    else {
        return None;
    };
    if *dst != Some(origin) || !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") {
        return None;
    }
    if !array_read_args_supported(method, args, *receiver) {
        return None;
    }
    typed_object_field_array_origin(function, def_map, array_fields, *receiver)
}

fn observed_method_param_type_map(module: &MirModule) -> ObservedMethodParamTypeMap {
    let mut facts = BTreeMap::new();
    for function in module.functions.values() {
        let def_map = build_value_def_map(function);
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                let MirInstruction::Call {
                    callee:
                        Some(crate::mir::Callee::Method {
                            box_name,
                            method,
                            receiver: Some(receiver),
                            ..
                        }),
                    args,
                    ..
                } = instruction
                else {
                    continue;
                };
                let user_args = logical_method_args(args, *receiver);
                let target = format!("{}.{}/{}", box_name, method, user_args.len());
                for (user_index, arg) in user_args.iter().enumerate() {
                    let param_index = user_index + 1;
                    let value_type = local_publishable_value_type(function, &def_map, *arg);
                    merge_param_type(&mut facts, (target.clone(), param_index), value_type);
                }
            }
        }
    }
    facts
}

fn refresh_function_typed_object_collection_field_element_value_types(
    function: &mut MirFunction,
    array_fields: &TypedObjectArrayFieldSet,
    element_types: &TypedObjectFieldElementTypeMap,
) {
    let def_map = build_value_def_map(function);
    let mut facts = Vec::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::Call {
                dst,
                callee:
                    Some(crate::mir::Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            if !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") {
                continue;
            }
            if !matches!(method.as_str(), "get" | "pop" | "remove") {
                continue;
            }
            let Some(dst) = dst else {
                continue;
            };
            let Some((owner_box, field)) =
                typed_object_field_array_origin(function, &def_map, array_fields, *receiver)
            else {
                continue;
            };
            if !array_read_args_supported(method, args, *receiver) {
                continue;
            }
            let Some(Some(element_type)) = element_types.get(&(owner_box, field)) else {
                continue;
            };
            facts.push((*dst, element_type.clone()));
        }
    }
    for (value, ty) in facts {
        if is_publishable_element_type(&ty) {
            function.metadata.value_types.insert(value, ty);
        }
    }
}

fn refresh_function_typed_object_field_value_types(
    function: &mut MirFunction,
    fields: &TypedObjectFieldStorageMap,
) {
    let def_map = build_value_def_map(function);
    let mut facts = Vec::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            let MirInstruction::FieldGet {
                dst, base, field, ..
            } = instruction
            else {
                continue;
            };
            let Some(base_box) = typed_object_value_box_name(function, &def_map, *base) else {
                continue;
            };
            if fields
                .get(&(base_box, field.clone()))
                .is_some_and(|storage| storage.uses_integer_lane())
            {
                facts.push(*dst);
            }
        }
    }
    for value in facts {
        function
            .metadata
            .value_types
            .insert(value, MirType::Integer);
    }
}

fn typed_object_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    function
        .metadata
        .value_types
        .get(&origin)
        .and_then(box_name_from_mir_type)
        .map(str::to_string)
        .or_else(|| {
            def_map
                .get(&origin)
                .and_then(|(block_id, instruction_index)| {
                    let block = function.blocks.get(block_id)?;
                    match block.instructions.get(*instruction_index)? {
                        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
                        MirInstruction::Phi { type_hint, .. } => type_hint
                            .as_ref()
                            .and_then(box_name_from_mir_type)
                            .map(str::to_string),
                        _ => None,
                    }
                })
        })
        .or_else(|| method_receiver_box_name(function, origin))
}

fn typed_object_field_array_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    array_fields: &TypedObjectArrayFieldSet,
    value: ValueId,
) -> Option<(String, String)> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    let MirInstruction::FieldGet {
        base,
        declared_type,
        field,
        dst,
        ..
    } = block.instructions.get(instruction_index)?
    else {
        return None;
    };
    if *dst != origin {
        return None;
    }
    let owner_box = typed_object_value_box_name(function, def_map, *base)?;
    let declared_array = declared_type
        .as_ref()
        .is_some_and(|ty| box_name_from_mir_type(ty) == Some("ArrayBox"));
    if !declared_array && !array_fields.contains(&(owner_box.clone(), field.clone())) {
        return None;
    }
    Some((owner_box, field.clone()))
}

fn array_write_value_arg(method: &str, args: &[ValueId], _receiver: ValueId) -> Option<ValueId> {
    match method {
        "push" | "set" | "insert" => args.last().copied(),
        _ => None,
    }
}

fn array_read_args_supported(method: &str, args: &[ValueId], receiver: ValueId) -> bool {
    let user_args = logical_method_args(args, receiver);
    match method {
        "get" | "remove" => user_args.len() == 1,
        "pop" => user_args.is_empty(),
        _ => false,
    }
}

fn logical_method_args(args: &[ValueId], receiver: ValueId) -> &[ValueId] {
    if args.first().copied() == Some(receiver) {
        &args[1..]
    } else {
        args
    }
}

fn publishable_value_type(
    function: &MirFunction,
    def_map: &ValueDefMap,
    observed_param_types: &ObservedMethodParamTypeMap,
    value: ValueId,
) -> Option<MirType> {
    let origin = resolve_value_origin(function, def_map, value);
    let ty = local_publishable_value_type(function, def_map, origin).or_else(|| {
        function
            .params
            .iter()
            .position(|param| *param == origin)
            .and_then(|index| observed_param_types.get(&(function.signature.name.clone(), index)))
            .cloned()
            .flatten()
    })?;
    is_publishable_element_type(&ty).then_some(ty)
}

fn local_publishable_value_type(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<MirType> {
    let origin = resolve_value_origin(function, def_map, value);
    function
        .metadata
        .value_types
        .get(&origin)
        .cloned()
        .or_else(|| {
            def_map
                .get(&origin)
                .and_then(|(block_id, instruction_index)| {
                    let block = function.blocks.get(block_id)?;
                    match block.instructions.get(*instruction_index)? {
                        MirInstruction::Const {
                            value: crate::mir::ConstValue::Integer(_),
                            ..
                        } => Some(MirType::Integer),
                        MirInstruction::Const {
                            value: crate::mir::ConstValue::String(_),
                            ..
                        } => Some(MirType::String),
                        MirInstruction::NewBox { box_type, .. } => {
                            Some(MirType::Box(box_type.clone()))
                        }
                        _ => None,
                    }
                })
        })
        .filter(is_publishable_element_type)
}

fn merge_field_element_type(
    fields: &mut TypedObjectFieldElementTypeMap,
    key: (String, String),
    next: Option<MirType>,
) {
    let Some(next) = next else {
        fields.insert(key, None);
        return;
    };
    match fields.get(&key) {
        None => {
            fields.insert(key, Some(next));
        }
        Some(Some(existing)) if existing == &next => {}
        Some(_) => {
            fields.insert(key, None);
        }
    }
}

fn merge_param_type(
    facts: &mut ObservedMethodParamTypeMap,
    key: (String, usize),
    next: Option<MirType>,
) {
    let Some(next) = next else {
        return;
    };
    match facts.get(&key) {
        None => {
            facts.insert(key, Some(next));
        }
        Some(Some(existing)) if existing == &next => {}
        Some(_) => {
            facts.insert(key, None);
        }
    }
}

fn is_publishable_element_type(ty: &MirType) -> bool {
    !matches!(ty, MirType::Unknown | MirType::Void)
}

fn method_receiver_box_name(function: &MirFunction, value: ValueId) -> Option<String> {
    if function.params.first().copied() != Some(value) {
        return None;
    }
    let (owner_and_method, _arity) = function.signature.name.rsplit_once('/')?;
    let (box_name, _method) = owner_and_method.rsplit_once('.')?;
    Some(box_name.to_string())
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
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::function::UserBoxFieldDecl;
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule,
    };

    #[test]
    fn publishes_integer_result_for_typed_object_array_field_get() {
        let mut module = MirModule::new("typed_object_array_element_type_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Holder".to_string(), vec!["values".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Holder".to_string(),
            vec![UserBoxFieldDecl {
                name: "values".to_string(),
                declared_type_name: Some("ArrayBox".to_string()),
                is_weak: false,
            }],
        );
        refresh_module_typed_object_plans(&mut module);

        let mut push_fn = MirFunction::new(
            FunctionSignature {
                name: "Holder.push_value/1".to_string(),
                params: vec![MirType::Unknown, MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        push_fn.params = vec![ValueId::new(0), ValueId::new(1)];
        push_fn
            .metadata
            .value_types
            .insert(ValueId::new(1), MirType::Integer);
        let push_block = push_fn
            .get_block_mut(BasicBlockId::new(0))
            .expect("push entry");
        push_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(0),
            field: "values".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        push_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1)],
            effects: EffectMask::IO,
        });

        let mut read_fn = MirFunction::new(
            FunctionSignature {
                name: "Holder.read_first/0".to_string(),
                params: vec![MirType::Unknown],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        read_fn.params = vec![ValueId::new(0)];
        let read_block = read_fn
            .get_block_mut(BasicBlockId::new(0))
            .expect("read entry");
        read_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(0),
            field: "values".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        read_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(0),
        });
        read_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3)],
            effects: EffectMask::PURE,
        });

        module.add_function(push_fn);
        module.add_function(read_fn);
        refresh_module_typed_object_field_value_types(&mut module);
        refresh_module_typed_object_collection_field_element_value_types(&mut module);

        let read_fn = module
            .get_function("Holder.read_first/0")
            .expect("read function");
        assert_eq!(
            read_fn.metadata.value_types.get(&ValueId::new(4)),
            Some(&MirType::Integer)
        );
    }

    #[test]
    fn publishes_integer_result_from_observed_method_param_array_write() {
        let mut module = MirModule::new("typed_object_array_observed_param_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Holder".to_string(), vec!["values".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Holder".to_string(),
            vec![UserBoxFieldDecl {
                name: "values".to_string(),
                declared_type_name: Some("ArrayBox".to_string()),
                is_weak: false,
            }],
        );
        refresh_module_typed_object_plans(&mut module);

        let mut set_fn = MirFunction::new(
            FunctionSignature {
                name: "Holder.set/1".to_string(),
                params: vec![MirType::Unknown, MirType::Unknown],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        set_fn.params = vec![ValueId::new(0), ValueId::new(1)];
        let set_block = set_fn
            .get_block_mut(BasicBlockId::new(0))
            .expect("set entry");
        set_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(0),
            field: "values".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        set_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(1),
        });
        set_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::IO,
        });
        set_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(4),
            base: ValueId::new(0),
            field: "values".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        set_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Integer(0),
        });
        set_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(6)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(5)],
            effects: EffectMask::PURE,
        });
        set_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(5), ValueId::new(6)],
            effects: EffectMask::IO,
        });

        let mut caller = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let caller_block = caller
            .get_block_mut(BasicBlockId::new(0))
            .expect("caller entry");
        caller_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(10),
            box_type: "Holder".to_string(),
            args: vec![],
        });
        caller_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Integer(7),
        });
        caller_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Holder".to_string(),
                method: "set".to_string(),
                receiver: Some(ValueId::new(10)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(11)],
            effects: EffectMask::IO,
        });

        let mut read_fn = MirFunction::new(
            FunctionSignature {
                name: "Holder.read_first/0".to_string(),
                params: vec![MirType::Unknown],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        read_fn.params = vec![ValueId::new(0)];
        let read_block = read_fn
            .get_block_mut(BasicBlockId::new(0))
            .expect("read entry");
        read_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(0),
            field: "values".to_string(),
            declared_type: Some(MirType::Box("ArrayBox".to_string())),
        });
        read_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(0),
        });
        read_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3)],
            effects: EffectMask::PURE,
        });

        module.add_function(set_fn);
        module.add_function(caller);
        module.add_function(read_fn);
        refresh_module_typed_object_field_value_types(&mut module);
        refresh_module_typed_object_collection_field_element_value_types(&mut module);

        let read_fn = module
            .get_function("Holder.read_first/0")
            .expect("read function");
        assert_eq!(
            read_fn.metadata.value_types.get(&ValueId::new(4)),
            Some(&MirType::Integer)
        );
    }
}
