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
use crate::mir::function::TypedObjectFieldStorage;
use crate::mir::same_module_body_shape::same_module_body_supported;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
mod return_shape;
mod value_type_publish;

use return_shape::{
    infer_user_box_method_return, UserBoxFieldReturnHints, UserBoxMethodInferredReturn,
};
use value_type_publish::{
    propagate_user_box_box_value_types, publish_generic_route_result_value_types,
    publish_user_box_field_get_value_types, publish_user_box_param_origin_value_types,
    publish_user_box_route_param_value_types, publish_user_box_route_result_value_types,
};

use crate::mir::{
    BasicBlockId, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

type ParamBoxOriginKey = (String, usize);
pub(super) type ParamBoxOriginMap = BTreeMap<ParamBoxOriginKey, BoxOriginInference>;
type FieldBoxOriginKey = (String, String);
pub(super) type FieldBoxOriginMap = BTreeMap<FieldBoxOriginKey, BoxOriginInference>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BoxOriginInference {
    Known(String),
    Conflict,
}

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
    target_inferred_return: Option<UserBoxMethodInferredReturn>,
    target_result_box_name: Option<String>,
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
        } else if self.is_direct_void_method_target()
            || self.is_direct_scalar_method_target()
            || self.is_direct_handle_method_target()
        {
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
        } else if self.is_direct_void_method_target() {
            Some("void_sentinel_i64_zero")
        } else if self.is_direct_scalar_method_target() {
            Some("scalar_i64")
        } else if self.is_direct_string_handle_method_target() {
            Some("string_handle")
        } else if self.is_direct_handle_method_target() {
            Some("object_handle")
        } else {
            None
        }
    }

    pub fn value_demand(&self) -> &'static str {
        if self.is_direct_handle_method_target() {
            "runtime_i64_or_handle"
        } else if self.is_direct_abi_target() {
            "scalar_i64"
        } else {
            "typed_user_box_method_contract_missing"
        }
    }

    pub fn result_origin(&self) -> &'static str {
        if self.is_direct_string_handle_method_target() {
            return "string";
        }
        "none"
    }

    pub fn target_result_box_name(&self) -> Option<&str> {
        if let Some(box_name) = &self.target_result_box_name {
            return Some(box_name.as_str());
        }
        if self.target_returns_string_handle() {
            return Some("StringBox");
        }
        match self.target_return_type {
            Some(MirType::Box(ref name)) => Some(name.as_str()),
            _ => None,
        }
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
        } else if self.is_direct_void_method_target()
            || self.is_direct_scalar_method_target()
            || self.is_direct_handle_method_target()
        {
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
        self.is_direct_birth_target()
            || self.is_direct_void_method_target()
            || self.is_direct_scalar_method_target()
            || self.is_direct_handle_method_target()
    }

    fn has_direct_target_contract(&self) -> bool {
        self.type_id.is_some()
            && self.target_exists
            && self.arity_matches() == Some(true)
            && self.target_body_supported
    }

    fn is_direct_birth_target(&self) -> bool {
        self.has_direct_target_contract() && self.method == "birth" && self.target_returns_void()
    }

    fn is_direct_scalar_method_target(&self) -> bool {
        self.has_direct_target_contract() && self.method != "birth" && self.target_returns_scalar()
    }

    fn is_direct_handle_method_target(&self) -> bool {
        self.has_direct_target_contract() && self.method != "birth" && self.target_returns_handle()
    }

    fn is_direct_string_handle_method_target(&self) -> bool {
        self.has_direct_target_contract()
            && self.method != "birth"
            && self.target_returns_string_handle()
    }

    fn return_type_supported(&self) -> bool {
        self.target_returns_scalar() || self.target_returns_void() || self.target_returns_handle()
    }

    fn is_direct_void_method_target(&self) -> bool {
        self.has_direct_target_contract() && self.method != "birth" && self.target_returns_void()
    }

    fn target_returns_scalar(&self) -> bool {
        match self.target_inferred_return {
            Some(UserBoxMethodInferredReturn::ScalarI64) => true,
            Some(
                UserBoxMethodInferredReturn::StringHandle
                | UserBoxMethodInferredReturn::ObjectHandle
                | UserBoxMethodInferredReturn::VoidSentinel,
            ) => false,
            None => matches!(
                self.target_return_type,
                Some(MirType::Integer | MirType::Bool)
            ),
        }
    }

    fn target_returns_void(&self) -> bool {
        if matches!(
            self.target_inferred_return,
            Some(
                UserBoxMethodInferredReturn::ScalarI64
                    | UserBoxMethodInferredReturn::StringHandle
                    | UserBoxMethodInferredReturn::ObjectHandle
            )
        ) {
            return false;
        }
        matches!(self.target_return_type, Some(MirType::Void))
            || matches!(
                self.target_inferred_return,
                Some(UserBoxMethodInferredReturn::VoidSentinel)
            )
    }

    fn target_returns_string_handle(&self) -> bool {
        matches!(self.target_return_type, Some(MirType::String))
            || matches!(self.target_return_type, Some(MirType::Box(ref name)) if name == "StringBox")
            || matches!(
                self.target_inferred_return,
                Some(UserBoxMethodInferredReturn::StringHandle)
            )
    }

    fn target_returns_handle(&self) -> bool {
        self.target_returns_string_handle()
            || matches!(self.target_return_type, Some(MirType::Box(_)))
            || matches!(
                self.target_inferred_return,
                Some(UserBoxMethodInferredReturn::ObjectHandle)
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
    for _ in 0..module.functions.len().saturating_mul(4).max(8) {
        let before = module
            .functions
            .iter()
            .map(|(name, function)| {
                (
                    name.clone(),
                    function.metadata.user_box_method_routes.clone(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let empty_field_return_hints = UserBoxFieldReturnHints::new();
        let targets =
            collect_method_targets(module, &typed_plan_type_ids, &empty_field_return_hints);
        let initial_param_box_origins =
            infer_user_box_method_param_box_origins(module, &targets, &BTreeMap::new());
        let field_box_origins =
            infer_user_box_field_box_origins(module, &targets, &initial_param_box_origins);
        let param_box_origins =
            infer_user_box_method_param_box_origins(module, &targets, &field_box_origins);
        let field_box_origins =
            infer_user_box_field_box_origins(module, &targets, &param_box_origins);
        let param_box_origins =
            infer_user_box_method_param_box_origins(module, &targets, &field_box_origins);
        let value_type_changed =
            publish_user_box_param_origin_value_types(module, &param_box_origins);
        let field_get_value_type_changed =
            publish_user_box_field_get_value_types(module, &param_box_origins, &field_box_origins);
        let field_return_hints = build_user_box_field_return_hints(module, &field_box_origins);
        let targets = collect_method_targets(module, &typed_plan_type_ids, &field_return_hints);
        for function in module.functions.values_mut() {
            refresh_function_user_box_method_routes_with_context(
                function,
                &targets,
                &typed_plan_type_ids,
                &param_box_origins,
                &field_box_origins,
            );
        }
        let route_result_value_type_changed = publish_user_box_route_result_value_types(module);
        let generic_result_value_type_changed = publish_generic_route_result_value_types(module);
        let propagated_value_type_changed = propagate_user_box_box_value_types(module);
        let route_value_type_changed = publish_user_box_route_param_value_types(
            module,
            &param_box_origins,
            &field_box_origins,
        );
        let route_changed = module.functions.iter().any(|(name, function)| {
            before.get(name).map_or(true, |routes| {
                routes != &function.metadata.user_box_method_routes
            })
        });
        let changed = value_type_changed
            || field_get_value_type_changed
            || route_result_value_type_changed
            || generic_result_value_type_changed
            || propagated_value_type_changed
            || route_value_type_changed
            || route_changed;
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
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
}

fn refresh_function_user_box_method_routes_with_context(
    function: &mut MirFunction,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) {
    let mut routes = Vec::new();
    let mut user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    user_box_names.extend(typed_plan_type_ids.keys().cloned());
    let def_map = build_value_def_map(function);
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
            let Some(route_box_name) = user_box_route_receiver_box_name(
                function,
                &def_map,
                &user_box_names,
                box_name,
                *certainty,
                *receiver,
                param_box_origins,
                field_box_origins,
            ) else {
                continue;
            };
            let target_symbol = method_target_symbol(&route_box_name, method, args.len());
            let target = targets.get(&target_symbol);
            let type_id = typed_plan_type_ids.get(&route_box_name).copied();
            routes.push(UserBoxMethodRoute {
                site: UserBoxMethodRouteSite::new(block_id, instruction_index),
                box_name: route_box_name,
                method: method.clone(),
                receiver_value: *receiver,
                arity: args.len(),
                result_value: *dst,
                target_symbol,
                target_exists: target.is_some(),
                target_arity: target.map(|target| target.arity),
                target_return_type: target.map(|target| target.return_type.clone()),
                target_inferred_return: target.and_then(|target| target.inferred_return),
                target_result_box_name: target.and_then(|target| target.result_box_name.clone()),
                target_body_supported: target.map(|target| target.body_supported).unwrap_or(false),
                type_id,
            });
        }
    }

    function.metadata.user_box_method_routes = routes;
}

fn infer_user_box_method_param_box_origins(
    module: &MirModule,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    field_box_origins: &FieldBoxOriginMap,
) -> ParamBoxOriginMap {
    let mut user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    user_box_names.extend(
        module
            .metadata
            .typed_object_plans
            .iter()
            .map(|plan| plan.box_name.clone()),
    );
    let typed_plan_fields = typed_object_plan_field_sets(module);
    let mut origins = ParamBoxOriginMap::new();

    for _ in 0..module.functions.len().max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for (param_index, box_name) in
                infer_param_box_origins_from_field_uses(function, &def_map, &typed_plan_fields)
            {
                if !user_box_names.contains(&box_name) {
                    continue;
                }
                changed |= merge_param_box_origin(
                    &mut origins,
                    (function.signature.name.clone(), param_index),
                    box_name,
                );
            }
            for block_id in sorted_block_ids(function) {
                let Some(block) = function.blocks.get(&block_id) else {
                    continue;
                };
                for instruction in &block.instructions {
                    let MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(receiver),
                                certainty,
                                ..
                            }),
                        args,
                        ..
                    } = instruction
                    else {
                        continue;
                    };
                    let Some(route_box_name) = user_box_route_receiver_box_name(
                        function,
                        &def_map,
                        &user_box_names,
                        box_name,
                        *certainty,
                        *receiver,
                        &current,
                        field_box_origins,
                    ) else {
                        continue;
                    };
                    let target_symbol = method_target_symbol(&route_box_name, method, args.len());
                    if !targets.contains_key(&target_symbol) {
                        continue;
                    }

                    changed |= merge_param_box_origin(
                        &mut origins,
                        (target_symbol.clone(), 0),
                        route_box_name,
                    );
                    for (arg_index, arg) in args.iter().enumerate() {
                        let Some(arg_box_name) = user_box_value_box_name(
                            function,
                            &def_map,
                            *arg,
                            &current,
                            field_box_origins,
                        ) else {
                            continue;
                        };
                        if !user_box_names.contains(&arg_box_name) {
                            continue;
                        }
                        changed |= merge_param_box_origin(
                            &mut origins,
                            (target_symbol.clone(), arg_index + 1),
                            arg_box_name,
                        );
                    }
                    for (arg_index, arg) in args.iter().enumerate() {
                        let Some(target_param_box_name) =
                            param_box_origin(&current, &target_symbol, arg_index + 1)
                        else {
                            continue;
                        };
                        let Some(caller_param_index) = value_param_index(function, &def_map, *arg)
                        else {
                            continue;
                        };
                        if !param_accepts_inferred_box_origin(function, caller_param_index) {
                            continue;
                        }
                        changed |= merge_param_box_origin(
                            &mut origins,
                            (function.signature.name.clone(), caller_param_index),
                            target_param_box_name,
                        );
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    origins
}

fn typed_object_plan_field_sets(module: &MirModule) -> BTreeMap<String, BTreeSet<String>> {
    module
        .metadata
        .typed_object_plans
        .iter()
        .map(|plan| {
            (
                plan.box_name.clone(),
                plan.fields
                    .iter()
                    .map(|field| field.name.clone())
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect()
}

fn infer_param_box_origins_from_field_uses(
    function: &MirFunction,
    def_map: &ValueDefMap,
    typed_plan_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<(usize, String)> {
    let mut param_fields = BTreeMap::<usize, BTreeSet<String>>::new();
    for block_id in sorted_block_ids(function) {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::FieldGet { base, field, .. }
                | MirInstruction::FieldSet { base, field, .. } => {
                    let Some(param_index) = value_param_index(function, def_map, *base) else {
                        continue;
                    };
                    if !param_accepts_inferred_box_origin(function, param_index) {
                        continue;
                    }
                    param_fields
                        .entry(param_index)
                        .or_default()
                        .insert(field.clone());
                }
                _ => {}
            }
        }
    }

    param_fields
        .into_iter()
        .filter_map(|(param_index, fields)| {
            let box_name = unique_typed_object_plan_for_fields(&fields, typed_plan_fields)?;
            Some((param_index, box_name))
        })
        .collect()
}

fn unique_typed_object_plan_for_fields(
    fields: &BTreeSet<String>,
    typed_plan_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Option<String> {
    if fields.is_empty() {
        return None;
    }
    let mut candidates = typed_plan_fields
        .iter()
        .filter(|(_box_name, plan_fields)| fields.is_subset(plan_fields))
        .map(|(box_name, _plan_fields)| box_name.clone());
    let first = candidates.next()?;
    if candidates.next().is_none() {
        Some(first)
    } else {
        None
    }
}

fn param_accepts_inferred_box_origin(function: &MirFunction, param_index: usize) -> bool {
    matches!(
        function.signature.params.get(param_index),
        Some(MirType::Unknown) | None
    )
}

fn infer_user_box_field_box_origins(
    module: &MirModule,
    targets: &BTreeMap<String, UserBoxMethodTargetFacts>,
    param_box_origins: &ParamBoxOriginMap,
) -> FieldBoxOriginMap {
    let mut user_box_names = targets
        .values()
        .map(|target| target.box_name.clone())
        .collect::<BTreeSet<_>>();
    user_box_names.extend(
        module
            .metadata
            .typed_object_plans
            .iter()
            .map(|plan| plan.box_name.clone()),
    );
    let birth_field_params = collect_birth_field_param_bindings(module);
    let mut origins = FieldBoxOriginMap::new();

    for _ in 0..module.functions.len().saturating_mul(2).max(1) {
        let current = origins.clone();
        let mut changed = false;
        for function in module.functions.values() {
            let def_map = build_value_def_map(function);
            for block_id in sorted_block_ids(function) {
                let Some(block) = function.blocks.get(&block_id) else {
                    continue;
                };
                for instruction in &block.instructions {
                    let MirInstruction::FieldSet {
                        base, field, value, ..
                    } = instruction
                    else {
                        continue;
                    };
                    let Some(base_box) = user_box_value_box_name(
                        function,
                        &def_map,
                        *base,
                        param_box_origins,
                        &current,
                    ) else {
                        continue;
                    };
                    let Some(value_box) = user_box_value_box_name(
                        function,
                        &def_map,
                        *value,
                        param_box_origins,
                        &current,
                    ) else {
                        continue;
                    };
                    if !user_box_names.contains(&base_box)
                        || !(user_box_names.contains(&value_box) || value_box == "StringBox")
                    {
                        continue;
                    }
                    changed |=
                        merge_field_box_origin(&mut origins, (base_box, field.clone()), value_box);
                }
                for instruction in &block.instructions {
                    let MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(receiver),
                                certainty,
                                ..
                            }),
                        args,
                        ..
                    } = instruction
                    else {
                        continue;
                    };
                    if method != "birth" {
                        continue;
                    }
                    let Some(route_box_name) = user_box_route_receiver_box_name(
                        function,
                        &def_map,
                        &user_box_names,
                        box_name,
                        *certainty,
                        *receiver,
                        param_box_origins,
                        &current,
                    ) else {
                        continue;
                    };
                    for ((birth_box, field), param_index) in &birth_field_params {
                        if birth_box != &route_box_name || *param_index == 0 {
                            continue;
                        }
                        let Some(arg) = args.get(param_index - 1) else {
                            continue;
                        };
                        let Some(value_box) = user_box_value_box_name(
                            function,
                            &def_map,
                            *arg,
                            param_box_origins,
                            &current,
                        ) else {
                            continue;
                        };
                        if !(user_box_names.contains(&value_box) || value_box == "StringBox") {
                            continue;
                        }
                        changed |= merge_field_box_origin(
                            &mut origins,
                            (route_box_name.clone(), field.clone()),
                            value_box,
                        );
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    origins
}

fn collect_birth_field_param_bindings(module: &MirModule) -> BTreeMap<(String, String), usize> {
    let mut bindings = BTreeMap::new();
    for (name, function) in &module.functions {
        let Some((box_name, method, _arity)) = parse_method_symbol(name) else {
            continue;
        };
        if method != "birth" {
            continue;
        }
        let def_map = build_value_def_map(function);
        let receiver = function.params.first().copied();
        for block_id in sorted_block_ids(function) {
            let Some(block) = function.blocks.get(&block_id) else {
                continue;
            };
            for instruction in &block.instructions {
                let MirInstruction::FieldSet {
                    base, field, value, ..
                } = instruction
                else {
                    continue;
                };
                if Some(resolve_value_origin(function, &def_map, *base)) != receiver {
                    continue;
                }
                let value_origin = resolve_value_origin(function, &def_map, *value);
                let Some(param_index) = function
                    .params
                    .iter()
                    .position(|param| param == value)
                    .or_else(|| {
                        function
                            .params
                            .iter()
                            .position(|param| *param == value_origin)
                    })
                else {
                    continue;
                };
                bindings.insert((box_name.to_string(), field.clone()), param_index);
            }
        }
    }
    bindings
}

fn build_user_box_field_return_hints(
    module: &MirModule,
    field_box_origins: &FieldBoxOriginMap,
) -> UserBoxFieldReturnHints {
    let mut hints = UserBoxFieldReturnHints::new();
    for plan in &module.metadata.typed_object_plans {
        for field in &plan.fields {
            let hint = match field.storage {
                TypedObjectFieldStorage::I64 => UserBoxMethodInferredReturn::ScalarI64,
                TypedObjectFieldStorage::Handle => UserBoxMethodInferredReturn::ObjectHandle,
            };
            hints.insert((plan.box_name.clone(), field.name.clone()), hint);
        }
    }
    for ((box_name, field), origin) in field_box_origins {
        let Some(field_box) = box_origin_known(origin) else {
            continue;
        };
        let hint = if field_box == "StringBox" {
            UserBoxMethodInferredReturn::StringHandle
        } else {
            UserBoxMethodInferredReturn::ObjectHandle
        };
        hints.insert((box_name.clone(), field.clone()), hint);
    }
    hints
}

fn merge_param_box_origin(
    origins: &mut ParamBoxOriginMap,
    key: ParamBoxOriginKey,
    box_name: String,
) -> bool {
    match origins.get(&key) {
        Some(BoxOriginInference::Known(existing)) if existing == &box_name => false,
        Some(BoxOriginInference::Conflict) => false,
        Some(BoxOriginInference::Known(_)) => {
            origins.insert(key, BoxOriginInference::Conflict);
            true
        }
        None => {
            origins.insert(key, BoxOriginInference::Known(box_name));
            true
        }
    }
}

fn merge_field_box_origin(
    origins: &mut FieldBoxOriginMap,
    key: FieldBoxOriginKey,
    box_name: String,
) -> bool {
    match origins.get(&key) {
        Some(BoxOriginInference::Known(existing)) if existing == &box_name => false,
        Some(BoxOriginInference::Conflict) => false,
        Some(BoxOriginInference::Known(_)) => {
            origins.insert(key, BoxOriginInference::Conflict);
            true
        }
        None => {
            origins.insert(key, BoxOriginInference::Known(box_name));
            true
        }
    }
}

fn user_box_route_receiver_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    user_box_names: &BTreeSet<String>,
    callee_box_name: &str,
    certainty: TypeCertainty,
    receiver: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> Option<String> {
    if certainty == TypeCertainty::Known && user_box_names.contains(callee_box_name) {
        return Some(callee_box_name.to_string());
    }
    user_box_value_box_name(
        function,
        def_map,
        receiver,
        param_box_origins,
        field_box_origins,
    )
    .filter(|box_name| user_box_names.contains(box_name))
}

fn user_box_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    param_box_origins: &ParamBoxOriginMap,
    field_box_origins: &FieldBoxOriginMap,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    if let Some(box_name) = value_box_name(function, origin).map(str::to_string) {
        return Some(box_name);
    }
    if let Some(box_name) = route_result_box_name(function, origin).map(str::to_string) {
        return Some(box_name);
    }
    if let Some((block_id, instruction_index)) = def_map.get(&origin).copied() {
        let block = function.blocks.get(&block_id)?;
        match block.instructions.get(instruction_index)? {
            MirInstruction::Const {
                value: ConstValue::String(_),
                ..
            } => return Some("StringBox".to_string()),
            MirInstruction::NewBox { box_type, .. } => return Some(box_type.clone()),
            MirInstruction::Phi {
                inputs, type_hint, ..
            } => {
                if let Some(box_name) = type_hint.as_ref().and_then(box_name_from_type) {
                    return Some(box_name.to_string());
                }
                if let Some(box_name) = phi_input_box_name(function, def_map, inputs) {
                    return Some(box_name);
                }
            }
            MirInstruction::FieldGet { base, field, .. } => {
                let base_box = user_box_value_box_name(
                    function,
                    def_map,
                    *base,
                    param_box_origins,
                    field_box_origins,
                )?;
                if let Some(field_box) = field_box_origin(field_box_origins, &base_box, field) {
                    return Some(field_box);
                }
            }
            _ => {}
        }
    }
    function
        .params
        .iter()
        .position(|param| *param == origin)
        .and_then(|index| {
            param_box_origin(param_box_origins, &function.signature.name, index).or_else(|| {
                (index == 0)
                    .then(|| method_receiver_box_name(&function.signature.name))
                    .flatten()
            })
        })
}

fn value_param_index(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<usize> {
    let mut visiting = BTreeSet::new();
    value_param_index_inner(function, def_map, value, &mut visiting)
}

fn value_param_index_inner(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visiting: &mut BTreeSet<ValueId>,
) -> Option<usize> {
    if !visiting.insert(value) {
        return None;
    }
    if let Some(index) = function.params.iter().position(|param| *param == value) {
        visiting.remove(&value);
        return Some(index);
    }
    let result = def_map
        .get(&value)
        .and_then(|(block_id, instruction_index)| {
            function
                .blocks
                .get(block_id)
                .and_then(|block| block.instructions.get(*instruction_index))
        })
        .and_then(|instruction| match instruction {
            MirInstruction::Copy { src, .. } => {
                value_param_index_inner(function, def_map, *src, visiting)
            }
            MirInstruction::Phi { inputs, .. } => {
                let mut inferred = None;
                for (_incoming_block, incoming_value) in inputs {
                    let index =
                        value_param_index_inner(function, def_map, *incoming_value, visiting)?;
                    inferred = match inferred {
                        None => Some(index),
                        Some(existing) if existing == index => Some(existing),
                        Some(_) => return None,
                    };
                }
                inferred
            }
            _ => None,
        });
    visiting.remove(&value);
    result
}

fn phi_input_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inputs: &[(BasicBlockId, ValueId)],
) -> Option<String> {
    let mut inferred = None;
    for (_, input) in inputs {
        let origin = resolve_value_origin(function, def_map, *input);
        let box_name = value_box_name(function, origin)
            .or_else(|| route_result_box_name(function, origin))?
            .to_string();
        inferred = match inferred {
            None => Some(box_name),
            Some(existing) if existing == box_name => Some(existing),
            _ => return None,
        };
    }
    inferred
}

fn route_result_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
    function
        .metadata
        .user_box_method_routes
        .iter()
        .find(|route| route.reason().is_none() && route.result_value() == Some(value))
        .and_then(UserBoxMethodRoute::target_result_box_name)
        .or_else(|| {
            function
                .metadata
                .generic_method_routes
                .iter()
                .find(|route| route.result_value() == Some(value))
                .and_then(generic_method_route_result_box_name)
        })
        .or_else(|| {
            function
                .metadata
                .global_call_routes
                .iter()
                .find(|route| route.result_value() == Some(value))
                .and_then(global_call_route_result_box_name)
        })
}

fn generic_method_route_result_box_name(
    route: &crate::mir::generic_method_route_plan::GenericMethodRoute,
) -> Option<&str> {
    route
        .result_origin_box()
        .or_else(|| match route.route_kind_tag() {
            "string_substring" => Some("StringBox"),
            "map_keys_array" => Some("ArrayBox"),
            _ => None,
        })
}

fn value_box_name(function: &MirFunction, value: ValueId) -> Option<&str> {
    function
        .metadata
        .value_types
        .get(&value)
        .and_then(box_name_from_type)
        .or_else(|| {
            function
                .params
                .iter()
                .position(|param| *param == value)
                .and_then(|index| function.signature.params.get(index))
                .and_then(box_name_from_type)
        })
}

fn box_name_from_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::String => Some("StringBox"),
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

fn global_call_route_result_box_name(
    route: &crate::mir::global_call_route_plan::GlobalCallRoute,
) -> Option<&'static str> {
    if route.result_origin() == "string" {
        return Some("StringBox");
    }
    match route.return_shape() {
        Some("string_handle" | "string_handle_or_null") => Some("StringBox"),
        Some("array_handle") => Some("ArrayBox"),
        Some("map_handle") => Some("MapBox"),
        _ => None,
    }
}

fn param_box_origin(
    param_box_origins: &ParamBoxOriginMap,
    function_name: &str,
    index: usize,
) -> Option<String> {
    match param_box_origins.get(&(function_name.to_string(), index)) {
        Some(BoxOriginInference::Known(box_name)) => Some(box_name.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

fn field_box_origin(
    field_box_origins: &FieldBoxOriginMap,
    box_name: &str,
    field: &str,
) -> Option<String> {
    match field_box_origins.get(&(box_name.to_string(), field.to_string())) {
        Some(BoxOriginInference::Known(field_box)) => Some(field_box.clone()),
        Some(BoxOriginInference::Conflict) | None => None,
    }
}

fn box_origin_known(origin: &BoxOriginInference) -> Option<&str> {
    match origin {
        BoxOriginInference::Known(box_name) => Some(box_name.as_str()),
        BoxOriginInference::Conflict => None,
    }
}

fn method_receiver_box_name(symbol: &str) -> Option<String> {
    let (owner_and_method, _arity) = symbol.rsplit_once('/')?;
    let (box_name, _method) = owner_and_method.rsplit_once('.')?;
    Some(box_name.to_string())
}

fn sorted_block_ids(function: &MirFunction) -> Vec<BasicBlockId> {
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    block_ids
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserBoxMethodTargetFacts {
    box_name: String,
    arity: usize,
    return_type: MirType,
    inferred_return: Option<UserBoxMethodInferredReturn>,
    result_box_name: Option<String>,
    body_supported: bool,
}

fn collect_method_targets(
    module: &MirModule,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    field_return_hints: &UserBoxFieldReturnHints,
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
                    inferred_return: infer_user_box_method_return(function, field_return_hints),
                    result_box_name: infer_user_box_method_result_box_name(function),
                    body_supported: same_module_body_supported(function, typed_plan_type_ids),
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

fn infer_user_box_method_result_box_name(function: &MirFunction) -> Option<String> {
    let def_map = build_value_def_map(function);
    let mut result = None;
    let mut saw_box = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value: Some(value) } = instruction else {
                continue;
            };
            let mut visiting = BTreeSet::new();
            let box_name = return_value_box_name(function, &def_map, *value, &mut visiting)?;
            let Some(box_name) = box_name else {
                continue;
            };
            saw_box = true;
            match &result {
                None => result = Some(box_name),
                Some(existing) if existing == &box_name => {}
                Some(_) => return None,
            }
        }
    }
    if saw_box {
        result
    } else {
        None
    }
}

fn return_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visiting: &mut BTreeSet<ValueId>,
) -> Option<Option<String>> {
    let origin = resolve_value_origin(function, def_map, value);
    if !visiting.insert(origin) {
        return None;
    }
    let result = return_value_box_name_inner(function, def_map, origin, visiting);
    visiting.remove(&origin);
    result
}

fn return_value_box_name_inner(
    function: &MirFunction,
    def_map: &ValueDefMap,
    origin: ValueId,
    visiting: &mut BTreeSet<ValueId>,
) -> Option<Option<String>> {
    if let Some(box_name) = value_box_name(function, origin) {
        return Some(Some(box_name.to_string()));
    }
    if let Some(box_name) = route_result_box_name(function, origin) {
        return Some(Some(box_name.to_string()));
    }
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return None;
    };
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::Const {
            value: ConstValue::Null | ConstValue::Void,
            ..
        } => Some(None),
        MirInstruction::NewBox { box_type, .. } => Some(Some(box_type.clone())),
        MirInstruction::Phi {
            inputs, type_hint, ..
        } => {
            if let Some(box_name) = type_hint.as_ref().and_then(box_name_from_type) {
                return Some(Some(box_name.to_string()));
            }
            let mut result = None;
            for (_incoming_block, incoming_value) in inputs {
                let box_name = return_value_box_name(function, def_map, *incoming_value, visiting)?;
                let Some(box_name) = box_name else {
                    continue;
                };
                match &result {
                    None => result = Some(box_name),
                    Some(existing) if existing == &box_name => {}
                    Some(_) => return None,
                }
            }
            Some(result)
        }
        _ => None,
    }
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
    use crate::mir::function::{TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan};
    use crate::mir::generic_method_route_plan::test_support::{
        array_push, runtime_data_map_get_mixed_i64_key_with_result_origin_box,
    };
    use crate::mir::{
        BasicBlock, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirInstruction,
    };

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
    fn refresh_module_user_box_method_routes_accepts_birth_with_string_handle_const() {
        let mut module = MirModule::new("user_box_birth_string_const_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Manifest".to_string(), vec!["root".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Manifest".to_string(),
            type_id: 9,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Manifest.birth/0".to_string(),
                params: vec![MirType::Box("Manifest".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("".to_string()),
        });
        birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "root".to_string(),
            value: ValueId::new(1),
            declared_type: Some(MirType::String),
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
                box_name: "Manifest".to_string(),
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
        assert_eq!(route.proof(), "typed_user_box_birth_same_module");
        assert_eq!(route.reason(), None);
        assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
    }

    #[test]
    fn refresh_module_user_box_method_routes_accepts_void_method_with_generic_route() {
        let mut module = MirModule::new("user_box_void_method_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Manifest".to_string(), vec!["chunk_ids".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Manifest".to_string(),
            type_id: 9,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut add_chunk = MirFunction::new(
            FunctionSignature {
                name: "Manifest.addChunk/1".to_string(),
                params: vec![
                    MirType::Box("Manifest".to_string()),
                    MirType::Box("StringBox".to_string()),
                ],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        add_chunk.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut add_block = BasicBlock::new(BasicBlockId::new(0));
        add_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
        add_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Void,
        });
        add_block.add_instruction(MirInstruction::KeepAlive {
            values: vec![ValueId::new(0), ValueId::new(1)],
        });
        add_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        add_chunk
            .metadata
            .generic_method_routes
            .push(array_push(0, 0, 2, 3));
        add_chunk.add_block(add_block);

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
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Manifest".to_string(),
                method: "addChunk".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        });
        main.add_block(block);

        module.add_function(add_chunk);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main function");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.proof(), "typed_user_box_method_same_module");
        assert_eq!(route.reason(), None);
        assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
        assert_eq!(route.definition_owner(), "typed_object_method");
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

    #[test]
    fn refresh_module_user_box_method_routes_accepts_string_handle_method_target() {
        let mut module = MirModule::new("user_box_string_handle_method_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Manifest".to_string(), vec!["name".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Manifest".to_string(),
            type_id: 11,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut name = MirFunction::new(
            FunctionSignature {
                name: "Manifest.name/0".to_string(),
                params: vec![MirType::Box("Manifest".to_string())],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        name.params = vec![ValueId::new(0)];
        let mut name_block = BasicBlock::new(BasicBlockId::new(0));
        name_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("payload-a".to_string()),
        });
        name_block.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(3),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
            type_hint: Some(MirType::Integer),
        });
        name_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        name.add_block(name_block);

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
                box_name: "Manifest".to_string(),
                method: "name".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        main.add_block(block);

        module.add_function(name);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main function");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.proof(), "typed_user_box_method_same_module");
        assert_eq!(route.reason(), None);
        assert_eq!(route.return_shape(), Some("string_handle"));
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.result_origin(), "string");
        assert_eq!(route.definition_owner(), "typed_object_method");
    }

    #[test]
    fn refresh_module_user_box_method_routes_recovers_receiver_box_from_param_origin() {
        let mut module = MirModule::new("user_box_param_receiver_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Store".to_string(), vec!["items".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Store".to_string(),
            type_id: 13,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut put = MirFunction::new(
            FunctionSignature {
                name: "Store.put/1".to_string(),
                params: vec![
                    MirType::Box("Store".to_string()),
                    MirType::Box("StringBox".to_string()),
                ],
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        put.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut put_block = BasicBlock::new(BasicBlockId::new(0));
        put_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        put_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        put.add_block(put_block);

        let mut caller = MirFunction::new(
            FunctionSignature {
                name: "Caller.run/1".to_string(),
                params: vec![MirType::Box("Store".to_string())],
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        caller.params = vec![ValueId::new(0)];
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("chunk".to_string()),
        });
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "put".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        caller.add_block(block);

        module.add_function(put);
        module.add_function(caller);

        refresh_module_user_box_method_routes(&mut module);

        let caller = module.get_function("Caller.run/1").expect("caller");
        let route = &caller.metadata.user_box_method_routes[0];
        assert_eq!(route.box_name(), "Store");
        assert_eq!(route.method(), "put");
        assert_eq!(route.reason(), None, "{route:?}");
        assert_eq!(route.proof(), "typed_user_box_method_same_module");
        assert_eq!(route.return_shape(), Some("string_handle"));
    }

    #[test]
    fn refresh_module_user_box_method_routes_recovers_receiver_box_from_call_arg_origin() {
        let mut module = MirModule::new("user_box_call_arg_receiver_route_test".to_string());
        for name in ["Store", "Worker"] {
            module
                .metadata
                .user_box_decls
                .insert(name.to_string(), Vec::new());
        }
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Store".to_string(),
            type_id: 13,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Worker".to_string(),
            type_id: 14,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut put = MirFunction::new(
            FunctionSignature {
                name: "Store.put/1".to_string(),
                params: vec![
                    MirType::Box("Store".to_string()),
                    MirType::Box("StringBox".to_string()),
                ],
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        put.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut put_block = BasicBlock::new(BasicBlockId::new(0));
        put_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        put_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        put.add_block(put_block);

        let mut run = MirFunction::new(
            FunctionSignature {
                name: "Worker.run/2".to_string(),
                params: vec![
                    MirType::Box("Worker".to_string()),
                    MirType::Unknown,
                    MirType::Box("StringBox".to_string()),
                ],
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        run.params = vec![ValueId::new(0), ValueId::new(1), ValueId::new(2)];
        let mut run_block = BasicBlock::new(BasicBlockId::new(0));
        run_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(1),
        });
        run_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(2),
        });
        run_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "put".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(4)],
            effects: EffectMask::PURE,
        });
        run_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(5)),
        });
        run.add_block(run_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut main_block = BasicBlock::new(BasicBlockId::new(0));
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Store".to_string(),
            args: Vec::new(),
        });
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "Worker".to_string(),
            args: Vec::new(),
        });
        main_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("chunk".to_string()),
        });
        main_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Worker".to_string(),
                method: "run".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(1), ValueId::new(3)],
            effects: EffectMask::PURE,
        });
        main_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        main.add_block(main_block);

        module.add_function(put);
        module.add_function(run);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let run = module.get_function("Worker.run/2").expect("run");
        let route = run
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "put")
            .expect("Store.put route");
        assert_eq!(route.box_name(), "Store");
        assert_eq!(route.reason(), None, "{route:?}");
        assert_eq!(route.return_shape(), Some("string_handle"));
        assert_eq!(
            run.metadata.value_types.get(&ValueId::new(1)),
            Some(&MirType::Box("Store".to_string()))
        );

        let main = module.get_function("main").expect("main");
        let route = main
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "run")
            .expect("Worker.run route");
        assert_eq!(route.reason(), None, "{route:?}");
        assert_eq!(route.return_shape(), Some("string_handle"));
    }

    #[test]
    fn refresh_module_user_box_method_routes_recovers_receiver_box_from_generic_result_origin() {
        let mut module = MirModule::new("user_box_generic_result_receiver_route_test".to_string());
        for name in ["ContentChunk", "Store"] {
            module
                .metadata
                .user_box_decls
                .insert(name.to_string(), Vec::new());
        }
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "ContentChunk".to_string(),
            type_id: 21,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Store".to_string(),
            type_id: 22,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut retain = MirFunction::new(
            FunctionSignature {
                name: "ContentChunk.retain/0".to_string(),
                params: vec![MirType::Box("ContentChunk".to_string())],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        retain.params = vec![ValueId::new(0)];
        let mut retain_block = BasicBlock::new(BasicBlockId::new(0));
        retain_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        });
        retain_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(1),
            field: "ref_count".to_string(),
            declared_type: None,
        });
        retain_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        retain_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(1),
        });
        retain_block.add_instruction(MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(4),
        });
        retain_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(1),
            field: "ref_count".to_string(),
            value: ValueId::new(5),
            declared_type: None,
        });
        retain_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(6),
            base: ValueId::new(1),
            field: "ref_count".to_string(),
            declared_type: None,
        });
        retain_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(7),
            src: ValueId::new(6),
        });
        retain_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(7)),
        });
        retain.add_block(retain_block);
        retain
            .metadata
            .value_types
            .insert(ValueId::new(5), MirType::Integer);

        let mut put = MirFunction::new(
            FunctionSignature {
                name: "Store.put/0".to_string(),
                params: vec![MirType::Box("Store".to_string())],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        put.params = vec![ValueId::new(0)];
        let mut put_block = BasicBlock::new(BasicBlockId::new(0));
        put_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(7),
        });
        put_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(10)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
        put_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        put_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "retain".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: Vec::new(),
            effects: EffectMask::PURE,
        });
        put_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        put.add_block(put_block);
        put.metadata.generic_method_routes.push(
            runtime_data_map_get_mixed_i64_key_with_result_origin_box(
                0,
                1,
                10,
                1,
                2,
                "ContentChunk",
            ),
        );

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut main_block = BasicBlock::new(BasicBlockId::new(0));
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(20),
            box_type: "Store".to_string(),
            args: Vec::new(),
        });
        main_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(21)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "put".to_string(),
                receiver: Some(ValueId::new(20)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: Vec::new(),
            effects: EffectMask::PURE,
        });
        main_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(21)),
        });
        main.add_block(main_block);

        module.add_function(retain);
        module.add_function(put);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let put = module.get_function("Store.put/0").expect("Store.put");
        let retain_route = put
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "retain")
            .expect("ContentChunk.retain route");
        assert_eq!(retain_route.box_name(), "ContentChunk");
        assert_eq!(retain_route.reason(), None, "{retain_route:?}");
        assert_eq!(retain_route.return_shape(), Some("scalar_i64"));

        let main = module.get_function("main").expect("main");
        let put_route = main
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "put")
            .expect("Store.put route");
        assert_eq!(put_route.box_name(), "Store");
        assert_eq!(put_route.reason(), None, "{put_route:?}");
        assert_eq!(put_route.return_shape(), Some("scalar_i64"));
    }

    #[test]
    fn refresh_module_user_box_method_routes_propagates_callee_param_box_to_caller_param() {
        let mut module = MirModule::new("user_box_callee_param_origin_backprop_test".to_string());
        for name in ["Handle", "Page", "Heap"] {
            module
                .metadata
                .user_box_decls
                .insert(name.to_string(), Vec::new());
        }
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Handle".to_string(),
            type_id: 51,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 2,
            fields: vec![
                TypedObjectFieldPlan {
                    name: "page_id".to_string(),
                    slot: 0,
                    declared_type_name: None,
                    storage: TypedObjectFieldStorage::I64,
                    is_weak: false,
                },
                TypedObjectFieldPlan {
                    name: "block_id".to_string(),
                    slot: 1,
                    declared_type_name: None,
                    storage: TypedObjectFieldStorage::I64,
                    is_weak: false,
                },
            ],
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Page".to_string(),
            type_id: 52,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 1,
            fields: vec![TypedObjectFieldPlan {
                name: "page_id".to_string(),
                slot: 0,
                declared_type_name: None,
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            }],
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Heap".to_string(),
            type_id: 53,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut page_release = MirFunction::new(
            FunctionSignature {
                name: "Page.release/1".to_string(),
                params: vec![MirType::Box("Page".to_string()), MirType::Unknown],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        page_release.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut page_block = BasicBlock::new(BasicBlockId::new(0));
        page_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        page_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(3),
            base: ValueId::new(2),
            field: "block_id".to_string(),
            declared_type: None,
        });
        page_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        page_release.add_block(page_block);

        let mut heap_release = MirFunction::new(
            FunctionSignature {
                name: "Heap.release/1".to_string(),
                params: vec![MirType::Box("Heap".to_string()), MirType::Unknown],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        heap_release.params = vec![ValueId::new(10), ValueId::new(11)];
        let mut heap_block = BasicBlock::new(BasicBlockId::new(0));
        heap_block.add_instruction(MirInstruction::Phi {
            dst: ValueId::new(12),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(11))],
            type_hint: None,
        });
        heap_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(12),
        });
        heap_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(14),
            base: ValueId::new(13),
            field: "page_id".to_string(),
            declared_type: None,
        });
        heap_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(15),
            box_type: "Page".to_string(),
            args: Vec::new(),
        });
        heap_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(16)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Page".to_string(),
                method: "release".to_string(),
                receiver: Some(ValueId::new(15)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(13)],
            effects: EffectMask::PURE,
        });
        heap_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(16)),
        });
        heap_release.add_block(heap_block);

        module.add_function(page_release);
        module.add_function(heap_release);

        refresh_module_user_box_method_routes(&mut module);

        let page_release = module.get_function("Page.release/1").expect("Page.release");
        assert_eq!(
            page_release.metadata.value_types.get(&ValueId::new(1)),
            Some(&MirType::Box("Handle".to_string()))
        );

        let heap_release = module.get_function("Heap.release/1").expect("Heap.release");
        assert_eq!(
            heap_release.metadata.value_types.get(&ValueId::new(11)),
            Some(&MirType::Box("Handle".to_string()))
        );
        assert_eq!(
            heap_release.metadata.value_types.get(&ValueId::new(13)),
            Some(&MirType::Box("Handle".to_string()))
        );
    }

    #[test]
    fn refresh_module_user_box_method_routes_refines_placeholder_param_for_string_field_return() {
        let mut module = MirModule::new("user_box_string_field_return_refinement_test".to_string());
        for name in ["ContentChunk", "Store"] {
            module
                .metadata
                .user_box_decls
                .insert(name.to_string(), Vec::new());
        }
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "ContentChunk".to_string(),
            type_id: 41,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 1,
            fields: vec![TypedObjectFieldPlan {
                name: "data".to_string(),
                slot: 0,
                declared_type_name: None,
                storage: TypedObjectFieldStorage::Handle,
                is_weak: false,
            }],
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Store".to_string(),
            type_id: 42,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut chunk_birth = MirFunction::new(
            FunctionSignature {
                name: "ContentChunk.birth/1".to_string(),
                params: vec![MirType::Box("ContentChunk".to_string()), MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        chunk_birth.params = vec![ValueId::new(0), ValueId::new(1)];
        let mut chunk_birth_block = BasicBlock::new(BasicBlockId::new(0));
        chunk_birth_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(0),
        });
        chunk_birth_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(1),
        });
        chunk_birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(2),
            field: "data".to_string(),
            value: ValueId::new(3),
            declared_type: None,
        });
        chunk_birth_block.set_terminator(MirInstruction::Return { value: None });
        chunk_birth.add_block(chunk_birth_block);

        let mut put = MirFunction::new(
            FunctionSignature {
                name: "Store.put/1".to_string(),
                params: vec![MirType::Box("Store".to_string()), MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        put.params = vec![ValueId::new(10), ValueId::new(11)];
        let mut put_block = BasicBlock::new(BasicBlockId::new(0));
        put_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(12),
            box_type: "ContentChunk".to_string(),
            args: vec![ValueId::new(11)],
        });
        put_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(12),
        });
        put_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(14),
            src: ValueId::new(11),
        });
        put_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(15)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ContentChunk".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(13)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(14)],
            effects: EffectMask::PURE,
        });
        put_block.set_terminator(MirInstruction::Return { value: None });
        put.add_block(put_block);

        let mut read_data = MirFunction::new(
            FunctionSignature {
                name: "Store.readData/1".to_string(),
                params: vec![
                    MirType::Box("Store".to_string()),
                    MirType::Box("StringBox".to_string()),
                ],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        read_data.params = vec![ValueId::new(20), ValueId::new(21)];
        let mut read_block = BasicBlock::new(BasicBlockId::new(0));
        read_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(22)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "get".to_string(),
                receiver: Some(ValueId::new(30)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(21)],
            effects: EffectMask::PURE,
        });
        read_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(23),
            base: ValueId::new(22),
            field: "data".to_string(),
            declared_type: None,
        });
        read_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(24),
            src: ValueId::new(23),
        });
        read_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(24)),
        });
        read_data.add_block(read_block);
        read_data.metadata.generic_method_routes.push(
            runtime_data_map_get_mixed_i64_key_with_result_origin_box(
                0,
                0,
                30,
                21,
                22,
                "ContentChunk",
            ),
        );

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: Vec::new(),
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut main_block = BasicBlock::new(BasicBlockId::new(0));
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(40),
            box_type: "Store".to_string(),
            args: Vec::new(),
        });
        main_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(41),
            value: ConstValue::String("chunk".to_string()),
        });
        main_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Store".to_string(),
                method: "put".to_string(),
                receiver: Some(ValueId::new(40)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(41)],
            effects: EffectMask::PURE,
        });
        main_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(42)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Store".to_string(),
                method: "readData".to_string(),
                receiver: Some(ValueId::new(40)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(41)],
            effects: EffectMask::PURE,
        });
        main_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(42)),
        });
        main.add_block(main_block);

        module.add_function(chunk_birth);
        module.add_function(put);
        module.add_function(read_data);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let put = module.get_function("Store.put/1").expect("Store.put");
        assert_eq!(
            put.metadata.value_types.get(&ValueId::new(11)),
            Some(&MirType::Box("StringBox".to_string()))
        );

        let read_data = module
            .get_function("Store.readData/1")
            .expect("Store.readData");
        assert_eq!(
            read_data.metadata.value_types.get(&ValueId::new(23)),
            Some(&MirType::Box("StringBox".to_string()))
        );

        let main = module.get_function("main").expect("main");
        let read_route = main
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "readData")
            .expect("Store.readData route");
        assert_eq!(read_route.reason(), None, "{read_route:?}");
        assert_eq!(read_route.return_shape(), Some("string_handle"));
        assert_eq!(read_route.target_result_box_name(), Some("StringBox"));
    }

    #[test]
    fn refresh_module_user_box_method_routes_recovers_receiver_box_from_field_origin() {
        let mut module = MirModule::new("user_box_field_receiver_route_test".to_string());
        for name in ["Heap", "Store"] {
            module
                .metadata
                .user_box_decls
                .insert(name.to_string(), Vec::new());
        }
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Heap".to_string(),
            type_id: 31,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Store".to_string(),
            type_id: 32,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut allocate = MirFunction::new(
            FunctionSignature {
                name: "Heap.allocate/0".to_string(),
                params: vec![MirType::Box("Heap".to_string())],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        allocate.params = vec![ValueId::new(0)];
        let mut allocate_block = BasicBlock::new(BasicBlockId::new(0));
        allocate_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(99),
        });
        allocate_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        allocate.add_block(allocate_block);

        let mut birth = MirFunction::new(
            FunctionSignature {
                name: "Store.birth/0".to_string(),
                params: vec![MirType::Box("Store".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        birth.params = vec![ValueId::new(0)];
        let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
        birth_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "Heap".to_string(),
            args: Vec::new(),
        });
        birth_block.add_instruction(MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "allocator".to_string(),
            value: ValueId::new(1),
            declared_type: None,
        });
        birth_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        });
        birth_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        birth.add_block(birth_block);

        let mut put = MirFunction::new(
            FunctionSignature {
                name: "Store.put/0".to_string(),
                params: vec![MirType::Box("Store".to_string())],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        put.params = vec![ValueId::new(0)];
        let mut put_block = BasicBlock::new(BasicBlockId::new(0));
        put_block.add_instruction(MirInstruction::FieldGet {
            dst: ValueId::new(1),
            base: ValueId::new(0),
            field: "allocator".to_string(),
            declared_type: None,
        });
        put_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        put_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "allocate".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: Vec::new(),
            effects: EffectMask::PURE,
        });
        put_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        put.add_block(put_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut main_block = BasicBlock::new(BasicBlockId::new(0));
        main_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(10),
            box_type: "Store".to_string(),
            args: Vec::new(),
        });
        main_block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "put".to_string(),
                receiver: Some(ValueId::new(10)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: Vec::new(),
            effects: EffectMask::PURE,
        });
        main_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(11)),
        });
        main.add_block(main_block);

        module.add_function(allocate);
        module.add_function(birth);
        module.add_function(put);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let put = module.get_function("Store.put/0").expect("Store.put");
        let allocate_route = put
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "allocate")
            .expect("Heap.allocate route");
        assert_eq!(allocate_route.box_name(), "Heap");
        assert_eq!(allocate_route.reason(), None, "{allocate_route:?}");
        assert_eq!(allocate_route.return_shape(), Some("scalar_i64"));

        let main = module.get_function("main").expect("main");
        let put_route = main
            .metadata
            .user_box_method_routes
            .iter()
            .find(|route| route.method() == "put")
            .expect("Store.put route");
        assert_eq!(put_route.box_name(), "Store");
        assert_eq!(put_route.reason(), None, "{put_route:?}");
        assert_eq!(put_route.return_shape(), Some("scalar_i64"));
    }

    #[test]
    fn refresh_module_user_box_method_routes_accepts_object_handle_method_target() {
        let mut module = MirModule::new("user_box_object_handle_method_route_test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Manifest".to_string(), vec!["name".to_string()]);
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Manifest".to_string(),
            type_id: 12,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut identity = MirFunction::new(
            FunctionSignature {
                name: "Manifest.identity/0".to_string(),
                params: vec![MirType::Box("Manifest".to_string())],
                return_type: MirType::Box("Manifest".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        identity.params = vec![ValueId::new(0)];
        let mut identity_block = BasicBlock::new(BasicBlockId::new(0));
        identity_block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        });
        identity_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        identity.add_block(identity_block);

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
                box_name: "Manifest".to_string(),
                method: "identity".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        main.add_block(block);

        module.add_function(identity);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main function");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.proof(), "typed_user_box_method_same_module");
        assert_eq!(route.reason(), None);
        assert_eq!(route.return_shape(), Some("object_handle"));
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.result_origin(), "none");
        assert_eq!(route.definition_owner(), "typed_object_method");
    }

    #[test]
    fn refresh_module_user_box_method_routes_accepts_nullable_object_handle_method_target() {
        let mut module =
            MirModule::new("user_box_nullable_object_handle_method_route_test".to_string());
        for name in ["Allocator", "Handle"] {
            module
                .metadata
                .user_box_decls
                .insert(name.to_string(), Vec::new());
        }
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Allocator".to_string(),
            type_id: 41,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Handle".to_string(),
            type_id: 42,
            layout_kind: "runtime_slot_object_v0".to_string(),
            field_count: 0,
            fields: Vec::new(),
        });

        let mut allocate = MirFunction::new(
            FunctionSignature {
                name: "Allocator.allocate/0".to_string(),
                params: vec![MirType::Box("Allocator".to_string())],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        allocate.params = vec![ValueId::new(0)];
        let mut null_block = BasicBlock::new(BasicBlockId::new(0));
        null_block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Null,
        });
        null_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        let mut handle_block = BasicBlock::new(BasicBlockId::new(1));
        handle_block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "Handle".to_string(),
            args: Vec::new(),
        });
        handle_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        allocate.add_block(null_block);
        allocate.add_block(handle_block);

        let mut main = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Box("Handle".to_string()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "Allocator".to_string(),
            args: Vec::new(),
        });
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "allocate".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Union,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: Vec::new(),
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(4)),
        });
        main.add_block(block);

        module.add_function(allocate);
        module.add_function(main);

        refresh_module_user_box_method_routes(&mut module);

        let main = module.get_function("main").expect("main");
        let route = &main.metadata.user_box_method_routes[0];
        assert_eq!(route.reason(), None, "{route:?}");
        assert_eq!(route.return_shape(), Some("object_handle"));
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    }
}
