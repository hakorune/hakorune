use super::me_call_header_observation::{
    prepare_me_lowered_call_v1, MeCallHeaderObservationPortV1, MeCallHeaderSourceV1,
    PreparedMeReceiverV1,
};
use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_lowering_invocation::ModuleLoweringInvocationV1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceTransportV1, RawSourceTransportPortV1,
};
use super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawInvocationChildPortV1, RawLegacyChildLoweringPortV1,
};
use super::recursive_child_lowering_rawport_tests::{
    birth_collector, collector, collector_with_return_type, instructions, int, new_expr,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{
    BasicBlockId, Effect, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirModule,
    MirType,
};

#[test]
fn raw_invocation_port_lends_collector_brand_once() {
    let brand = ModuleInvocationBrandV1::legacy_test();
    let mut builder = MirBuilder::new();
    let mut invocation = ModuleLoweringInvocationV1::with_collector(
        &mut builder,
        ModuleDraftCollectorV1::with_brand(brand),
    );
    invocation.with_module_port(|_builder, module_port| {
        let port = RawInvocationChildPortV1::new(module_port);
        let observed = port
            .with_invocation_brand(|observed| observed)
            .expect("branded collector");
        assert_eq!(observed, brand);
    });
}

#[test]
fn raw_invocation_port_rejects_unbranded_collector_before_callback() {
    let mut builder = MirBuilder::new();
    let mut invocation = ModuleLoweringInvocationV1::with_collector(&mut builder, collector());
    invocation.with_module_port(|_builder, module_port| {
        let port = RawInvocationChildPortV1::new(module_port);
        let mut called = false;
        let error = port
            .with_invocation_brand(|_| {
                called = true;
            })
            .expect_err("unbranded collector must reject");
        assert_eq!(
            error,
            super::module_draft_collector::CollectorReceiptBrandErrorV1::CollectorUnbranded
        );
        assert!(!called);
    });
}

#[test]
fn raw_invocation_port_reborrows_one_collector_backed_header_view() {
    let mut builder = MirBuilder::new();
    let mut invocation = ModuleLoweringInvocationV1::with_collector(&mut builder, collector());
    invocation.with_module_port(|_builder, module_port| {
        let mut port = RawInvocationChildPortV1::new(module_port);
        port.with_headers(|headers| {
            assert_eq!(headers.signature("Prefix.f/1").unwrap().params.len(), 1)
        });
        port.reborrow()
            .with_headers(|headers| assert!(headers.contains_symbol("Prefix.f/1")));
    });
}

#[test]
fn headerport_annotation_matches_legacy_module_signature_without_ambient_module() {
    let symbol = "Prefix.f/1";
    let signature = FunctionSignature {
        name: symbol.to_owned(),
        params: vec![MirType::Integer],
        return_type: MirType::Box("Result".to_owned()),
        effects: EffectMask::READ.add(Effect::ReadHeap),
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("headerport_annotation/0".to_owned());
    legacy.current_module = Some(MirModule::new("legacy-header-module".to_owned()));
    legacy
        .current_module
        .as_mut()
        .unwrap()
        .add_function(MirFunction::new(signature, BasicBlockId(0)));
    let mut port_builder = MirBuilder::new();
    port_builder.enter_function_for_test("headerport_annotation/0".to_owned());
    let dst = crate::mir::ValueId(11);
    super::calls::annotation::annotate_call_result_from_func_name(&mut legacy, dst, symbol);
    let mut invocation = ModuleLoweringInvocationV1::with_collector(
        &mut port_builder,
        collector_with_return_type(MirType::Box("Result".to_owned())),
    );
    invocation.with_header_port(|builder, headers| {
        super::calls::annotation::annotate_call_result_from_func_name_with_lookup(
            builder,
            dst,
            symbol,
            Some(headers),
        );
    });
    assert!(port_builder.current_module.is_none());
    assert_eq!(
        legacy.function_state.type_ctx.value_types.get(&dst),
        port_builder.function_state.type_ctx.value_types.get(&dst)
    );
    assert_eq!(
        legacy.function_state.type_ctx.value_origin_newbox.get(&dst),
        port_builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&dst)
    );
}

#[test]
fn explicit_header_authority_survives_unified_call_post_success() {
    let symbol = "Prefix.f/1";
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("headerport_call/0".to_owned());
    let stale = MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    builder.current_module = Some(MirModule::new("stale-call-module".to_owned()));
    builder.current_module.as_mut().unwrap().add_function(stale);
    let dst = crate::mir::ValueId(12);
    let mut invocation = ModuleLoweringInvocationV1::with_collector(
        &mut builder,
        collector_with_return_type(MirType::Box("Result".to_owned())),
    );
    invocation.with_header_port(|builder, headers| {
        let arg = crate::mir::builder::emission::constant::emit_integer(builder, 1).unwrap();
        builder
            .emit_unified_call_with_lookup(
                Some(dst),
                super::calls::CallTarget::Global(symbol.to_owned()),
                vec![arg],
                Some(headers),
            )
            .unwrap();
    });
    drop(invocation);
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&dst),
        Some(&MirType::Box("Result".to_owned()))
    );
}

#[test]
fn headerport_birth_presence_matches_legacy_newbox_branch() {
    let birth = MirFunction::new(
        FunctionSignature {
            name: "Prefix.birth/1".to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::READ.add(Effect::ReadHeap),
        },
        BasicBlockId(0),
    );
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("headerport_birth/0".to_owned());
    legacy.current_module = Some(MirModule::new("legacy-birth-module".to_owned()));
    legacy.current_module.as_mut().unwrap().add_function(birth);
    let mut legacy_port = RawLegacyChildLoweringPortV1;
    let legacy_value = drive_legacy_expression_v1(
        &mut legacy,
        &mut legacy_port,
        new_expr("Prefix", vec![int(7)]),
    )
    .unwrap();
    let mut port_builder = MirBuilder::new();
    port_builder.enter_function_for_test("headerport_birth/0".to_owned());
    let invocation_value = {
        let mut invocation =
            ModuleLoweringInvocationV1::with_collector(&mut port_builder, birth_collector());
        let value = invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            port.with_source_transport_v1(
                RawInvocationSourceTransportV1::root((), RawInvocationRootLineageV1::ScriptRoot),
                |port, ()| {
                    drive_legacy_expression_v1(builder, port, new_expr("Prefix", vec![int(7)]))
                },
            )
        });
        drop(invocation);
        value.unwrap()
    };
    assert_eq!(legacy_value, invocation_value);
    assert_eq!(instructions(&legacy), instructions(&port_builder));
    assert!(port_builder.current_module.is_none());
}

#[test]
fn raw_invocation_me_header_ignores_stale_module_signature() {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(crate::mir::MirModule::new("stale-me-module".to_string()));
    builder
        .current_module
        .as_mut()
        .unwrap()
        .add_function(MirFunction::new(
            FunctionSignature {
                name: "Prefix.f/1".to_string(),
                params: vec![MirType::Box("Prefix".to_string()), MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        ));
    let mut invocation = ModuleLoweringInvocationV1::with_collector(&mut builder, collector());
    invocation.with_module_port(|builder, module_port| {
        let mut port = RawInvocationChildPortV1::new(module_port);
        let observation = port.observe_me_call_parameters(builder, "Prefix.f/1");
        assert_eq!(
            observation.source(),
            MeCallHeaderSourceV1::InvocationCollector
        );
        let prepared = prepare_me_lowered_call_v1(observation, Some(crate::mir::ValueId(4)))
            .expect("collector header should be present");
        assert_eq!(prepared.receiver(), &PreparedMeReceiverV1::Static);
    });
}

#[test]
fn raw_invocation_header_miss_does_not_retry_stale_current_module() {
    let symbol = "Ghost.m/1";
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_invocation_header_miss/0".to_owned());
    builder.current_module = Some(crate::mir::MirModule::new("stale-miss-module".to_string()));
    builder
        .current_module
        .as_mut()
        .unwrap()
        .add_function(MirFunction::new(
            FunctionSignature {
                name: symbol.to_string(),
                params: vec![MirType::Box("Ghost".to_string()), MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        ));
    let instructions_before = instructions(&builder);
    let next_value_before = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .next_value_id;
    {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut builder,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            let observation = port.observe_me_call_parameters(builder, symbol);
            assert_eq!(
                observation.source(),
                MeCallHeaderSourceV1::InvocationCollector
            );
            assert!(matches!(
                &observation,
                super::me_call_header_observation::MeCallParameterObservationV1::Missing { .. }
            ));
            assert!(prepare_me_lowered_call_v1(observation, None).is_none());
        });
    }
    assert_eq!(instructions(&builder), instructions_before);
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id,
        next_value_before
    );
}
