use super::generic_string_body::GenericPureStringReject;
use super::shape_blocker::propagated_unknown_global_target_blocker;
use super::{
    lookup_global_call_target, supported_backend_global, GlobalCallShapeBlocker,
    GlobalCallTargetFacts, GlobalCallTargetShape, GlobalCallTargetShapeReason,
};
use crate::mir::{BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

// Return-profile evidence only: this must not make the target lowerable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericStringReturnValueClass {
    Unknown,
    Void,
    String,
    StringOrVoid,
    Object,
    Other,
}

fn seed_generic_string_return_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
) {
    let mut changed = false;
    for (index, param) in function.params.iter().enumerate() {
        if let Some(class) = function
            .signature
            .params
            .get(index)
            .and_then(generic_string_return_value_class_from_type)
        {
            set_generic_string_return_value_class(values, *param, class, &mut changed);
        }
    }
    for (value, ty) in &function.metadata.value_types {
        if let Some(class) = generic_string_return_metadata_value_class_from_type(ty) {
            set_generic_string_return_value_class(values, *value, class, &mut changed);
        }
    }
}

fn generic_string_return_value_class_from_type(
    ty: &MirType,
) -> Option<GenericStringReturnValueClass> {
    match ty {
        MirType::String => Some(GenericStringReturnValueClass::String),
        MirType::Box(name) if name == "StringBox" => Some(GenericStringReturnValueClass::String),
        MirType::Box(_) => Some(GenericStringReturnValueClass::Object),
        MirType::Integer | MirType::Bool | MirType::Float => {
            Some(GenericStringReturnValueClass::Other)
        }
        MirType::Void => Some(GenericStringReturnValueClass::Void),
        _ => None,
    }
}

fn generic_string_return_metadata_value_class_from_type(
    ty: &MirType,
) -> Option<GenericStringReturnValueClass> {
    match ty {
        MirType::String => Some(GenericStringReturnValueClass::String),
        MirType::Box(name) if name == "StringBox" => Some(GenericStringReturnValueClass::String),
        MirType::Box(_) => Some(GenericStringReturnValueClass::Object),
        MirType::Void => Some(GenericStringReturnValueClass::Void),
        _ => None,
    }
}

pub(super) fn generic_string_void_sentinel_return_global_blocker(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    let mut values = BTreeMap::<ValueId, GenericStringReturnValueClass>::new();
    let mut blockers = BTreeMap::<ValueId, GlobalCallShapeBlocker>::new();
    seed_generic_string_return_values(function, &mut values);

    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                refine_generic_string_return_value_class(
                    instruction,
                    targets,
                    &mut values,
                    &mut changed,
                );
                refine_generic_string_return_blocker(
                    instruction,
                    targets,
                    &mut blockers,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }

    let mut saw_void = false;
    let mut return_blocker = None;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    let class = generic_string_return_value_class(&values, *value);
                    if let Some(blocker) = blockers.get(value).cloned() {
                        if return_blocker.is_none() {
                            return_blocker = Some(blocker);
                        }
                    }
                    if class.is_void_like() {
                        saw_void = true;
                    }
                }
                MirInstruction::Return { value: None } => saw_void = true,
                _ => {}
            }
        }
    }

    if saw_void {
        return_blocker.map(|blocker| {
            GenericPureStringReject::with_shape_blocker(
                GlobalCallTargetShapeReason::GenericStringGlobalTargetShapeUnknown,
                blocker,
            )
        })
    } else {
        None
    }
}

pub(super) fn generic_string_return_object_boundary_candidate(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    let values = refined_generic_string_return_values(function, targets);
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Return { value: Some(value) }
                        if generic_string_return_value_class(&values, *value)
                            == GenericStringReturnValueClass::Object
                )
            })
    })
}

pub(super) fn generic_string_void_sentinel_return_candidate(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    let values = refined_generic_string_return_values(function, targets);

    let mut saw_string = false;
    let mut saw_void = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    match generic_string_return_value_class(&values, *value) {
                        GenericStringReturnValueClass::String => saw_string = true,
                        GenericStringReturnValueClass::StringOrVoid => {
                            saw_string = true;
                            saw_void = true;
                        }
                        GenericStringReturnValueClass::Void => saw_void = true,
                        GenericStringReturnValueClass::Unknown
                        | GenericStringReturnValueClass::Object
                        | GenericStringReturnValueClass::Other => {
                            return false;
                        }
                    }
                }
                MirInstruction::Return { value: None } => saw_void = true,
                _ => {}
            }
        }
    }
    saw_string && saw_void
}

fn refined_generic_string_return_values(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> BTreeMap<ValueId, GenericStringReturnValueClass> {
    let mut values = BTreeMap::<ValueId, GenericStringReturnValueClass>::new();
    seed_generic_string_return_values(function, &mut values);

    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                refine_generic_string_return_value_class(
                    instruction,
                    targets,
                    &mut values,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }
    values
}

fn refine_generic_string_return_value_class(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::String(_) => GenericStringReturnValueClass::String,
                ConstValue::Null | ConstValue::Void => GenericStringReturnValueClass::Void,
                ConstValue::Integer(_) | ConstValue::Bool(_) => {
                    GenericStringReturnValueClass::Other
                }
                _ => GenericStringReturnValueClass::Unknown,
            };
            set_generic_string_return_value_class(values, *dst, class, changed);
        }
        MirInstruction::NewBox { dst, box_type, .. } => {
            let class = if box_type == "StringBox" {
                GenericStringReturnValueClass::String
            } else {
                GenericStringReturnValueClass::Object
            };
            set_generic_string_return_value_class(values, *dst, class, changed);
        }
        MirInstruction::Copy { dst, src } => {
            let class = generic_string_return_value_class(values, *src);
            if class != GenericStringReturnValueClass::Unknown {
                set_generic_string_return_value_class(values, *dst, class, changed);
            } else {
                let dst_class = generic_string_return_value_class(values, *dst);
                if dst_class != GenericStringReturnValueClass::Unknown {
                    set_generic_string_return_value_class(values, *src, dst_class, changed);
                }
            }
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let mut class = GenericStringReturnValueClass::Unknown;
            let mut saw_unknown = false;
            let mut saw_string_like = false;
            let mut saw_other = false;
            let mut saw_void = false;
            let mut saw_non_string = false;
            let type_hint_class = type_hint
                .as_ref()
                .and_then(generic_string_return_value_class_from_type);
            for (_, value) in inputs {
                let input_class = generic_string_return_value_class(values, *value);
                if input_class == GenericStringReturnValueClass::Unknown {
                    saw_unknown = true;
                    continue;
                }
                saw_string_like |= matches!(
                    input_class,
                    GenericStringReturnValueClass::String
                        | GenericStringReturnValueClass::StringOrVoid
                );
                saw_non_string |= matches!(
                    input_class,
                    GenericStringReturnValueClass::Void | GenericStringReturnValueClass::Other
                );
                saw_other |= input_class == GenericStringReturnValueClass::Other;
                saw_void |= input_class == GenericStringReturnValueClass::Void;
                class = merge_generic_string_return_value_class(class, input_class);
            }
            if saw_unknown && saw_string_like && !saw_non_string {
                // Loop-carried string builders often carry raw i64 metadata for
                // handle values. Preserve the observed string base instead of
                // collapsing the return profile to an ABI-only blocker.
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::String,
                    changed,
                );
            } else if type_hint_class == Some(GenericStringReturnValueClass::Other)
                && !saw_string_like
                && !saw_void
            {
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::Other,
                    changed,
                );
            } else if saw_unknown && saw_other && !saw_string_like && !saw_void {
                // Loop-carried scalar indices commonly feed substring bounds in
                // string-or-void scanners. Keep them scalar in the return profile
                // instead of blocking the string return candidate.
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::Other,
                    changed,
                );
            } else if !saw_unknown {
                set_generic_string_return_value_class(values, *dst, class, changed);
            }
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = generic_string_return_value_class(values, *lhs);
            let rhs_class = generic_string_return_value_class(values, *rhs);
            let class = if *op == BinaryOp::Add
                && (lhs_class == GenericStringReturnValueClass::String
                    || rhs_class == GenericStringReturnValueClass::String)
            {
                GenericStringReturnValueClass::String
            } else if lhs_class == GenericStringReturnValueClass::Unknown
                || rhs_class == GenericStringReturnValueClass::Unknown
            {
                GenericStringReturnValueClass::Unknown
            } else {
                GenericStringReturnValueClass::Other
            };
            if class == GenericStringReturnValueClass::String {
                set_generic_string_return_string_handle_value_class(values, *dst, changed);
            } else {
                set_generic_string_return_value_class(values, *dst, class, changed);
            }
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            if generic_string_return_compare_proves_scalar(*op) {
                let lhs_class = generic_string_return_value_class(values, *lhs);
                let rhs_class = generic_string_return_value_class(values, *rhs);
                if lhs_class == GenericStringReturnValueClass::Unknown
                    && rhs_class == GenericStringReturnValueClass::Other
                {
                    set_generic_string_return_value_class(
                        values,
                        *lhs,
                        GenericStringReturnValueClass::Other,
                        changed,
                    );
                }
                if rhs_class == GenericStringReturnValueClass::Unknown
                    && lhs_class == GenericStringReturnValueClass::Other
                {
                    set_generic_string_return_value_class(
                        values,
                        *rhs,
                        GenericStringReturnValueClass::Other,
                        changed,
                    );
                }
            }
            set_generic_string_return_value_class(
                values,
                *dst,
                GenericStringReturnValueClass::Other,
                changed,
            );
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            ..
        } if name == "env.get/1" => {
            if let Some(dst) = dst {
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::String,
                    changed,
                );
            }
        }
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
            if let Some(dst) = dst {
                let receiver_class = generic_string_return_value_class(values, *receiver);
                if generic_string_return_accepts_substring_method(
                    box_name,
                    method,
                    args,
                    receiver_class,
                    values,
                ) {
                    set_generic_string_return_value_class(
                        values,
                        *dst,
                        GenericStringReturnValueClass::String,
                        changed,
                    );
                }
            }
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            ..
        } => {
            let class = lookup_global_call_target(name, targets)
                .map(|target| match target.shape() {
                    GlobalCallTargetShape::GenericPureStringBody => {
                        GenericStringReturnValueClass::String
                    }
                    GlobalCallTargetShape::ParserProgramJsonBody => {
                        GenericStringReturnValueClass::String
                    }
                    GlobalCallTargetShape::ProgramJsonEmitBody => {
                        GenericStringReturnValueClass::String
                    }
                    GlobalCallTargetShape::GenericStringOrVoidSentinelBody => {
                        GenericStringReturnValueClass::StringOrVoid
                    }
                    GlobalCallTargetShape::NumericI64Leaf
                    | GlobalCallTargetShape::GenericStringVoidLoggingBody => {
                        GenericStringReturnValueClass::Other
                    }
                    GlobalCallTargetShape::GenericI64Body => GenericStringReturnValueClass::Other,
                    GlobalCallTargetShape::Unknown => GenericStringReturnValueClass::Unknown,
                })
                .unwrap_or(GenericStringReturnValueClass::Unknown);
            if let Some(dst) = dst {
                set_generic_string_return_value_class(values, *dst, class, changed);
            }
        }
        _ => {}
    }
}

fn generic_string_return_accepts_substring_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericStringReturnValueClass,
    values: &BTreeMap<ValueId, GenericStringReturnValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "substring"
        && args.len() == 2
        && receiver_class == GenericStringReturnValueClass::String
        && args.iter().all(|arg| {
            generic_string_return_value_class(values, *arg) == GenericStringReturnValueClass::Other
        })
}

fn generic_string_return_compare_proves_scalar(op: crate::mir::CompareOp) -> bool {
    !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
}

fn refine_generic_string_return_blocker(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    blockers: &mut BTreeMap<ValueId, GlobalCallShapeBlocker>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            ..
        } if !supported_backend_global(name) => {
            let blocker = match lookup_global_call_target(name, targets) {
                Some(target) if target.shape() == GlobalCallTargetShape::Unknown => {
                    Some(propagated_unknown_global_target_blocker(name, target))
                }
                Some(_) => None,
                None => Some(GlobalCallShapeBlocker {
                    symbol: crate::mir::naming::normalize_static_global_name(name),
                    reason: Some(GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing),
                }),
            };
            if let Some(blocker) = blocker {
                set_generic_string_return_blocker(blockers, *dst, blocker, changed);
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(blocker) = blockers.get(src).cloned() {
                set_generic_string_return_blocker(blockers, *dst, blocker, changed);
            }
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            if let Some(blocker) = inputs
                .iter()
                .filter_map(|(_, value)| blockers.get(value).cloned())
                .next()
            {
                set_generic_string_return_blocker(blockers, *dst, blocker, changed);
            }
        }
        _ => {}
    }
}

fn set_generic_string_return_blocker(
    blockers: &mut BTreeMap<ValueId, GlobalCallShapeBlocker>,
    value: ValueId,
    blocker: GlobalCallShapeBlocker,
    changed: &mut bool,
) {
    match blockers.get(&value) {
        Some(existing) if *existing == blocker => {}
        Some(_) => {}
        None => {
            blockers.insert(value, blocker);
            *changed = true;
        }
    }
}

fn generic_string_return_value_class(
    values: &BTreeMap<ValueId, GenericStringReturnValueClass>,
    value: ValueId,
) -> GenericStringReturnValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericStringReturnValueClass::Unknown)
}

fn merge_generic_string_return_value_class(
    lhs: GenericStringReturnValueClass,
    rhs: GenericStringReturnValueClass,
) -> GenericStringReturnValueClass {
    match (lhs, rhs) {
        (GenericStringReturnValueClass::Unknown, class)
        | (class, GenericStringReturnValueClass::Unknown) => class,
        (same_lhs, same_rhs) if same_lhs == same_rhs => same_lhs,
        (GenericStringReturnValueClass::Object, _) | (_, GenericStringReturnValueClass::Object) => {
            GenericStringReturnValueClass::Object
        }
        (
            GenericStringReturnValueClass::String,
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid,
        )
        | (
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid,
            GenericStringReturnValueClass::String,
        )
        | (GenericStringReturnValueClass::Void, GenericStringReturnValueClass::StringOrVoid)
        | (GenericStringReturnValueClass::StringOrVoid, GenericStringReturnValueClass::Void) => {
            GenericStringReturnValueClass::StringOrVoid
        }
        _ => GenericStringReturnValueClass::Other,
    }
}

fn set_generic_string_return_value_class(
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    value: ValueId,
    class: GenericStringReturnValueClass,
    changed: &mut bool,
) {
    if class == GenericStringReturnValueClass::Unknown {
        return;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => {}
        Some(GenericStringReturnValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(existing) => {
            let merged = merge_generic_string_return_value_class(existing, class);
            if merged != existing {
                values.insert(value, merged);
                *changed = true;
            }
        }
    }
}

fn set_generic_string_return_string_handle_value_class(
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    value: ValueId,
    changed: &mut bool,
) {
    match values.get(&value).copied() {
        Some(GenericStringReturnValueClass::String) => {}
        Some(GenericStringReturnValueClass::Unknown | GenericStringReturnValueClass::Other)
        | None => {
            values.insert(value, GenericStringReturnValueClass::String);
            *changed = true;
        }
        Some(existing) => {
            let merged = merge_generic_string_return_value_class(
                existing,
                GenericStringReturnValueClass::String,
            );
            if merged != existing {
                values.insert(value, merged);
                *changed = true;
            }
        }
    }
}

impl GenericStringReturnValueClass {
    fn is_void_like(self) -> bool {
        matches!(
            self,
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid
        )
    }
}
