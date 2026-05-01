/*!
 * MIR-owned route plans for unsupported global user calls.
 *
 * This module does not make global calls lowerable. It records the typed
 * owner boundary in MIR metadata so backend shims can fail-fast from a plan
 * instead of rediscovering unsupported `Global(...)` names from raw MIR.
 */

use super::{
    BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::BTreeMap;

mod generic_i64_body;
mod model;

use generic_i64_body::is_generic_i64_body_function;
pub use model::{
    GlobalCallRoute, GlobalCallRouteSite, GlobalCallTargetFacts, GlobalCallTargetShape,
};
use model::{GlobalCallShapeBlocker, GlobalCallTargetClassification, GlobalCallTargetShapeReason};

fn supported_backend_global(name: &str) -> bool {
    matches!(name, "print")
}

pub fn refresh_module_global_call_routes(module: &mut MirModule) {
    let targets = collect_global_call_targets(module);
    for function in module.functions.values_mut() {
        refresh_function_global_call_routes_with_targets(function, &targets);
    }
}

pub fn refresh_function_global_call_routes(function: &mut MirFunction) {
    refresh_function_global_call_routes_with_targets(function, &BTreeMap::new());
}

fn collect_global_call_targets(module: &MirModule) -> BTreeMap<String, GlobalCallTargetFacts> {
    let mut targets = module
        .functions
        .iter()
        .map(|(name, function)| {
            let arity = if function.params.is_empty() {
                function.signature.params.len()
            } else {
                function.params.len()
            };
            (
                name.clone(),
                GlobalCallTargetFacts::present_with_symbol_and_return_type(
                    name.clone(),
                    arity,
                    function.signature.return_type.clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut function_names = module.functions.keys().collect::<Vec<_>>();
    function_names.sort();
    for _ in 0..module.functions.len() {
        let mut changed = false;
        for name in &function_names {
            let name = *name;
            let Some(function) = module.functions.get(name) else {
                continue;
            };
            let Some(current) = targets.get(name).cloned() else {
                continue;
            };
            let classification = classify_global_call_target_shape(function, &targets);
            if current.shape() != classification.shape
                || current.shape_reason() != classification.reason
                || current.shape_blocker != classification.blocker
            {
                targets.insert(name.clone(), current.with_classification(classification));
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    targets
}

fn classify_global_call_target_shape(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> GlobalCallTargetClassification {
    if function.params.len() != function.signature.params.len() {
        return GlobalCallTargetClassification::unknown(
            GlobalCallTargetShapeReason::ParamBindingMismatch,
        );
    }
    if function
        .signature
        .params
        .iter()
        .all(|ty| *ty == MirType::Integer)
        && function.signature.return_type == MirType::Integer
        && is_numeric_i64_leaf_function(function)
    {
        return GlobalCallTargetClassification::direct(GlobalCallTargetShape::NumericI64Leaf);
    }
    if is_generic_i64_body_function(function, targets) {
        return GlobalCallTargetClassification::direct(GlobalCallTargetShape::GenericI64Body);
    }
    if function.signature.return_type == MirType::Void {
        if let Some(reject) = generic_string_void_sentinel_body_reject_reason(function, targets) {
            if reject.reason
                == GlobalCallTargetShapeReason::GenericStringReturnVoidSentinelCandidate
                && reject.blocker.is_none()
            {
                return GlobalCallTargetClassification::direct(
                    GlobalCallTargetShape::GenericStringOrVoidSentinelBody,
                );
            }
            return if let Some(blocker) = reject.blocker {
                GlobalCallTargetClassification::unknown_with_blocker(
                    reject.reason,
                    blocker.symbol,
                    blocker.reason,
                )
            } else {
                GlobalCallTargetClassification::unknown(reject.reason)
            };
        }
    }
    if let Some(reject) = generic_pure_string_body_reject_reason(function, targets) {
        if let Some(blocker) = reject.blocker {
            GlobalCallTargetClassification::unknown_with_blocker(
                reject.reason,
                blocker.symbol,
                blocker.reason,
            )
        } else {
            GlobalCallTargetClassification::unknown(reject.reason)
        }
    } else {
        GlobalCallTargetClassification::direct(GlobalCallTargetShape::GenericPureStringBody)
    }
}

fn is_numeric_i64_leaf_function(function: &MirFunction) -> bool {
    if function.blocks.len() != 1 {
        return false;
    }
    let Some(block) = function.blocks.get(&function.entry_block) else {
        return false;
    };
    matches!(
        block.terminator,
        Some(MirInstruction::Return { value: Some(_) })
    ) && block
        .instructions
        .iter()
        .all(is_numeric_i64_leaf_instruction)
}

fn is_numeric_i64_leaf_instruction(instruction: &MirInstruction) -> bool {
    match instruction {
        MirInstruction::Const {
            value: ConstValue::Integer(_),
            ..
        } => true,
        MirInstruction::Copy { .. } => true,
        MirInstruction::BinOp { op, .. } => matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
        ),
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericPureValueClass {
    Unknown,
    I64,
    Bool,
    String,
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
        if let Some(class) = generic_pure_value_class_from_type(ty) {
            set_value_class(values, *value, class, &mut changed);
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct GenericPureStringReject {
    reason: GlobalCallTargetShapeReason,
    blocker: Option<GlobalCallShapeBlocker>,
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

    fn with_shape_blocker(
        reason: GlobalCallTargetShapeReason,
        blocker: GlobalCallShapeBlocker,
    ) -> Self {
        Self {
            reason,
            blocker: Some(blocker),
        }
    }
}

fn generic_pure_string_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if !generic_pure_string_abi_type_is_handle_compatible(&function.signature.return_type) {
        if function.signature.return_type == MirType::Void {
            if let Some(reject) = generic_string_void_sentinel_body_reject_reason(function, targets)
            {
                return Some(reject);
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

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
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
                if class != GenericPureValueClass::String {
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

fn generic_pure_string_abi_type_is_handle_compatible(ty: &MirType) -> bool {
    match ty {
        MirType::Integer | MirType::String | MirType::Unknown => true,
        MirType::Box(name) => name == "StringBox",
        _ => false,
    }
}

fn generic_string_void_sentinel_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if !generic_string_void_sentinel_return_candidate(function, targets) {
        if let Some(reject) = generic_string_void_sentinel_return_global_blocker(function, targets)
        {
            return Some(reject);
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

// Return-profile evidence only: this must not make the target lowerable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericStringReturnValueClass {
    Unknown,
    Void,
    String,
    StringOrVoid,
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
        if let Some(class) = generic_string_return_value_class_from_type(ty) {
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
        MirType::Integer | MirType::Bool | MirType::Float => {
            Some(GenericStringReturnValueClass::Other)
        }
        MirType::Box(name) if matches!(name.as_str(), "IntegerBox" | "BoolBox" | "FloatBox") => {
            Some(GenericStringReturnValueClass::Other)
        }
        MirType::Void => Some(GenericStringReturnValueClass::Void),
        _ => None,
    }
}

fn generic_string_void_sentinel_return_global_blocker(
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
                    if generic_string_return_value_class(&values, *value).is_void_like() {
                        saw_void = true;
                    } else if return_blocker.is_none() {
                        return_blocker = blockers.get(value).cloned();
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

fn generic_string_void_sentinel_return_candidate(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
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
        MirInstruction::Phi { dst, inputs, .. } => {
            let mut class = GenericStringReturnValueClass::Unknown;
            let mut saw_unknown = false;
            for (_, value) in inputs {
                let input_class = generic_string_return_value_class(values, *value);
                if input_class == GenericStringReturnValueClass::Unknown {
                    saw_unknown = true;
                    break;
                }
                class = merge_generic_string_return_value_class(class, input_class);
            }
            if !saw_unknown {
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
            set_generic_string_return_value_class(values, *dst, class, changed);
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
                    GlobalCallTargetShape::GenericStringOrVoidSentinelBody => {
                        GenericStringReturnValueClass::StringOrVoid
                    }
                    GlobalCallTargetShape::NumericI64Leaf => GenericStringReturnValueClass::Other,
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

fn direct_unknown_global_target_blocker(
    name: &str,
    target: &GlobalCallTargetFacts,
) -> GlobalCallShapeBlocker {
    GlobalCallShapeBlocker {
        symbol: target.symbol().unwrap_or(name).to_string(),
        reason: target.shape_reason(),
    }
}

fn propagated_unknown_global_target_blocker(
    name: &str,
    target: &GlobalCallTargetFacts,
) -> GlobalCallShapeBlocker {
    if let Some(symbol) = target.shape_blocker_symbol() {
        return GlobalCallShapeBlocker {
            symbol: symbol.to_string(),
            reason: target
                .shape_blocker_reason()
                .or_else(|| target.shape_reason()),
        };
    }
    direct_unknown_global_target_blocker(name, target)
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

impl GenericStringReturnValueClass {
    fn is_void_like(self) -> bool {
        matches!(
            self,
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid
        )
    }
}

fn format_mir_type_label(ty: &MirType) -> String {
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
    has_string_surface: &mut bool,
    has_void_sentinel_const: &mut bool,
    changed: &mut bool,
) -> Option<GenericPureStringReject> {
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
                set_value_class(values, *dst, GenericPureValueClass::String, changed);
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
            let has_void_sentinel = lhs_class == GenericPureValueClass::VoidSentinel
                || rhs_class == GenericPureValueClass::VoidSentinel;
            if has_void_sentinel {
                let comparable =
                    matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
                        && matches!(
                            (lhs_class, rhs_class),
                            (
                                GenericPureValueClass::String,
                                GenericPureValueClass::VoidSentinel
                            ) | (
                                GenericPureValueClass::VoidSentinel,
                                GenericPureValueClass::String
                            ) | (
                                GenericPureValueClass::VoidSentinel,
                                GenericPureValueClass::VoidSentinel
                            )
                        );
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
            let mut all_string = !inputs.is_empty();
            let mut saw_unknown = false;
            for (_, value) in inputs {
                let class = value_class(values, *value);
                saw_unknown |= class == GenericPureValueClass::Unknown;
                saw_string |= class == GenericPureValueClass::String;
                all_string &= class == GenericPureValueClass::String;
                if class == GenericPureValueClass::VoidSentinel {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                    ));
                }
            }
            if saw_unknown {
                return None;
            } else if all_string {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::String, changed);
            } else if saw_string {
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
                set_value_class(values, *dst, GenericPureValueClass::String, changed);
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
        } if supported_backend_global(name) => None,
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            ..
        } if !supported_backend_global(name) => {
            let Some(target) = lookup_global_call_target(name, targets) else {
                return Some(GenericPureStringReject::with_blocker(
                    GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing,
                    crate::mir::naming::normalize_static_global_name(name),
                    None,
                ));
            };
            match target.shape() {
                GlobalCallTargetShape::GenericPureStringBody
                | GlobalCallTargetShape::GenericStringOrVoidSentinelBody => {
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

fn generic_pure_compare_proves_i64(op: crate::mir::CompareOp) -> bool {
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

fn value_class(
    values: &BTreeMap<ValueId, GenericPureValueClass>,
    value: ValueId,
) -> GenericPureValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericPureValueClass::Unknown)
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
        Some(GenericPureValueClass::Unknown | GenericPureValueClass::VoidSentinel) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(_) => {}
    }
}

fn refresh_function_global_call_routes_with_targets(
    function: &mut MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) {
    let mut routes = Vec::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            if supported_backend_global(name) {
                continue;
            }
            routes.push(GlobalCallRoute::new(
                GlobalCallRouteSite::new(block_id, instruction_index),
                name,
                args.len(),
                *dst,
                lookup_global_call_target(name, targets)
                    .cloned()
                    .unwrap_or_else(GlobalCallTargetFacts::missing),
            ));
        }
    }

    function.metadata.global_call_routes = routes;
}

fn lookup_global_call_target<'a>(
    name: &str,
    targets: &'a BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<&'a GlobalCallTargetFacts> {
    if let Some(target) = targets.get(name) {
        return Some(target);
    }
    let canonical = crate::mir::naming::normalize_static_global_name(name);
    if canonical == name {
        return None;
    }
    targets.get(&canonical)
}

#[cfg(test)]
mod tests;
