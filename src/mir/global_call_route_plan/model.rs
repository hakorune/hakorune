use super::type_label::format_mir_type_label;
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
    target: GlobalCallTargetFacts,
}

// Shape status/removal policy lives in:
// docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
// Do not add variants without updating that inventory and its removal path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlobalCallTargetShape {
    #[default]
    Unknown,
    NumericI64Leaf,
    GenericPureStringBody,
    GenericStringOrVoidSentinelBody,
    GenericI64Body,
    ParserProgramJsonBody,
    MirSchemaMapConstructorBody,
    BoxTypeInspectorDescribeBody,
    PatternUtilLocalValueProbeBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GlobalCallReturnContract {
    ScalarI64,
    StringHandle,
    StringHandleOrNull,
    VoidSentinelI64Zero,
    ArrayHandle,
    MapHandle,
    MixedRuntimeI64OrHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GlobalCallProof {
    ContractMissing,
    LeafNumericI64,
    GenericPureString,
    GenericStringOrVoidSentinel,
    GenericStringVoidLogging,
    GenericI64,
    ParserProgramJson,
    StaticStringArray,
    MirSchemaMapConstructor,
    BoxTypeInspectorDescribe,
    PatternUtilLocalValueProbe,
}

impl GlobalCallProof {
    pub(super) fn as_json_name(self) -> &'static str {
        match self {
            Self::ContractMissing => "typed_global_call_contract_missing",
            Self::LeafNumericI64 => "typed_global_call_leaf_numeric_i64",
            Self::GenericPureString => "typed_global_call_generic_pure_string",
            Self::GenericStringOrVoidSentinel => {
                "typed_global_call_generic_string_or_void_sentinel"
            }
            Self::GenericStringVoidLogging => "typed_global_call_generic_string_void_logging",
            Self::GenericI64 => "typed_global_call_generic_i64",
            Self::ParserProgramJson => "typed_global_call_parser_program_json",
            Self::StaticStringArray => "typed_global_call_static_string_array",
            Self::MirSchemaMapConstructor => "typed_global_call_mir_schema_map_constructor",
            Self::BoxTypeInspectorDescribe => "typed_global_call_box_type_inspector_describe",
            Self::PatternUtilLocalValueProbe => "typed_global_call_pattern_util_local_value_probe",
        }
    }

    fn from_shape(shape: GlobalCallTargetShape) -> Self {
        match shape {
            GlobalCallTargetShape::Unknown => Self::ContractMissing,
            GlobalCallTargetShape::NumericI64Leaf => Self::LeafNumericI64,
            GlobalCallTargetShape::GenericPureStringBody => Self::GenericPureString,
            GlobalCallTargetShape::GenericStringOrVoidSentinelBody => {
                Self::GenericStringOrVoidSentinel
            }
            GlobalCallTargetShape::GenericI64Body => Self::GenericI64,
            GlobalCallTargetShape::ParserProgramJsonBody => Self::ParserProgramJson,
            GlobalCallTargetShape::MirSchemaMapConstructorBody => Self::MirSchemaMapConstructor,
            GlobalCallTargetShape::BoxTypeInspectorDescribeBody => Self::BoxTypeInspectorDescribe,
            GlobalCallTargetShape::PatternUtilLocalValueProbeBody => {
                Self::PatternUtilLocalValueProbe
            }
        }
    }

    fn is_direct(self) -> bool {
        self != Self::ContractMissing
    }
}

impl Default for GlobalCallProof {
    fn default() -> Self {
        Self::ContractMissing
    }
}

impl GlobalCallReturnContract {
    pub(super) fn as_json_name(self) -> &'static str {
        match self {
            Self::ScalarI64 => "ScalarI64",
            Self::StringHandle => "string_handle",
            Self::StringHandleOrNull => "string_handle_or_null",
            Self::VoidSentinelI64Zero => "void_sentinel_i64_zero",
            Self::ArrayHandle => "array_handle",
            Self::MapHandle => "map_handle",
            Self::MixedRuntimeI64OrHandle => "mixed_runtime_i64_or_handle",
        }
    }

    pub(super) fn value_demand(self) -> &'static str {
        match self {
            Self::ScalarI64 | Self::VoidSentinelI64Zero => "scalar_i64",
            Self::StringHandle
            | Self::StringHandleOrNull
            | Self::ArrayHandle
            | Self::MapHandle
            | Self::MixedRuntimeI64OrHandle => "runtime_i64_or_handle",
        }
    }
}

impl GlobalCallTargetShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NumericI64Leaf => "numeric_i64_leaf",
            Self::GenericPureStringBody => "generic_pure_string_body",
            Self::GenericStringOrVoidSentinelBody => "generic_string_or_void_sentinel_body",
            Self::GenericI64Body => "generic_i64_body",
            Self::ParserProgramJsonBody => "parser_program_json_body",
            Self::MirSchemaMapConstructorBody => "mir_schema_map_constructor_body",
            Self::BoxTypeInspectorDescribeBody => "box_type_inspector_describe_body",
            Self::PatternUtilLocalValueProbeBody => "pattern_util_local_value_probe_body",
        }
    }

    pub(super) fn return_contract(self) -> Option<GlobalCallReturnContract> {
        match self {
            Self::NumericI64Leaf | Self::GenericI64Body => {
                Some(GlobalCallReturnContract::ScalarI64)
            }
            Self::GenericPureStringBody | Self::ParserProgramJsonBody => {
                Some(GlobalCallReturnContract::StringHandle)
            }
            Self::GenericStringOrVoidSentinelBody => {
                Some(GlobalCallReturnContract::StringHandleOrNull)
            }
            Self::MirSchemaMapConstructorBody | Self::BoxTypeInspectorDescribeBody => {
                Some(GlobalCallReturnContract::MapHandle)
            }
            Self::PatternUtilLocalValueProbeBody => {
                Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)
            }
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GlobalCallTargetShapeReason {
    ParamBindingMismatch,
    GenericStringReturnAbiNotHandleCompatible,
    GenericStringReturnObjectAbiNotHandleCompatible,
    GenericStringReturnVoidSentinelCandidate,
    GenericStringParamAbiNotHandleCompatible,
    GenericStringUnsupportedInstruction,
    GenericStringUnsupportedVoidSentinelConst,
    GenericStringUnsupportedCall,
    GenericStringUnsupportedMethodCall,
    GenericStringUnsupportedKnownReceiverMethod,
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
            Self::GenericStringUnsupportedKnownReceiverMethod => {
                "generic_string_unsupported_known_receiver_method"
            }
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
pub(super) struct GlobalCallTargetClassification {
    pub(super) shape: GlobalCallTargetShape,
    pub(super) return_contract: Option<GlobalCallReturnContract>,
    pub(super) proof: GlobalCallProof,
    pub(super) reason: Option<GlobalCallTargetShapeReason>,
    pub(super) blocker: Option<GlobalCallShapeBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GlobalCallShapeBlocker {
    pub(super) symbol: String,
    pub(super) reason: Option<GlobalCallTargetShapeReason>,
}

impl GlobalCallTargetClassification {
    pub(super) fn direct(shape: GlobalCallTargetShape) -> Self {
        Self {
            shape,
            return_contract: shape.return_contract(),
            proof: GlobalCallProof::from_shape(shape),
            reason: None,
            blocker: None,
        }
    }

    pub(super) fn direct_contract(
        proof: GlobalCallProof,
        return_contract: GlobalCallReturnContract,
    ) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
            return_contract: Some(return_contract),
            proof,
            reason: None,
            blocker: None,
        }
    }

    pub(super) fn unknown(reason: GlobalCallTargetShapeReason) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
            return_contract: None,
            proof: GlobalCallProof::ContractMissing,
            reason: Some(reason),
            blocker: None,
        }
    }

    pub(super) fn unknown_with_blocker(
        reason: GlobalCallTargetShapeReason,
        symbol: impl Into<String>,
        blocker_reason: Option<GlobalCallTargetShapeReason>,
    ) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
            return_contract: None,
            proof: GlobalCallProof::ContractMissing,
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
    return_contract: Option<GlobalCallReturnContract>,
    proof: GlobalCallProof,
    shape_reason: Option<GlobalCallTargetShapeReason>,
    pub(super) shape_blocker: Option<GlobalCallShapeBlocker>,
}

impl GlobalCallTargetFacts {
    pub fn missing() -> Self {
        Self::default()
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
            return_contract: None,
            proof: GlobalCallProof::ContractMissing,
            shape_reason: None,
            shape_blocker: None,
        }
    }

    #[cfg(test)]
    pub fn present_with_shape(arity: usize, shape: GlobalCallTargetShape) -> Self {
        Self {
            exists: true,
            symbol: None,
            arity: Some(arity),
            return_type: None,
            shape,
            return_contract: shape.return_contract(),
            proof: GlobalCallProof::from_shape(shape),
            shape_reason: None,
            shape_blocker: None,
        }
    }

    #[cfg(test)]
    pub fn present_static_string_array_contract(arity: usize) -> Self {
        Self {
            exists: true,
            symbol: None,
            arity: Some(arity),
            return_type: None,
            shape: GlobalCallTargetShape::Unknown,
            return_contract: Some(GlobalCallReturnContract::ArrayHandle),
            proof: GlobalCallProof::StaticStringArray,
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

    pub(super) fn return_contract(&self) -> Option<GlobalCallReturnContract> {
        self.return_contract
    }

    pub(super) fn proof(&self) -> GlobalCallProof {
        self.proof
    }

    pub(super) fn shape_reason(&self) -> Option<GlobalCallTargetShapeReason> {
        self.shape_reason
    }

    pub(super) fn shape_blocker_symbol(&self) -> Option<&str> {
        self.shape_blocker
            .as_ref()
            .map(|blocker| blocker.symbol.as_str())
    }

    pub(super) fn shape_blocker_reason(&self) -> Option<GlobalCallTargetShapeReason> {
        self.shape_blocker
            .as_ref()
            .and_then(|blocker| blocker.reason)
    }

    pub(super) fn with_classification(
        mut self,
        classification: GlobalCallTargetClassification,
    ) -> Self {
        self.shape = classification.shape;
        self.return_contract = classification.return_contract;
        self.proof = classification.proof;
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

    pub fn lowering_tier(&self) -> LoweringPlanTier {
        if self.is_direct_abi_target() {
            LoweringPlanTier::DirectAbi
        } else {
            LoweringPlanTier::Unsupported
        }
    }

    pub fn tier(&self) -> &'static str {
        self.lowering_tier().as_json_name()
    }

    pub fn lowering_emit_kind(&self) -> LoweringPlanEmitKind {
        if self.is_direct_abi_target() {
            LoweringPlanEmitKind::DirectFunctionCall
        } else {
            LoweringPlanEmitKind::Unsupported
        }
    }

    pub fn emit_kind(&self) -> &'static str {
        self.lowering_emit_kind().as_json_name()
    }

    pub fn proof(&self) -> &'static str {
        if self.is_direct_abi_target() {
            self.target.proof().as_json_name()
        } else {
            GlobalCallProof::ContractMissing.as_json_name()
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
        self.direct_return_contract()
            .map(GlobalCallReturnContract::value_demand)
            .unwrap_or("typed_global_call_contract_missing")
    }

    pub fn return_shape(&self) -> Option<&'static str> {
        self.direct_return_contract()
            .map(GlobalCallReturnContract::as_json_name)
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
}
