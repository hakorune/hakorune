/*!
 * MIR-owned route plans for typed user-box method calls.
 *
 * The first accepted surface is user-box `birth` as a normal same-module
 * method function. Backends consume this route instead of reclassifying raw
 * `Box.method` names or cloning VM InstanceBox behavior.
 */

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};
use crate::mir::same_module_body_shape::same_module_body_supported;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
mod origin_inference;
mod return_shape;
mod value_type_publish;

use origin_inference::{
    box_name_from_type, build_user_box_field_return_hints, field_box_origin,
    generic_method_route_result_box_name, infer_user_box_field_box_origins,
    infer_user_box_method_param_box_origins, param_box_origin, route_result_box_name,
    sorted_block_ids, user_box_route_receiver_box_name, user_box_value_box_name, value_box_name,
};
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
        if self.target_result_box_name.is_some() {
            return false;
        }
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
            || matches!(self.target_result_box_name.as_deref(), Some("StringBox"))
            || matches!(
                self.target_inferred_return,
                Some(UserBoxMethodInferredReturn::StringHandle)
            )
    }

    fn target_returns_handle(&self) -> bool {
        self.target_returns_string_handle()
            || self.target_result_box_name.is_some()
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
        return Some(None);
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
        MirInstruction::Select {
            then_val, else_val, ..
        } => merge_return_value_box_names(
            return_value_box_name(function, def_map, *then_val, visiting)?,
            return_value_box_name(function, def_map, *else_val, visiting)?,
        ),
        _ => None,
    }
}

fn merge_return_value_box_names(
    left: Option<String>,
    right: Option<String>,
) -> Option<Option<String>> {
    match (left, right) {
        (Some(left), Some(right)) if left == right => Some(Some(left)),
        (Some(_), Some(_)) => None,
        (Some(name), None) | (None, Some(name)) => Some(Some(name)),
        (None, None) => Some(None),
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
#[path = "user_box_method_route_plan/tests/mod.rs"]
mod tests;
