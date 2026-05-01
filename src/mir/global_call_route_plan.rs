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
}

impl GlobalCallTargetShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NumericI64Leaf => "numeric_i64_leaf",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GlobalCallTargetFacts {
    exists: bool,
    arity: Option<usize>,
    shape: GlobalCallTargetShape,
}

impl GlobalCallTargetFacts {
    pub fn missing() -> Self {
        Self::default()
    }

    pub fn present(arity: usize) -> Self {
        Self {
            exists: true,
            arity: Some(arity),
            shape: GlobalCallTargetShape::Unknown,
        }
    }

    pub fn present_with_shape(arity: usize, shape: GlobalCallTargetShape) -> Self {
        Self {
            exists: true,
            arity: Some(arity),
            shape,
        }
    }

    pub fn exists(&self) -> bool {
        self.exists
    }

    pub fn arity(&self) -> Option<usize> {
        self.arity
    }

    pub fn shape(&self) -> GlobalCallTargetShape {
        self.shape
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
        if self.is_numeric_i64_leaf_direct_target() {
            "DirectAbi"
        } else {
            "Unsupported"
        }
    }

    pub fn emit_kind(&self) -> &'static str {
        if self.is_numeric_i64_leaf_direct_target() {
            "direct_function_call"
        } else {
            "unsupported"
        }
    }

    pub fn proof(&self) -> &'static str {
        if self.is_numeric_i64_leaf_direct_target() {
            "typed_global_call_leaf_numeric_i64"
        } else {
            "typed_global_call_contract_missing"
        }
    }

    pub fn route_kind(&self) -> &'static str {
        "global.user_call"
    }

    pub fn callee_name(&self) -> &str {
        &self.callee_name
    }

    pub fn target_symbol(&self) -> Option<&str> {
        self.target_exists().then_some(self.callee_name())
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

    pub fn arity_matches(&self) -> Option<bool> {
        self.target_arity()
            .map(|target_arity| target_arity == self.arity)
    }

    pub fn value_demand(&self) -> &'static str {
        if self.is_numeric_i64_leaf_direct_target() {
            "scalar_i64"
        } else {
            "typed_global_call_contract_missing"
        }
    }

    pub fn return_shape(&self) -> Option<&'static str> {
        self.is_numeric_i64_leaf_direct_target()
            .then_some("ScalarI64")
    }

    pub fn reason(&self) -> Option<&'static str> {
        if self.is_numeric_i64_leaf_direct_target() {
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

    fn is_numeric_i64_leaf_direct_target(&self) -> bool {
        self.target_exists()
            && self.arity_matches() == Some(true)
            && self.target.shape() == GlobalCallTargetShape::NumericI64Leaf
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
    module
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
                GlobalCallTargetFacts::present_with_shape(
                    arity,
                    classify_global_call_target_shape(function),
                ),
            )
        })
        .collect()
}

fn classify_global_call_target_shape(function: &MirFunction) -> GlobalCallTargetShape {
    if !function
        .signature
        .params
        .iter()
        .all(|ty| *ty == MirType::Integer)
        || function.signature.return_type != MirType::Integer
    {
        return GlobalCallTargetShape::Unknown;
    }
    if function.params.len() != function.signature.params.len() {
        return GlobalCallTargetShape::Unknown;
    }
    if function.blocks.len() != 1 {
        return GlobalCallTargetShape::Unknown;
    }
    let Some(block) = function.blocks.get(&function.entry_block) else {
        return GlobalCallTargetShape::Unknown;
    };
    if !matches!(
        block.terminator,
        Some(MirInstruction::Return { value: Some(_) })
    ) {
        return GlobalCallTargetShape::Unknown;
    }
    if block
        .instructions
        .iter()
        .all(is_numeric_i64_leaf_instruction)
    {
        GlobalCallTargetShape::NumericI64Leaf
    } else {
        GlobalCallTargetShape::Unknown
    }
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
                targets
                    .get(name)
                    .cloned()
                    .unwrap_or_else(GlobalCallTargetFacts::missing),
            ));
        }
    }

    function.metadata.global_call_routes = routes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

    fn make_function_with_global_call(name: &str, dst: Option<ValueId>) -> MirFunction {
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
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });
        function
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
        assert_eq!(route.target_arity(), Some(2));
        assert_eq!(route.arity_matches(), Some(true));
        assert_eq!(route.tier(), "DirectAbi");
        assert_eq!(route.emit_kind(), "direct_function_call");
        assert_eq!(route.proof(), "typed_global_call_leaf_numeric_i64");
        assert_eq!(route.return_shape(), Some("ScalarI64"));
        assert_eq!(route.value_demand(), "scalar_i64");
        assert_eq!(route.reason(), None);
    }
}
