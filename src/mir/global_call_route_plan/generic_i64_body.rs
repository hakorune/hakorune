use super::generic_string_abi::generic_pure_string_abi_type_is_handle_compatible;
use super::generic_string_surface::generic_pure_compare_proves_i64;
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
    if !generic_i64_return_type_is_scalar(&function.signature.return_type) {
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
    let mut saw_scalar_return = false;
    let mut saw_void_sentinel_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    saw_return = true;
                    match generic_i64_value_class(&values, *value) {
                        GenericI64ValueClass::I64 | GenericI64ValueClass::Bool => {
                            saw_scalar_return = true;
                        }
                        GenericI64ValueClass::VoidSentinel => saw_void_sentinel_return = true,
                        _ => return false,
                    }
                    if saw_void_sentinel_return && !saw_scalar_return {
                        // A function that only returns null/void is not a scalar
                        // body. Mixed scalar/null bodies use null as the ABI zero
                        // sentinel.
                        continue;
                    }
                    if !generic_i64_return_value_class_is_scalar(generic_i64_value_class(
                        &values, *value,
                    )) {
                        return false;
                    }
                }
                MirInstruction::Return { value: None } => return false,
                _ => {}
            }
        }
    }
    saw_return && (!saw_void_sentinel_return || saw_scalar_return)
}

fn generic_i64_return_type_is_scalar(ty: &MirType) -> bool {
    matches!(
        ty,
        MirType::Integer | MirType::Bool | MirType::Unknown | MirType::Void
    )
}

fn generic_i64_return_value_class_is_scalar(class: GenericI64ValueClass) -> bool {
    matches!(
        class,
        GenericI64ValueClass::I64 | GenericI64ValueClass::Bool | GenericI64ValueClass::VoidSentinel
    )
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
            let eq_ne = matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne);
            let string_ordered =
                matches!(op, crate::mir::CompareOp::Lt | crate::mir::CompareOp::Gt);
            if eq_ne || string_ordered {
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
            let comparable = match (lhs_class, rhs_class) {
                (GenericI64ValueClass::String, GenericI64ValueClass::String) => {
                    eq_ne || string_ordered
                }
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
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            if inputs.is_empty() {
                return false;
            }
            let type_hint_class = type_hint
                .as_ref()
                .and_then(generic_i64_value_class_from_type);
            let dst_class = generic_i64_value_class(values, *dst);
            let mut merged = dst_class;
            for (_, value) in inputs {
                let class = generic_i64_value_class(values, *value);
                if class == GenericI64ValueClass::Unknown {
                    if dst_class == GenericI64ValueClass::Unknown
                        && matches!(
                            type_hint_class,
                            Some(GenericI64ValueClass::I64 | GenericI64ValueClass::Bool)
                        )
                    {
                        return set_generic_i64_value_class(
                            values,
                            *dst,
                            type_hint_class.unwrap(),
                            changed,
                        );
                    }
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
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            let cond_class = generic_i64_value_class(values, *cond);
            if cond_class == GenericI64ValueClass::Unknown {
                return *changed;
            }
            if !matches!(
                cond_class,
                GenericI64ValueClass::Bool | GenericI64ValueClass::I64
            ) {
                return false;
            }

            let then_class = generic_i64_value_class(values, *then_val);
            let else_class = generic_i64_value_class(values, *else_val);
            if then_class == GenericI64ValueClass::Unknown
                && else_class == GenericI64ValueClass::Unknown
            {
                return *changed;
            }
            if then_class == GenericI64ValueClass::Unknown {
                return set_generic_i64_value_class(values, *then_val, else_class, changed);
            }
            if else_class == GenericI64ValueClass::Unknown {
                return set_generic_i64_value_class(values, *else_val, then_class, changed);
            }
            let Some(selected_class) = generic_i64_select_value_class(then_class, else_class)
            else {
                return false;
            };
            set_generic_i64_value_class(values, *dst, selected_class, changed)
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
            } else if let Some(ready) = generic_i64_indexof_args_ready(
                box_name,
                method,
                args,
                receiver_class,
                values,
                changed,
            ) {
                if !ready {
                    true
                } else if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
                } else {
                    false
                }
            } else if let Some(ready) = generic_i64_contains_args_ready(
                box_name,
                method,
                args,
                receiver_class,
                values,
                changed,
            ) {
                if !ready {
                    true
                } else if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::Bool, changed)
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
                | GlobalCallTargetShape::ProgramJsonEmitBody
                | GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody => {
                    GenericI64ValueClass::String
                }
                GlobalCallTargetShape::StaticStringArrayBody => GenericI64ValueClass::Unknown,
                GlobalCallTargetShape::GenericStringOrVoidSentinelBody
                | GlobalCallTargetShape::BuilderRegistryDispatchBody => {
                    GenericI64ValueClass::StringOrVoid
                }
                GlobalCallTargetShape::NumericI64Leaf
                | GlobalCallTargetShape::GenericStringVoidLoggingBody => GenericI64ValueClass::I64,
                GlobalCallTargetShape::GenericI64Body => {
                    generic_i64_global_call_result_class(values, dst)
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

fn generic_i64_global_call_result_class(
    values: &BTreeMap<ValueId, GenericI64ValueClass>,
    dst: &Option<ValueId>,
) -> GenericI64ValueClass {
    if dst
        .map(|dst| generic_i64_value_class(values, dst) == GenericI64ValueClass::Bool)
        .unwrap_or(false)
    {
        GenericI64ValueClass::Bool
    } else {
        GenericI64ValueClass::I64
    }
}

fn generic_i64_select_value_class(
    then_class: GenericI64ValueClass,
    else_class: GenericI64ValueClass,
) -> Option<GenericI64ValueClass> {
    if then_class == else_class {
        Some(then_class)
    } else {
        None
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

fn generic_i64_indexof_args_ready(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    changed: &mut bool,
) -> Option<bool> {
    let supports_args = match method {
        "indexOf" => matches!(args.len(), 1 | 2),
        "lastIndexOf" => args.len() == 1,
        _ => false,
    };
    if !matches!(box_name, "RuntimeDataBox" | "StringBox")
        || !supports_args
        || receiver_class != GenericI64ValueClass::String
    {
        return None;
    }
    let mut ready = true;
    match generic_i64_value_class(values, args[0]) {
        GenericI64ValueClass::String => {}
        GenericI64ValueClass::Unknown => {
            if !set_generic_i64_value_class(values, args[0], GenericI64ValueClass::String, changed)
            {
                return None;
            }
            ready = false;
        }
        _ => return None,
    }
    if args.len() == 2 {
        match generic_i64_value_class(values, args[1]) {
            GenericI64ValueClass::I64 => {}
            GenericI64ValueClass::Unknown => {
                if !set_generic_i64_value_class(values, args[1], GenericI64ValueClass::I64, changed)
                {
                    return None;
                }
                ready = false;
            }
            _ => return None,
        }
    }
    Some(ready)
}

fn generic_i64_contains_args_ready(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericI64ValueClass,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    changed: &mut bool,
) -> Option<bool> {
    if !matches!(box_name, "RuntimeDataBox" | "StringBox")
        || method != "contains"
        || args.len() != 1
        || receiver_class != GenericI64ValueClass::String
    {
        return None;
    }
    match generic_i64_value_class(values, args[0]) {
        GenericI64ValueClass::String => Some(true),
        GenericI64ValueClass::Unknown => {
            if !set_generic_i64_value_class(values, args[0], GenericI64ValueClass::String, changed)
            {
                return None;
            }
            Some(false)
        }
        _ => None,
    }
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
