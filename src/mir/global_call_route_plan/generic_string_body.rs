use std::collections::{BTreeMap, BTreeSet};

use crate::mir::string_corridor::StringCorridorOp;
use crate::mir::{BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

use super::model::{
    GlobalCallShapeBlocker, GlobalCallTargetFacts, GlobalCallTargetShape,
    GlobalCallTargetShapeReason,
};
use super::shape_blocker::propagated_unknown_global_target_blocker;
use super::string_return_profile::{
    generic_string_return_object_boundary_candidate, generic_string_void_sentinel_return_candidate,
    generic_string_void_sentinel_return_global_blocker,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericPureValueClass {
    Unknown,
    I64,
    Bool,
    String,
    Array,
    Map,
    StringOrVoid,
    VoidSentinel,
}

fn seed_generic_pure_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
) {
    let mut changed = false;
    for (index, param) in function.params.iter().enumerate() {
        if let Some(class) = function
            .signature
            .params
            .get(index)
            .and_then(generic_pure_value_class_from_type)
        {
            set_value_class(values, *param, class, &mut changed);
        }
    }
    for (value, ty) in &function.metadata.value_types {
        if let Some(class) = generic_pure_metadata_value_class_from_type(ty) {
            set_value_class(values, *value, class, &mut changed);
        }
    }
}

fn seed_generic_pure_string_return_param_values(
    function: &MirFunction,
    values: &mut BTreeSet<ValueId>,
) {
    for (index, param) in function.params.iter().enumerate() {
        let Some(ty) = function.signature.params.get(index) else {
            continue;
        };
        if generic_pure_string_return_param_passthrough_type_is_candidate(ty) {
            values.insert(*param);
        }
    }
}

fn generic_pure_value_class_from_type(ty: &MirType) -> Option<GenericPureValueClass> {
    match ty {
        MirType::Integer => Some(GenericPureValueClass::I64),
        MirType::Bool => Some(GenericPureValueClass::Bool),
        MirType::String => Some(GenericPureValueClass::String),
        MirType::Box(name) => match name.as_str() {
            "IntegerBox" => Some(GenericPureValueClass::I64),
            "BoolBox" => Some(GenericPureValueClass::Bool),
            "StringBox" => Some(GenericPureValueClass::String),
            _ => None,
        },
        MirType::Void => Some(GenericPureValueClass::VoidSentinel),
        _ => None,
    }
}

fn generic_pure_metadata_value_class_from_type(ty: &MirType) -> Option<GenericPureValueClass> {
    match ty {
        MirType::String => Some(GenericPureValueClass::String),
        MirType::Box(name) => match name.as_str() {
            "IntegerBox" => Some(GenericPureValueClass::I64),
            "BoolBox" => Some(GenericPureValueClass::Bool),
            "StringBox" => Some(GenericPureValueClass::String),
            _ => None,
        },
        MirType::Void => Some(GenericPureValueClass::VoidSentinel),
        _ => None,
    }
}

fn generic_pure_string_return_param_passthrough_type_is_candidate(ty: &MirType) -> bool {
    matches!(ty, MirType::String | MirType::Unknown)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GenericPureStringReject {
    pub(super) reason: GlobalCallTargetShapeReason,
    pub(super) blocker: Option<GlobalCallShapeBlocker>,
}

impl GenericPureStringReject {
    fn new(reason: GlobalCallTargetShapeReason) -> Self {
        Self {
            reason,
            blocker: None,
        }
    }

    fn with_blocker(
        reason: GlobalCallTargetShapeReason,
        symbol: impl Into<String>,
        blocker_reason: Option<GlobalCallTargetShapeReason>,
    ) -> Self {
        Self {
            reason,
            blocker: Some(GlobalCallShapeBlocker {
                symbol: symbol.into(),
                reason: blocker_reason,
            }),
        }
    }

    pub(super) fn with_shape_blocker(
        reason: GlobalCallTargetShapeReason,
        blocker: GlobalCallShapeBlocker,
    ) -> Self {
        Self {
            reason,
            blocker: Some(blocker),
        }
    }
}

pub(super) fn generic_pure_string_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if !generic_pure_string_abi_type_is_handle_compatible(&function.signature.return_type) {
        if function.signature.return_type == MirType::Void {
            if let Some(reject) = generic_string_void_sentinel_body_reject_reason(function, targets)
            {
                return Some(reject);
            }
            if generic_string_return_object_boundary_candidate(function, targets) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
                ));
            }
        }
        if matches!(&function.signature.return_type, MirType::Box(name) if name != "StringBox") {
            return Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
            ));
        }
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnAbiNotHandleCompatible,
        ));
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringParamAbiNotHandleCompatible,
        ));
    }
    if function.params.len() != function.signature.params.len() {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::ParamBindingMismatch,
        ));
    }
    if let Some(reject) = generic_pure_string_known_receiver_return_blocker(function) {
        return Some(reject);
    }

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut return_param_values = BTreeSet::<ValueId>::new();
    let mut has_string_surface = false;
    let mut has_void_sentinel_const = false;
    seed_generic_pure_values(function, &mut values);
    seed_generic_pure_string_return_param_values(function, &mut return_param_values);
    seed_generic_pure_string_corridor_method_values(function, &mut values, &mut has_string_surface);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    let max_iterations = generic_pure_string_known_receiver_blocker_iteration_limit(function);
    for _ in 0..max_iterations {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in &block.instructions {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    instruction,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    terminator,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
        }
        if !changed {
            break;
        }
    }

    if !has_string_surface {
        if has_void_sentinel_const {
            return Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
            ));
        }
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringNoStringSurface,
        ));
    }

    let mut saw_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Return { value: Some(value) } = instruction {
                saw_return = true;
                let class = value_class(&values, *value);
                if class == GenericPureValueClass::VoidSentinel {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                    ));
                }
                if class != GenericPureValueClass::String
                    && !generic_pure_string_return_allows_param_passthrough(
                        function,
                        *value,
                        &return_param_values,
                    )
                {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringReturnNotString,
                    ));
                }
            } else if matches!(instruction, MirInstruction::Return { value: None }) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringReturnNotString,
                ));
            }
        }
    }
    if saw_return {
        None
    } else {
        Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnNotString,
        ))
    }
}

fn generic_pure_string_known_receiver_return_blocker(
    function: &MirFunction,
) -> Option<GenericPureStringReject> {
    let mut blockers = BTreeMap::<ValueId, String>::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                update_generic_pure_string_known_receiver_return_blockers(
                    instruction,
                    &mut blockers,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }

    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value: Some(value) } = instruction else {
                continue;
            };
            let Some(symbol) = blockers.get(value) else {
                continue;
            };
            return Some(GenericPureStringReject::with_blocker(
                GlobalCallTargetShapeReason::GenericStringUnsupportedKnownReceiverMethod,
                symbol.clone(),
                Some(GlobalCallTargetShapeReason::GenericStringUnsupportedKnownReceiverMethod),
            ));
        }
    }
    None
}

fn generic_pure_string_known_receiver_blocker_iteration_limit(function: &MirFunction) -> usize {
    function
        .blocks
        .values()
        .map(|block| block.instructions.len() + usize::from(block.terminator.is_some()))
        .sum::<usize>()
        .saturating_add(1)
}

fn update_generic_pure_string_known_receiver_return_blockers(
    instruction: &MirInstruction,
    blockers: &mut BTreeMap<ValueId, String>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            ..
        } if box_name == "ParserBox" => {
            let symbol = format!("{}.{}", box_name, method);
            if blockers.get(dst) != Some(&symbol) {
                blockers.insert(*dst, symbol);
                *changed = true;
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(symbol) = blockers.get(src).cloned() {
                if blockers.get(dst) != Some(&symbol) {
                    blockers.insert(*dst, symbol);
                    *changed = true;
                }
            }
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            let Some(symbol) = inputs
                .iter()
                .filter_map(|(_, value)| blockers.get(value))
                .next()
                .cloned()
            else {
                return;
            };
            if blockers.get(dst) != Some(&symbol) {
                blockers.insert(*dst, symbol);
                *changed = true;
            }
        }
        _ => {}
    }
}

fn generic_pure_string_return_allows_param_passthrough(
    function: &MirFunction,
    value: ValueId,
    return_param_values: &BTreeSet<ValueId>,
) -> bool {
    generic_pure_string_return_type_accepts_param_passthrough(&function.signature.return_type)
        && return_param_values.contains(&value)
}

fn generic_pure_string_return_type_accepts_param_passthrough(ty: &MirType) -> bool {
    matches!(ty, MirType::String) || matches!(ty, MirType::Box(name) if name == "StringBox")
}

pub(super) fn generic_pure_string_abi_type_is_handle_compatible(ty: &MirType) -> bool {
    match ty {
        MirType::Integer | MirType::String | MirType::Unknown => true,
        MirType::Box(name) => name == "StringBox",
        _ => false,
    }
}

fn seed_generic_pure_string_corridor_method_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    has_string_surface: &mut bool,
) {
    let mut changed = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Call {
                dst: Some(dst),
                callee:
                    Some(Callee::Method {
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
            let Some(expected_op) = generic_pure_string_corridor_method_op(box_name, method, args)
            else {
                continue;
            };
            let Some(fact) = function.metadata.string_corridor_facts.get(dst) else {
                continue;
            };
            if fact.op != expected_op {
                continue;
            }
            *has_string_surface = true;
            set_string_handle_value_class(values, *receiver, &mut changed);
            match fact.op {
                StringCorridorOp::StrLen => {
                    set_value_class(values, *dst, GenericPureValueClass::I64, &mut changed);
                }
                StringCorridorOp::StrSlice => {
                    for arg in args {
                        set_value_class(values, *arg, GenericPureValueClass::I64, &mut changed);
                    }
                    set_string_handle_value_class(values, *dst, &mut changed);
                }
                StringCorridorOp::FreezeStr => {}
            }
        }
    }
}

fn generic_pure_string_corridor_method_op(
    box_name: &str,
    method: &str,
    args: &[ValueId],
) -> Option<StringCorridorOp> {
    if !matches!(box_name, "RuntimeDataBox" | "StringBox") {
        return None;
    }
    match method {
        "length" if args.is_empty() => Some(StringCorridorOp::StrLen),
        "substring" if args.len() == 2 => Some(StringCorridorOp::StrSlice),
        _ => None,
    }
}

pub(super) fn generic_string_void_sentinel_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if !generic_string_void_sentinel_return_candidate(function, targets) {
        if let Some(reject) = generic_string_void_sentinel_return_global_blocker(function, targets)
        {
            return Some(reject);
        }
        if generic_string_return_object_boundary_candidate(function, targets) {
            return Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
            ));
        }
        return None;
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringParamAbiNotHandleCompatible,
        ));
    }

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut return_param_values = BTreeSet::<ValueId>::new();
    let mut has_string_surface = false;
    let mut has_void_sentinel_const = false;
    seed_generic_pure_values(function, &mut values);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in &block.instructions {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    instruction,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    terminator,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
        }
        if !changed {
            break;
        }
    }

    Some(GenericPureStringReject::new(
        GlobalCallTargetShapeReason::GenericStringReturnVoidSentinelCandidate,
    ))
}

pub(super) fn format_mir_type_label(ty: &MirType) -> String {
    match ty {
        MirType::Integer => "i64".to_string(),
        MirType::Float => "f64".to_string(),
        MirType::Bool => "i1".to_string(),
        MirType::String => "str".to_string(),
        MirType::Box(name) => format!("box<{}>", name),
        MirType::Array(inner) => format!("[{}]", format_mir_type_label(inner)),
        MirType::Future(inner) => format!("future<{}>", format_mir_type_label(inner)),
        MirType::WeakRef => "weakref".to_string(),
        MirType::Void => "void".to_string(),
        MirType::Unknown => "?".to_string(),
    }
}

fn generic_pure_string_instruction_reject_reason(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    return_param_values: &mut BTreeSet<ValueId>,
    has_string_surface: &mut bool,
    has_void_sentinel_const: &mut bool,
    changed: &mut bool,
) -> Option<GenericPureStringReject> {
    update_generic_pure_string_return_param_values(instruction, return_param_values, changed);
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::String(_) => {
                    *has_string_surface = true;
                    GenericPureValueClass::String
                }
                ConstValue::Integer(_) => GenericPureValueClass::I64,
                ConstValue::Bool(_) => GenericPureValueClass::Bool,
                ConstValue::Null | ConstValue::Void => {
                    *has_void_sentinel_const = true;
                    GenericPureValueClass::VoidSentinel
                }
                _ => GenericPureValueClass::Unknown,
            };
            set_value_class(values, *dst, class, changed);
            if class != GenericPureValueClass::Unknown {
                return None;
            }
            Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
            ))
        }
        MirInstruction::Copy { dst, src } => {
            let class = value_class(values, *src);
            if class != GenericPureValueClass::Unknown {
                set_proven_flow_value_class(values, *dst, class, changed);
            } else {
                let dst_class = value_class(values, *dst);
                if dst_class != GenericPureValueClass::Unknown {
                    set_value_class(values, *src, dst_class, changed);
                }
            }
            None
        }
        MirInstruction::NewBox {
            dst,
            box_type,
            args,
        } => {
            if !args.is_empty() {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            let class = match box_type.as_str() {
                "ArrayBox" => GenericPureValueClass::Array,
                "MapBox" => GenericPureValueClass::Map,
                _ => {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                    ));
                }
            };
            set_value_class(values, *dst, class, changed);
            None
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            if *op != BinaryOp::Add
                && *op != BinaryOp::Sub
                && *op != BinaryOp::Mul
                && *op != BinaryOp::Div
                && *op != BinaryOp::Mod
            {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            let lhs_class = value_class(values, *lhs);
            let rhs_class = value_class(values, *rhs);
            if *op == BinaryOp::Add
                && (lhs_class == GenericPureValueClass::String
                    || rhs_class == GenericPureValueClass::String)
            {
                *has_string_surface = true;
                set_string_handle_value_class(values, *dst, changed);
                return None;
            }
            if lhs_class == GenericPureValueClass::Unknown
                || rhs_class == GenericPureValueClass::Unknown
            {
                return None;
            }
            if lhs_class == GenericPureValueClass::VoidSentinel
                || rhs_class == GenericPureValueClass::VoidSentinel
            {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                ));
            }
            let class = if *op == BinaryOp::Add {
                GenericPureValueClass::I64
            } else if lhs_class == GenericPureValueClass::String
                || rhs_class == GenericPureValueClass::String
            {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                GenericPureValueClass::I64
            };
            set_value_class(values, *dst, class, changed);
            None
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = value_class(values, *lhs);
            let rhs_class = value_class(values, *rhs);
            if generic_pure_compare_proves_i64(*op) {
                if lhs_class == GenericPureValueClass::Unknown
                    && rhs_class == GenericPureValueClass::I64
                {
                    set_value_class(values, *lhs, GenericPureValueClass::I64, changed);
                    return None;
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && lhs_class == GenericPureValueClass::I64
                {
                    set_value_class(values, *rhs, GenericPureValueClass::I64, changed);
                    return None;
                }
            }
            if lhs_class == GenericPureValueClass::Unknown
                || rhs_class == GenericPureValueClass::Unknown
            {
                return None;
            }
            let has_void_sentinel = generic_pure_value_class_is_void_like(lhs_class)
                || generic_pure_value_class_is_void_like(rhs_class);
            if has_void_sentinel {
                let comparable =
                    matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
                        && generic_pure_void_sentinel_compare_is_supported(lhs_class, rhs_class);
                if !comparable {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                    ));
                }
                set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
                return None;
            }
            if lhs_class == GenericPureValueClass::String
                || rhs_class == GenericPureValueClass::String
            {
                if !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne) {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                    ));
                }
                *has_string_surface = true;
            }
            set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
            None
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            let mut saw_string = false;
            let mut saw_string_or_void = false;
            let mut saw_void_sentinel = false;
            let mut saw_scalar = false;
            let mut saw_array = false;
            let mut saw_map = false;
            let mut all_string = !inputs.is_empty();
            let mut all_array = !inputs.is_empty();
            let mut all_map = !inputs.is_empty();
            let mut saw_unknown = false;
            for (_, value) in inputs {
                let class = value_class(values, *value);
                saw_unknown |= class == GenericPureValueClass::Unknown;
                saw_string |= class == GenericPureValueClass::String;
                saw_string_or_void |= class == GenericPureValueClass::StringOrVoid;
                saw_void_sentinel |= class == GenericPureValueClass::VoidSentinel;
                saw_array |= class == GenericPureValueClass::Array;
                saw_map |= class == GenericPureValueClass::Map;
                saw_scalar |= matches!(
                    class,
                    GenericPureValueClass::I64 | GenericPureValueClass::Bool
                );
                all_string &= class == GenericPureValueClass::String;
                all_array &= class == GenericPureValueClass::Array;
                all_map &= class == GenericPureValueClass::Map;
            }
            if saw_unknown {
                return None;
            } else if all_string {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::String, changed);
            } else if all_array {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::Array, changed);
            } else if all_map {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::Map, changed);
            } else if saw_string_or_void && !saw_scalar {
                *has_string_surface = true;
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    changed,
                );
            } else if saw_void_sentinel && !saw_scalar && (saw_string || saw_string_or_void) {
                *has_string_surface = true;
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    changed,
                );
            } else if saw_void_sentinel && !saw_scalar {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::VoidSentinel,
                    changed,
                );
            } else if saw_string || saw_array || saw_map {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::I64, changed);
            }
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            ..
        } if name == "env.get/1" => {
            if let Some(dst) = dst {
                *has_string_surface = true;
                set_string_handle_value_class(values, *dst, changed);
            }
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if generic_pure_string_accepts_env_set(name, args, values) => {
            if let Some(dst) = dst {
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
            }
            None
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(_)),
            ..
        } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedExternCall,
        )),
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
            let receiver_class = value_class(values, *receiver);
            if receiver_class == GenericPureValueClass::Unknown
                || args
                    .iter()
                    .any(|arg| value_class(values, *arg) == GenericPureValueClass::Unknown)
            {
                if *changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                ));
            }
            if generic_pure_string_accepts_length_method(box_name, method, args, receiver_class) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_array_len_method(box_name, method, args, receiver_class)
            {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_indexof_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_substring_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                *has_string_surface = true;
                set_value_class(values, *dst, GenericPureValueClass::String, changed);
                return None;
            }
            Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
            ))
        }
        MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
        )),
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if super::supported_backend_global(name) => None,
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            ..
        } if !super::supported_backend_global(name) => {
            let Some(target) = super::lookup_global_call_target(name, targets) else {
                return Some(GenericPureStringReject::with_blocker(
                    GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing,
                    crate::mir::naming::normalize_static_global_name(name),
                    None,
                ));
            };
            match target.shape() {
                GlobalCallTargetShape::GenericPureStringBody
                | GlobalCallTargetShape::GenericStringOrVoidSentinelBody
                | GlobalCallTargetShape::ParserProgramJsonBody
                | GlobalCallTargetShape::ProgramJsonEmitBody => {
                    if let Some(dst) = dst {
                        *has_string_surface = true;
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::String,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::NumericI64Leaf | GlobalCallTargetShape::GenericI64Body => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::I64,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::Unknown => {
                    Some(GenericPureStringReject::with_shape_blocker(
                        GlobalCallTargetShapeReason::GenericStringGlobalTargetShapeUnknown,
                        propagated_unknown_global_target_blocker(name, target),
                    ))
                }
            }
        }
        MirInstruction::Call { .. } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedCall,
        )),
        MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::ReleaseStrong { .. } => None,
        _ => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
        )),
    }
}

fn generic_pure_string_accepts_length_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "length"
        && args.is_empty()
        && receiver_class == GenericPureValueClass::String
}

fn generic_pure_string_accepts_array_len_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "ArrayBox")
        && matches!(method, "len" | "length" | "size")
        && args.is_empty()
        && receiver_class == GenericPureValueClass::Array
}

fn generic_pure_string_accepts_indexof_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "indexOf"
        && args.len() == 1
        && receiver_class == GenericPureValueClass::String
        && args
            .iter()
            .all(|arg| value_class(values, *arg) == GenericPureValueClass::String)
}

fn generic_pure_string_accepts_env_set(
    name: &str,
    args: &[ValueId],
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(
        name,
        "env.set/2" | "env.set" | "nyash.env.set/2" | "nyash.env.set"
    ) && args.len() == 2
        && args
            .iter()
            .all(|arg| value_class(values, *arg) == GenericPureValueClass::String)
}

pub(super) fn generic_pure_compare_proves_i64(op: crate::mir::CompareOp) -> bool {
    !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
}

fn generic_pure_string_accepts_substring_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericPureValueClass,
    values: &BTreeMap<ValueId, GenericPureValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "substring"
        && args.len() == 2
        && receiver_class == GenericPureValueClass::String
        && args
            .iter()
            .all(|arg| value_class(values, *arg) == GenericPureValueClass::I64)
}

fn update_generic_pure_string_return_param_values(
    instruction: &MirInstruction,
    values: &mut BTreeSet<ValueId>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Copy { dst, src } if values.contains(src) => {
            if values.insert(*dst) {
                *changed = true;
            }
        }
        MirInstruction::Phi { dst, inputs, .. }
            if !inputs.is_empty() && inputs.iter().all(|(_, value)| values.contains(value)) =>
        {
            if values.insert(*dst) {
                *changed = true;
            }
        }
        _ => {}
    }
}

fn value_class(
    values: &BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
) -> GenericPureValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericPureValueClass::Unknown)
}

fn generic_pure_value_class_is_void_like(class: GenericPureValueClass) -> bool {
    matches!(
        class,
        GenericPureValueClass::StringOrVoid | GenericPureValueClass::VoidSentinel
    )
}

fn generic_pure_void_sentinel_compare_is_supported(
    lhs_class: GenericPureValueClass,
    rhs_class: GenericPureValueClass,
) -> bool {
    matches!(
        (lhs_class, rhs_class),
        (
            GenericPureValueClass::String,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::String
        ) | (
            GenericPureValueClass::StringOrVoid,
            GenericPureValueClass::VoidSentinel
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::StringOrVoid
        ) | (
            GenericPureValueClass::VoidSentinel,
            GenericPureValueClass::VoidSentinel
        )
    )
}

fn set_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    class: GenericPureValueClass,
    changed: &mut bool,
) {
    if class == GenericPureValueClass::Unknown {
        return;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => {}
        Some(GenericPureValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(_) => {}
    }
}

fn set_string_handle_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    changed: &mut bool,
) {
    match values.get(&value).copied() {
        Some(GenericPureValueClass::String) => {}
        Some(GenericPureValueClass::Unknown)
        | Some(GenericPureValueClass::I64)
        | Some(GenericPureValueClass::StringOrVoid)
        | Some(GenericPureValueClass::VoidSentinel)
        | None => {
            values.insert(value, GenericPureValueClass::String);
            *changed = true;
        }
        Some(_) => {}
    }
}

fn set_proven_flow_value_class(
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
    class: GenericPureValueClass,
    changed: &mut bool,
) {
    if class == GenericPureValueClass::Unknown {
        return;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => {}
        Some(GenericPureValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(GenericPureValueClass::VoidSentinel)
            if matches!(
                class,
                GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
            ) =>
        {
            values.insert(value, GenericPureValueClass::StringOrVoid);
            *changed = true;
        }
        Some(GenericPureValueClass::String)
            if matches!(
                class,
                GenericPureValueClass::StringOrVoid | GenericPureValueClass::VoidSentinel
            ) =>
        {
            values.insert(value, GenericPureValueClass::StringOrVoid);
            *changed = true;
        }
        Some(GenericPureValueClass::StringOrVoid) if class == GenericPureValueClass::String => {}
        Some(GenericPureValueClass::StringOrVoid)
            if class == GenericPureValueClass::VoidSentinel => {}
        Some(GenericPureValueClass::I64)
            if matches!(
                class,
                GenericPureValueClass::String
                    | GenericPureValueClass::StringOrVoid
                    | GenericPureValueClass::VoidSentinel
            ) =>
        {
            values.insert(value, class);
            *changed = true;
        }
        Some(_) => {}
    }
}
