use super::super::type_label::format_mir_type_label;
use super::{
    GlobalCallDefinitionOwner, GlobalCallProof, GlobalCallReturnContract, GlobalCallTargetFacts,
};
use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};
use crate::mir::{BasicBlockId, MirType, ValueId};

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
    arg0_origin_box: Option<String>,
    lowering_override: Option<GlobalCallLoweringOverride>,
    target: GlobalCallTargetFacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalCallLoweringOverride {
    BuiltinPrint,
    Stage1EmitProgramJson,
}

impl GlobalCallLoweringOverride {
    fn route_kind(self) -> &'static str {
        match self {
            Self::BuiltinPrint => "global.print",
            Self::Stage1EmitProgramJson => "stage1.emit_program_json_v0",
        }
    }

    fn target_symbol(self) -> &'static str {
        match self {
            Self::BuiltinPrint => "print",
            Self::Stage1EmitProgramJson => "nyash.stage1.emit_program_json_v0_h",
        }
    }

    fn lowering_tier(self) -> LoweringPlanTier {
        LoweringPlanTier::ColdRuntime
    }

    fn lowering_emit_kind(self) -> LoweringPlanEmitKind {
        LoweringPlanEmitKind::RuntimeCall
    }

    fn proof(self) -> GlobalCallProof {
        match self {
            Self::BuiltinPrint => GlobalCallProof::VoidSideEffect,
            Self::Stage1EmitProgramJson => GlobalCallProof::Stage1EmitProgramJson,
        }
    }

    fn return_contract(self) -> GlobalCallReturnContract {
        match self {
            Self::BuiltinPrint => GlobalCallReturnContract::VoidSentinelI64Zero,
            Self::Stage1EmitProgramJson => GlobalCallReturnContract::StringHandle,
        }
    }

    fn effect_tags(self) -> &'static [&'static str] {
        match self {
            Self::BuiltinPrint => &["print"],
            Self::Stage1EmitProgramJson => &["stage1.emit_program_json"],
        }
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
            arg0_origin_box: None,
            lowering_override: None,
            target,
        }
    }

    pub fn with_arg0_origin_box(mut self, arg0_origin_box: Option<String>) -> Self {
        self.arg0_origin_box = arg0_origin_box;
        self
    }

    pub fn with_optional_lowering_override(
        mut self,
        lowering_override: Option<GlobalCallLoweringOverride>,
    ) -> Self {
        self.lowering_override = lowering_override;
        self
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

    pub fn lowering_tier(&self) -> LoweringPlanTier {
        if let Some(lowering_override) = self.lowering_override {
            lowering_override.lowering_tier()
        } else if self.is_direct_abi_target() {
            LoweringPlanTier::DirectAbi
        } else {
            LoweringPlanTier::Unsupported
        }
    }

    pub fn tier(&self) -> &'static str {
        self.lowering_tier().as_json_name()
    }

    pub fn lowering_emit_kind(&self) -> LoweringPlanEmitKind {
        if let Some(lowering_override) = self.lowering_override {
            lowering_override.lowering_emit_kind()
        } else if self.is_direct_abi_target() {
            LoweringPlanEmitKind::DirectFunctionCall
        } else {
            LoweringPlanEmitKind::Unsupported
        }
    }

    pub fn emit_kind(&self) -> &'static str {
        self.lowering_emit_kind().as_json_name()
    }

    pub fn proof(&self) -> &'static str {
        if let Some(lowering_override) = self.lowering_override {
            lowering_override.proof().as_json_name()
        } else if self.is_direct_abi_target() {
            self.target.proof().as_json_name()
        } else {
            GlobalCallProof::ContractMissing.as_json_name()
        }
    }

    pub fn route_kind(&self) -> &'static str {
        if let Some(lowering_override) = self.lowering_override {
            lowering_override.route_kind()
        } else {
            "global.user_call"
        }
    }

    pub fn is_builtin_print(&self) -> bool {
        matches!(
            self.lowering_override,
            Some(GlobalCallLoweringOverride::BuiltinPrint)
        )
    }

    pub fn callee_name(&self) -> &str {
        &self.callee_name
    }

    pub fn target_symbol(&self) -> Option<&str> {
        if let Some(lowering_override) = self.lowering_override {
            return Some(lowering_override.target_symbol());
        }
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

    pub fn arg0_origin_box(&self) -> Option<&str> {
        self.arg0_origin_box.as_deref()
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

    pub fn target_result_box_name(&self) -> Option<&str> {
        if !self.is_direct_abi_target() {
            return None;
        }
        match self.target.return_type()? {
            MirType::String => Some("StringBox"),
            MirType::Box(name) => Some(name.as_str()),
            MirType::Array(_) => Some("ArrayBox"),
            _ => None,
        }
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
        self.effective_return_contract()
            .map(GlobalCallReturnContract::value_demand)
            .unwrap_or("typed_global_call_contract_missing")
    }

    pub fn need_kind(&self) -> Option<&'static str> {
        match self.lowering_override {
            Some(GlobalCallLoweringOverride::BuiltinPrint) => Some("printf"),
            _ => None,
        }
    }

    pub fn return_shape(&self) -> Option<&'static str> {
        self.effective_return_contract()
            .map(GlobalCallReturnContract::as_json_name)
    }

    pub fn result_origin(&self) -> &'static str {
        if let Some(lowering_override) = self.lowering_override {
            lowering_override.proof().result_origin()
        } else if self.is_direct_abi_target() {
            self.target.proof().result_origin()
        } else {
            "none"
        }
    }

    fn definition_owner_kind(&self) -> GlobalCallDefinitionOwner {
        if let Some(lowering_override) = self.lowering_override {
            return lowering_override.proof().definition_owner();
        }
        if self.is_direct_abi_target() {
            return self.target.proof().definition_owner();
        }
        if self.reason() == Some("missing_multi_function_emitter") {
            return GlobalCallDefinitionOwner::UniformMir;
        }
        GlobalCallDefinitionOwner::None
    }

    pub fn definition_owner(&self) -> &'static str {
        self.definition_owner_kind().as_json_name()
    }

    pub fn emit_trace_consumer(&self) -> &'static str {
        self.definition_owner_kind().emit_trace_consumer()
    }

    pub fn reason(&self) -> Option<&'static str> {
        if self.lowering_override.is_some() || self.is_direct_abi_target() {
            return None;
        }
        match self.arity_matches() {
            Some(true) => Some("missing_multi_function_emitter"),
            Some(false) => Some("global_call_arity_mismatch"),
            None => Some("unknown_global_callee"),
        }
    }

    pub fn reason_detail(&self) -> Option<String> {
        match self.reason()? {
            "missing_multi_function_emitter" => Some(format!(
                "callee `{}` exists with matching arity {}, but the backend has no multi-function emitter for this route",
                self.callee_name(),
                self.arity()
            )),
            "global_call_arity_mismatch" => Some(format!(
                "callee `{}` exists, but call arity {} does not match target arity {}",
                self.callee_name(),
                self.arity(),
                self.target_arity()
                    .map(|arity| arity.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            )),
            "unknown_global_callee" => Some(format!(
                "callee `{}` is not present in the current MIR module",
                self.callee_name()
            )),
            _ => None,
        }
    }

    pub fn reason_hint(&self) -> Option<&'static str> {
        match self.reason()? {
            "missing_multi_function_emitter" => Some(
                "target exists; keep route metadata and implement or select a backend function-emission owner",
            ),
            "global_call_arity_mismatch" => {
                Some("target exists; compare source call arguments with the lowered function signature")
            }
            "unknown_global_callee" => Some(
                "if this is an imported static-box call, verify the import target is registered in hako.toml module_roots and that the import bundle merged its functions",
            ),
            _ => None,
        }
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        if let Some(lowering_override) = self.lowering_override {
            lowering_override.effect_tags()
        } else {
            &["call.global"]
        }
    }

    fn is_direct_abi_target(&self) -> bool {
        self.target_exists()
            && self.arity_matches() == Some(true)
            && self.target.proof().is_direct()
            && self.target.return_contract().is_some()
    }

    fn direct_return_contract(&self) -> Option<GlobalCallReturnContract> {
        if self.is_direct_abi_target() {
            self.target.return_contract()
        } else {
            None
        }
    }

    fn effective_return_contract(&self) -> Option<GlobalCallReturnContract> {
        self.lowering_override
            .map(GlobalCallLoweringOverride::return_contract)
            .or_else(|| self.direct_return_contract())
    }
}
