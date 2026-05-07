/*!
 * MIR-owned route plans for typed user-box method calls.
 *
 * The first accepted surface is user-box `birth` as a normal same-module
 * method function. Backends consume this route instead of reclassifying raw
 * `Box.method` names or cloning VM InstanceBox behavior.
 */

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};
use crate::mir::definitions::call_unified::TypeCertainty;
mod body_shape;

use body_shape::user_box_method_body_supported;

use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, MirModule, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserBoxMethodRouteSite {
    block: BasicBlockId,
    instruction_index: usize,
}

impl UserBoxMethodRouteSite {
    pub fn new(block: BasicBlockId, instruction_index: usize) -> Self {
        Self {
            block,
            instruction_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBoxMethodRoute {
    site: UserBoxMethodRouteSite,
    box_name: String,
    method: String,
    receiver_value: ValueId,
    arity: usize,
    result_value: Option<ValueId>,
    target_symbol: String,
    target_exists: bool,
    target_arity: Option<usize>,
    target_return_type: Option<MirType>,
    target_body_supported: bool,
    type_id: Option<u32>,
}

impl UserBoxMethodRoute {
    pub fn block(&self) -> BasicBlockId {
        self.site.block
    }

    pub fn instruction_index(&self) -> usize {
        self.site.instruction_index
    }

    pub fn route_id(&self) -> &'static str {
        "user_box.method_call"
    }

    pub fn core_op(&self) -> &'static str {
        "UserBoxMethodCall"
    }

    pub fn route_kind(&self) -> &'static str {
        if self.method == "birth" {
            "user_box.birth"
        } else {
            "user_box.method"
        }
    }

    pub fn lowering_tier(&self) -> LoweringPlanTier {
        if self.is_direct_abi_target() {
            LoweringPlanTier::DirectAbi
        } else {
            LoweringPlanTier::Unsupported
        }
    }

    pub fn lowering_emit_kind(&self) -> LoweringPlanEmitKind {
        if self.is_direct_abi_target() {
            LoweringPlanEmitKind::DirectFunctionCall
        } else {
            LoweringPlanEmitKind::Unsupported
        }
    }

    pub fn proof(&self) -> &'static str {
        if self.is_direct_birth_target() {
            "typed_user_box_birth_same_module"
        } else if self.is_direct_instance_method_target() {
            "typed_user_box_method_same_module"
        } else {
            "typed_user_box_method_contract_missing"
        }
    }

    pub fn box_name(&self) -> &str {
        &self.box_name
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn receiver_value(&self) -> ValueId {
        self.receiver_value
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn result_value(&self) -> Option<ValueId> {
        self.result_value
    }

    pub fn target_symbol(&self) -> &str {
        &self.target_symbol
    }

    pub fn target_exists(&self) -> bool {
        self.target_exists
    }

    pub fn target_arity(&self) -> Option<usize> {
        self.target_arity
    }

    pub fn target_return_type(&self) -> Option<String> {
        self.target_return_type
            .as_ref()
            .map(format_user_box_method_type_label)
    }

    pub fn target_body_supported(&self) -> bool {
        self.target_body_supported
    }

    pub fn arity_matches(&self) -> Option<bool> {
        self.target_arity
            .map(|target_arity| target_arity == self.arity + 1)
    }

    pub fn type_id(&self) -> Option<u32> {
        self.type_id
    }

    pub fn return_shape(&self) -> Option<&'static str> {
        if self.is_direct_birth_target() {
            Some("void_sentinel_i64_zero")
        } else if self.is_direct_instance_method_target() {
            Some("scalar_i64")
        } else {
            None
        }
    }

    pub fn value_demand(&self) -> &'static str {
        if self.is_direct_abi_target() {
            "scalar_i64"
        } else {
            "typed_user_box_method_contract_missing"
        }
    }

    pub fn result_origin(&self) -> &'static str {
        "none"
    }

    pub fn definition_owner(&self) -> &'static str {
        if self.is_direct_abi_target() {
            "typed_object_method"
        } else {
            "none"
        }
    }

    pub fn emit_trace_consumer(&self) -> &'static str {
        if self.is_direct_birth_target() {
            "mir_call_user_box_birth_same_module_emit"
        } else if self.is_direct_instance_method_target() {
            "mir_call_user_box_method_same_module_emit"
        } else {
            "mir_call_user_box_method_unknown_emit"
        }
    }

    pub fn reason(&self) -> Option<&'static str> {
        if self.is_direct_abi_target() {
            return None;
        }
        if self.type_id.is_none() {
            return Some("typed_object_plan_missing");
        }
        if !self.target_exists {
            return Some("user_box_method_target_missing");
        }
        if self.arity_matches() == Some(false) {
            return Some("user_box_method_arity_mismatch");
        }
        if !self.target_body_supported {
            if self.method == "birth" {
                return Some("user_box_birth_body_unsupported");
            }
            return Some("user_box_method_body_unsupported");
        }
        if !self.return_type_supported() {
            return Some("user_box_method_return_type_unsupported");
        }
        Some("user_box_method_contract_missing")
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        if self.method == "birth" {
            &["call.user_box.birth"]
        } else {
            &["call.user_box.method"]
        }
    }

    fn is_direct_abi_target(&self) -> bool {
        self.is_direct_birth_target() || self.is_direct_instance_method_target()
    }

    fn has_direct_target_contract(&self) -> bool {
        self.type_id.is_some()
            && self.target_exists
            && self.arity_matches() == Some(true)
            && self.target_body_supported
    }

    fn is_direct_birth_target(&self) -> bool {
        self.has_direct_target_contract()
            && self.method == "birth"
            && matches!(self.target_return_type, Some(MirType::Void))
    }

    fn is_direct_instance_method_target(&self) -> bool {
        self.has_direct_target_contract() && self.method != "birth" && self.return_type_supported()
    }

    fn return_type_supported(&self) -> bool {
        matches!(
            self.target_return_type,
            Some(MirType::Integer | MirType::Bool)
        )
    }
}

pub fn refresh_module_user_box_method_routes(module: &mut MirModule) {
    let typed_plan_type_ids = module
        .metadata
        .typed_object_plans
        .iter()
        .map(|plan| (plan.box_name.clone(), plan.type_id))
        .collect::<BTreeMap<_, _>>();
    for _ in 0..module.functions.len().max(1) {
        let before = module
            .functions
            .iter()
            .map(|(name, function)| (name.clone(), function.metadata.user_box_method_routes.clone()))
            .collect::<BTreeMap<_, _>>();
        let targets = collect_method_targets(module, &typed_plan_type_ids);
        for function in module.functions.values_mut() {
            refresh_function_user_box_method_routes_with_context(
                function,
                &targets,
                &typed_plan_type_ids,
            );
        }
        let changed = module.functions.iter().any(|(name, function)| {
            before
                .get(name)
                .map_or(true, |routes| {
                    routes != &function.metadata.user_box_method_routes
                })
        });
        if !changed {
            break;
        }
    }
}

pub fn refresh_function_user_box_method_routes(function: &mut MirFunction) {
    refresh_function_user_box_method_routes_with_context(
        function,
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
}

fn refresh_function_user_box_method_routes_with_context(
    function: &mut MirFunction,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) {
    let mut routes = Vec::new();
    let user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst,
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        certainty,
                        box_kind: _,
                    }),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            if *certainty != TypeCertainty::Known {
                continue;
            }
            if !user_box_names.contains(box_name) {
                continue;
            }
            let target_symbol = method_target_symbol(box_name, method, args.len());
            let target = targets.get(&target_symbol);
            routes.push(UserBoxMethodRoute {
                site: UserBoxMethodRouteSite::new(block_id, instruction_index),
                box_name: box_name.clone(),
                method: method.clone(),
                receiver_value: *receiver,
                arity: args.len(),
                result_value: *dst,
                target_symbol,
                target_exists: target.is_some(),
                target_arity: target.map(|target| target.arity),
                target_return_type: target.map(|target| target.return_type.clone()),
                target_body_supported: target.map(|target| target.body_supported).unwrap_or(false),
                type_id: typed_plan_type_ids.get(box_name).copied(),
            });
        }
    }

    function.metadata.user_box_method_routes = routes;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserBoxMethodTargetFacts {
    box_name: String,
    arity: usize,
    return_type: MirType,
    body_supported: bool,
}

fn collect_method_targets(
    module: &MirModule,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> BTreeMap<String, UserBoxMethodTargetFacts> {
    module
        .functions
        .iter()
        .filter_map(|(name, function)| {
            let (box_name, _method, arity) = parse_method_symbol(name)?;
            if !module.metadata.user_box_decls.contains_key(box_name)
                && !module.metadata.user_box_field_decls.contains_key(box_name)
            {
                return None;
            }
            let target_arity = if function.params.is_empty() {
                function.signature.params.len()
            } else {
                function.params.len()
            };
            // Method symbols encode only explicit arguments; receiver is an
            // extra uniform ABI parameter at index 0.
            if target_arity != arity + 1 {
                return None;
            }
            Some((
                name.clone(),
                UserBoxMethodTargetFacts {
                    box_name: box_name.to_string(),
                    arity: target_arity,
                    return_type: function.signature.return_type.clone(),
                    body_supported: user_box_method_body_supported(function, typed_plan_type_ids),
                },
            ))
        })
        .collect()
}

fn parse_method_symbol(name: &str) -> Option<(&str, &str, usize)> {
    let (owner_and_method, arity_s) = name.rsplit_once('/')?;
    let (box_name, method) = owner_and_method.rsplit_once('.')?;
    let arity = arity_s.parse::<usize>().ok()?;
    Some((box_name, method, arity))
}

fn method_target_symbol(box_name: &str, method: &str, arity: usize) -> String {
    format!("{box_name}.{method}/{arity}")
}

fn format_user_box_method_type_label(ty: &MirType) -> String {
    match ty {
        MirType::Integer => "i64".to_string(),
        MirType::Bool => "i1".to_string(),
        MirType::Float => "f64".to_string(),
        MirType::String => "StringBox".to_string(),
        MirType::Void => "void".to_string(),
        MirType::Box(name) => name.clone(),
        MirType::Array(_) => "ArrayBox".to_string(),
        MirType::Future(_) => "FutureBox".to_string(),
        MirType::WeakRef => "WeakRef".to_string(),
        MirType::Unknown => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::TypedObjectPlan;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirInstruction};

    #[test]
    fn refresh_module_user_box_method_routes_accepts_birth_same_module_target() {
        let mut module = MirModule::new("user_box_birth_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Pair".to_string(), vec!["left".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Pair".to_string(),
            type_id: 7,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Pair.birth/0".to_string(),
                params: vec![MirType::Box("Pair".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Void,
        });
        birth_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        birth.add_block(birth_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Pair".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        main.add_block(block);

        module.add_function(birth);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main function");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.route_id(), "user_box.method_call");
        assert_eq!(route.proof(), "typed_user_box_birth_same_module");
        assert_eq!(route.target_symbol(), "Pair.birth/0");
        assert_eq!(route.target_arity(), Some(1));
        assert_eq!(route.arity_matches(), Some(true));
        assert!(route.target_body_supported());
        assert_eq!(route.type_id(), Some(7));
        assert_eq!(route.definition_owner(), "typed_object_method");
    }

    #[test]
    fn refresh_module_user_box_method_routes_rejects_unsupported_birth_body() {
        let mut module = MirModule::new("user_box_birth_route_reject_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Pair".to_string(), vec!["left".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Pair".to_string(),
            type_id: 7,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Pair.birth/0".to_string(),
                params: vec![MirType::Box("Pair".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("helper".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
        birth_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        });
        birth_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        birth.add_block(birth_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Pair".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        main.add_block(block);

        module.add_function(birth);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main function");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.proof(), "typed_user_box_method_contract_missing");
        assert!(!route.target_body_supported());
        assert_eq!(route.reason(), Some("user_box_birth_body_unsupported"));
        assert_eq!(route.definition_owner(), "none");
    }

    #[test]
    fn refresh_module_user_box_method_routes_accepts_scalar_instance_method_target() {
        let mut module = MirModule::new("user_box_method_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Pair".to_string(), vec!["left".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Pair".to_string(),
            type_id: 7,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut sum = MirFunction::new(
            FunctionSignature {
                name: "Pair.sum/0".to_string(),
                params: vec![MirType::Box("Pair".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        sum.params = vec![ValueId::new(0)];
        let mut sum_block = BasicBlock::new(BasicBlockId::new(0));
        sum_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(30),
        });
        sum_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        sum.add_block(sum_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Pair".to_string(),
                method: "sum".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        main.add_block(block);

        module.add_function(sum);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main function");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.route_id(), "user_box.method_call");
        assert_eq!(route.route_kind(), "user_box.method");
        assert_eq!(route.proof(), "typed_user_box_method_same_module");
        assert_eq!(route.target_symbol(), "Pair.sum/0");
        assert_eq!(route.target_arity(), Some(1));
        assert_eq!(route.arity_matches(), Some(true));
        assert!(route.target_body_supported());
        assert_eq!(route.return_shape(), Some("scalar_i64"));
        assert_eq!(route.type_id(), Some(7));
        assert_eq!(route.definition_owner(), "typed_object_method");
    }
}
