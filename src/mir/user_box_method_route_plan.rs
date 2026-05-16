/*!
 * MIR-owned route plans for typed user-box method calls.
 *
 * The first accepted surface is user-box `birth` as a normal same-module
 * method function. Backends consume this route instead of reclassifying raw
 * `Box.method` names or cloning VM InstanceBox behavior.
 */

use std::collections::BTreeMap;

use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};
mod convergence;
mod materialization;
mod origin_inference;
mod return_shape;
mod target_collection;
mod value_type_publish;

use return_shape::UserBoxMethodInferredReturn;

#[cfg(test)]
use crate::mir::Callee;
use crate::mir::{BasicBlockId, MirFunction, MirModule, MirType, ValueId};

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
    convergence::refresh_module_user_box_method_routes_fixpoint(module);
}

pub fn refresh_function_user_box_method_routes(function: &mut MirFunction) {
    materialization::refresh_function_user_box_method_routes_with_context(
        function,
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
        &BTreeMap::new(),
    );
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
