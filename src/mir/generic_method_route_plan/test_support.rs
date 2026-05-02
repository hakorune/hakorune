use super::*;

fn site(block: u32, instruction_index: usize) -> GenericMethodRouteSite {
    GenericMethodRouteSite::new(BasicBlockId::new(block), instruction_index)
}

fn evidence(
    receiver_origin_box: Option<&str>,
    key_route: Option<GenericMethodKeyRoute>,
) -> GenericMethodRouteEvidence {
    GenericMethodRouteEvidence::new(receiver_origin_box.map(str::to_string), key_route)
}

fn operands(receiver: u32, key: Option<u32>, result: u32) -> GenericMethodRouteOperands {
    GenericMethodRouteOperands::new(
        ValueId::new(receiver),
        key.map(ValueId::new),
        Some(ValueId::new(result)),
    )
}

fn decision(
    route_kind: GenericMethodRouteKind,
    proof: GenericMethodRouteProof,
    core_op: CoreMethodOp,
    lowering_tier: CoreMethodLoweringTier,
    return_shape: Option<GenericMethodReturnShape>,
    value_demand: GenericMethodValueDemand,
    publication_policy: Option<GenericMethodPublicationPolicy>,
) -> GenericMethodRouteDecision {
    GenericMethodRouteDecision::new(
        route_kind,
        proof,
        Some(CoreMethodOpCarrier::manifest(core_op, lowering_tier)),
        return_shape,
        value_demand,
        publication_policy,
    )
}

struct FixtureSpec<'a> {
    block: u32,
    instruction_index: usize,
    box_name: &'a str,
    method: &'a str,
    arity: usize,
    receiver_origin_box: Option<&'a str>,
    key_route: Option<GenericMethodKeyRoute>,
    receiver: u32,
    key: Option<u32>,
    result: u32,
    route_kind: GenericMethodRouteKind,
    proof: GenericMethodRouteProof,
    core_op: CoreMethodOp,
    lowering_tier: CoreMethodLoweringTier,
    return_shape: Option<GenericMethodReturnShape>,
    value_demand: GenericMethodValueDemand,
    publication_policy: Option<GenericMethodPublicationPolicy>,
}

fn route(spec: FixtureSpec<'_>) -> GenericMethodRoute {
    GenericMethodRoute::new(
        site(spec.block, spec.instruction_index),
        GenericMethodRouteSurface::new(spec.box_name, spec.method, spec.arity),
        evidence(spec.receiver_origin_box, spec.key_route),
        operands(spec.receiver, spec.key, spec.result),
        decision(
            spec.route_kind,
            spec.proof,
            spec.core_op,
            spec.lowering_tier,
            spec.return_shape,
            spec.value_demand,
            spec.publication_policy,
        ),
    )
}

pub(crate) fn map_contains_i64(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "MapBox",
        method: "has",
        arity: 1,
        receiver_origin_box: Some("MapBox"),
        key_route: Some(GenericMethodKeyRoute::I64Const),
        receiver,
        key: Some(key),
        result,
        route_kind: GenericMethodRouteKind::MapContainsI64,
        proof: GenericMethodRouteProof::HasSurfacePolicy,
        core_op: CoreMethodOp::MapHas,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
        return_shape: None,
        value_demand: GenericMethodValueDemand::ReadRef,
        publication_policy: None,
    })
}

pub(crate) fn runtime_data_map_get_mixed_i64_key(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "RuntimeDataBox",
        method: "get",
        arity: 1,
        receiver_origin_box: Some("MapBox"),
        key_route: Some(GenericMethodKeyRoute::I64Const),
        receiver,
        key: Some(key),
        result,
        route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
        proof: GenericMethodRouteProof::GetSurfacePolicy,
        core_op: CoreMethodOp::MapGet,
        lowering_tier: CoreMethodLoweringTier::ColdFallback,
        return_shape: Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
        value_demand: GenericMethodValueDemand::RuntimeI64OrHandle,
        publication_policy: Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
    })
}

pub(crate) fn runtime_data_map_get_scalar_i64_same_key(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "RuntimeDataBox",
        method: "get",
        arity: 1,
        receiver_origin_box: Some("MapBox"),
        key_route: Some(GenericMethodKeyRoute::I64Const),
        receiver,
        key: Some(key),
        result,
        route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
        proof: GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
        core_op: CoreMethodOp::MapGet,
        lowering_tier: CoreMethodLoweringTier::ColdFallback,
        return_shape: Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
        value_demand: GenericMethodValueDemand::ScalarI64,
        publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
    })
}

pub(crate) fn mir_json_numeric_value_field_get(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    GenericMethodRoute::new(
        site(block, instruction_index),
        GenericMethodRouteSurface::new("RuntimeDataBox", "get", 1),
        evidence(None, Some(GenericMethodKeyRoute::UnknownAny)).with_key_const_text("value"),
        operands(receiver, Some(key), result),
        decision(
            GenericMethodRouteKind::RuntimeDataLoadAny,
            GenericMethodRouteProof::MirJsonNumericValueField,
            CoreMethodOp::MapGet,
            CoreMethodLoweringTier::ColdFallback,
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        ),
    )
}

pub(crate) fn string_substring(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "StringBox",
        method: "substring",
        arity: 2,
        receiver_origin_box: Some("StringBox"),
        key_route: None,
        receiver,
        key: None,
        result,
        route_kind: GenericMethodRouteKind::StringSubstring,
        proof: GenericMethodRouteProof::SubstringSurfacePolicy,
        core_op: CoreMethodOp::StringSubstring,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
        return_shape: None,
        value_demand: GenericMethodValueDemand::ReadRef,
        publication_policy: None,
    })
}

pub(crate) fn map_len(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "MapBox",
        method: "size",
        arity: 0,
        receiver_origin_box: Some("MapBox"),
        key_route: None,
        receiver,
        key: None,
        result,
        route_kind: GenericMethodRouteKind::MapEntryCount,
        proof: GenericMethodRouteProof::LenSurfacePolicy,
        core_op: CoreMethodOp::MapLen,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
        return_shape: Some(GenericMethodReturnShape::ScalarI64),
        value_demand: GenericMethodValueDemand::ScalarI64,
        publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
    })
}

pub(crate) fn array_push(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "ArrayBox",
        method: "push",
        arity: 1,
        receiver_origin_box: Some("ArrayBox"),
        key_route: None,
        receiver,
        key: None,
        result,
        route_kind: GenericMethodRouteKind::ArrayAppendAny,
        proof: GenericMethodRouteProof::PushSurfacePolicy,
        core_op: CoreMethodOp::ArrayPush,
        lowering_tier: CoreMethodLoweringTier::ColdFallback,
        return_shape: Some(GenericMethodReturnShape::ScalarI64),
        value_demand: GenericMethodValueDemand::WriteAny,
        publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
    })
}

pub(crate) fn map_set_i64_key(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "MapBox",
        method: "set",
        arity: 2,
        receiver_origin_box: Some("MapBox"),
        key_route: Some(GenericMethodKeyRoute::I64Const),
        receiver,
        key: Some(key),
        result,
        route_kind: GenericMethodRouteKind::MapStoreAny,
        proof: GenericMethodRouteProof::SetSurfacePolicy,
        core_op: CoreMethodOp::MapSet,
        lowering_tier: CoreMethodLoweringTier::ColdFallback,
        return_shape: None,
        value_demand: GenericMethodValueDemand::WriteAny,
        publication_policy: None,
    })
}

pub(crate) fn map_get_unknown_key(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "MapBox",
        method: "get",
        arity: 1,
        receiver_origin_box: Some("MapBox"),
        key_route: Some(GenericMethodKeyRoute::UnknownAny),
        receiver,
        key: Some(key),
        result,
        route_kind: GenericMethodRouteKind::MapLoadAny,
        proof: GenericMethodRouteProof::GetSurfacePolicy,
        core_op: CoreMethodOp::MapGet,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
        return_shape: None,
        value_demand: GenericMethodValueDemand::ReadRef,
        publication_policy: None,
    })
}

pub(crate) fn array_get_i64_key(
    block: u32,
    instruction_index: usize,
    receiver: u32,
    key: u32,
    result: u32,
) -> GenericMethodRoute {
    route(FixtureSpec {
        block,
        instruction_index,
        box_name: "ArrayBox",
        method: "get",
        arity: 1,
        receiver_origin_box: Some("ArrayBox"),
        key_route: Some(GenericMethodKeyRoute::I64Const),
        receiver,
        key: Some(key),
        result,
        route_kind: GenericMethodRouteKind::ArraySlotLoadAny,
        proof: GenericMethodRouteProof::GetSurfacePolicy,
        core_op: CoreMethodOp::ArrayGet,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
        return_shape: None,
        value_demand: GenericMethodValueDemand::ReadRef,
        publication_policy: None,
    })
}
