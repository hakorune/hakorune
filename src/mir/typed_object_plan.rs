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

pub fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan> {
    storage_inference::build_typed_object_plans(module)
}

type TypedObjectFieldStorageMap = BTreeMap<(String, String), TypedObjectFieldStorage>;

fn typed_object_field_storage_map(module: &MirModule) -> TypedObjectFieldStorageMap {
    let mut fields = BTreeMap::new();
    for plan in &module.metadata.typed_object_plans {
        for field in &plan.fields {
            fields.insert((plan.box_name.clone(), field.name.clone()), field.storage);
        }
    }
    fields
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
            if fields.get(&(base_box, field.clone())) == Some(&TypedObjectFieldStorage::I64) {
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
}

fn box_name_from_mir_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}
