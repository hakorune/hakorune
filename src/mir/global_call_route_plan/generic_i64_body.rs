use super::generic_string_body::{
    generic_pure_compare_proves_i64, generic_pure_string_abi_type_is_handle_compatible,
};
use super::{
    lookup_global_call_target, supported_backend_global, GlobalCallTargetFacts,
    GlobalCallTargetShape,
};
use crate::mir::{BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericI64ValueClass {
    Unknown,
    I64,
    Bool,
    String,
    StringOrVoid,
    VoidSentinel,
}

fn seed_generic_i64_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
) -> bool {
    let mut changed = false;
    for (index, param) in function.params.iter().enumerate() {
        if let Some(class) = function
            .signature
            .params
            .get(index)
            .and_then(generic_i64_value_class_from_type)
        {
            if !set_generic_i64_value_class(values, *param, class, &mut changed) {
                return false;
            }
        }
    }
    for (value, ty) in &function.metadata.value_types {
        if let Some(class) = generic_i64_value_class_from_type(ty) {
            if !set_generic_i64_value_class(values, *value, class, &mut changed) {
                return false;
            }
        }
    }
    true
}

fn generic_i64_value_class_from_type(ty: &MirType) -> Option<GenericI64ValueClass> {
    match ty {
        MirType::Integer => Some(GenericI64ValueClass::I64),
        MirType::Bool => Some(GenericI64ValueClass::Bool),
        MirType::String => Some(GenericI64ValueClass::String),
        MirType::Box(name) => match name.as_str() {
            "IntegerBox" => Some(GenericI64ValueClass::I64),
            "BoolBox" => Some(GenericI64ValueClass::Bool),
            "StringBox" => Some(GenericI64ValueClass::String),
            _ => None,
        },
        MirType::Void => Some(GenericI64ValueClass::VoidSentinel),
        _ => None,
    }
}

pub(super) fn is_generic_i64_body_function(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    if function.signature.return_type != MirType::Integer {
        return false;
    }
    if function.params.len() != function.signature.params.len() {
        return false;
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return false;
    }

    let mut values = BTreeMap::<ValueId, GenericI64ValueClass>::new();
    if !seed_generic_i64_values(function, &mut values) {
        return false;
    }
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                if !generic_i64_body_refine_instruction(
                    instruction,
                    targets,
                    &mut values,
                    &mut changed,
                ) {
                    return false;
                }
            }
        }
        if !changed {
            break;
        }
    }

    let mut saw_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    saw_return = true;
                    if generic_i64_value_class(&values, *value) != GenericI64ValueClass::I64 {
                        return false;
                    }
                }
                MirInstruction::Return { value: None } => return false,
                _ => {}
            }
        }
    }
    saw_return
}

fn generic_i64_body_refine_instruction(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    changed: &mut bool,
) -> bool {
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::Integer(_) => GenericI64ValueClass::I64,
                ConstValue::Bool(_) => GenericI64ValueClass::Bool,
                ConstValue::String(_) => GenericI64ValueClass::String,
                ConstValue::Null | ConstValue::Void => GenericI64ValueClass::VoidSentinel,
                _ => return false,
            };
            set_generic_i64_value_class(values, *dst, class, changed)
        }
        MirInstruction::Copy { dst, src } => {
            let class = generic_i64_value_class(values, *src);
            if class != GenericI64ValueClass::Unknown {
                set_generic_i64_value_class(values, *dst, class, changed)
            } else {
                let dst_class = generic_i64_value_class(values, *dst);
                if dst_class != GenericI64ValueClass::Unknown {
                    set_generic_i64_value_class(values, *src, dst_class, changed)
                } else {
                    true
                }
            }
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            if !matches!(
                op,
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
            ) {
                return false;
            }
            let lhs_class = generic_i64_value_class(values, *lhs);
            let rhs_class = generic_i64_value_class(values, *rhs);
            if *op == BinaryOp::Add
                && (lhs_class == GenericI64ValueClass::String
                    || rhs_class == GenericI64ValueClass::String)
            {
                return set_generic_i64_string_handle_value_class(values, *dst, changed);
            }
            if lhs_class == GenericI64ValueClass::Unknown
                || rhs_class == GenericI64ValueClass::Unknown
            {
                return true;
            }
            if lhs_class == GenericI64ValueClass::I64 && rhs_class == GenericI64ValueClass::I64 {
                set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
            } else {
                false
            }
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = generic_i64_value_class(values, *lhs);
            let rhs_class = generic_i64_value_class(values, *rhs);
            if generic_pure_compare_proves_i64(*op) {
                if lhs_class == GenericI64ValueClass::Unknown
                    && rhs_class == GenericI64ValueClass::I64
                {
                    return set_generic_i64_value_class(
                        values,
                        *lhs,
                        GenericI64ValueClass::I64,
                        changed,
                    );
                }
                if rhs_class == GenericI64ValueClass::Unknown
                    && lhs_class == GenericI64ValueClass::I64
                {
                    return set_generic_i64_value_class(
                        values,
                        *rhs,
                        GenericI64ValueClass::I64,
                        changed,
                    );
                }
            }
            if matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne) {
                if lhs_class == GenericI64ValueClass::Unknown
                    && rhs_class == GenericI64ValueClass::String
                {
                    return set_generic_i64_value_class(
                        values,
                        *lhs,
                        GenericI64ValueClass::String,
                        changed,
                    );
                }
                if rhs_class == GenericI64ValueClass::Unknown
                    && lhs_class == GenericI64ValueClass::String
                {
                    return set_generic_i64_value_class(
                        values,
                        *rhs,
                        GenericI64ValueClass::String,
                        changed,
                    );
                }
            }
            if lhs_class == GenericI64ValueClass::Unknown
                || rhs_class == GenericI64ValueClass::Unknown
            {
                return true;
            }
            let eq_ne = matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne);
            let comparable = match (lhs_class, rhs_class) {
                (GenericI64ValueClass::String, GenericI64ValueClass::String) => eq_ne,
                (GenericI64ValueClass::String, GenericI64ValueClass::VoidSentinel)
                | (GenericI64ValueClass::VoidSentinel, GenericI64ValueClass::String)
                | (GenericI64ValueClass::StringOrVoid, GenericI64ValueClass::VoidSentinel)
                | (GenericI64ValueClass::VoidSentinel, GenericI64ValueClass::StringOrVoid) => eq_ne,
                (GenericI64ValueClass::I64, GenericI64ValueClass::I64) => true,
                (GenericI64ValueClass::Bool, GenericI64ValueClass::Bool) => eq_ne,
                _ => false,
            };
            if !comparable {
                return false;
            }
            set_generic_i64_value_class(values, *dst, GenericI64ValueClass::Bool, changed)
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            if inputs.is_empty() {
                return false;
            }
            let dst_class = generic_i64_value_class(values, *dst);
            let mut merged = dst_class;
            for (_, value) in inputs {
                let class = generic_i64_value_class(values, *value);
                if class == GenericI64ValueClass::Unknown {
                    if dst_class != GenericI64ValueClass::Unknown
                        && !set_generic_i64_value_class(values, *value, dst_class, changed)
                    {
                        return false;
                    }
                    return true;
                }
                if merged == GenericI64ValueClass::Unknown {
                    merged = class;
                } else if merged != class
                    && !(merged == GenericI64ValueClass::Bool && class == GenericI64ValueClass::I64)
                {
                    return false;
                }
            }
            set_generic_i64_value_class(values, *dst, merged, changed)
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            ..
        } if name == "env.get/1" => {
            if let Some(dst) = dst {
                set_generic_i64_value_class(values, *dst, GenericI64ValueClass::String, changed)
            } else {
                false
            }
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(_)),
            ..
        }
        | MirInstruction::Call {
            callee: Some(Callee::Method { receiver: None, .. }),
            ..
        } => false,
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } => {
            let receiver_class = generic_i64_value_class(values, *receiver);
            if receiver_class == GenericI64ValueClass::Unknown {
                if !set_generic_i64_value_class(
                    values,
                    *receiver,
                    GenericI64ValueClass::String,
                    changed,
                ) {
                    return false;
                }
                return true;
            }
            if generic_i64_accepts_length_method(box_name, method, args, receiver_class) {
                if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
                } else {
                    false
                }
            } else if let Some(ready) =
                generic_i64_substring_args_ready(box_name, method, args, receiver_class, values)
            {
                if !ready {
                    true
                } else if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::String, changed)
                } else {
                    false
                }
            } else {
                false
            }
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if supported_backend_global(name) => dst.is_none() && args.len() == 1,
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            ..
        } => {
            let Some(target) = lookup_global_call_target(name, targets) else {
                return false;
            };
            let class = match target.shape() {
                GlobalCallTargetShape::GenericPureStringBody
                | GlobalCallTargetShape::ParserProgramJsonBody
                | GlobalCallTargetShape::ProgramJsonEmitBody => GenericI64ValueClass::String,
                GlobalCallTargetShape::GenericStringOrVoidSentinelBody => {
                    GenericI64ValueClass::StringOrVoid
                }
                GlobalCallTargetShape::NumericI64Leaf | GlobalCallTargetShape::GenericI64Body => {
                    GenericI64ValueClass::I64
                }
                GlobalCallTargetShape::Unknown => return false,
            };
            if let Some(dst) = dst {
                set_generic_i64_value_class(values, *dst, class, changed)
            } else {
                false
            }
        }
        MirInstruction::Call { .. } => false,
        MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::ReleaseStrong { .. } => true,
        _ => false,
    }
}

fn generic_i64_accepts_length_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "length"
        && args.is_empty()
        && receiver_class == GenericI64ValueClass::String
}

fn generic_i64_substring_args_ready(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
    values: &BTreeMap<ValueId, GenericI64ValueClass>,
) -> Option<bool> {
    if !matches!(box_name, "RuntimeDataBox" | "StringBox")
        || method != "substring"
        || args.len() != 2
        || receiver_class != GenericI64ValueClass::String
    {
        return None;
    }
    let mut ready = true;
    for arg in args {
        match generic_i64_value_class(values, *arg) {
            GenericI64ValueClass::I64 => {}
            GenericI64ValueClass::Unknown => ready = false,
            _ => return None,
        }
    }
    Some(ready)
}

fn generic_i64_value_class(
    values: &BTreeMap<ValueId, GenericI64ValueClass>,
    value: ValueId,
) -> GenericI64ValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericI64ValueClass::Unknown)
}

fn set_generic_i64_value_class(
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    value: ValueId,
    class: GenericI64ValueClass,
    changed: &mut bool,
) -> bool {
    if class == GenericI64ValueClass::Unknown {
        return true;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => true,
        Some(GenericI64ValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::VoidSentinel)
            if matches!(
                class,
                GenericI64ValueClass::String | GenericI64ValueClass::StringOrVoid
            ) =>
        {
            values.insert(value, class);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::String) if class == GenericI64ValueClass::StringOrVoid => {
            values.insert(value, GenericI64ValueClass::StringOrVoid);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::StringOrVoid)
            if matches!(
                class,
                GenericI64ValueClass::String | GenericI64ValueClass::VoidSentinel
            ) =>
        {
            true
        }
        Some(GenericI64ValueClass::I64)
            if matches!(
                class,
                GenericI64ValueClass::String
                    | GenericI64ValueClass::StringOrVoid
                    | GenericI64ValueClass::VoidSentinel
            ) =>
        {
            values.insert(value, class);
            *changed = true;
            true
        }
        Some(_) => false,
    }
}

fn set_generic_i64_string_handle_value_class(
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    value: ValueId,
    changed: &mut bool,
) -> bool {
    match values.get(&value).copied() {
        Some(GenericI64ValueClass::String) => true,
        Some(GenericI64ValueClass::StringOrVoid) => {
            values.insert(value, GenericI64ValueClass::String);
            *changed = true;
            true
        }
        Some(GenericI64ValueClass::Unknown) | None => {
            values.insert(value, GenericI64ValueClass::String);
            *changed = true;
            true
        }
        // String handles are raw i64 at the ABI layer. For `String + ...`, the
        // operation itself is the semantic proof that this value is a string.
        Some(GenericI64ValueClass::I64) => {
            values.insert(value, GenericI64ValueClass::String);
            *changed = true;
            true
        }
        Some(_) => false,
    }
}
