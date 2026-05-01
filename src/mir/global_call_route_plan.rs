/*!
 * MIR-owned route plans for unsupported global user calls.
 *
 * This module does not make global calls lowerable. It records the typed
 * owner boundary in MIR metadata so backend shims can fail-fast from a plan
 * instead of rediscovering unsupported `Global(...)` names from raw MIR.
 */

use super::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType,
    ValueId,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalCallRouteSite {
    block: BasicBlockId,
    instruction_index: usize,
}

impl GlobalCallRouteSite {
    pub fn new(block: BasicBlockId, instruction_index: usize) -> Self {
        Self {
            block,
            instruction_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalCallRoute {
    site: GlobalCallRouteSite,
    callee_name: String,
    arity: usize,
    result_value: Option<ValueId>,
    target: GlobalCallTargetFacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlobalCallTargetShape {
    #[default]
    Unknown,
    NumericI64Leaf,
    GenericPureStringBody,
    GenericStringOrVoidSentinelBody,
    GenericI64Body,
}

impl GlobalCallTargetShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NumericI64Leaf => "numeric_i64_leaf",
            Self::GenericPureStringBody => "generic_pure_string_body",
            Self::GenericStringOrVoidSentinelBody => "generic_string_or_void_sentinel_body",
            Self::GenericI64Body => "generic_i64_body",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GlobalCallTargetShapeReason {
    ParamBindingMismatch,
    GenericStringReturnAbiNotHandleCompatible,
    GenericStringReturnObjectAbiNotHandleCompatible,
    GenericStringReturnVoidSentinelCandidate,
    GenericStringParamAbiNotHandleCompatible,
    GenericStringUnsupportedInstruction,
    GenericStringUnsupportedVoidSentinelConst,
    GenericStringUnsupportedCall,
    GenericStringUnsupportedMethodCall,
    GenericStringUnsupportedExternCall,
    GenericStringGlobalTargetMissing,
    GenericStringGlobalTargetShapeUnknown,
    GenericStringNoStringSurface,
    GenericStringReturnNotString,
}

impl GlobalCallTargetShapeReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::ParamBindingMismatch => "param_binding_mismatch",
            Self::GenericStringReturnAbiNotHandleCompatible => {
                "generic_string_return_abi_not_handle_compatible"
            }
            Self::GenericStringReturnObjectAbiNotHandleCompatible => {
                "generic_string_return_object_abi_not_handle_compatible"
            }
            Self::GenericStringReturnVoidSentinelCandidate => {
                "generic_string_return_void_sentinel_candidate"
            }
            Self::GenericStringParamAbiNotHandleCompatible => {
                "generic_string_param_abi_not_handle_compatible"
            }
            Self::GenericStringUnsupportedInstruction => "generic_string_unsupported_instruction",
            Self::GenericStringUnsupportedVoidSentinelConst => {
                "generic_string_unsupported_void_sentinel_const"
            }
            Self::GenericStringUnsupportedCall => "generic_string_unsupported_call",
            Self::GenericStringUnsupportedMethodCall => "generic_string_unsupported_method_call",
            Self::GenericStringUnsupportedExternCall => "generic_string_unsupported_extern_call",
            Self::GenericStringGlobalTargetMissing => "generic_string_global_target_missing",
            Self::GenericStringGlobalTargetShapeUnknown => {
                "generic_string_global_target_shape_unknown"
            }
            Self::GenericStringNoStringSurface => "generic_string_no_string_surface",
            Self::GenericStringReturnNotString => "generic_string_return_not_string",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalCallTargetClassification {
    shape: GlobalCallTargetShape,
    reason: Option<GlobalCallTargetShapeReason>,
    blocker: Option<GlobalCallShapeBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalCallShapeBlocker {
    symbol: String,
    reason: Option<GlobalCallTargetShapeReason>,
}

impl GlobalCallTargetClassification {
    fn direct(shape: GlobalCallTargetShape) -> Self {
        Self {
            shape,
            reason: None,
            blocker: None,
        }
    }

    fn unknown(reason: GlobalCallTargetShapeReason) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
            reason: Some(reason),
            blocker: None,
        }
    }

    fn unknown_with_blocker(
        reason: GlobalCallTargetShapeReason,
        symbol: impl Into<String>,
        blocker_reason: Option<GlobalCallTargetShapeReason>,
    ) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
            reason: Some(reason),
            blocker: Some(GlobalCallShapeBlocker {
                symbol: symbol.into(),
                reason: blocker_reason,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GlobalCallTargetFacts {
    exists: bool,
    symbol: Option<String>,
    arity: Option<usize>,
    return_type: Option<MirType>,
    shape: GlobalCallTargetShape,
    shape_reason: Option<GlobalCallTargetShapeReason>,
    shape_blocker: Option<GlobalCallShapeBlocker>,
}

impl GlobalCallTargetFacts {
    pub fn missing() -> Self {
        Self::default()
    }

    pub fn present(arity: usize) -> Self {
        Self {
            exists: true,
            symbol: None,
            arity: Some(arity),
            return_type: None,
            shape: GlobalCallTargetShape::Unknown,
            shape_reason: None,
            shape_blocker: None,
        }
    }

    pub fn present_with_symbol(symbol: impl Into<String>, arity: usize) -> Self {
        Self {
            exists: true,
            symbol: Some(symbol.into()),
            arity: Some(arity),
            return_type: None,
            shape: GlobalCallTargetShape::Unknown,
            shape_reason: None,
            shape_blocker: None,
        }
    }

    pub fn present_with_symbol_and_return_type(
        symbol: impl Into<String>,
        arity: usize,
        return_type: MirType,
    ) -> Self {
        Self {
            exists: true,
            symbol: Some(symbol.into()),
            arity: Some(arity),
            return_type: Some(return_type),
            shape: GlobalCallTargetShape::Unknown,
            shape_reason: None,
            shape_blocker: None,
        }
    }

    pub fn present_with_shape(arity: usize, shape: GlobalCallTargetShape) -> Self {
        Self {
            exists: true,
            symbol: None,
            arity: Some(arity),
            return_type: None,
            shape,
            shape_reason: None,
            shape_blocker: None,
        }
    }

    pub fn exists(&self) -> bool {
        self.exists
    }

    pub fn arity(&self) -> Option<usize> {
        self.arity
    }

    pub fn symbol(&self) -> Option<&str> {
        self.symbol.as_deref()
    }

    pub fn return_type(&self) -> Option<&MirType> {
        self.return_type.as_ref()
    }

    pub fn shape(&self) -> GlobalCallTargetShape {
        self.shape
    }

    fn shape_reason(&self) -> Option<GlobalCallTargetShapeReason> {
        self.shape_reason
    }

    fn shape_blocker_symbol(&self) -> Option<&str> {
        self.shape_blocker
            .as_ref()
            .map(|blocker| blocker.symbol.as_str())
    }

    fn shape_blocker_reason(&self) -> Option<GlobalCallTargetShapeReason> {
        self.shape_blocker
            .as_ref()
            .and_then(|blocker| blocker.reason)
    }

    fn with_classification(mut self, classification: GlobalCallTargetClassification) -> Self {
        self.shape = classification.shape;
        self.shape_reason = classification.reason;
        self.shape_blocker = classification.blocker;
        self
    }
}

impl GlobalCallRoute {
    pub fn new(
        site: GlobalCallRouteSite,
        callee_name: impl Into<String>,
        arity: usize,
        result_value: Option<ValueId>,
        target: GlobalCallTargetFacts,
    ) -> Self {
        Self {
            site,
            callee_name: callee_name.into(),
            arity,
            result_value,
            target,
        }
    }

    pub fn block(&self) -> BasicBlockId {
        self.site.block
    }

    pub fn instruction_index(&self) -> usize {
        self.site.instruction_index
    }

    pub fn route_id(&self) -> &'static str {
        "global.user_call"
    }

    pub fn core_op(&self) -> &'static str {
        "UserGlobalCall"
    }

    pub fn tier(&self) -> &'static str {
        if self.is_direct_abi_target() {
            "DirectAbi"
        } else {
            "Unsupported"
        }
    }

    pub fn emit_kind(&self) -> &'static str {
        if self.is_direct_abi_target() {
            "direct_function_call"
        } else {
            "unsupported"
        }
    }

    pub fn proof(&self) -> &'static str {
        match self.direct_target_shape() {
            Some(GlobalCallTargetShape::NumericI64Leaf) => "typed_global_call_leaf_numeric_i64",
            Some(GlobalCallTargetShape::GenericPureStringBody) => {
                "typed_global_call_generic_pure_string"
            }
            Some(GlobalCallTargetShape::GenericStringOrVoidSentinelBody) => {
                "typed_global_call_generic_string_or_void_sentinel"
            }
            Some(GlobalCallTargetShape::GenericI64Body) => "typed_global_call_generic_i64",
            _ => "typed_global_call_contract_missing",
        }
    }

    pub fn route_kind(&self) -> &'static str {
        "global.user_call"
    }

    pub fn callee_name(&self) -> &str {
        &self.callee_name
    }

    pub fn target_symbol(&self) -> Option<&str> {
        if !self.target_exists() {
            return None;
        }
        self.target.symbol().or(Some(self.callee_name()))
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn result_value(&self) -> Option<ValueId> {
        self.result_value
    }

    pub fn target_exists(&self) -> bool {
        self.target.exists()
    }

    pub fn target_arity(&self) -> Option<usize> {
        self.target.arity()
    }

    pub fn target_return_type(&self) -> Option<String> {
        if !self.target_exists() {
            return None;
        }
        self.target.return_type().map(format_mir_type_label)
    }

    pub fn target_shape(&self) -> Option<&'static str> {
        self.target_exists()
            .then_some(self.target.shape().as_str())
            .filter(|shape| *shape != "unknown")
    }

    pub fn target_shape_reason(&self) -> Option<&'static str> {
        if !self.target_exists() || self.target_shape().is_some() {
            return None;
        }
        self.target.shape_reason().map(|reason| reason.as_str())
    }

    pub fn target_shape_blocker_symbol(&self) -> Option<&str> {
        if !self.target_exists() || self.target_shape().is_some() {
            return None;
        }
        self.target.shape_blocker_symbol()
    }

    pub fn target_shape_blocker_reason(&self) -> Option<&'static str> {
        if !self.target_exists() || self.target_shape().is_some() {
            return None;
        }
        self.target
            .shape_blocker_reason()
            .map(|reason| reason.as_str())
    }

    pub fn arity_matches(&self) -> Option<bool> {
        self.target_arity()
            .map(|target_arity| target_arity == self.arity)
    }

    pub fn value_demand(&self) -> &'static str {
        match self.direct_target_shape() {
            Some(GlobalCallTargetShape::NumericI64Leaf)
            | Some(GlobalCallTargetShape::GenericI64Body) => "scalar_i64",
            Some(
                GlobalCallTargetShape::GenericPureStringBody
                | GlobalCallTargetShape::GenericStringOrVoidSentinelBody,
            ) => "runtime_i64_or_handle",
            _ => "typed_global_call_contract_missing",
        }
    }

    pub fn return_shape(&self) -> Option<&'static str> {
        match self.direct_target_shape() {
            Some(GlobalCallTargetShape::NumericI64Leaf)
            | Some(GlobalCallTargetShape::GenericI64Body) => Some("ScalarI64"),
            Some(GlobalCallTargetShape::GenericPureStringBody) => Some("string_handle"),
            Some(GlobalCallTargetShape::GenericStringOrVoidSentinelBody) => {
                Some("string_handle_or_null")
            }
            _ => None,
        }
    }

    pub fn reason(&self) -> Option<&'static str> {
        if self.is_direct_abi_target() {
            return None;
        }
        match self.arity_matches() {
            Some(true) => Some("missing_multi_function_emitter"),
            Some(false) => Some("global_call_arity_mismatch"),
            None => Some("unknown_global_callee"),
        }
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        &["call.global"]
    }

    fn is_direct_abi_target(&self) -> bool {
        self.direct_target_shape().is_some()
    }

    fn direct_target_shape(&self) -> Option<GlobalCallTargetShape> {
        if !(self.target_exists() && self.arity_matches() == Some(true)) {
            return None;
        }
        match self.target.shape() {
            GlobalCallTargetShape::NumericI64Leaf
            | GlobalCallTargetShape::GenericPureStringBody
            | GlobalCallTargetShape::GenericStringOrVoidSentinelBody
            | GlobalCallTargetShape::GenericI64Body => Some(self.target.shape()),
            GlobalCallTargetShape::Unknown => None,
        }
    }
}

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
    for _ in 0..module.functions.len() {
        let mut changed = false;
        for (name, function) in &module.functions {
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
enum GenericI64ValueClass {
    Unknown,
    I64,
    Bool,
    String,
    VoidSentinel,
}

fn is_generic_i64_body_function(
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
    for param in &function.params {
        values.insert(*param, GenericI64ValueClass::String);
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
            if class == GenericI64ValueClass::Unknown {
                true
            } else {
                set_generic_i64_value_class(values, *dst, class, changed)
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
            if lhs_class == GenericI64ValueClass::Unknown
                || rhs_class == GenericI64ValueClass::Unknown
            {
                return true;
            }
            let eq_ne = matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne);
            let comparable = match (lhs_class, rhs_class) {
                (GenericI64ValueClass::String, GenericI64ValueClass::String) => eq_ne,
                (GenericI64ValueClass::String, GenericI64ValueClass::VoidSentinel)
                | (GenericI64ValueClass::VoidSentinel, GenericI64ValueClass::String) => eq_ne,
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
            let mut merged = GenericI64ValueClass::Unknown;
            for (_, value) in inputs {
                let class = generic_i64_value_class(values, *value);
                if class == GenericI64ValueClass::Unknown {
                    return true;
                }
                if merged == GenericI64ValueClass::Unknown {
                    merged = class;
                } else if merged != class {
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
            callee: Some(Callee::Method { .. }),
            ..
        } => false,
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if supported_backend_global(name) => false,
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
                | GlobalCallTargetShape::GenericStringOrVoidSentinelBody => {
                    GenericI64ValueClass::String
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
        Some(_) => false,
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
                set_value_class(values, *dst, class, changed);
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
                set_value_class(values, *dst, GenericPureValueClass::String, changed);
            } else if saw_string {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
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
                        set_value_class(values, *dst, GenericPureValueClass::String, changed);
                    }
                    None
                }
                GlobalCallTargetShape::NumericI64Leaf | GlobalCallTargetShape::GenericI64Body => {
                    if let Some(dst) = dst {
                        set_value_class(values, *dst, GenericPureValueClass::I64, changed);
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
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, CompareOp, EffectMask, FunctionSignature, MirType};

    fn make_function_with_global_call_args(
        name: &str,
        dst: Option<ValueId>,
        args: Vec<ValueId>,
    ) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = function
            .blocks
            .entry(BasicBlockId::new(0))
            .or_insert_with(|| BasicBlock::new(BasicBlockId::new(0)));
        block.instructions.push(MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(Callee::Global(name.to_string())),
            args,
            effects: EffectMask::PURE,
        });
        function
    }

    fn make_function_with_global_call(name: &str, dst: Option<ValueId>) -> MirFunction {
        make_function_with_global_call_args(name, dst, vec![ValueId::new(1), ValueId::new(2)])
    }

    #[test]
    fn refresh_function_global_call_routes_records_unsupported_global_call() {
        let mut function = make_function_with_global_call(
            "Stage1ModeContractBox.resolve_mode/0",
            Some(ValueId::new(7)),
        );
        refresh_function_global_call_routes(&mut function);

        assert_eq!(function.metadata.global_call_routes.len(), 1);
        let route = &function.metadata.global_call_routes[0];
        assert_eq!(route.block(), BasicBlockId::new(0));
        assert_eq!(route.instruction_index(), 0);
        assert_eq!(route.callee_name(), "Stage1ModeContractBox.resolve_mode/0");
        assert_eq!(route.arity(), 2);
        assert_eq!(route.result_value(), Some(ValueId::new(7)));
        assert_eq!(route.tier(), "Unsupported");
        assert!(!route.target_exists());
        assert_eq!(route.target_arity(), None);
        assert_eq!(route.target_return_type(), None);
        assert_eq!(route.target_shape(), None);
        assert_eq!(route.reason(), Some("unknown_global_callee"));
    }

    #[test]
    fn refresh_function_global_call_routes_skips_print_surface() {
        let mut function = make_function_with_global_call("print", None);
        refresh_function_global_call_routes(&mut function);
        assert!(function.metadata.global_call_routes.is_empty());
    }

    #[test]
    fn refresh_module_global_call_routes_records_target_facts() {
        let mut module = MirModule::new("global_call_target_test".to_string());
        let caller = make_function_with_global_call(
            "Stage1ModeContractBox.resolve_mode/0",
            Some(ValueId::new(7)),
        );
        let callee = MirFunction::new(
            FunctionSignature {
                name: "Stage1ModeContractBox.resolve_mode/0".to_string(),
                params: vec![MirType::Integer, MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Stage1ModeContractBox.resolve_mode/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert!(route.target_exists());
        assert_eq!(
            route.target_symbol(),
            Some("Stage1ModeContractBox.resolve_mode/0")
        );
        assert_eq!(route.target_arity(), Some(2));
        assert_eq!(route.target_return_type(), Some("i64".to_string()));
        assert_eq!(route.arity_matches(), Some(true));
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_no_string_surface")
        );
        assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
    }

    #[test]
    fn refresh_module_global_call_routes_marks_string_or_void_sentinel_body_direct_target() {
        let mut module = MirModule::new("global_call_void_sentinel_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.maybe_text/0",
            Some(ValueId::new(7)),
            vec![],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.maybe_text/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("ok".to_string()),
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.maybe_text/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert!(route.target_exists());
        assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
        assert_eq!(route.target_return_type(), Some("void".to_string()));
        assert_eq!(
            route.target_shape(),
            Some("generic_string_or_void_sentinel_body"),
            "reason={:?}",
            route.target_shape_reason()
        );
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(
            route.proof(),
            "typed_global_call_generic_string_or_void_sentinel"
        );
        assert_eq!(route.tier(), "DirectAbi");
        assert_eq!(route.return_shape(), Some("string_handle_or_null"));
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_accepts_substring_void_sentinel_body() {
        let mut module = MirModule::new("global_call_substring_void_sentinel_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.slice_or_null/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.slice_or_null/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1)];
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Bool(true),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(0),
            },
            MirInstruction::Const {
                dst: ValueId::new(4),
                value: ConstValue::Integer(4),
            },
        ]);
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(2),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3), ValueId::new(4)],
            effects: EffectMask::PURE,
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(5)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(6)),
        });
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.slice_or_null/1".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(
            route.target_shape(),
            Some("generic_string_or_void_sentinel_body"),
            "reason={:?}",
            route.target_shape_reason()
        );
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(
            route.proof(),
            "typed_global_call_generic_string_or_void_sentinel"
        );
        assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    }

    #[test]
    fn refresh_module_global_call_routes_accepts_mixed_param_substring_void_sentinel_body() {
        let mut module =
            MirModule::new("global_call_mixed_substring_void_sentinel_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.slice_or_null/2",
            Some(ValueId::new(7)),
            vec![ValueId::new(1), ValueId::new(2)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.slice_or_null/2".to_string(),
                params: vec![MirType::Unknown, MirType::Unknown],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1), ValueId::new(2)];
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::String(String::new()),
            },
            MirInstruction::BinOp {
                dst: ValueId::new(4),
                op: BinaryOp::Add,
                lhs: ValueId::new(3),
                rhs: ValueId::new(1),
            },
            MirInstruction::Const {
                dst: ValueId::new(5),
                value: ConstValue::Integer(0),
            },
            MirInstruction::Copy {
                dst: ValueId::new(12),
                src: ValueId::new(2),
            },
            MirInstruction::Compare {
                dst: ValueId::new(6),
                op: CompareOp::Lt,
                lhs: ValueId::new(12),
                rhs: ValueId::new(5),
            },
        ]);
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(6),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(7),
                value: ConstValue::Integer(1),
            },
            MirInstruction::Copy {
                dst: ValueId::new(13),
                src: ValueId::new(2),
            },
            MirInstruction::BinOp {
                dst: ValueId::new(8),
                op: BinaryOp::Add,
                lhs: ValueId::new(13),
                rhs: ValueId::new(7),
            },
            MirInstruction::Const {
                dst: ValueId::new(9),
                value: ConstValue::Integer(4),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: "substring".to_string(),
                    receiver: Some(ValueId::new(4)),
                    certainty: TypeCertainty::Union,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: vec![ValueId::new(13), ValueId::new(8)],
                effects: EffectMask::PURE,
            },
        ]);
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(10)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(11)),
        });
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.slice_or_null/2".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(
            route.target_shape(),
            Some("generic_string_or_void_sentinel_body"),
            "reason={:?}",
            route.target_shape_reason()
        );
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(
            route.proof(),
            "typed_global_call_generic_string_or_void_sentinel"
        );
        assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    }

    #[test]
    fn refresh_module_global_call_routes_marks_void_sentinel_child_blocker() {
        let mut module = MirModule::new("global_call_void_sentinel_child_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.maybe_text/0",
            Some(ValueId::new(7)),
            vec![],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.maybe_text/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("ok".to_string()),
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        let flag = MirFunction::new(
            FunctionSignature {
                name: "Helper.flag/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module.functions.insert("Helper.flag/0".to_string(), flag);
        module
            .functions
            .insert("Helper.maybe_text/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert!(route.target_exists());
        assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
        assert_eq!(route.target_return_type(), Some("void".to_string()));
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_global_target_shape_unknown")
        );
        assert_eq!(route.target_shape_blocker_symbol(), Some("Helper.flag/0"));
        assert_eq!(
            route.target_shape_blocker_reason(),
            Some("generic_string_no_string_surface")
        );
        assert_eq!(route.tier(), "Unsupported");
        assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
    }

    #[test]
    fn refresh_module_global_call_routes_marks_void_sentinel_return_child_blocker() {
        let mut module = MirModule::new("global_call_void_sentinel_return_child_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.maybe_text/0",
            Some(ValueId::new(7)),
            vec![],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.maybe_text/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.pending/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        let pending = MirFunction::new(
            FunctionSignature {
                name: "Helper.pending/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.pending/0".to_string(), pending);
        module
            .functions
            .insert("Helper.maybe_text/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert!(route.target_exists());
        assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
        assert_eq!(route.target_return_type(), Some("void".to_string()));
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_global_target_shape_unknown")
        );
        assert_eq!(
            route.target_shape_blocker_symbol(),
            Some("Helper.pending/0")
        );
        assert_eq!(
            route.target_shape_blocker_reason(),
            Some("generic_string_no_string_surface")
        );
    }

    #[test]
    fn string_return_blocker_ignores_direct_string_child_targets() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Helper.maybe_text/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.text/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        function.blocks.insert(BasicBlockId::new(1), text_block);
        function.blocks.insert(BasicBlockId::new(2), void_block);
        let mut targets = BTreeMap::new();
        targets.insert(
            "Helper.text/0".to_string(),
            GlobalCallTargetFacts::present_with_shape(
                0,
                GlobalCallTargetShape::GenericPureStringBody,
            ),
        );

        assert_eq!(
            generic_string_void_sentinel_return_global_blocker(&function, &targets),
            None
        );
    }

    #[test]
    fn refresh_module_global_call_routes_accepts_void_typed_direct_sentinel_child_return() {
        let mut module = MirModule::new("global_call_void_typed_sentinel_child_test".to_string());
        let caller =
            make_function_with_global_call_args("Helper.parent/0", Some(ValueId::new(7)), vec![]);
        let mut child = MirFunction::new(
            FunctionSignature {
                name: "Helper.child/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        child
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .instructions
            .push(MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::Bool(true),
            });
        child
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Branch {
                condition: ValueId::new(1),
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            });
        let mut child_text_block = BasicBlock::new(BasicBlockId::new(1));
        child_text_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("ok".to_string()),
        });
        child_text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut child_void_block = BasicBlock::new(BasicBlockId::new(2));
        child_void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        child_void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        child.blocks.insert(BasicBlockId::new(1), child_text_block);
        child.blocks.insert(BasicBlockId::new(2), child_void_block);

        let mut parent = MirFunction::new(
            FunctionSignature {
                name: "Helper.parent/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        parent
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .instructions
            .push(MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::Bool(true),
            });
        parent
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Branch {
                condition: ValueId::new(1),
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            });
        let mut parent_text_block = BasicBlock::new(BasicBlockId::new(1));
        parent_text_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        parent_text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut parent_void_block = BasicBlock::new(BasicBlockId::new(2));
        parent_void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        parent_void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        parent
            .blocks
            .insert(BasicBlockId::new(1), parent_text_block);
        parent
            .blocks
            .insert(BasicBlockId::new(2), parent_void_block);
        parent
            .metadata
            .value_types
            .insert(ValueId::new(2), MirType::Void);
        module.functions.insert("main".to_string(), caller);
        module.functions.insert("Helper.child/0".to_string(), child);
        module
            .functions
            .insert("Helper.parent/0".to_string(), parent);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(
            route.target_shape(),
            Some("generic_string_or_void_sentinel_body"),
            "reason={:?} blocker={:?}/{:?}",
            route.target_shape_reason(),
            route.target_shape_blocker_symbol(),
            route.target_shape_blocker_reason()
        );
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    }

    #[test]
    fn refresh_module_global_call_routes_propagates_return_child_blocker_transitively() {
        let mut module =
            MirModule::new("global_call_void_sentinel_transitive_child_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.maybe_text/0",
            Some(ValueId::new(7)),
            vec![],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.maybe_text/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.wrapper/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        let mut wrapper = MirFunction::new(
            FunctionSignature {
                name: "Helper.wrapper/0".to_string(),
                params: vec![],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        wrapper_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.map/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        wrapper_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        let map = MirFunction::new(
            FunctionSignature {
                name: "Helper.map/0".to_string(),
                params: vec![],
                return_type: MirType::Box("MapBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.wrapper/0".to_string(), wrapper);
        module.functions.insert("Helper.map/0".to_string(), map);
        module
            .functions
            .insert("Helper.maybe_text/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_global_target_shape_unknown")
        );
        assert_eq!(route.target_shape_blocker_symbol(), Some("Helper.map/0"));
        assert_eq!(
            route.target_shape_blocker_reason(),
            Some("generic_string_return_object_abi_not_handle_compatible")
        );
    }

    #[test]
    fn refresh_module_global_call_routes_marks_void_sentinel_const_reason() {
        let mut module = MirModule::new("global_call_void_const_reason_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.flag/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.flag/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1)];
        let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        module.functions.insert("main".to_string(), caller);
        module.functions.insert("Helper.flag/1".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_unsupported_void_sentinel_const")
        );
        assert_eq!(route.target_shape_blocker_symbol(), None);
        assert_eq!(route.target_shape_blocker_reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_marks_object_return_abi_reason() {
        let mut module = MirModule::new("global_call_object_return_reason_test".to_string());
        let caller =
            make_function_with_global_call_args("Helper.map/0", Some(ValueId::new(7)), vec![]);
        let callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.map/0".to_string(),
                params: vec![],
                return_type: MirType::Box("MapBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        module.functions.insert("main".to_string(), caller);
        module.functions.insert("Helper.map/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_return_object_abi_not_handle_compatible")
        );
        assert_eq!(route.target_shape_blocker_symbol(), None);
        assert_eq!(route.target_shape_blocker_reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_allows_null_guard_before_method_blocker() {
        let mut module = MirModule::new("global_call_null_guard_method_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.preview/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.preview/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1)];
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Void,
            },
            MirInstruction::Compare {
                dst: ValueId::new(3),
                op: CompareOp::Eq,
                lhs: ValueId::new(1),
                rhs: ValueId::new(2),
            },
        ]);
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(3),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut null_block = BasicBlock::new(BasicBlockId::new(1));
        null_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("<null>".to_string()),
        });
        null_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        let mut method_block = BasicBlock::new(BasicBlockId::new(2));
        method_block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "debugPreview".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        method_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(5)),
        });
        callee.blocks.insert(BasicBlockId::new(1), null_block);
        callee.blocks.insert(BasicBlockId::new(2), method_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.preview/1".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_unsupported_method_call")
        );
        assert_eq!(route.target_shape_blocker_symbol(), None);
        assert_eq!(route.target_shape_blocker_reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_accepts_runtime_data_string_length_method() {
        let mut module = MirModule::new("global_call_string_len_method_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.debug_len/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut coerce = MirFunction::new(
            FunctionSignature {
                name: "Helper.coerce/1".to_string(),
                params: vec![MirType::Integer],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        coerce.params = vec![ValueId::new(1)];
        let coerce_block = coerce.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        coerce_block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::String(String::new()),
            },
            MirInstruction::BinOp {
                dst: ValueId::new(3),
                op: BinaryOp::Add,
                lhs: ValueId::new(2),
                rhs: ValueId::new(1),
            },
        ]);
        coerce_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });

        let mut debug_len = MirFunction::new(
            FunctionSignature {
                name: "Helper.debug_len/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        debug_len.params = vec![ValueId::new(1)];
        let block = debug_len.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.extend([
            MirInstruction::Call {
                dst: Some(ValueId::new(2)),
                func: ValueId::INVALID,
                callee: Some(Callee::Global("Helper.coerce/1".to_string())),
                args: vec![ValueId::new(1)],
                effects: EffectMask::PURE,
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(3)),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: "length".to_string(),
                    receiver: Some(ValueId::new(2)),
                    certainty: TypeCertainty::Union,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: vec![],
                effects: EffectMask::PURE,
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(4)),
                func: ValueId::INVALID,
                callee: Some(Callee::Global("Helper.coerce/1".to_string())),
                args: vec![ValueId::new(3)],
                effects: EffectMask::PURE,
            },
        ]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.coerce/1".to_string(), coerce);
        module
            .functions
            .insert("Helper.debug_len/1".to_string(), debug_len);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    }

    #[test]
    fn refresh_module_global_call_routes_accepts_runtime_data_string_substring_method() {
        let mut module = MirModule::new("global_call_string_substring_method_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.debug_preview/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.debug_preview/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1)];
        let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(0),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(64),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(4)),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: "substring".to_string(),
                    receiver: Some(ValueId::new(1)),
                    certainty: TypeCertainty::Union,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: vec![ValueId::new(2), ValueId::new(3)],
                effects: EffectMask::PURE,
            },
        ]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.debug_preview/1".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    }

    #[test]
    fn refresh_module_global_call_routes_accepts_print_in_generic_pure_string_body() {
        let mut module = MirModule::new("global_call_string_print_method_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.debug_print/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.debug_print/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::String,
                effects: EffectMask::IO,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1)];
        let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::String("[debug] ".to_string()),
            },
            MirInstruction::BinOp {
                dst: ValueId::new(3),
                op: BinaryOp::Add,
                lhs: ValueId::new(2),
                rhs: ValueId::new(1),
            },
            MirInstruction::Call {
                dst: None,
                func: ValueId::INVALID,
                callee: Some(Callee::Global("print".to_string())),
                args: vec![ValueId::new(3)],
                effects: EffectMask::IO,
            },
        ]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.debug_print/1".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    }

    #[test]
    fn refresh_module_global_call_routes_marks_generic_i64_body_direct_target() {
        let mut module = MirModule::new("global_call_generic_i64_test".to_string());
        let caller =
            make_function_with_global_call_args("Helper.debug/0", Some(ValueId::new(7)), vec![]);
        let mut wrapper = MirFunction::new(
            FunctionSignature {
                name: "Helper.debug/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        wrapper_block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("DEBUG".to_string()),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(2)),
                func: ValueId::INVALID,
                callee: Some(Callee::Global("Helper.flag/1".to_string())),
                args: vec![ValueId::new(1)],
                effects: EffectMask::PURE,
            },
        ]);
        wrapper_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });

        let mut flag = MirFunction::new(
            FunctionSignature {
                name: "Helper.flag/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        flag.params = vec![ValueId::new(1)];
        let entry = flag.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend([
            MirInstruction::Call {
                dst: Some(ValueId::new(2)),
                func: ValueId::INVALID,
                callee: Some(Callee::Extern("env.get/1".to_string())),
                args: vec![ValueId::new(1)],
                effects: EffectMask::PURE,
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Void,
            },
            MirInstruction::Compare {
                dst: ValueId::new(4),
                op: CompareOp::Ne,
                lhs: ValueId::new(2),
                rhs: ValueId::new(3),
            },
        ]);
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(4),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
        yes_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Integer(1),
        });
        yes_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(5)),
        });
        let mut no_block = BasicBlock::new(BasicBlockId::new(2));
        no_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(0),
        });
        no_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(6)),
        });
        flag.blocks.insert(BasicBlockId::new(1), yes_block);
        flag.blocks.insert(BasicBlockId::new(2), no_block);

        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.debug/0".to_string(), wrapper);
        module.functions.insert("Helper.flag/1".to_string(), flag);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), Some("generic_i64_body"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.tier(), "DirectAbi");
        assert_eq!(route.proof(), "typed_global_call_generic_i64");
        assert_eq!(route.return_shape(), Some("ScalarI64"));
        assert_eq!(route.value_demand(), "scalar_i64");

        let wrapper_route = &module.functions["Helper.debug/0"]
            .metadata
            .global_call_routes[0];
        assert_eq!(wrapper_route.target_shape(), Some("generic_i64_body"));
        assert_eq!(wrapper_route.proof(), "typed_global_call_generic_i64");
    }

    #[test]
    fn refresh_module_global_call_routes_marks_method_call_shape_reason() {
        let mut module = MirModule::new("global_call_method_reason_test".to_string());
        let caller = make_function_with_global_call_args(
            "Helper.slice/1",
            Some(ValueId::new(7)),
            vec![ValueId::new(1)],
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.slice/1".to_string(),
                params: vec![MirType::String],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1)];
        let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(0),
            },
            MirInstruction::Const {
                dst: ValueId::new(3),
                value: ConstValue::Integer(1),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(4)),
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: "RuntimeDataBox".to_string(),
                    method: "debugPreview".to_string(),
                    receiver: Some(ValueId::new(1)),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: vec![ValueId::new(2), ValueId::new(3)],
                effects: EffectMask::PURE,
            },
        ]);
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.slice/1".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_unsupported_method_call")
        );
        assert_eq!(route.target_shape_blocker_symbol(), None);
        assert_eq!(route.target_shape_blocker_reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_marks_unknown_child_target_shape_reason() {
        let mut module = MirModule::new("global_call_child_reason_test".to_string());
        let caller =
            make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(7)), vec![]);
        let mut wrapper = MirFunction::new(
            FunctionSignature {
                name: "Helper.wrapper/0".to_string(),
                params: vec![],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.pending/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        let pending = MirFunction::new(
            FunctionSignature {
                name: "Helper.pending/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.wrapper/0".to_string(), wrapper);
        module
            .functions
            .insert("Helper.pending/0".to_string(), pending);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_global_target_shape_unknown")
        );
        assert_eq!(
            route.target_shape_blocker_symbol(),
            Some("Helper.pending/0")
        );
        assert_eq!(
            route.target_shape_blocker_reason(),
            Some("generic_string_no_string_surface")
        );
    }

    #[test]
    fn refresh_module_global_call_routes_marks_numeric_i64_leaf_direct_target() {
        let mut module = MirModule::new("global_call_leaf_test".to_string());
        let caller = make_function_with_global_call("Helper.add/2", Some(ValueId::new(7)));
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.add/2".to_string(),
                params: vec![MirType::Integer, MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1), ValueId::new(2)];
        let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        module.functions.insert("main".to_string(), caller);
        module.functions.insert("Helper.add/2".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert!(route.target_exists());
        assert_eq!(route.target_symbol(), Some("Helper.add/2"));
        assert_eq!(route.target_return_type(), Some("i64".to_string()));
        assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.target_arity(), Some(2));
        assert_eq!(route.arity_matches(), Some(true));
        assert_eq!(route.tier(), "DirectAbi");
        assert_eq!(route.emit_kind(), "direct_function_call");
        assert_eq!(route.proof(), "typed_global_call_leaf_numeric_i64");
        assert_eq!(route.return_shape(), Some("ScalarI64"));
        assert_eq!(route.value_demand(), "scalar_i64");
        assert_eq!(route.reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_resolves_static_entry_alias_to_target_symbol() {
        let mut module = MirModule::new("global_call_static_entry_alias_test".to_string());
        let caller =
            make_function_with_global_call_args("main._helper/0", Some(ValueId::new(7)), vec![]);
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Main._helper/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(42),
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Main._helper/0".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(route.callee_name(), "main._helper/0");
        assert!(route.target_exists());
        assert_eq!(route.target_symbol(), Some("Main._helper/0"));
        assert_eq!(route.target_arity(), Some(0));
        assert_eq!(route.target_return_type(), Some("i64".to_string()));
        assert_eq!(route.arity_matches(), Some(true));
        assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.tier(), "DirectAbi");
        assert_eq!(route.reason(), None);
    }

    #[test]
    fn refresh_module_global_call_routes_marks_generic_pure_string_body_direct_target() {
        let mut module = MirModule::new("global_call_generic_string_test".to_string());
        let caller = make_function_with_global_call("Helper.normalize/2", Some(ValueId::new(7)));
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: "Helper.normalize/2".to_string(),
                params: vec![MirType::String, MirType::String],
                return_type: MirType::String,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = vec![ValueId::new(1), ValueId::new(9)];
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::String("dev".to_string()),
            },
            MirInstruction::Compare {
                dst: ValueId::new(3),
                op: CompareOp::Eq,
                lhs: ValueId::new(1),
                rhs: ValueId::new(2),
            },
        ]);
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(3),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });

        let mut then_block = BasicBlock::new(BasicBlockId::new(1));
        then_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("vm".to_string()),
        });
        then_block.set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });

        let mut else_block = BasicBlock::new(BasicBlockId::new(2));
        else_block.instructions.push(MirInstruction::Copy {
            dst: ValueId::new(5),
            src: ValueId::new(1),
        });
        else_block.set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });

        let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
        merge_block.instructions.push(MirInstruction::Phi {
            dst: ValueId::new(6),
            inputs: vec![
                (BasicBlockId::new(1), ValueId::new(4)),
                (BasicBlockId::new(2), ValueId::new(5)),
            ],
            type_hint: Some(MirType::String),
        });
        merge_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(6)),
        });

        callee.blocks.insert(BasicBlockId::new(1), then_block);
        callee.blocks.insert(BasicBlockId::new(2), else_block);
        callee.blocks.insert(BasicBlockId::new(3), merge_block);
        module.functions.insert("main".to_string(), caller);
        module
            .functions
            .insert("Helper.normalize/2".to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert!(route.target_exists());
        assert_eq!(route.target_symbol(), Some("Helper.normalize/2"));
        assert_eq!(route.target_return_type(), Some("str".to_string()));
        assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(route.target_arity(), Some(2));
        assert_eq!(route.arity_matches(), Some(true));
        assert_eq!(route.tier(), "DirectAbi");
        assert_eq!(route.emit_kind(), "direct_function_call");
        assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
        assert_eq!(route.return_shape(), Some("string_handle"));
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.reason(), None);
    }
}
