//! Direct-JSON representation witness for passive Ownership SSA transport.
//!
//! Ownership opcodes are transportable before they have production callers,
//! but direct JSON must not invent the strong-ownable representation.  This
//! box accepts only exact `MirType::Box` + `StorageClass::BoxRef` metadata for
//! every value named by `CopyOwned` / `DestroyOwned`.

use crate::mir::{storage_class::StorageClass, MirFunction, MirInstruction, MirType, ValueId};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

pub(crate) fn apply_and_verify(
    function_json: &Value,
    function: &mut MirFunction,
) -> Result<(), String> {
    let ownership_values = collect_ownership_values(function)?;
    if ownership_values.is_empty() {
        return Ok(());
    }

    let metadata = function_json
        .get("metadata")
        .and_then(Value::as_object)
        .ok_or_else(|| freeze("ownership op requires function metadata"))?;
    let value_types = required_map(metadata, "value_types")?;
    let storage_classes = required_map(metadata, "storage_classes")?;

    for value in ownership_values {
        let key = value.as_u32().to_string();
        let ty = parse_box_type(value_types.get(&key), value)?;
        let storage = storage_classes
            .get(&key)
            .and_then(Value::as_str)
            .ok_or_else(|| freeze(&format!("%{} missing storage class", value.as_u32())))?;
        if storage != StorageClass::BoxRef.as_str() {
            return Err(freeze(&format!(
                "%{} storage must be box_ref, got {storage}",
                value.as_u32()
            )));
        }
        function.metadata.value_types.insert(value, ty);
        function
            .metadata
            .value_storage_classes
            .insert(value, StorageClass::BoxRef);
    }

    verify_copy_owned_type_equality(function)
}

fn collect_ownership_values(function: &MirFunction) -> Result<BTreeSet<ValueId>, String> {
    let mut values = BTreeSet::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::CopyOwned { dst, src } => {
                    if dst == src {
                        return Err(freeze("copy_owned dst must differ from src"));
                    }
                    values.insert(*dst);
                    values.insert(*src);
                }
                MirInstruction::DestroyOwned { value } => {
                    values.insert(*value);
                }
                _ => {}
            }
        }
    }
    Ok(values)
}

fn required_map<'a>(
    metadata: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a Map<String, Value>, String> {
    metadata
        .get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| freeze(&format!("ownership op requires metadata.{field}")))
}

fn parse_box_type(value: Option<&Value>, id: ValueId) -> Result<MirType, String> {
    let object = value
        .and_then(Value::as_object)
        .ok_or_else(|| freeze(&format!("%{} missing exact value type", id.as_u32())))?;
    if object.get("kind").and_then(Value::as_str) != Some("handle") {
        return Err(freeze(&format!("%{} type must be a handle", id.as_u32())));
    }
    let box_type = object
        .get("box_type")
        .and_then(Value::as_str)
        .filter(|name| !name.is_empty())
        .ok_or_else(|| freeze(&format!("%{} missing box_type", id.as_u32())))?;
    Ok(MirType::Box(box_type.to_string()))
}

fn verify_copy_owned_type_equality(function: &MirFunction) -> Result<(), String> {
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            if let MirInstruction::CopyOwned { dst, src } = instruction {
                let dst_ty = function.metadata.value_types.get(dst);
                let src_ty = function.metadata.value_types.get(src);
                if dst_ty != src_ty {
                    return Err(freeze(&format!(
                        "copy_owned type mismatch: dst=%{} src=%{}",
                        dst.as_u32(),
                        src.as_u32()
                    )));
                }
            }
        }
    }
    Ok(())
}

fn freeze(message: &str) -> String {
    format!("[freeze:contract][ownership-json-witness] {message}")
}
