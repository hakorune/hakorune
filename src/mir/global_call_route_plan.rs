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
}

impl GlobalCallTargetShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NumericI64Leaf => "numeric_i64_leaf",
            Self::GenericPureStringBody => "generic_pure_string_body",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GlobalCallTargetShapeReason {
    ParamBindingMismatch,
    GenericStringReturnAbiNotHandleCompatible,
    GenericStringParamAbiNotHandleCompatible,
    GenericStringUnsupportedInstructionOrCall,
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
            Self::GenericStringParamAbiNotHandleCompatible => {
                "generic_string_param_abi_not_handle_compatible"
            }
            Self::GenericStringUnsupportedInstructionOrCall => {
                "generic_string_unsupported_instruction_or_call"
            }
            Self::GenericStringNoStringSurface => "generic_string_no_string_surface",
            Self::GenericStringReturnNotString => "generic_string_return_not_string",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GlobalCallTargetClassification {
    shape: GlobalCallTargetShape,
    reason: Option<GlobalCallTargetShapeReason>,
}

impl GlobalCallTargetClassification {
    fn direct(shape: GlobalCallTargetShape) -> Self {
        Self {
            shape,
            reason: None,
        }
    }

    fn unknown(reason: GlobalCallTargetShapeReason) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
            reason: Some(reason),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GlobalCallTargetFacts {
    exists: bool,
    symbol: Option<String>,
    arity: Option<usize>,
    shape: GlobalCallTargetShape,
    shape_reason: Option<GlobalCallTargetShapeReason>,
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
            shape: GlobalCallTargetShape::Unknown,
            shape_reason: None,
        }
    }

    pub fn present_with_symbol(symbol: impl Into<String>, arity: usize) -> Self {
        Self {
            exists: true,
            symbol: Some(symbol.into()),
            arity: Some(arity),
            shape: GlobalCallTargetShape::Unknown,
            shape_reason: None,
        }
    }

    pub fn present_with_shape(arity: usize, shape: GlobalCallTargetShape) -> Self {
        Self {
            exists: true,
            symbol: None,
            arity: Some(arity),
            shape,
            shape_reason: None,
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

    pub fn shape(&self) -> GlobalCallTargetShape {
        self.shape
    }

    fn shape_reason(&self) -> Option<GlobalCallTargetShapeReason> {
        self.shape_reason
    }

    fn with_classification(mut self, classification: GlobalCallTargetClassification) -> Self {
        self.shape = classification.shape;
        self.shape_reason = classification.reason;
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

    pub fn arity_matches(&self) -> Option<bool> {
        self.target_arity()
            .map(|target_arity| target_arity == self.arity)
    }

    pub fn value_demand(&self) -> &'static str {
        match self.direct_target_shape() {
            Some(GlobalCallTargetShape::NumericI64Leaf) => "scalar_i64",
            Some(GlobalCallTargetShape::GenericPureStringBody) => "runtime_i64_or_handle",
            _ => "typed_global_call_contract_missing",
        }
    }

    pub fn return_shape(&self) -> Option<&'static str> {
        match self.direct_target_shape() {
            Some(GlobalCallTargetShape::NumericI64Leaf) => Some("ScalarI64"),
            Some(GlobalCallTargetShape::GenericPureStringBody) => Some("string_handle"),
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
            | GlobalCallTargetShape::GenericPureStringBody => Some(self.target.shape()),
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
                GlobalCallTargetFacts::present_with_symbol(name.clone(), arity),
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
    if let Some(reason) = generic_pure_string_body_reject_reason(function, targets) {
        GlobalCallTargetClassification::unknown(reason)
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
}

fn generic_pure_string_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GlobalCallTargetShapeReason> {
    if !generic_pure_string_abi_type_is_handle_compatible(&function.signature.return_type) {
        return Some(GlobalCallTargetShapeReason::GenericStringReturnAbiNotHandleCompatible);
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return Some(GlobalCallTargetShapeReason::GenericStringParamAbiNotHandleCompatible);
    }
    if function.params.len() != function.signature.params.len() {
        return Some(GlobalCallTargetShapeReason::ParamBindingMismatch);
    }

    let mut values = BTreeMap::<ValueId, GenericPureValueClass>::new();
    let mut has_string_surface = false;
    for param in &function.params {
        values.insert(*param, GenericPureValueClass::String);
    }
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in &block.instructions {
                if !mark_generic_pure_string_instruction(
                    instruction,
                    targets,
                    &mut values,
                    &mut has_string_surface,
                    &mut changed,
                ) {
                    return Some(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstructionOrCall,
                    );
                }
            }
            if let Some(terminator) = &block.terminator {
                if !mark_generic_pure_string_instruction(
                    terminator,
                    targets,
                    &mut values,
                    &mut has_string_surface,
                    &mut changed,
                ) {
                    return Some(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstructionOrCall,
                    );
                }
            }
        }
        if !changed {
            break;
        }
    }

    if !has_string_surface {
        return Some(GlobalCallTargetShapeReason::GenericStringNoStringSurface);
    }

    let mut saw_return = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Return { value: Some(value) } = instruction {
                saw_return = true;
                if value_class(&values, *value) != GenericPureValueClass::String {
                    return Some(GlobalCallTargetShapeReason::GenericStringReturnNotString);
                }
            } else if matches!(instruction, MirInstruction::Return { value: None }) {
                return Some(GlobalCallTargetShapeReason::GenericStringReturnNotString);
            }
        }
    }
    if saw_return {
        None
    } else {
        Some(GlobalCallTargetShapeReason::GenericStringReturnNotString)
    }
}

fn generic_pure_string_abi_type_is_handle_compatible(ty: &MirType) -> bool {
    match ty {
        MirType::Integer | MirType::String | MirType::Unknown => true,
        MirType::Box(name) => name == "StringBox",
        _ => false,
    }
}

fn mark_generic_pure_string_instruction(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    has_string_surface: &mut bool,
    changed: &mut bool,
) -> bool {
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::String(_) => {
                    *has_string_surface = true;
                    GenericPureValueClass::String
                }
                ConstValue::Integer(_) => GenericPureValueClass::I64,
                ConstValue::Bool(_) => GenericPureValueClass::Bool,
                _ => GenericPureValueClass::Unknown,
            };
            set_value_class(values, *dst, class, changed);
            class != GenericPureValueClass::Unknown
        }
        MirInstruction::Copy { dst, src } => {
            let class = value_class(values, *src);
            if class != GenericPureValueClass::Unknown {
                set_value_class(values, *dst, class, changed);
            }
            true
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
                return false;
            }
            let lhs_class = value_class(values, *lhs);
            let rhs_class = value_class(values, *rhs);
            if lhs_class == GenericPureValueClass::Unknown
                || rhs_class == GenericPureValueClass::Unknown
            {
                return true;
            }
            let class = if *op == BinaryOp::Add {
                if lhs_class == GenericPureValueClass::String
                    || rhs_class == GenericPureValueClass::String
                {
                    *has_string_surface = true;
                    GenericPureValueClass::String
                } else {
                    GenericPureValueClass::I64
                }
            } else if lhs_class == GenericPureValueClass::String
                || rhs_class == GenericPureValueClass::String
            {
                return false;
            } else {
                GenericPureValueClass::I64
            };
            set_value_class(values, *dst, class, changed);
            true
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = value_class(values, *lhs);
            let rhs_class = value_class(values, *rhs);
            if lhs_class == GenericPureValueClass::Unknown
                || rhs_class == GenericPureValueClass::Unknown
            {
                return true;
            }
            if lhs_class == GenericPureValueClass::String
                || rhs_class == GenericPureValueClass::String
            {
                if !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne) {
                    return false;
                }
                *has_string_surface = true;
            }
            set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
            true
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
            }
            if saw_unknown {
                return true;
            } else if all_string {
                set_value_class(values, *dst, GenericPureValueClass::String, changed);
            } else if saw_string {
                return false;
            } else {
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
            }
            true
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
            true
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            ..
        } if !supported_backend_global(name) => {
            let Some(target) = lookup_global_call_target(name, targets) else {
                return false;
            };
            match target.shape() {
                GlobalCallTargetShape::GenericPureStringBody => {
                    if let Some(dst) = dst {
                        *has_string_surface = true;
                        set_value_class(values, *dst, GenericPureValueClass::String, changed);
                    }
                    true
                }
                GlobalCallTargetShape::NumericI64Leaf => {
                    if let Some(dst) = dst {
                        set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                    }
                    true
                }
                GlobalCallTargetShape::Unknown => false,
            }
        }
        MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::ReleaseStrong { .. } => true,
        _ => false,
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
        assert_eq!(route.arity_matches(), Some(true));
        assert_eq!(route.target_shape(), None);
        assert_eq!(
            route.target_shape_reason(),
            Some("generic_string_no_string_surface")
        );
        assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
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
