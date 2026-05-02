use crate::mir::core_method_op::CoreMethodOpCarrier;
use crate::mir::generic_method_route_facts::{
    GenericMethodKeyRoute, GenericMethodPublicationPolicy, GenericMethodReturnShape,
    GenericMethodValueDemand,
};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericMethodRouteKind {
    RuntimeDataLoadAny,
    RuntimeDataContainsAny,
    MapLoadAny,
    MapEntryCount,
    ArraySlotLoadAny,
    ArrayContainsAny,
    ArraySlotLen,
    ArrayAppendAny,
    ArrayStoreAny,
    MapStoreAny,
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
        match self {
            Self::RuntimeDataLoadAny | Self::MapLoadAny | Self::ArraySlotLoadAny => {
                "generic_method.get"
            }
            Self::RuntimeDataContainsAny
            | Self::ArrayContainsAny
            | Self::MapContainsAny
            | Self::MapContainsI64 => "generic_method.has",
            Self::MapEntryCount | Self::ArraySlotLen | Self::StringLen => "generic_method.len",
            Self::ArrayAppendAny => "generic_method.push",
            Self::ArrayStoreAny | Self::MapStoreAny => "generic_method.set",
            Self::StringSubstring => "generic_method.substring",
            Self::StringIndexOf => "generic_method.indexOf",
            Self::StringLastIndexOf => "generic_method.lastIndexOf",
            Self::StringContains => "generic_method.contains",
        }
    }

    fn emit_kind(self) -> &'static str {
        match self {
            Self::RuntimeDataLoadAny | Self::MapLoadAny | Self::ArraySlotLoadAny => "get",
            Self::RuntimeDataContainsAny
            | Self::ArrayContainsAny
            | Self::MapContainsAny
            | Self::MapContainsI64 => "has",
            Self::MapEntryCount | Self::ArraySlotLen | Self::StringLen => "len",
            Self::ArrayAppendAny => "push",
            Self::ArrayStoreAny | Self::MapStoreAny => "set",
            Self::StringSubstring => "substring",
            Self::StringIndexOf => "indexOf",
            Self::StringLastIndexOf => "lastIndexOf",
            Self::StringContains => "contains",
        }
    }

    pub(crate) fn helper_symbol(self) -> &'static str {
        match self {
            Self::RuntimeDataLoadAny => "nyash.runtime_data.get_hh",
            Self::RuntimeDataContainsAny => "nyash.runtime_data.has_hh",
            Self::MapLoadAny => "nyash.map.slot_load_hh",
            Self::MapEntryCount => "nyash.map.entry_count_i64",
            Self::ArraySlotLoadAny => "nyash.array.slot_load_hi",
            Self::ArrayContainsAny => "nyash.array.has_hh",
            Self::ArraySlotLen => "nyash.array.slot_len_h",
            Self::ArrayAppendAny => "nyash.array.slot_append_hh",
            Self::ArrayStoreAny => "nyash.array.slot_store_*",
            Self::MapStoreAny => "nyash.map.slot_store_hhh",
            Self::StringLen => "nyash.string.len_h",
            Self::StringSubstring => "nyash.string.substring_hii",
            Self::StringIndexOf => "nyash.string.indexOf_hh",
            Self::StringLastIndexOf => "nyash.string.lastIndexOf_hh",
            Self::StringContains => "nyash.string.contains_hh",
            Self::MapContainsAny => "nyash.map.probe_hh",
            Self::MapContainsI64 => "nyash.map.probe_hi",
        }
    }

    fn effect_tags(self) -> &'static [&'static str] {
        match self {
            Self::RuntimeDataLoadAny | Self::MapLoadAny | Self::ArraySlotLoadAny => &["read.key"],
            Self::RuntimeDataContainsAny
            | Self::ArrayContainsAny
            | Self::MapContainsAny
            | Self::MapContainsI64 => &["probe.key"],
            Self::MapEntryCount | Self::ArraySlotLen | Self::StringLen => &["observe.len"],
            Self::ArrayAppendAny => &["mutate.shape"],
            Self::ArrayStoreAny | Self::MapStoreAny => &["mutate.slot"],
            Self::StringSubstring => &["observe.substring"],
            Self::StringIndexOf => &["observe.indexof"],
            Self::StringLastIndexOf => &["observe.last_indexof"],
            Self::StringContains => &["observe.contains"],
        }
    }

    fn tag(self) -> &'static str {
        match self {
            Self::RuntimeDataLoadAny => "runtime_data_load_any",
            Self::RuntimeDataContainsAny => "runtime_data_contains_any",
            Self::MapLoadAny => "map_load_any",
            Self::MapEntryCount => "map_entry_count",
            Self::ArraySlotLoadAny => "array_slot_load_any",
            Self::ArrayContainsAny => "array_contains_any",
            Self::ArraySlotLen => "array_slot_len",
            Self::ArrayAppendAny => "array_append_any",
            Self::ArrayStoreAny => "array_store_any",
            Self::MapStoreAny => "map_store_any",
            Self::StringLen => "string_len",
            Self::StringSubstring => "string_substring",
            Self::StringIndexOf => "string_indexof",
            Self::StringLastIndexOf => "string_last_indexof",
            Self::StringContains => "string_contains",
            Self::MapContainsAny => "map_contains_any",
            Self::MapContainsI64 => "map_contains_i64",
        }
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
    PushSurfacePolicy,
    SetSurfacePolicy,
    SubstringSurfacePolicy,
    IndexOfSurfacePolicy,
    LastIndexOfSurfacePolicy,
    ContainsSurfacePolicy,
    MapSetScalarI64DominatesNoEscape,
    MapSetScalarI64SameKeyNoEscape,
    MirJsonNumericValueField,
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
            Self::PushSurfacePolicy => "push_surface_policy",
            Self::SetSurfacePolicy => "set_surface_policy",
            Self::SubstringSurfacePolicy => "substring_surface_policy",
            Self::IndexOfSurfacePolicy => "indexof_surface_policy",
            Self::LastIndexOfSurfacePolicy => "lastindexof_surface_policy",
            Self::ContainsSurfacePolicy => "contains_surface_policy",
            Self::MapSetScalarI64DominatesNoEscape => "map_set_scalar_i64_dominates_no_escape",
            Self::MapSetScalarI64SameKeyNoEscape => "map_set_scalar_i64_same_key_no_escape",
            Self::MirJsonNumericValueField => "mir_json_numeric_value_field",
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
        }
    }

    pub(crate) fn with_key_const_text(mut self, text: impl Into<String>) -> Self {
        self.key_const_text = Some(text.into());
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

    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.decision.route_kind.effect_tags()
    }

    #[cfg(test)]
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
