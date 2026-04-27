/*!
 * MIR-owned route plans for generic method policy.
 *
 * This module owns narrow generic method route-policy decisions so `.inc`
 * codegen can consume pre-decided route tags instead of classifying method
 * surfaces from backend-local strings.
 */

use super::generic_method_route_facts::{
    classify_key_route, const_i64_value, receiver_origin_box_name, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};
use super::{
    build_value_def_map, resolve_value_origin, BasicBlockId, Callee, CoreMethodLoweringTier,
    CoreMethodOp, CoreMethodOpCarrier, MirFunction, MirInstruction, MirModule, ValueDefMap,
    ValueId,
};
use crate::mir::verification::utils::compute_dominators;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteKind {
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
    MapContainsAny,
    MapContainsI64,
}

impl GenericMethodRouteKind {
    pub fn route_id(self) -> &'static str {
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
        }
    }

    pub fn emit_kind(self) -> &'static str {
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
        }
    }

    pub fn helper_symbol(self) -> &'static str {
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
            Self::MapContainsAny => "nyash.map.probe_hh",
            Self::MapContainsI64 => "nyash.map.probe_hi",
        }
    }

    pub fn effect_tags(self) -> &'static [&'static str] {
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
        }
    }
}

impl std::fmt::Display for GenericMethodRouteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeDataLoadAny => f.write_str("runtime_data_load_any"),
            Self::RuntimeDataContainsAny => f.write_str("runtime_data_contains_any"),
            Self::MapLoadAny => f.write_str("map_load_any"),
            Self::MapEntryCount => f.write_str("map_entry_count"),
            Self::ArraySlotLoadAny => f.write_str("array_slot_load_any"),
            Self::ArrayContainsAny => f.write_str("array_contains_any"),
            Self::ArraySlotLen => f.write_str("array_slot_len"),
            Self::ArrayAppendAny => f.write_str("array_append_any"),
            Self::ArrayStoreAny => f.write_str("array_store_any"),
            Self::MapStoreAny => f.write_str("map_store_any"),
            Self::StringLen => f.write_str("string_len"),
            Self::StringSubstring => f.write_str("string_substring"),
            Self::StringIndexOf => f.write_str("string_indexof"),
            Self::MapContainsAny => f.write_str("map_contains_any"),
            Self::MapContainsI64 => f.write_str("map_contains_i64"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodRouteProof {
    GetSurfacePolicy,
    HasSurfacePolicy,
    LenSurfacePolicy,
    PushSurfacePolicy,
    SetSurfacePolicy,
    SubstringSurfacePolicy,
    IndexOfSurfacePolicy,
    MapSetScalarI64DominatesNoEscape,
    MapSetScalarI64SameKeyNoEscape,
}

impl std::fmt::Display for GenericMethodRouteProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetSurfacePolicy => f.write_str("get_surface_policy"),
            Self::HasSurfacePolicy => f.write_str("has_surface_policy"),
            Self::LenSurfacePolicy => f.write_str("len_surface_policy"),
            Self::PushSurfacePolicy => f.write_str("push_surface_policy"),
            Self::SetSurfacePolicy => f.write_str("set_surface_policy"),
            Self::SubstringSurfacePolicy => f.write_str("substring_surface_policy"),
            Self::IndexOfSurfacePolicy => f.write_str("indexof_surface_policy"),
            Self::MapSetScalarI64DominatesNoEscape => {
                f.write_str("map_set_scalar_i64_dominates_no_escape")
            }
            Self::MapSetScalarI64SameKeyNoEscape => {
                f.write_str("map_set_scalar_i64_same_key_no_escape")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericMethodRouteSurface {
    pub box_name: String,
    pub method: String,
    pub arity: usize,
}

impl GenericMethodRouteSurface {
    pub fn new(box_name: impl Into<String>, method: impl Into<String>, arity: usize) -> Self {
        Self {
            box_name: box_name.into(),
            method: method.into(),
            arity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericMethodRouteEvidence {
    pub receiver_origin_box: Option<String>,
    pub key_route: Option<GenericMethodKeyRoute>,
}

impl GenericMethodRouteEvidence {
    pub fn new(
        receiver_origin_box: Option<String>,
        key_route: Option<GenericMethodKeyRoute>,
    ) -> Self {
        Self {
            receiver_origin_box,
            key_route,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericMethodRouteDecision {
    pub route_kind: GenericMethodRouteKind,
    pub proof: GenericMethodRouteProof,
    pub core_method: Option<CoreMethodOpCarrier>,
    pub return_shape: Option<GenericMethodReturnShape>,
    pub value_demand: GenericMethodValueDemand,
    pub publication_policy: Option<GenericMethodPublicationPolicy>,
}

impl GenericMethodRouteDecision {
    pub fn new(
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
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub surface: GenericMethodRouteSurface,
    pub evidence: GenericMethodRouteEvidence,
    pub receiver_value: ValueId,
    pub key_value: Option<ValueId>,
    pub result_value: Option<ValueId>,
    pub decision: GenericMethodRouteDecision,
}

impl GenericMethodRoute {
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

    pub fn arity(&self) -> usize {
        self.surface.arity
    }

    pub fn receiver_origin_box(&self) -> Option<&str> {
        self.evidence.receiver_origin_box.as_deref()
    }

    pub fn key_route(&self) -> Option<GenericMethodKeyRoute> {
        self.evidence.key_route
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.decision.route_kind.effect_tags()
    }

    pub fn route_kind(&self) -> GenericMethodRouteKind {
        self.decision.route_kind
    }

    pub fn proof(&self) -> GenericMethodRouteProof {
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

pub fn refresh_module_generic_method_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_generic_method_routes(function);
    }
}

pub fn refresh_function_generic_method_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let def_map = build_value_def_map(function);
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            if let Some(route) =
                match_generic_has_route(function, &def_map, block_id, instruction_index, inst)
                    .or_else(|| {
                        match_generic_get_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_len_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_substring_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_indexof_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_push_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
                    .or_else(|| {
                        match_generic_set_route(
                            function,
                            &def_map,
                            block_id,
                            instruction_index,
                            inst,
                        )
                    })
            {
                routes.push(route);
            }
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.generic_method_routes = routes;
}

fn match_generic_has_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "has" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| matches!(box_name.as_str(), "ArrayBox" | "MapBox").then(|| box_name.clone()));
    let key_route = classify_key_route(function, def_map, args[0]);
    let (route_kind, core_method) = match box_name.as_str() {
        "ArrayBox" if receiver_origin_box.as_deref() == Some("ArrayBox") => (
            GenericMethodRouteKind::ArrayContainsAny,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
        "MapBox" => (
            map_has_route_kind_for_key(key_route),
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
        "RuntimeDataBox"
            if receiver_origin_box.as_deref() == Some("MapBox")
                && key_route == GenericMethodKeyRoute::I64Const =>
        {
            (
                GenericMethodRouteKind::MapContainsI64,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapHas,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
            )
        }
        "RuntimeDataBox" if receiver_origin_box.as_deref() == Some("ArrayBox") => (
            GenericMethodRouteKind::ArrayContainsAny,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
        ),
        "RuntimeDataBox" => (GenericMethodRouteKind::RuntimeDataContainsAny, None),
        _ => return None,
    };

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        receiver_value: *receiver,
        key_value: Some(args[0]),
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::HasSurfacePolicy,
            core_method,
            None,
            GenericMethodValueDemand::ReadRef,
            None,
        ),
    })
}

fn match_generic_get_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "get" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| matches!(box_name.as_str(), "ArrayBox" | "MapBox").then(|| box_name.clone()));
    let key_route = classify_key_route(function, def_map, args[0]);

    if box_name == "ArrayBox" && receiver_origin_box.as_deref() == Some("ArrayBox") {
        return Some(GenericMethodRoute {
            block,
            instruction_index,
            surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            evidence: GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
            receiver_value: *receiver,
            key_value: Some(args[0]),
            result_value: *dst,
            decision: GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArraySlotLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::ArrayGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
    }

    if box_name == "MapBox" && receiver_origin_box.as_deref() == Some("MapBox") {
        return Some(GenericMethodRoute {
            block,
            instruction_index,
            surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            evidence: GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
            receiver_value: *receiver,
            key_value: Some(args[0]),
            result_value: *dst,
            decision: GenericMethodRouteDecision::new(
                GenericMethodRouteKind::MapLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
    }

    if box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("ArrayBox") {
        return Some(GenericMethodRoute {
            block,
            instruction_index,
            surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
            evidence: GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
            receiver_value: *receiver,
            key_value: Some(args[0]),
            result_value: *dst,
            decision: GenericMethodRouteDecision::new(
                GenericMethodRouteKind::ArraySlotLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::ArrayGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
    }

    if box_name != "RuntimeDataBox" || receiver_origin_box.as_deref() != Some("MapBox") {
        return None;
    }

    let scalar_proof = prove_scalar_i64_map_get_store_fact(
        function,
        def_map,
        block,
        instruction_index,
        *receiver,
        args[0],
    );
    let (proof, return_shape, value_demand, publication_policy) = if let Some(proof) = scalar_proof
    {
        (
            proof.route_proof,
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        )
    } else {
        (
            GenericMethodRouteProof::GetSurfacePolicy,
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            GenericMethodValueDemand::RuntimeI64OrHandle,
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
        )
    };

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        receiver_value: *receiver,
        key_value: Some(args[0]),
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            proof,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            return_shape,
            value_demand,
            publication_policy,
        ),
    })
}

fn match_generic_len_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if !is_len_method(method) || !args.is_empty() {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| len_surface_origin_box_name(box_name).map(str::to_string));
    let (route_kind, core_op) =
        match len_surface_origin_box_name(box_name).or(receiver_origin_box.as_deref()) {
            Some("MapBox") => (GenericMethodRouteKind::MapEntryCount, CoreMethodOp::MapLen),
            Some("ArrayBox") => (GenericMethodRouteKind::ArraySlotLen, CoreMethodOp::ArrayLen),
            Some("StringBox") => (GenericMethodRouteKind::StringLen, CoreMethodOp::StringLen),
            _ => return None,
        };

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 0),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, None),
        receiver_value: *receiver,
        key_value: None,
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::LenSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                core_op,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    })
}

fn match_generic_substring_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "substring" || !(args.len() == 1 || args.len() == 2) {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "StringBox").then(|| "StringBox".to_string()));
    if box_name != "StringBox"
        && !(box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("StringBox"))
    {
        return None;
    }

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, None),
        receiver_value: *receiver,
        key_value: None,
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            GenericMethodRouteKind::StringSubstring,
            GenericMethodRouteProof::SubstringSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::StringSubstring,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            None,
            GenericMethodValueDemand::ReadRef,
            None,
        ),
    })
}

fn match_generic_indexof_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "indexOf" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "StringBox").then(|| "StringBox".to_string()));
    if box_name != "StringBox"
        && !(box_name == "RuntimeDataBox" && receiver_origin_box.as_deref() == Some("StringBox"))
    {
        return None;
    }

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), args.len()),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, None),
        receiver_value: *receiver,
        key_value: None,
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            GenericMethodRouteKind::StringIndexOf,
            GenericMethodRouteProof::IndexOfSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::StringIndexOf,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    })
}

fn match_generic_push_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "push" || args.len() != 1 {
        return None;
    }

    let receiver_origin_box = receiver_origin_box_name(function, def_map, *receiver)
        .or_else(|| (box_name == "ArrayBox").then(|| "ArrayBox".to_string()));
    if receiver_origin_box.as_deref() != Some("ArrayBox")
        || !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox")
    {
        return None;
    }

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 1),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, None),
        receiver_value: *receiver,
        key_value: None,
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            GenericMethodRouteKind::ArrayAppendAny,
            GenericMethodRouteProof::PushSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::ArrayPush,
                CoreMethodLoweringTier::ColdFallback,
            )),
            Some(GenericMethodReturnShape::ScalarI64),
            GenericMethodValueDemand::WriteAny,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    })
}

fn match_generic_set_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> Option<GenericMethodRoute> {
    let MirInstruction::Call {
        dst,
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "set" || args.len() != 2 {
        return None;
    }

    let (route_kind, core_op) = match box_name.as_str() {
        "ArrayBox" => (
            GenericMethodRouteKind::ArrayStoreAny,
            CoreMethodOp::ArraySet,
        ),
        "MapBox" => (GenericMethodRouteKind::MapStoreAny, CoreMethodOp::MapSet),
        _ => return None,
    };
    let receiver_origin_box =
        receiver_origin_box_name(function, def_map, *receiver).or_else(|| Some(box_name.clone()));
    let key_route = classify_key_route(function, def_map, args[0]);

    Some(GenericMethodRoute {
        block,
        instruction_index,
        surface: GenericMethodRouteSurface::new(box_name.clone(), method.clone(), 2),
        evidence: GenericMethodRouteEvidence::new(receiver_origin_box, Some(key_route)),
        receiver_value: *receiver,
        key_value: Some(args[0]),
        result_value: *dst,
        decision: GenericMethodRouteDecision::new(
            route_kind,
            GenericMethodRouteProof::SetSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                core_op,
                CoreMethodLoweringTier::ColdFallback,
            )),
            None,
            GenericMethodValueDemand::WriteAny,
            None,
        ),
    })
}

fn is_len_method(method: &str) -> bool {
    matches!(method, "len" | "length" | "size")
}

fn len_surface_origin_box_name(box_name: &str) -> Option<&'static str> {
    match box_name {
        "MapBox" => Some("MapBox"),
        "ArrayBox" => Some("ArrayBox"),
        "StringBox" => Some("StringBox"),
        _ => None,
    }
}

fn map_has_route_kind_for_key(key_route: GenericMethodKeyRoute) -> GenericMethodRouteKind {
    match key_route {
        GenericMethodKeyRoute::I64Const => GenericMethodRouteKind::MapContainsI64,
        GenericMethodKeyRoute::UnknownAny => GenericMethodRouteKind::MapContainsAny,
    }
}

#[derive(Clone, Copy)]
struct MapSetCallShape {
    receiver: ValueId,
    key: ValueId,
    value: ValueId,
}

#[derive(Clone, Copy)]
struct MapSetCandidate {
    block: BasicBlockId,
    instruction_index: usize,
    stored_value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScalarI64MapGetStoreFact {
    pub route_proof: GenericMethodRouteProof,
    pub stored_value: i64,
}

pub(crate) fn prove_scalar_i64_map_get_store_fact(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<ScalarI64MapGetStoreFact> {
    if let Some(stored_value) = prove_same_block_scalar_i64_map_get(
        function,
        def_map,
        block_id,
        get_instruction_index,
        get_receiver,
        get_key,
    ) {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
            stored_value,
        });
    }
    if let Some(stored_value) =
        prove_dominating_scalar_i64_map_get(function, def_map, block_id, get_receiver, get_key)
    {
        return Some(ScalarI64MapGetStoreFact {
            route_proof: GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape,
            stored_value,
        });
    }
    None
}

fn prove_same_block_scalar_i64_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block_id: BasicBlockId,
    get_instruction_index: usize,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<i64> {
    let Some(get_key_const) = const_i64_value(function, def_map, get_key) else {
        return None;
    };
    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let Some(block) = function.blocks.get(&block_id) else {
        return None;
    };

    for inst in block.instructions.iter().take(get_instruction_index).rev() {
        if let Some(set_call) = map_set_call_shape(inst) {
            if same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                let Some(set_key_const) = const_i64_value(function, def_map, set_call.key) else {
                    return None;
                };
                if set_key_const != get_key_const {
                    return None;
                }
                return const_i64_value(function, def_map, set_call.value);
            }
        }

        if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
            return None;
        }
    }

    None
}

fn prove_dominating_scalar_i64_map_get(
    function: &MirFunction,
    def_map: &ValueDefMap,
    get_block_id: BasicBlockId,
    get_receiver: ValueId,
    get_key: ValueId,
) -> Option<i64> {
    let Some(get_key_const) = const_i64_value(function, def_map, get_key) else {
        return None;
    };
    let receiver_root = resolve_value_origin(function, def_map, get_receiver);
    let dominators = compute_dominators(function);
    let mut candidates = Vec::new();

    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if block_id == get_block_id || !dominators.dominates(block_id, get_block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let Some(set_call) = map_set_call_shape(inst) else {
                continue;
            };
            if !same_value_origin(function, def_map, set_call.receiver, receiver_root) {
                continue;
            }
            if const_i64_value(function, def_map, set_call.key) != Some(get_key_const) {
                continue;
            }
            let Some(stored_value) = const_i64_value(function, def_map, set_call.value) else {
                continue;
            };
            candidates.push(MapSetCandidate {
                block: block_id,
                instruction_index,
                stored_value,
            });
        }
    }

    candidates.into_iter().rev().find_map(|candidate| {
        dominating_candidate_has_no_same_receiver_escape(
            function,
            def_map,
            &dominators,
            candidate,
            receiver_root,
        )
        .then_some(candidate.stored_value)
    })
}

fn dominating_candidate_has_no_same_receiver_escape(
    function: &MirFunction,
    def_map: &ValueDefMap,
    dominators: &crate::mir::verification::utils::DominatorTree,
    candidate: MapSetCandidate,
    receiver_root: ValueId,
) -> bool {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        if !dominators.dominates(candidate.block, block_id) {
            continue;
        }
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        let start = if block_id == candidate.block {
            candidate.instruction_index + 1
        } else {
            0
        };
        for inst in block.instructions.iter().skip(start) {
            if instruction_may_escape_or_mutate_receiver(function, def_map, inst, receiver_root) {
                return false;
            }
        }
    }
    true
}

fn map_set_call_shape(inst: &MirInstruction) -> Option<MapSetCallShape> {
    let MirInstruction::Call {
        callee:
            Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                ..
            }),
        args,
        ..
    } = inst
    else {
        return None;
    };
    if method != "set" || !matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox") {
        return None;
    }

    let (key, value) = match args.as_slice() {
        [key, value] => (*key, *value),
        // Some source routes still carry the receiver as the first argument.
        [_receiver_arg, key, value] => (*key, *value),
        _ => return None,
    };
    Some(MapSetCallShape {
        receiver: *receiver,
        key,
        value,
    })
}

pub(crate) fn instruction_may_escape_or_mutate_receiver(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    receiver_root: ValueId,
) -> bool {
    if !instruction_uses_origin(function, def_map, inst, receiver_root) {
        return false;
    }

    match inst {
        MirInstruction::Copy { .. } | MirInstruction::KeepAlive { .. } => false,
        MirInstruction::Call {
            callee:
                Some(Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            ..
        } if same_value_origin(function, def_map, *receiver, receiver_root)
            && matches!(method.as_str(), "get" | "has") =>
        {
            false
        }
        _ => true,
    }
}

fn instruction_uses_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    inst: &MirInstruction,
    origin: ValueId,
) -> bool {
    inst.used_values()
        .into_iter()
        .any(|value| same_value_origin(function, def_map, value, origin))
}

fn same_value_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    origin: ValueId,
) -> bool {
    resolve_value_origin(function, def_map, value) == origin
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<u32>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: args.into_iter().map(ValueId::new).collect(),
            effects: EffectMask::PURE,
        }
    }

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn route_for<'a>(
        function: &'a MirFunction,
        box_name: &str,
        method: &str,
        result: Option<u32>,
    ) -> &'a GenericMethodRoute {
        let result_value = result.map(ValueId::new);
        function
            .metadata
            .generic_method_routes
            .iter()
            .find(|route| {
                route.box_name() == box_name
                    && route.method() == method
                    && route.result_value == result_value
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing generic method route box={box_name} method={method} result={result_value:?}"
                )
            })
    }

    #[test]
    fn generic_method_route_metadata_tokens_come_from_route_kind() {
        let route = GenericMethodRoute {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            surface: GenericMethodRouteSurface::new(
                "MapBox",
                "__raw_method_must_not_drive_metadata",
                1,
            ),
            evidence: GenericMethodRouteEvidence::new(
                Some("MapBox".to_string()),
                Some(GenericMethodKeyRoute::I64Const),
            ),
            receiver_value: ValueId::new(1),
            key_value: Some(ValueId::new(2)),
            result_value: Some(ValueId::new(3)),
            decision: GenericMethodRouteDecision::new(
                GenericMethodRouteKind::MapContainsI64,
                GenericMethodRouteProof::HasSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapHas,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        };

        assert_eq!(route.route_id(), "generic_method.has");
        assert_eq!(route.emit_kind(), "has");
        assert_eq!(route.effect_tags(), &["probe.key"]);
    }

    #[test]
    fn detects_mapbox_has_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.block, BasicBlockId::new(0));
        assert_eq!(route.instruction_index, 0);
        assert_eq!(route.box_name(), "MapBox");
        assert_eq!(route.method(), "has");
        assert_eq!(route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(route.receiver_value, ValueId::new(1));
        assert_eq!(route.key_value, Some(ValueId::new(2)));
        assert_eq!(route.result_value, Some(ValueId::new(3)));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::MapContainsAny);
        assert_eq!(route.proof(), GenericMethodRouteProof::HasSurfacePolicy);
        let core_method = route.core_method().expect("MapBox.has core method op");
        assert_eq!(core_method.op, CoreMethodOp::MapHas);
        assert_eq!(
            core_method.proof.to_string(),
            "core_method_contract_manifest"
        );
        assert_eq!(core_method.lowering_tier.to_string(), "warm_direct_abi");
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_direct_arraybox_has_as_arrayhas_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "ArrayBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "ArrayBox");
        assert_eq!(route.method(), "has");
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayContainsAny);
        assert_eq!(route.route_kind().helper_symbol(), "nyash.array.has_hh");
        assert_eq!(route.proof(), GenericMethodRouteProof::HasSurfacePolicy);
        let core_method = route.core_method().expect("ArrayBox.has core method op");
        assert_eq!(core_method.op, CoreMethodOp::ArrayHas);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_direct_mapbox_get_as_warm_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.get");
        assert_eq!(route.box_name(), "MapBox");
        assert_eq!(route.method(), "get");
        assert_eq!(route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::MapLoadAny);
        assert_eq!(route.route_kind().helper_symbol(), "nyash.map.slot_load_hh");
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        let core_method = route.core_method().expect("MapBox.get core method op");
        assert_eq!(core_method.op, CoreMethodOp::MapGet);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_direct_arraybox_get_as_warm_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "ArrayBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.get");
        assert_eq!(route.box_name(), "ArrayBox");
        assert_eq!(route.method(), "get");
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
        assert_eq!(
            route.route_kind().helper_symbol(),
            "nyash.array.slot_load_hi"
        );
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        let core_method = route.core_method().expect("ArrayBox.get core method op");
        assert_eq!(core_method.op, CoreMethodOp::ArrayGet);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_runtime_data_arraybox_push_as_cold_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(3), "RuntimeDataBox", "push", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.push");
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "push");
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.key_route(), None);
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
        assert_eq!(
            route.route_kind().helper_symbol(),
            "nyash.array.slot_append_hh"
        );
        assert_eq!(route.proof(), GenericMethodRouteProof::PushSurfacePolicy);
        let core_method = route
            .core_method()
            .expect("RuntimeDataBox Array-origin push core method op");
        assert_eq!(core_method.op, CoreMethodOp::ArrayPush);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::ColdFallback
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64)
        );
        assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
    }

    #[test]
    fn records_runtime_data_arraybox_get_as_warm_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.get");
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "get");
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
        assert_eq!(
            route.route_kind().helper_symbol(),
            "nyash.array.slot_load_hi"
        );
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        let core_method = route
            .core_method()
            .expect("RuntimeDataBox Array-origin get core method op");
        assert_eq!(core_method.op, CoreMethodOp::ArrayGet);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_runtime_data_has_mapbox_receiver_origin_without_promotion() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 2, vec![3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(
            route.route_kind(),
            GenericMethodRouteKind::RuntimeDataContainsAny
        );
        assert!(route.core_method().is_none());
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_runtime_data_arraybox_has_as_arrayhas_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 2, vec![3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "has");
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayContainsAny);
        assert_eq!(route.route_kind().helper_symbol(), "nyash.array.has_hh");
        let core_method = route.core_method().expect("ArrayHas carrier");
        assert_eq!(core_method.op, CoreMethodOp::ArrayHas);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_i64_const_key_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 3, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        assert_eq!(
            function.metadata.generic_method_routes[0].key_route(),
            Some(GenericMethodKeyRoute::I64Const)
        );
    }

    #[test]
    fn records_direct_len_family_core_method_routes() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(method_call(Some(4), "MapBox", "size", 1, vec![]));
        block.add_instruction(method_call(Some(5), "ArrayBox", "length", 2, vec![]));
        block.add_instruction(method_call(Some(6), "StringBox", "len", 3, vec![]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 3);
        let map_route = &function.metadata.generic_method_routes[0];
        assert_eq!(map_route.route_id(), "generic_method.len");
        assert_eq!(map_route.method(), "size");
        assert_eq!(map_route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(map_route.key_route(), None);
        assert_eq!(map_route.key_value, None);
        assert_eq!(
            map_route.route_kind(),
            GenericMethodRouteKind::MapEntryCount
        );
        assert_eq!(map_route.proof(), GenericMethodRouteProof::LenSurfacePolicy);
        let map_core = map_route.core_method().expect("MapLen carrier");
        assert_eq!(map_core.op, CoreMethodOp::MapLen);
        assert_eq!(
            map_core.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(
            map_route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64)
        );
        assert_eq!(
            map_route.value_demand(),
            GenericMethodValueDemand::ScalarI64
        );
        assert_eq!(
            map_route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );

        let array_route = &function.metadata.generic_method_routes[1];
        assert_eq!(array_route.method(), "length");
        assert_eq!(array_route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(
            array_route.route_kind(),
            GenericMethodRouteKind::ArraySlotLen
        );
        let array_core = array_route.core_method().expect("ArrayLen carrier");
        assert_eq!(array_core.op, CoreMethodOp::ArrayLen);

        let string_route = &function.metadata.generic_method_routes[2];
        assert_eq!(string_route.method(), "len");
        assert_eq!(string_route.receiver_origin_box(), Some("StringBox"));
        assert_eq!(string_route.route_kind(), GenericMethodRouteKind::StringLen);
        let string_core = string_route.core_method().expect("StringLen carrier");
        assert_eq!(string_core.op, CoreMethodOp::StringLen);
    }

    #[test]
    fn records_runtime_data_len_from_receiver_origin() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(3), "RuntimeDataBox", "length", 2, vec![]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "length");
        assert_eq!(route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::MapEntryCount);
        let core_method = route.core_method().expect("RuntimeData MapLen carrier");
        assert_eq!(core_method.op, CoreMethodOp::MapLen);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.key_route(), None);
        assert_eq!(route.key_value, None);
        assert_eq!(route.arity(), 0);
    }

    #[test]
    fn records_direct_substring_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(method_call(
            Some(5),
            "StringBox",
            "substring",
            1,
            vec![2, 3],
        ));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.substring");
        assert_eq!(route.box_name(), "StringBox");
        assert_eq!(route.method(), "substring");
        assert_eq!(route.arity(), 2);
        assert_eq!(route.receiver_origin_box(), Some("StringBox"));
        assert_eq!(route.key_route(), None);
        assert_eq!(route.key_value, None);
        assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
        assert_eq!(
            route.proof(),
            GenericMethodRouteProof::SubstringSurfacePolicy
        );
        let core_method = route.core_method().expect("StringSubstring carrier");
        assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_runtime_data_substring_from_string_origin() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "StringBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(
            Some(5),
            "RuntimeDataBox",
            "substring",
            2,
            vec![3, 4],
        ));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "substring");
        assert_eq!(route.arity(), 2);
        assert_eq!(route.receiver_origin_box(), Some("StringBox"));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
        let core_method = route
            .core_method()
            .expect("RuntimeData StringSubstring carrier");
        assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
    }

    #[test]
    fn records_direct_indexof_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(method_call(Some(5), "StringBox", "indexOf", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.indexOf");
        assert_eq!(route.box_name(), "StringBox");
        assert_eq!(route.method(), "indexOf");
        assert_eq!(route.arity(), 1);
        assert_eq!(route.receiver_origin_box(), Some("StringBox"));
        assert_eq!(route.key_route(), None);
        assert_eq!(route.key_value, None);
        assert_eq!(route.route_kind(), GenericMethodRouteKind::StringIndexOf);
        assert_eq!(route.proof(), GenericMethodRouteProof::IndexOfSurfacePolicy);
        let core_method = route.core_method().expect("StringIndexOf carrier");
        assert_eq!(core_method.op, CoreMethodOp::StringIndexOf);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64)
        );
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
    }

    #[test]
    fn records_runtime_data_indexof_from_string_origin() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "StringBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(
            Some(5),
            "RuntimeDataBox",
            "indexOf",
            2,
            vec![3],
        ));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.indexOf");
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "indexOf");
        assert_eq!(route.arity(), 1);
        assert_eq!(route.receiver_origin_box(), Some("StringBox"));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::StringIndexOf);
        let core_method = route
            .core_method()
            .expect("RuntimeData StringIndexOf carrier");
        assert_eq!(core_method.op, CoreMethodOp::StringIndexOf);
    }

    #[test]
    fn records_direct_array_push_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(method_call(Some(4), "ArrayBox", "push", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.push");
        assert_eq!(route.box_name(), "ArrayBox");
        assert_eq!(route.method(), "push");
        assert_eq!(route.arity(), 1);
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.key_route(), None);
        assert_eq!(route.key_value, None);
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
        assert_eq!(route.proof(), GenericMethodRouteProof::PushSurfacePolicy);
        let core_method = route.core_method().expect("ArrayPush carrier");
        assert_eq!(core_method.op, CoreMethodOp::ArrayPush);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::ColdFallback
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64)
        );
        assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
    }

    #[test]
    fn records_runtime_data_arraybox_push_through_copy_as_cold_core_method_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "push", 2, vec![3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.route_id(), "generic_method.push");
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "push");
        assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
        assert_eq!(route.proof(), GenericMethodRouteProof::PushSurfacePolicy);
        let core_method = route
            .core_method()
            .expect("RuntimeDataBox Array-origin push core method op");
        assert_eq!(core_method.op, CoreMethodOp::ArrayPush);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::ColdFallback
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64)
        );
        assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
    }

    #[test]
    fn records_direct_array_and_map_set_core_method_routes() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::Integer(0),
        });
        block.add_instruction(method_call(Some(5), "ArrayBox", "set", 2, vec![1, 3]));
        block.add_instruction(method_call(Some(6), "MapBox", "set", 4, vec![1, 3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 2);
        let array_route = &function.metadata.generic_method_routes[0];
        assert_eq!(array_route.route_id(), "generic_method.set");
        assert_eq!(array_route.box_name(), "ArrayBox");
        assert_eq!(array_route.method(), "set");
        assert_eq!(array_route.arity(), 2);
        assert_eq!(array_route.receiver_origin_box(), Some("ArrayBox"));
        assert_eq!(
            array_route.key_route(),
            Some(GenericMethodKeyRoute::I64Const)
        );
        assert_eq!(array_route.key_value, Some(ValueId::new(1)));
        assert_eq!(
            array_route.route_kind(),
            GenericMethodRouteKind::ArrayStoreAny
        );
        assert_eq!(
            array_route.proof(),
            GenericMethodRouteProof::SetSurfacePolicy
        );
        let array_core = array_route.core_method().expect("ArraySet carrier");
        assert_eq!(array_core.op, CoreMethodOp::ArraySet);
        assert_eq!(
            array_core.lowering_tier,
            CoreMethodLoweringTier::ColdFallback
        );
        assert_eq!(array_route.return_shape(), None);
        assert_eq!(
            array_route.value_demand(),
            GenericMethodValueDemand::WriteAny
        );
        assert_eq!(array_route.publication_policy(), None);

        let map_route = &function.metadata.generic_method_routes[1];
        assert_eq!(map_route.box_name(), "MapBox");
        assert_eq!(map_route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(map_route.key_route(), Some(GenericMethodKeyRoute::I64Const));
        assert_eq!(map_route.route_kind(), GenericMethodRouteKind::MapStoreAny);
        let map_core = map_route.core_method().expect("MapSet carrier");
        assert_eq!(map_core.op, CoreMethodOp::MapSet);
        assert_eq!(map_core.lowering_tier, CoreMethodLoweringTier::ColdFallback);
        assert_eq!(map_route.return_shape(), None);
        assert_eq!(map_route.value_demand(), GenericMethodValueDemand::WriteAny);
        assert_eq!(map_route.publication_policy(), None);
    }

    #[test]
    fn leaves_runtime_data_set_metadata_absent_for_fallback() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "set", 2, vec![3, 4]));

        refresh_function_generic_method_routes(&mut function);

        assert!(function.metadata.generic_method_routes.is_empty());
    }

    #[test]
    fn promotes_runtime_data_mapbox_i64_has_to_map_contains_i64() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        });
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "has", 1, vec![3]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
        assert_eq!(route.route_kind(), GenericMethodRouteKind::MapContainsI64);
        let core_method = route.core_method().expect("MapHas carrier");
        assert_eq!(core_method.op, CoreMethodOp::MapHas);
        assert_eq!(route.route_kind().helper_symbol(), "nyash.map.probe_hi");
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.publication_policy(), None);
    }

    #[test]
    fn records_runtime_data_mapbox_get_as_cold_metadata_only() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "get");
        assert_eq!(route.receiver_origin_box(), Some("MapBox"));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
        assert_eq!(
            route.route_kind(),
            GenericMethodRouteKind::RuntimeDataLoadAny
        );
        assert_eq!(
            route.route_kind().helper_symbol(),
            "nyash.runtime_data.get_hh"
        );
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        let core_method = route.core_method().expect("MapGet carrier");
        assert_eq!(core_method.op, CoreMethodOp::MapGet);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::ColdFallback
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
        assert_eq!(
            route.value_demand(),
            GenericMethodValueDemand::RuntimeI64OrHandle
        );
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        );
    }

    #[test]
    fn proves_same_block_runtime_data_get_scalar_i64_return_shape() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![1, 2, 3]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        assert_eq!(route.box_name(), "RuntimeDataBox");
        assert_eq!(route.method(), "get");
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
        assert_eq!(
            route.proof(),
            GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
        );
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
        assert_eq!(
            route.route_kind().helper_symbol(),
            "nyash.runtime_data.get_hh"
        );
    }

    #[test]
    fn rejects_same_block_get_scalar_shape_when_store_value_is_not_i64() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "StringBox".to_string(),
            args: vec![],
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 2);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
        assert_eq!(
            route.value_demand(),
            GenericMethodValueDemand::RuntimeI64OrHandle
        );
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        );
    }

    #[test]
    fn rejects_same_block_get_scalar_shape_after_unknown_same_receiver_mutation() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(None, "MapBox", "clear", 1, vec![]));
        block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 2);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        );
    }

    #[test]
    fn rejects_same_block_get_scalar_shape_after_different_key_same_receiver_set() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(4),
            value: crate::mir::ConstValue::Integer(2),
        });
        block.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
        block.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![4, 3]));
        block.add_instruction(method_call(Some(7), "RuntimeDataBox", "get", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 3);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(7));
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
    }

    #[test]
    fn proves_dominating_preheader_scalar_i64_map_get_return_shape() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
        function.add_block(body);

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 2);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
        assert_eq!(
            route.proof(),
            GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape
        );
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
        );
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
        assert_eq!(
            route.route_kind().helper_symbol(),
            "nyash.runtime_data.get_hh"
        );
    }

    #[test]
    fn rejects_dominating_preheader_scalar_shape_after_body_mutation() {
        let mut function = make_function();
        let entry_id = BasicBlockId::new(0);
        let body_id = BasicBlockId::new(1);
        let entry = function.blocks.get_mut(&entry_id).expect("entry");
        entry.successors.insert(body_id);
        entry.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "MapBox".to_string(),
            args: vec![],
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: crate::mir::ConstValue::Integer(-1),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(3),
            value: crate::mir::ConstValue::Integer(7),
        });
        entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

        let mut body = BasicBlock::new(body_id);
        body.predecessors.insert(entry_id);
        body.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
        body.add_instruction(method_call(None, "MapBox", "clear", 1, vec![]));
        function.add_block(body);

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 2);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
        assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
        assert_eq!(
            route.return_shape(),
            Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
        );
        assert_eq!(
            route.publication_policy(),
            Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
        );
    }

    #[test]
    fn detects_runtime_data_has_route() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "RuntimeDataBox", "has", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        assert_eq!(
            function.metadata.generic_method_routes[0].route_kind(),
            GenericMethodRouteKind::RuntimeDataContainsAny
        );
        assert!(function.metadata.generic_method_routes[0]
            .core_method()
            .is_none());
        assert_eq!(
            function.metadata.generic_method_routes[0].return_shape(),
            None
        );
        assert_eq!(
            function.metadata.generic_method_routes[0].publication_policy(),
            None
        );
    }

    #[test]
    fn rejects_unknown_generic_method_surface() {
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), "MapBox", "unknown", 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert!(function.metadata.generic_method_routes.is_empty());
    }
}
