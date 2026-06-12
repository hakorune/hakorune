use crate::mir::MirType;

#[path = "route.rs"]
mod route;

pub use route::{GlobalCallLoweringOverride, GlobalCallRoute, GlobalCallRouteSite};

// Shape status/removal policy lives in:
// docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
// Do not add variants without updating that inventory and its removal path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlobalCallTargetShape {
    #[default]
    Unknown,
    NumericI64Leaf,
    GenericPureStringBody,
    GenericI64Body,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GlobalCallReturnContract {
    ScalarI64,
    StringHandle,
    StringHandleOrNull,
    VoidSentinelI64Zero,
    ArrayHandle,
    MapHandle,
    ObjectHandle,
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
    Stage1EmitProgramJson,
    StaticStringArray,
    MirSchemaMapConstructor,
    BoxTypeInspectorDescribe,
    PatternUtilLocalValueProbe,
    SameModuleObjectHandle,
    SameModuleScalarI64,
    SameModuleVoidSentinel,
    VoidSideEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GlobalCallDefinitionOwner {
    None,
    DiagnosticsOnly,
    LeafI64,
    GenericI64OrLeaf,
    ModuleGeneric,
    UniformMir,
    RuntimeHelper,
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
            Self::Stage1EmitProgramJson => "typed_global_call_stage1_emit_program_json",
            Self::StaticStringArray => "typed_global_call_static_string_array",
            Self::MirSchemaMapConstructor => "typed_global_call_mir_schema_map_constructor",
            Self::BoxTypeInspectorDescribe => "typed_global_call_box_type_inspector_describe",
            Self::PatternUtilLocalValueProbe => "typed_global_call_pattern_util_local_value_probe",
            Self::SameModuleObjectHandle => "typed_global_call_same_module_object_handle",
            Self::SameModuleScalarI64 => "typed_global_call_same_module_scalar_i64",
            Self::SameModuleVoidSentinel => "typed_global_call_same_module_void_sentinel",
            Self::VoidSideEffect => "typed_global_call_void_side_effect",
        }
    }

    fn from_shape(shape: GlobalCallTargetShape) -> Self {
        match shape {
            GlobalCallTargetShape::Unknown => Self::ContractMissing,
            GlobalCallTargetShape::NumericI64Leaf => Self::LeafNumericI64,
            GlobalCallTargetShape::GenericPureStringBody => Self::GenericPureString,
            GlobalCallTargetShape::GenericI64Body => Self::GenericI64,
        }
    }

    fn is_direct(self) -> bool {
        self != Self::ContractMissing
    }

    fn result_origin(self) -> &'static str {
        match self {
            Self::GenericPureString
            | Self::GenericStringOrVoidSentinel
            | Self::ParserProgramJson
            | Self::Stage1EmitProgramJson => "string",
            Self::StaticStringArray => "array_string_birth",
            Self::MirSchemaMapConstructor | Self::BoxTypeInspectorDescribe => "map_birth",
            Self::ContractMissing
            | Self::LeafNumericI64
            | Self::GenericStringVoidLogging
            | Self::GenericI64
            | Self::PatternUtilLocalValueProbe
            | Self::SameModuleObjectHandle
            | Self::SameModuleScalarI64
            | Self::SameModuleVoidSentinel
            | Self::VoidSideEffect => "none",
        }
    }

    fn definition_owner(self) -> GlobalCallDefinitionOwner {
        match self {
            Self::ContractMissing => GlobalCallDefinitionOwner::None,
            Self::LeafNumericI64 => GlobalCallDefinitionOwner::LeafI64,
            Self::GenericI64 => GlobalCallDefinitionOwner::GenericI64OrLeaf,
            Self::ParserProgramJson => GlobalCallDefinitionOwner::DiagnosticsOnly,
            Self::GenericStringVoidLogging
            | Self::StaticStringArray
            | Self::MirSchemaMapConstructor
            | Self::BoxTypeInspectorDescribe
            | Self::GenericStringOrVoidSentinel
            | Self::PatternUtilLocalValueProbe
            | Self::SameModuleObjectHandle
            | Self::SameModuleScalarI64
            | Self::SameModuleVoidSentinel
            | Self::VoidSideEffect => GlobalCallDefinitionOwner::UniformMir,
            Self::Stage1EmitProgramJson => GlobalCallDefinitionOwner::RuntimeHelper,
            Self::GenericPureString => GlobalCallDefinitionOwner::ModuleGeneric,
        }
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
            Self::ObjectHandle => "object_handle",
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
            | Self::ObjectHandle
            | Self::MixedRuntimeI64OrHandle => "runtime_i64_or_handle",
        }
    }
}

impl GlobalCallDefinitionOwner {
    fn as_json_name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DiagnosticsOnly => "diagnostics_only",
            Self::LeafI64 => "leaf_i64",
            Self::GenericI64OrLeaf => "generic_i64_or_leaf",
            Self::ModuleGeneric => "module_generic",
            Self::UniformMir => "uniform_mir",
            Self::RuntimeHelper => "runtime_helper",
        }
    }

    fn emit_trace_consumer(self) -> &'static str {
        match self {
            Self::DiagnosticsOnly => "mir_call_global_diagnostics_only_emit",
            Self::LeafI64 => "mir_call_global_leaf_emit",
            Self::GenericI64OrLeaf => "mir_call_global_generic_i64_emit",
            Self::ModuleGeneric => "mir_call_global_module_generic_emit",
            Self::UniformMir => "mir_call_global_uniform_mir_emit",
            Self::RuntimeHelper => "mir_call_stage1_emit_program_json_emit",
            Self::None => "mir_call_global_unknown_emit",
        }
    }
}

impl GlobalCallTargetShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NumericI64Leaf => "numeric_i64_leaf",
            Self::GenericPureStringBody => "generic_pure_string_body",
            Self::GenericI64Body => "generic_i64_body",
        }
    }

    pub(super) fn return_contract(self) -> Option<GlobalCallReturnContract> {
        match self {
            Self::NumericI64Leaf | Self::GenericI64Body => {
                Some(GlobalCallReturnContract::ScalarI64)
            }
            Self::GenericPureStringBody => Some(GlobalCallReturnContract::StringHandle),
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
