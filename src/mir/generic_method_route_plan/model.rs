use crate::mir::core_method_op::{CoreMethodOpCarrier, LoweringPlanEmitKind, LoweringPlanTier};
use crate::mir::generated::generic_method_route_descriptors::descriptor_for_route_kind;
use crate::mir::generic_method_route_facts::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericMethodRouteKind {
    RuntimeDataLoadAny,
    RuntimeDataContainsAny,
    MapLoadScalarI64,
    MapLoadI64Any,
    MapLoadAny,
    MapEntryCount,
    MapKeysArray,
    AnyLength,
    ArraySlotLoadAny,
    ArrayContainsAny,
    ArraySlotLen,
    ArrayAppendAny,
    ArrayStoreAny,
    MapStoreI64,
    MapStoreAny,
    MapDeleteAny,
    StringLen,
    StringSubstring,
    StringIndexOf,
    StringLastIndexOf,
    StringContains,
    MapContainsAny,
    MapContainsI64,
}

impl GenericMethodRouteKind {
    fn route_id(self) -> &'static str {
        descriptor_for_route_kind(self).route_id
    }

    fn emit_kind(self) -> &'static str {
        descriptor_for_route_kind(self).emit_kind
    }

    pub(crate) fn helper_symbol(self) -> &'static str {
        descriptor_for_route_kind(self).helper_symbol
    }

    fn effect_tags(self) -> &'static [&'static str] {
        descriptor_for_route_kind(self).effects
    }

    fn tag(self) -> &'static str {
        descriptor_for_route_kind(self).tag
    }
}

impl std::fmt::Display for GenericMethodRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.tag())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericMethodRouteProof {
    GetSurfacePolicy,
    HasSurfacePolicy,
    LenSurfacePolicy,
    KeysSurfacePolicy,
    PushSurfacePolicy,
    SetSurfacePolicy,
    DeleteSurfacePolicy,
    SubstringSurfacePolicy,
    IndexOfSurfacePolicy,
    LastIndexOfSurfacePolicy,
    ContainsSurfacePolicy,
    MapSetScalarI64DominatesNoEscape,
    MapSetScalarI64SameKeyNoEscape,
    MapSetScalarI64CoveredDynamicI64KeyNoEscape,
    MirJsonNumericValueField,
    MirJsonConstValueField,
    MirJsonPhiIncomingArrayItem,
    MirJsonPhiIncomingPairScalar,
    MirJsonCalleeField,
    MirJsonVidArrayItem,
    MirJsonEffectsArrayItem,
    MirJsonInstField,
    MirJsonBlockInstArrayItem,
    MirJsonBlockField,
    MirJsonFunctionField,
    MirJsonModuleField,
    MirJsonModuleFunctionArrayItem,
    MirJsonFunctionBlockArrayItem,
    MirJsonParamsArrayItem,
    MirJsonFlagsRecAccess,
    MirJsonFlagsKeys,
}

impl std::fmt::Display for GenericMethodRouteProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.tag())
    }
}

impl GenericMethodRouteProof {
    fn tag(self) -> &'static str {
        match self {
            Self::GetSurfacePolicy => "get_surface_policy",
            Self::HasSurfacePolicy => "has_surface_policy",
            Self::LenSurfacePolicy => "len_surface_policy",
            Self::KeysSurfacePolicy => "keys_surface_policy",
            Self::PushSurfacePolicy => "push_surface_policy",
            Self::SetSurfacePolicy => "set_surface_policy",
            Self::DeleteSurfacePolicy => "delete_surface_policy",
            Self::SubstringSurfacePolicy => "substring_surface_policy",
            Self::IndexOfSurfacePolicy => "indexof_surface_policy",
            Self::LastIndexOfSurfacePolicy => "lastindexof_surface_policy",
            Self::ContainsSurfacePolicy => "contains_surface_policy",
            Self::MapSetScalarI64DominatesNoEscape => "map_set_scalar_i64_dominates_no_escape",
            Self::MapSetScalarI64SameKeyNoEscape => "map_set_scalar_i64_same_key_no_escape",
            Self::MapSetScalarI64CoveredDynamicI64KeyNoEscape => {
                "map_set_scalar_i64_covered_dynamic_i64_key_no_escape"
            }
            Self::MirJsonNumericValueField => "mir_json_numeric_value_field",
            Self::MirJsonConstValueField => "mir_json_const_value_field",
            Self::MirJsonPhiIncomingArrayItem => "mir_json_phi_incoming_array_item",
            Self::MirJsonPhiIncomingPairScalar => "mir_json_phi_incoming_pair_scalar",
            Self::MirJsonCalleeField => "mir_json_callee_field",
            Self::MirJsonVidArrayItem => "mir_json_vid_array_item",
            Self::MirJsonEffectsArrayItem => "mir_json_effects_array_item",
            Self::MirJsonInstField => "mir_json_inst_field",
            Self::MirJsonBlockInstArrayItem => "mir_json_block_inst_array_item",
            Self::MirJsonBlockField => "mir_json_block_field",
            Self::MirJsonFunctionField => "mir_json_function_field",
            Self::MirJsonModuleField => "mir_json_module_field",
            Self::MirJsonModuleFunctionArrayItem => "mir_json_module_function_array_item",
            Self::MirJsonFunctionBlockArrayItem => "mir_json_function_block_array_item",
            Self::MirJsonParamsArrayItem => "mir_json_params_array_item",
            Self::MirJsonFlagsRecAccess => "mir_json_flags_rec_access",
            Self::MirJsonFlagsKeys => "mir_json_flags_keys",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericMethodRouteSurface {
    box_name: String,
    method: String,
    arity: usize,
}

impl GenericMethodRouteSurface {
    pub(crate) fn new(
        box_name: impl Into<String>,
        method: impl Into<String>,
        arity: usize,
    ) -> Self {
        Self {
            box_name: box_name.into(),
            method: method.into(),
            arity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GenericMethodRouteSite {
    block: BasicBlockId,
    instruction_index: usize,
}

impl GenericMethodRouteSite {
    pub(crate) fn new(block: BasicBlockId, instruction_index: usize) -> Self {
        Self {
            block,
            instruction_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericMethodRouteEvidence {
    receiver_origin_box: Option<String>,
    key_route: Option<GenericMethodKeyRoute>,
    key_const_text: Option<String>,
    result_origin_box: Option<String>,
}

impl GenericMethodRouteEvidence {
    pub(crate) fn new(
        receiver_origin_box: Option<String>,
        key_route: Option<GenericMethodKeyRoute>,
    ) -> Self {
        Self {
            receiver_origin_box,
            key_route,
            key_const_text: None,
            result_origin_box: None,
        }
    }

    pub(crate) fn with_key_const_text(mut self, text: impl Into<String>) -> Self {
        self.key_const_text = Some(text.into());
        self
    }

    pub(crate) fn with_result_origin_box(mut self, box_name: Option<String>) -> Self {
        self.result_origin_box = box_name;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GenericMethodRouteOperands {
    receiver_value: ValueId,
    key_value: Option<ValueId>,
    result_value: Option<ValueId>,
}

impl GenericMethodRouteOperands {
    pub(crate) fn new(
        receiver_value: ValueId,
        key_value: Option<ValueId>,
        result_value: Option<ValueId>,
    ) -> Self {
        Self {
            receiver_value,
            key_value,
            result_value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericMethodRouteDecision {
    route_kind: GenericMethodRouteKind,
    proof: GenericMethodRouteProof,
    core_method: Option<CoreMethodOpCarrier>,
    return_shape: Option<GenericMethodReturnShape>,
    value_demand: GenericMethodValueDemand,
    publication_policy: Option<GenericMethodPublicationPolicy>,
}

impl GenericMethodRouteDecision {
    pub(crate) fn new(
        route_kind: GenericMethodRouteKind,
        proof: GenericMethodRouteProof,
        core_method: Option<CoreMethodOpCarrier>,
        return_shape: Option<GenericMethodReturnShape>,
        value_demand: GenericMethodValueDemand,
        publication_policy: Option<GenericMethodPublicationPolicy>,
    ) -> Self {
        Self {
            route_kind,
            proof,
            core_method,
            return_shape,
            value_demand,
            publication_policy,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericMethodRoute {
    site: GenericMethodRouteSite,
    surface: GenericMethodRouteSurface,
    evidence: GenericMethodRouteEvidence,
    operands: GenericMethodRouteOperands,
    decision: GenericMethodRouteDecision,
}

impl GenericMethodRoute {
    pub(crate) fn new(
        site: GenericMethodRouteSite,
        surface: GenericMethodRouteSurface,
        evidence: GenericMethodRouteEvidence,
        operands: GenericMethodRouteOperands,
        decision: GenericMethodRouteDecision,
    ) -> Self {
        Self {
            site,
            surface,
            evidence,
            operands,
            decision,
        }
    }

    pub fn box_name(&self) -> &str {
        self.surface.box_name.as_str()
    }

    pub fn method(&self) -> &str {
        self.surface.method.as_str()
    }

    pub fn route_id(&self) -> &'static str {
        self.decision.route_kind.route_id()
    }

    pub fn emit_kind(&self) -> &'static str {
        self.decision.route_kind.emit_kind()
    }

    pub fn route_kind_tag(&self) -> &'static str {
        self.decision.route_kind.tag()
    }

    pub fn helper_symbol(&self) -> &'static str {
        self.decision.route_kind.helper_symbol()
    }

    pub fn proof_tag(&self) -> &'static str {
        self.decision.proof.tag()
    }

    pub fn arity(&self) -> usize {
        self.surface.arity
    }

    pub fn block(&self) -> BasicBlockId {
        self.site.block
    }

    pub fn instruction_index(&self) -> usize {
        self.site.instruction_index
    }

    pub fn receiver_value(&self) -> ValueId {
        self.operands.receiver_value
    }

    pub fn key_value(&self) -> Option<ValueId> {
        self.operands.key_value
    }

    pub fn result_value(&self) -> Option<ValueId> {
        self.operands.result_value
    }

    pub fn receiver_origin_box(&self) -> Option<&str> {
        self.evidence.receiver_origin_box.as_deref()
    }

    pub fn key_route(&self) -> Option<GenericMethodKeyRoute> {
        self.evidence.key_route
    }

    pub fn key_const_text(&self) -> Option<&str> {
        self.evidence.key_const_text.as_deref()
    }

    pub fn result_origin_box(&self) -> Option<&str> {
        self.evidence.result_origin_box.as_deref()
    }

    pub(crate) fn override_result_origin_box(&mut self, box_name: String) {
        self.evidence.result_origin_box = Some(box_name);
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.decision.route_kind.effect_tags()
    }

    pub(crate) fn route_kind(&self) -> GenericMethodRouteKind {
        self.decision.route_kind
    }

    #[cfg(test)]
    pub(crate) fn proof(&self) -> GenericMethodRouteProof {
        self.decision.proof
    }

    pub fn core_method(&self) -> Option<CoreMethodOpCarrier> {
        self.decision.core_method
    }

    pub fn lowering_tier(&self) -> Option<LoweringPlanTier> {
        self.decision
            .core_method
            .map(|carrier| carrier.lowering_tier.plan_tier())
    }

    pub fn lowering_emit_kind(&self) -> Option<LoweringPlanEmitKind> {
        self.decision
            .core_method
            .map(|carrier| carrier.lowering_tier.plan_emit_kind())
    }

    pub fn return_shape(&self) -> Option<GenericMethodReturnShape> {
        self.decision.return_shape
    }

    pub fn value_demand(&self) -> GenericMethodValueDemand {
        self.decision.value_demand
    }

    pub fn publication_policy(&self) -> Option<GenericMethodPublicationPolicy> {
        self.decision.publication_policy
    }
}
