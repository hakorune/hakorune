use super::type_label::format_mir_type_label;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlobalCallTargetShape {
    #[default]
    Unknown,
    NumericI64Leaf,
    GenericPureStringBody,
    GenericStringOrVoidSentinelBody,
    GenericStringVoidLoggingBody,
    GenericI64Body,
    ParserProgramJsonBody,
    ProgramJsonEmitBody,
    JsonFragInstructionArrayNormalizerBody,
    StaticStringArrayBody,
    BuilderRegistryDispatchBody,
    MirSchemaMapConstructorBody,
    BoxTypeInspectorDescribeBody,
    PatternUtilLocalValueProbeBody,
}

impl GlobalCallTargetShape {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NumericI64Leaf => "numeric_i64_leaf",
            Self::GenericPureStringBody => "generic_pure_string_body",
            Self::GenericStringOrVoidSentinelBody => "generic_string_or_void_sentinel_body",
            Self::GenericStringVoidLoggingBody => "generic_string_void_logging_body",
            Self::GenericI64Body => "generic_i64_body",
            Self::ParserProgramJsonBody => "parser_program_json_body",
            Self::ProgramJsonEmitBody => "program_json_emit_body",
            Self::JsonFragInstructionArrayNormalizerBody => {
                "jsonfrag_instruction_array_normalizer_body"
            }
            Self::StaticStringArrayBody => "static_string_array_body",
            Self::BuilderRegistryDispatchBody => "builder_registry_dispatch_body",
            Self::MirSchemaMapConstructorBody => "mir_schema_map_constructor_body",
            Self::BoxTypeInspectorDescribeBody => "box_type_inspector_describe_body",
            Self::PatternUtilLocalValueProbeBody => "pattern_util_local_value_probe_body",
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
            reason: None,
            blocker: None,
        }
    }

    pub(super) fn unknown(reason: GlobalCallTargetShapeReason) -> Self {
        Self {
            shape: GlobalCallTargetShape::Unknown,
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
            Some(GlobalCallTargetShape::GenericStringVoidLoggingBody) => {
                "typed_global_call_generic_string_void_logging"
            }
            Some(GlobalCallTargetShape::GenericI64Body) => "typed_global_call_generic_i64",
            Some(GlobalCallTargetShape::ParserProgramJsonBody) => {
                "typed_global_call_parser_program_json"
            }
            Some(GlobalCallTargetShape::ProgramJsonEmitBody) => {
                "typed_global_call_program_json_emit"
            }
            Some(GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody) => {
                "typed_global_call_jsonfrag_instruction_array_normalizer"
            }
            Some(GlobalCallTargetShape::StaticStringArrayBody) => {
                "typed_global_call_static_string_array"
            }
            Some(GlobalCallTargetShape::BuilderRegistryDispatchBody) => {
                "typed_global_call_builder_registry_dispatch"
            }
            Some(GlobalCallTargetShape::MirSchemaMapConstructorBody) => {
                "typed_global_call_mir_schema_map_constructor"
            }
            Some(GlobalCallTargetShape::BoxTypeInspectorDescribeBody) => {
                "typed_global_call_box_type_inspector_describe"
            }
            Some(GlobalCallTargetShape::PatternUtilLocalValueProbeBody) => {
                "typed_global_call_pattern_util_local_value_probe"
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
            | Some(GlobalCallTargetShape::GenericStringVoidLoggingBody)
            | Some(GlobalCallTargetShape::GenericI64Body) => "scalar_i64",
            Some(
                GlobalCallTargetShape::GenericPureStringBody
                | GlobalCallTargetShape::GenericStringOrVoidSentinelBody
                | GlobalCallTargetShape::ParserProgramJsonBody
                | GlobalCallTargetShape::ProgramJsonEmitBody
                | GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody
                | GlobalCallTargetShape::StaticStringArrayBody
                | GlobalCallTargetShape::MirSchemaMapConstructorBody
                | GlobalCallTargetShape::BoxTypeInspectorDescribeBody
                | GlobalCallTargetShape::PatternUtilLocalValueProbeBody
                | GlobalCallTargetShape::BuilderRegistryDispatchBody,
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
            Some(GlobalCallTargetShape::GenericStringVoidLoggingBody) => {
                Some("void_sentinel_i64_zero")
            }
            Some(GlobalCallTargetShape::ParserProgramJsonBody) => Some("string_handle"),
            Some(GlobalCallTargetShape::ProgramJsonEmitBody) => Some("string_handle"),
            Some(GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody) => {
                Some("string_handle")
            }
            Some(GlobalCallTargetShape::StaticStringArrayBody) => Some("array_handle"),
            Some(GlobalCallTargetShape::BuilderRegistryDispatchBody) => {
                Some("string_handle_or_null")
            }
            Some(GlobalCallTargetShape::MirSchemaMapConstructorBody) => Some("map_handle"),
            Some(GlobalCallTargetShape::BoxTypeInspectorDescribeBody) => Some("map_handle"),
            Some(GlobalCallTargetShape::PatternUtilLocalValueProbeBody) => {
                Some("mixed_runtime_i64_or_handle")
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
            | GlobalCallTargetShape::GenericStringVoidLoggingBody
            | GlobalCallTargetShape::GenericI64Body
            | GlobalCallTargetShape::ParserProgramJsonBody
            | GlobalCallTargetShape::ProgramJsonEmitBody
            | GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody
            | GlobalCallTargetShape::StaticStringArrayBody
            | GlobalCallTargetShape::MirSchemaMapConstructorBody
            | GlobalCallTargetShape::BoxTypeInspectorDescribeBody
            | GlobalCallTargetShape::PatternUtilLocalValueProbeBody
            | GlobalCallTargetShape::BuilderRegistryDispatchBody => Some(self.target.shape()),
            GlobalCallTargetShape::Unknown => None,
        }
    }
}
