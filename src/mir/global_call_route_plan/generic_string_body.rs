use std::collections::{BTreeMap, BTreeSet};

use crate::mir::extern_call_route_plan::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, UnaryOp,
    ValueId,
};

use super::generic_string_abi::{
    generic_pure_string_abi_type_is_handle_compatible,
    generic_pure_string_return_allows_param_passthrough,
};
use super::generic_string_corridor::seed_generic_pure_string_corridor_method_values;
use super::generic_string_facts::{
    generic_pure_string_iteration_limit, generic_pure_value_class_from_type,
    generic_pure_value_class_is_void_like, generic_pure_void_sentinel_compare_is_supported,
    seed_generic_pure_string_return_param_values, seed_generic_pure_values,
    set_guarded_non_void_array_value_class, set_guarded_non_void_map_value_class,
    set_guarded_non_void_scalar_value_class, set_guarded_non_void_string_value_class,
    set_proven_flow_value_class, set_string_handle_value_class, set_value_class,
    update_generic_pure_string_return_param_values, value_class, GenericPureValueClass,
};
use super::generic_string_guards::generic_pure_string_non_void_guard_phi_values;
use super::generic_string_reject::GenericPureStringReject;
use super::generic_string_surface::{
    generic_pure_compare_proves_i64, generic_pure_string_accepts_array_len_method,
    generic_pure_string_accepts_array_push_method,
    generic_pure_string_accepts_collection_birth_method,
    generic_pure_string_accepts_contains_method, generic_pure_string_accepts_env_set,
    generic_pure_string_accepts_indexof_method, generic_pure_string_accepts_lastindexof_method,
    generic_pure_string_accepts_length_method, generic_pure_string_accepts_map_set_method,
    generic_pure_string_accepts_string_compare, generic_pure_string_accepts_substring_method,
    generic_pure_string_compare_can_infer_string, generic_pure_string_global_name_is_self,
};
use super::model::{GlobalCallTargetFacts, GlobalCallTargetShape, GlobalCallTargetShapeReason};
use super::shape_blocker::propagated_unknown_global_target_blocker;
use super::string_return_profile::{
    generic_string_return_object_boundary_candidate, generic_string_void_sentinel_return_candidate,
    generic_string_void_sentinel_return_global_blocker,
};

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
    let non_void_string_values = generic_pure_string_non_void_guard_phi_values(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    let max_iterations = generic_pure_string_iteration_limit(function);
    for _ in 0..max_iterations {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    instruction_index,
                    instruction,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    block.instructions.len(),
                    terminator,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
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
    seed_generic_pure_string_corridor_method_values(function, &mut values, &mut has_string_surface);
    let non_void_string_values = generic_pure_string_non_void_guard_phi_values(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    instruction_index,
                    instruction,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    block.instructions.len(),
                    terminator,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
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

pub(super) fn generic_string_void_logging_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if function.signature.return_type != MirType::Void {
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

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut return_param_values = BTreeSet::<ValueId>::new();
    let mut has_string_surface = false;
    let mut has_void_sentinel_const = false;
    seed_generic_pure_values(function, &mut values);
    let non_void_string_values = generic_pure_string_non_void_guard_phi_values(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    let max_iterations = generic_pure_string_iteration_limit(function);
    for _ in 0..max_iterations {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    instruction_index,
                    instruction,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
                    &mut changed,
                ) {
                    return Some(reject);
                }
            }
            if let Some(terminator) = &block.terminator {
                if let Some(reject) = generic_pure_string_instruction_reject_reason(
                    function,
                    *block_id,
                    block.instructions.len(),
                    terminator,
                    targets,
                    &mut values,
                    &mut return_param_values,
                    &mut has_string_surface,
                    &mut has_void_sentinel_const,
                    &non_void_string_values,
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

    if !has_string_surface || !generic_string_void_logging_has_logging_call(function, targets) {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringNoStringSurface,
        ));
    }

    let mut saw_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    saw_return = true;
                    if value_class(&values, *value) != GenericPureValueClass::VoidSentinel {
                        return Some(GenericPureStringReject::new(
                            GlobalCallTargetShapeReason::GenericStringReturnNotString,
                        ));
                    }
                }
                MirInstruction::Return { value: None } => saw_return = true,
                _ => {}
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

fn generic_string_void_logging_has_logging_call(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(Callee::Global(name)),
                        ..
                    } if name == "print"
                ) || matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(Callee::Global(name)),
                        ..
                    } if super::lookup_global_call_target(name, targets)
                        .map(|target| target.shape() == GlobalCallTargetShape::GenericStringVoidLoggingBody)
                        .unwrap_or(false)
                )
            })
    })
}

fn generic_pure_string_instruction_reject_reason(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    return_param_values: &mut BTreeSet<ValueId>,
    has_string_surface: &mut bool,
    has_void_sentinel_const: &mut bool,
    non_void_string_values: &BTreeSet<ValueId>,
    changed: &mut bool,
) -> Option<GenericPureStringReject> {
    let current_function_name = function.signature.name.as_str();
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
                if lhs_class == GenericPureValueClass::Unknown {
                    set_string_handle_value_class(values, *lhs, changed);
                }
                if rhs_class == GenericPureValueClass::Unknown {
                    set_string_handle_value_class(values, *rhs, changed);
                }
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
            if generic_pure_string_compare_can_infer_string(*op) {
                if lhs_class == GenericPureValueClass::Unknown
                    && rhs_class == GenericPureValueClass::String
                {
                    set_string_handle_value_class(values, *lhs, changed);
                    return None;
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && lhs_class == GenericPureValueClass::String
                {
                    set_string_handle_value_class(values, *rhs, changed);
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
                if !generic_pure_string_accepts_string_compare(*op, lhs_class, rhs_class) {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                    ));
                }
                *has_string_surface = true;
            }
            set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
            None
        }
        MirInstruction::UnaryOp {
            dst,
            op: UnaryOp::Not,
            operand,
        } => {
            let operand_class = value_class(values, *operand);
            if operand_class == GenericPureValueClass::Unknown {
                return None;
            }
            if !matches!(
                operand_class,
                GenericPureValueClass::Bool | GenericPureValueClass::I64
            ) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
            None
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let dst_class = value_class(values, *dst);
            let mut saw_scalar_or_void = false;
            let mut saw_string = false;
            let mut saw_string_or_void = false;
            let mut saw_void_sentinel = false;
            let mut saw_scalar = false;
            let mut saw_array = false;
            let mut saw_map = false;
            let mut saw_array_or_void = false;
            let mut saw_map_or_void = false;
            let mut all_string = !inputs.is_empty();
            let mut all_array = !inputs.is_empty();
            let mut all_map = !inputs.is_empty();
            let mut saw_unknown = false;
            for (_, value) in inputs {
                let class = value_class(values, *value);
                saw_unknown |= class == GenericPureValueClass::Unknown;
                saw_scalar_or_void |= class == GenericPureValueClass::ScalarOrVoid;
                saw_string |= class == GenericPureValueClass::String;
                saw_string_or_void |= class == GenericPureValueClass::StringOrVoid;
                saw_void_sentinel |= class == GenericPureValueClass::VoidSentinel;
                saw_array |= class == GenericPureValueClass::Array;
                saw_map |= class == GenericPureValueClass::Map;
                saw_array_or_void |= class == GenericPureValueClass::ArrayOrVoid;
                saw_map_or_void |= class == GenericPureValueClass::MapOrVoid;
                saw_scalar |= matches!(
                    class,
                    GenericPureValueClass::I64 | GenericPureValueClass::Bool
                );
                all_string &= class == GenericPureValueClass::String;
                all_array &= class == GenericPureValueClass::Array;
                all_map &= class == GenericPureValueClass::Map;
            }
            let type_hint_class = type_hint
                .as_ref()
                .and_then(generic_pure_value_class_from_type);
            if saw_unknown {
                if dst_class != GenericPureValueClass::Unknown
                    && inputs.iter().all(|(_, value)| {
                        let class = value_class(values, *value);
                        class == GenericPureValueClass::Unknown || class == dst_class
                    })
                {
                    for (_, value) in inputs {
                        if value_class(values, *value) == GenericPureValueClass::Unknown {
                            set_proven_flow_value_class(values, *value, dst_class, changed);
                        }
                    }
                }
                if matches!(
                    type_hint_class,
                    Some(GenericPureValueClass::I64 | GenericPureValueClass::Bool)
                ) && !saw_string
                    && !saw_scalar_or_void
                    && !saw_string_or_void
                    && !saw_void_sentinel
                    && !saw_array
                    && !saw_map
                    && !saw_array_or_void
                    && !saw_map_or_void
                {
                    set_proven_flow_value_class(values, *dst, type_hint_class.unwrap(), changed);
                }
                return None;
            } else if non_void_string_values.contains(dst)
                && saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_void_sentinel
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                set_guarded_non_void_scalar_value_class(values, *dst, changed);
            } else if non_void_string_values.contains(dst)
                && saw_string_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                *has_string_surface = true;
                set_guarded_non_void_string_value_class(values, *dst, changed);
            } else if non_void_string_values.contains(dst)
                && saw_array_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_map
                && !saw_map_or_void
            {
                set_guarded_non_void_array_value_class(values, *dst, changed);
            } else if non_void_string_values.contains(dst)
                && saw_map_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_array_or_void
            {
                set_guarded_non_void_map_value_class(values, *dst, changed);
            } else if all_string {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::String, changed);
            } else if all_array {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::Array, changed);
            } else if all_map {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::Map, changed);
            } else if (saw_array_or_void || (saw_void_sentinel && saw_array))
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_map
                && !saw_map_or_void
            {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::ArrayOrVoid,
                    changed,
                );
            } else if (saw_map_or_void || (saw_void_sentinel && saw_map))
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_array_or_void
            {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::MapOrVoid,
                    changed,
                );
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
            } else if (saw_scalar_or_void || (saw_void_sentinel && saw_scalar))
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::ScalarOrVoid,
                    changed,
                );
            } else if saw_void_sentinel && !saw_scalar {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::VoidSentinel,
                    changed,
                );
            } else if saw_string || saw_array || saw_map || saw_array_or_void || saw_map_or_void {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::I64, changed);
            }
            None
        }
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            let cond_class = value_class(values, *cond);
            if cond_class == GenericPureValueClass::Unknown {
                if *changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            if !matches!(
                cond_class,
                GenericPureValueClass::Bool | GenericPureValueClass::I64
            ) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }

            let then_class = value_class(values, *then_val);
            let else_class = value_class(values, *else_val);
            if then_class == GenericPureValueClass::Unknown
                && else_class == GenericPureValueClass::Unknown
            {
                if *changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            if then_class == GenericPureValueClass::Unknown {
                set_proven_flow_value_class(values, *then_val, else_class, changed);
                return None;
            }
            if else_class == GenericPureValueClass::Unknown {
                set_proven_flow_value_class(values, *else_val, then_class, changed);
                return None;
            }

            let Some(selected_class) = generic_pure_select_value_class(then_class, else_class)
            else {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            };
            if matches!(
                selected_class,
                GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
            ) {
                *has_string_surface = true;
            }
            set_proven_flow_value_class(values, *dst, selected_class, changed);
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if matches!(
            classify_extern_call_route(name, args.len()),
            Some(
                ExternCallRouteKind::EnvGet
                    | ExternCallRouteKind::Stage1EmitProgramJson
                    | ExternCallRouteKind::Stage1EmitMirFromSource
            )
        ) =>
        {
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
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if classify_extern_call_route(name, args.len())
            == Some(ExternCallRouteKind::HostBridgeExternInvoke) =>
        {
            if let Some(dst) = dst {
                *has_string_surface = true;
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    changed,
                );
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
            if let Some(class) =
                generic_pure_string_route_value_class(function, block, instruction_index)
            {
                if matches!(
                    class,
                    GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
                ) {
                    *has_string_surface = true;
                }
                if let Some(dst) = dst {
                    set_proven_flow_value_class(values, *dst, class, changed);
                }
                return None;
            }
            let receiver_class = value_class(values, *receiver);
            if generic_pure_string_accepts_collection_birth_method(
                box_name,
                method,
                args,
                receiver_class,
            ) {
                if let Some(dst) = dst {
                    set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                }
                return None;
            }
            if generic_pure_string_accepts_array_push_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                if let Some(dst) = dst {
                    set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                }
                return None;
            }
            if generic_pure_string_accepts_map_set_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                if let Some(dst) = dst {
                    set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                }
                return None;
            }
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
            if generic_pure_string_accepts_length_method(
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
            if generic_pure_string_accepts_lastindexof_method(
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
            if generic_pure_string_accepts_contains_method(
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
                set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
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
            args,
            ..
        } if !super::supported_backend_global(name) => {
            if is_hostbridge_extern_invoke_symbol(name, args.len()) {
                if let Some(dst) = dst {
                    *has_string_surface = true;
                    set_proven_flow_value_class(
                        values,
                        *dst,
                        GenericPureValueClass::StringOrVoid,
                        changed,
                    );
                }
                return None;
            }
            if generic_pure_string_global_name_is_self(name, current_function_name) {
                if let Some(dst) = dst {
                    *has_string_surface = true;
                    set_proven_flow_value_class(
                        values,
                        *dst,
                        GenericPureValueClass::String,
                        changed,
                    );
                }
                return None;
            }
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
                | GlobalCallTargetShape::ProgramJsonEmitBody
                | GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody => {
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
                GlobalCallTargetShape::StaticStringArrayBody => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::Array,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::MirSchemaMapConstructorBody
                | GlobalCallTargetShape::BoxTypeInspectorDescribeBody => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::Map,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::BuilderRegistryDispatchBody => {
                    if let Some(dst) = dst {
                        *has_string_surface = true;
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::StringOrVoid,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::PatternUtilLocalValueProbeBody => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::ScalarOrVoid,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::GenericStringVoidLoggingBody => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::VoidSentinel,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallTargetShape::NumericI64Leaf => {
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
                GlobalCallTargetShape::GenericI64Body => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            generic_pure_string_generic_i64_target_value_class(target),
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

fn generic_pure_string_route_value_class(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
) -> Option<GenericPureValueClass> {
    let route = function
        .metadata
        .generic_method_routes
        .iter()
        .find(|route| {
            route.block() == block
                && route.instruction_index() == instruction_index
                && matches!(
                    route.route_id(),
                    "generic_method.get" | "generic_method.keys"
                )
        })?;
    match route.proof_tag() {
        "mir_json_const_value_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "type" => Some(GenericPureValueClass::StringOrVoid),
                "value" => Some(GenericPureValueClass::I64),
                _ => None,
            }
        }
        "mir_json_phi_incoming_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::Array)
        }
        "mir_json_phi_incoming_pair_scalar" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::I64)
        }
        "mir_json_callee_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "receiver" => Some(GenericPureValueClass::ScalarOrVoid),
                "type" | "name" | "box_name" | "method" | "box_type" => {
                    Some(GenericPureValueClass::StringOrVoid)
                }
                _ => None,
            }
        }
        "mir_json_vid_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::I64)
        }
        "mir_json_effects_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::StringOrVoid)
        }
        "mir_json_block_inst_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::ScalarOrVoid)
        }
        "mir_json_function_block_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::ScalarOrVoid)
        }
        "mir_json_params_array_item" if route.route_kind_tag() == "array_slot_load_any" => {
            Some(GenericPureValueClass::I64)
        }
        "mir_json_flags_rec_access" => match route.route_kind_tag() {
            "array_slot_load_any" => Some(GenericPureValueClass::String),
            "runtime_data_load_any" => Some(GenericPureValueClass::StringOrVoid),
            _ => None,
        },
        "mir_json_flags_keys" if route.route_kind_tag() == "map_keys_array" => {
            Some(GenericPureValueClass::Array)
        }
        "mir_json_block_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "instructions" => Some(GenericPureValueClass::Array),
                "id" => Some(GenericPureValueClass::ScalarOrVoid),
                _ => None,
            }
        }
        "mir_json_function_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "name" => Some(GenericPureValueClass::StringOrVoid),
                "params" | "blocks" => Some(GenericPureValueClass::Array),
                "flags" => Some(GenericPureValueClass::Map),
                _ => None,
            }
        }
        "mir_json_module_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "functions" => Some(GenericPureValueClass::Array),
                "functions_0" => Some(GenericPureValueClass::Map),
                _ => None,
            }
        }
        "mir_json_module_function_array_item"
            if route.route_kind_tag() == "array_slot_load_any" =>
        {
            Some(GenericPureValueClass::Map)
        }
        "mir_json_inst_field" if route.route_kind_tag() == "runtime_data_load_any" => {
            match route.key_const_text()? {
                "op" | "operation" | "op_kind" | "cmp" | "value" => {
                    Some(GenericPureValueClass::StringOrVoid)
                }
                "args" | "effects" => Some(GenericPureValueClass::Array),
                "dst" | "lhs" | "rhs" | "cond" | "then" | "else" | "target" | "incoming"
                | "values" | "mir_call" | "callee" | "func" | "name" => {
                    Some(GenericPureValueClass::ScalarOrVoid)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn generic_pure_string_generic_i64_target_value_class(
    target: &GlobalCallTargetFacts,
) -> GenericPureValueClass {
    match target.return_type() {
        Some(MirType::Bool) => GenericPureValueClass::Bool,
        Some(MirType::Unknown | MirType::Void) => GenericPureValueClass::ScalarOrVoid,
        _ => GenericPureValueClass::I64,
    }
}

fn generic_pure_select_value_class(
    then_class: GenericPureValueClass,
    else_class: GenericPureValueClass,
) -> Option<GenericPureValueClass> {
    if then_class == else_class {
        return Some(then_class);
    }
    match (then_class, else_class) {
        (GenericPureValueClass::String, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::String)
        | (GenericPureValueClass::StringOrVoid, GenericPureValueClass::String)
        | (GenericPureValueClass::String, GenericPureValueClass::StringOrVoid)
        | (GenericPureValueClass::StringOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::StringOrVoid) => {
            Some(GenericPureValueClass::StringOrVoid)
        }
        (GenericPureValueClass::Array, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::Array)
        | (GenericPureValueClass::ArrayOrVoid, GenericPureValueClass::Array)
        | (GenericPureValueClass::Array, GenericPureValueClass::ArrayOrVoid)
        | (GenericPureValueClass::ArrayOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::ArrayOrVoid) => {
            Some(GenericPureValueClass::ArrayOrVoid)
        }
        (GenericPureValueClass::Map, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::Map)
        | (GenericPureValueClass::MapOrVoid, GenericPureValueClass::Map)
        | (GenericPureValueClass::Map, GenericPureValueClass::MapOrVoid)
        | (GenericPureValueClass::MapOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::MapOrVoid) => {
            Some(GenericPureValueClass::MapOrVoid)
        }
        (GenericPureValueClass::I64, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::I64)
        | (GenericPureValueClass::Bool, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::Bool)
        | (GenericPureValueClass::ScalarOrVoid, GenericPureValueClass::I64)
        | (GenericPureValueClass::I64, GenericPureValueClass::ScalarOrVoid)
        | (GenericPureValueClass::ScalarOrVoid, GenericPureValueClass::Bool)
        | (GenericPureValueClass::Bool, GenericPureValueClass::ScalarOrVoid)
        | (GenericPureValueClass::ScalarOrVoid, GenericPureValueClass::VoidSentinel)
        | (GenericPureValueClass::VoidSentinel, GenericPureValueClass::ScalarOrVoid) => {
            Some(GenericPureValueClass::ScalarOrVoid)
        }
        _ => None,
    }
}
