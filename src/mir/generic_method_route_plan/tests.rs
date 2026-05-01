use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::global_call_route_plan::{
    GlobalCallRoute, GlobalCallRouteSite, GlobalCallTargetFacts, GlobalCallTargetShape,
};
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
                    && route.result_value() == result_value
            })
            .unwrap_or_else(|| {
                panic!(
                    "missing generic method route box={box_name} method={method} result={result_value:?}"
                )
            })
}

#[test]
fn generic_method_route_metadata_tokens_come_from_route_kind() {
    let route = GenericMethodRoute::new(
        GenericMethodRouteSite::new(BasicBlockId::new(0), 0),
        GenericMethodRouteSurface::new("MapBox", "__raw_method_must_not_drive_metadata", 1),
        GenericMethodRouteEvidence::new(
            Some("MapBox".to_string()),
            Some(GenericMethodKeyRoute::I64Const),
        ),
        GenericMethodRouteOperands::new(
            ValueId::new(1),
            Some(ValueId::new(2)),
            Some(ValueId::new(3)),
        ),
        GenericMethodRouteDecision::new(
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
    );

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
    assert_eq!(route.block(), BasicBlockId::new(0));
    assert_eq!(route.instruction_index(), 0);
    assert_eq!(route.box_name(), "MapBox");
    assert_eq!(route.method(), "has");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.receiver_value(), ValueId::new(1));
    assert_eq!(route.key_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), Some(ValueId::new(3)));
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
    assert_eq!(map_route.key_value(), None);
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
    assert_eq!(route.key_value(), None);
    assert_eq!(route.arity(), 0);
}

#[test]
fn records_runtime_data_string_len_from_generic_global_call_origin() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.coerce/1".to_string())),
        args: vec![ValueId::new(0)],
        effects: EffectMask::PURE,
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "length", 2, vec![]));
    function
        .metadata
        .global_call_routes
        .push(GlobalCallRoute::new(
            GlobalCallRouteSite::new(BasicBlockId::new(0), 0),
            "Helper.coerce/1",
            1,
            Some(ValueId::new(1)),
            GlobalCallTargetFacts::present_with_shape(
                1,
                GlobalCallTargetShape::GenericPureStringBody,
            ),
        ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "length");
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringLen);
    let core_method = route.core_method().expect("StringLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringLen);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
}

#[test]
fn records_runtime_data_substring_from_generic_global_call_phi_origin() {
    let mut function = make_function();
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.coerce/1".to_string())),
        args: vec![ValueId::new(0)],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    let mut merge = BasicBlock::new(BasicBlockId::new(1));
    merge.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(2),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
        type_hint: None,
    });
    merge.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    merge.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(0),
    });
    merge.add_instruction(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(64),
    });
    merge.add_instruction(method_call(
        Some(6),
        "RuntimeDataBox",
        "substring",
        3,
        vec![4, 5],
    ));
    function.add_block(merge);
    function
        .metadata
        .global_call_routes
        .push(GlobalCallRoute::new(
            GlobalCallRouteSite::new(BasicBlockId::new(0), 0),
            "Helper.coerce/1",
            1,
            Some(ValueId::new(1)),
            GlobalCallTargetFacts::present_with_shape(
                1,
                GlobalCallTargetShape::GenericPureStringBody,
            ),
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
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
}

#[test]
fn records_runtime_data_substring_from_string_concat_origin() {
    let mut function = make_function();
    function.params = vec![ValueId::new(0)];
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String(String::new()),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(2),
        op: BinaryOp::Add,
        lhs: ValueId::new(1),
        rhs: ValueId::new(0),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(8),
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
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
    let core_method = route
        .core_method()
        .expect("RuntimeData StringSubstring carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
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
    assert_eq!(route.key_value(), None);
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
    assert_eq!(route.key_value(), None);
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
    assert_eq!(route.key_value(), None);
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
    assert_eq!(array_route.key_value(), Some(ValueId::new(1)));
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
