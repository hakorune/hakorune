use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::compilation_context::RecordLocalFieldValue;
use crate::mir::builder::control_flow::edgecfg::api::{EdgeStub, ExitKind, Frag};
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::builder::MirBuilder;
use crate::mir::edge_args::JumpArgsLayout;
use crate::mir::function::{MirFunction, MirModule};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::region::RegionId;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, VerifiedResolvedFunctionV1,
};
use crate::mir::value_kind::MirValueKind;
use crate::mir::{BasicBlockId, BindingId, EdgeArgs, MirType, ValueId};
use hakorune_mir_builder::BoxCompilationContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InjectedCheckpoint {
    BeforeSkeleton,
    AfterSkeleton,
    AfterParameters,
    AfterBody,
    AfterFinalize,
}

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn resolved_product() -> Arc<VerifiedResolvedFunctionV1> {
    let function = ASTNode::FunctionDeclaration {
        name: "outer_resolved".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let view = FunctionSyntaxViewV1::from_ast(&function).unwrap();
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap()
}

fn seal_outer_frag(builder: &mut MirBuilder) {
    let entry = builder.function_state.current_block.unwrap();
    let mut frag = Frag::new(entry);
    frag.wires.push(EdgeStub::new(
        entry,
        ExitKind::Return,
        None,
        EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: Vec::new(),
        },
    ));
    builder
        .function_state
        .frag_emit_session
        .emit_and_seal(
            builder.function_state.current_function.as_mut().unwrap(),
            &frag,
        )
        .unwrap();
    assert!(builder
        .function_state
        .frag_emit_session
        .is_sealed_for_test(entry));
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(MirModule::new("session-fixture".into()));
    builder.enter_function_for_test("outer".into());
    builder
        .function_state
        .variable_ctx
        .insert("outer_value".into(), ValueId::new(700));
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(ValueId::new(700), MirType::Integer);
    builder
        .function_state
        .type_ctx
        .value_kinds
        .insert(ValueId::new(720), MirValueKind::Parameter(0));
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(ValueId::new(721), "OuterOriginBox".into());
    builder
        .function_state
        .type_ctx
        .string_literals
        .insert(ValueId::new(722), "outer literal".into());
    builder
        .function_state
        .type_ctx
        .map_value_types
        .insert(ValueId::new(723), MirType::Integer);
    builder
        .function_state
        .type_ctx
        .map_literal_value_types
        .insert((ValueId::new(724), "outer-key".into()), MirType::String);
    builder
        .function_state
        .binding_ctx
        .insert("outer_binding".into(), BindingId::new(719));
    builder
        .function_state
        .resolved_binding_state
        .install(&resolved_product())
        .unwrap();
    builder
        .function_state
        .compilation
        .register_record_local_value(
            ValueId::new(725),
            "OuterRecord".into(),
            vec![RecordLocalFieldValue {
                name: "field".into(),
                declared_type_name: Some("Integer".into()),
                value: ValueId::new(726),
            }],
        );
    builder.function_state.scope.push_lexical_scope();
    builder
        .function_state
        .scope
        .loop_header_stack
        .push(BasicBlockId::new(701));
    builder
        .function_state
        .scope
        .loop_exit_stack
        .push(BasicBlockId::new(702));
    builder
        .function_state
        .scope
        .if_merge_stack
        .push(BasicBlockId::new(703));
    builder
        .scope_ctx
        .debug_scope_stack
        .push("outer/debug".into());
    builder
        .function_state
        .scope
        .function_param_names
        .insert("arg".into());
    builder
        .function_state
        .scope
        .fastmem_region_stack
        .push(FastMemRegionId(704));
    builder.function_state.pending_phis.push((
        BasicBlockId::new(705),
        ValueId::new(706),
        "outer_phi".into(),
    ));
    builder.function_state.local_ssa_map.insert(
        (BasicBlockId::new(707), ValueId::new(708), 0),
        ValueId::new(709),
    );
    builder.function_state.schedule_mat_map.insert(
        (BasicBlockId::new(710), ValueId::new(711)),
        ValueId::new(712),
    );
    builder
        .function_state
        .pin_slot_names
        .insert(ValueId::new(713), "outer_pin".into());
    let defer = &mut builder.function_state.protected_region.return_defer;
    defer.activate(ValueId::new(714), BasicBlockId::new(715));
    defer.mark_emitted();
    builder.function_state.protected_region.cleanup.active = true;
    builder.function_state.protected_region.cleanup.allow_return = true;
    builder.function_state.protected_region.cleanup.allow_throw = true;
    builder.function_state.suppress_pin_entry_copy_next = true;
    builder.function_state.in_unified_boxcall_fallback = true;
    builder.recursion_depth = 7;
    builder.comp_ctx.current_static_box = Some("OuterBox".into());
    builder.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    builder
        .function_state
        .compilation
        .reserve_value_id(ValueId::new(716));
    builder.function_state.compilation.fn_body_ast = Some(vec![literal(717)]);
    builder.metadata_ctx.set_current_span(Span::new(1, 2, 3, 4));
    builder.metadata_ctx.push_region(RegionId(718));
    builder.record_value_origin_span(ValueId::new(727), Span::new(7, 2, 7, 9));
    builder.record_value_origin_caller(ValueId::new(728), std::panic::Location::caller());
    seal_outer_frag(&mut builder);
    builder
}

fn assert_outer_state(builder: &MirBuilder) {
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.signature.name.as_str()),
        Some("outer")
    );
    assert!(builder.function_state.current_block.is_some());
    assert_eq!(
        builder.function_state.variable_ctx.lookup("outer_value"),
        Some(ValueId::new(700))
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_types
            .get(&ValueId::new(700)),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_kinds
            .get(&ValueId::new(720)),
        Some(&MirValueKind::Parameter(0))
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&ValueId::new(721)),
        Some(&"OuterOriginBox".into())
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .string_literals
            .get(&ValueId::new(722)),
        Some(&"outer literal".into())
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_value_types
            .get(&ValueId::new(723)),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_literal_value_types
            .get(&(ValueId::new(724), "outer-key".into())),
        Some(&MirType::String)
    );
    assert_eq!(
        builder.function_state.binding_ctx.lookup("outer_binding"),
        Some(BindingId::new(719))
    );
    assert!(builder.function_state.resolved_binding_state.is_installed());
    let outer_record = builder
        .function_state
        .compilation
        .record_local_value(ValueId::new(725))
        .expect("outer record-local scratch must restore");
    assert_eq!(outer_record.record_name, "OuterRecord");
    assert_eq!(outer_record.fields.len(), 1);
    assert_eq!(outer_record.fields[0].name, "field");
    assert_eq!(
        outer_record.fields[0].declared_type_name.as_deref(),
        Some("Integer")
    );
    assert_eq!(outer_record.fields[0].value, ValueId::new(726));
    assert_eq!(builder.function_state.scope.lexical_scope_stack.len(), 1);
    assert_eq!(
        builder.function_state.scope.loop_header_stack,
        vec![BasicBlockId::new(701)]
    );
    assert_eq!(
        builder.function_state.scope.loop_exit_stack,
        vec![BasicBlockId::new(702)]
    );
    assert_eq!(
        builder.function_state.scope.if_merge_stack,
        vec![BasicBlockId::new(703)]
    );
    assert_eq!(builder.scope_ctx.debug_scope_stack, vec!["outer/debug"]);
    assert!(builder
        .function_state
        .scope
        .function_param_names
        .contains("arg"));
    assert_eq!(
        builder.function_state.scope.fastmem_region_stack,
        vec![FastMemRegionId(704)]
    );
    assert_eq!(builder.function_state.pending_phis.len(), 1);
    assert_eq!(builder.function_state.local_ssa_map.len(), 1);
    assert_eq!(builder.function_state.schedule_mat_map.len(), 1);
    assert_eq!(builder.function_state.pin_slot_names.len(), 1);
    let protected_region = builder.function_state.protected_region;
    assert!(protected_region.return_defer.is_active());
    assert_eq!(
        protected_region.return_defer.retained_slot(),
        Some(ValueId::new(714))
    );
    assert_eq!(
        protected_region.return_defer.retained_target(),
        Some(BasicBlockId::new(715))
    );
    assert!(protected_region.return_defer.emitted());
    assert!(protected_region.cleanup.active);
    assert!(protected_region.cleanup.allow_return);
    assert!(protected_region.cleanup.allow_throw);
    assert!(builder.function_state.suppress_pin_entry_copy_next);
    assert!(builder.function_state.in_unified_boxcall_fallback);
    assert_eq!(builder.recursion_depth, 7);
    assert_eq!(
        builder.comp_ctx.current_static_box.as_deref(),
        Some("OuterBox")
    );
    assert!(builder.comp_ctx.current_slot_registry.is_some());
    assert!(builder
        .function_state
        .compilation
        .is_reserved_value_id(ValueId::new(716)));
    assert_eq!(
        builder
            .function_state
            .compilation
            .fn_body_ast
            .as_ref()
            .map(|body| body.len()),
        Some(1)
    );
    assert_eq!(builder.metadata_ctx.current_span(), Span::new(1, 2, 3, 4));
    assert_eq!(
        builder.metadata_ctx.current_region_stack(),
        &[RegionId(718)]
    );
    assert_eq!(
        builder.value_origin_span(ValueId::new(727)),
        Some(Span::new(7, 2, 7, 9))
    );
    assert!(builder.value_origin_caller(ValueId::new(728)).is_some());
    assert!(builder
        .function_state
        .frag_emit_session
        .is_sealed_for_test(BasicBlockId::new(0)));
}

fn assert_child_entry_is_reset(builder: &MirBuilder) {
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
    assert!(builder
        .function_state
        .variable_ctx
        .lookup("outer_value")
        .is_none());
    assert!(builder.function_state.type_ctx.value_types.is_empty());
    assert!(builder.function_state.type_ctx.value_kinds.is_empty());
    assert!(builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .is_empty());
    assert!(builder.function_state.type_ctx.string_literals.is_empty());
    assert!(builder.function_state.type_ctx.map_value_types.is_empty());
    assert!(builder
        .function_state
        .type_ctx
        .map_literal_value_types
        .is_empty());
    assert!(builder.function_state.binding_ctx.is_empty());
    assert!(!builder.function_state.resolved_binding_state.is_installed());
    assert!(builder
        .function_state
        .compilation
        .record_local_value(ValueId::new(725))
        .is_none());
    assert!(builder.function_state.scope.lexical_scope_stack.is_empty());
    assert!(builder.function_state.scope.loop_header_stack.is_empty());
    assert!(builder.function_state.scope.loop_exit_stack.is_empty());
    assert!(builder.function_state.scope.if_merge_stack.is_empty());
    assert!(builder.scope_ctx.debug_scope_stack.is_empty());
    assert!(builder.function_state.scope.function_param_names.is_empty());
    assert!(builder.function_state.scope.fastmem_region_stack.is_empty());
    assert!(builder.comp_ctx.current_slot_registry.is_none());
    assert!(!builder
        .function_state
        .compilation
        .is_reserved_value_id(ValueId::new(716)));
    assert_eq!(
        builder
            .function_state
            .compilation
            .fn_body_ast
            .as_ref()
            .map(Vec::len),
        Some(0),
        "the child owns its empty legacy body capture rather than the outer body"
    );
    assert_eq!(
        builder.value_origin_span(ValueId::new(727)),
        Some(Span::new(7, 2, 7, 9)),
        "metadata origin spans are an explicit no-isolation control before METAISO"
    );
    assert!(
        builder.value_origin_caller(ValueId::new(728)).is_some(),
        "metadata origin callers are an explicit no-isolation control before METAISO"
    );
    assert!(!builder
        .function_state
        .frag_emit_session
        .is_sealed_for_test(BasicBlockId::new(0)));
}

fn run_injected_checkpoint(
    builder: &mut MirBuilder,
    checkpoint: InjectedCheckpoint,
) -> Result<(), String> {
    let function_name = "Injected.run/0".to_string();
    let session_name = function_name.clone();
    let body = Vec::new();
    builder.with_function_lowering_session(&session_name, body.clone(), move |builder| {
        if checkpoint == InjectedCheckpoint::BeforeSkeleton {
            return Err("injected:before_skeleton".into());
        }
        builder.create_function_skeleton(function_name.clone(), &[], &body)?;
        if checkpoint == InjectedCheckpoint::AfterSkeleton {
            return Err("injected:after_skeleton".into());
        }
        builder.setup_function_params(&[])?;
        if checkpoint == InjectedCheckpoint::AfterParameters {
            return Err("injected:after_parameters".into());
        }
        let mut port = RawLegacyChildLoweringPortV1;
        let program = super::function_lowering::wrap_in_program(body.clone());
        let _ = builder.build_expression_impl_with_port_v1(&mut port, program)?;
        if checkpoint == InjectedCheckpoint::AfterBody {
            return Err("injected:after_body".into());
        }
        let draft = builder.finalize_function_draft(false)?;
        assert!(builder
            .current_module
            .as_ref()
            .unwrap()
            .get_function("Injected.run/0")
            .is_none());
        if checkpoint == InjectedCheckpoint::AfterFinalize {
            return Err("injected:after_finalize".into());
        }
        Ok(draft)
    })
}

#[test]
fn every_fallible_checkpoint_restores_caller_and_publishes_nothing() {
    for checkpoint in [
        InjectedCheckpoint::BeforeSkeleton,
        InjectedCheckpoint::AfterSkeleton,
        InjectedCheckpoint::AfterParameters,
        InjectedCheckpoint::AfterBody,
        InjectedCheckpoint::AfterFinalize,
    ] {
        let mut builder = seeded_builder();
        let error = run_injected_checkpoint(&mut builder, checkpoint).unwrap_err();
        assert!(error.contains("injected:"), "{checkpoint:?}: {error}");
        assert_outer_state(&builder);
        assert!(builder
            .current_module
            .as_ref()
            .unwrap()
            .get_function("Injected.run/0")
            .is_none());
    }
}

#[test]
fn primary_and_cleanup_errors_are_both_preserved() {
    let mut builder = seeded_builder();
    let error = builder
        .with_function_lowering_session("Injected.cleanup/0", Vec::new(), |builder| {
            builder.metadata_ctx.push_region(RegionId(800));
            builder.metadata_ctx.push_region(RegionId(801));
            Err("injected:primary".to_string())
        })
        .unwrap_err();

    assert!(error.contains("canonical_function_session/during_cleanup"));
    assert!(error.contains("injected:primary"));
    assert!(error.contains("observer_region_stack"));
    assert_outer_state(&builder);
}

#[test]
fn static_and_instance_drafts_commit_only_after_caller_restore() {
    let mut builder = seeded_builder();
    builder
        .lower_static_method_as_function(
            "Fixture.static/0".into(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            DeclarationAttrs::default(),
        )
        .unwrap();
    assert_outer_state(&builder);
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .get_function("Fixture.static/0")
        .is_some());

    builder
        .lower_method_as_function(
            "Fixture.run/0".into(),
            "Fixture".into(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            DeclarationAttrs::default(),
        )
        .unwrap();
    assert_outer_state(&builder);
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .get_function("Fixture.run/0")
        .is_some());

    builder
        .lower_method_as_function(
            "Fixture.birth/0".into(),
            "Fixture".into(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            DeclarationAttrs::default(),
        )
        .unwrap();
    assert_outer_state(&builder);
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .get_function("Fixture.birth/0")
        .is_some());
}

#[test]
fn child_entry_resets_captured_function_owned_state_before_restoring_outer_state() {
    let mut builder = seeded_builder();
    let error = builder
        .with_function_lowering_session("Injected.child_entry/0", Vec::new(), |child| {
            assert_child_entry_is_reset(child);

            child
                .function_state
                .variable_ctx
                .insert("child_value".into(), ValueId::new(900));
            child
                .function_state
                .type_ctx
                .value_types
                .insert(ValueId::new(900), MirType::String);
            child
                .function_state
                .type_ctx
                .value_kinds
                .insert(ValueId::new(903), MirValueKind::Local(0));
            child
                .function_state
                .type_ctx
                .value_origin_newbox
                .insert(ValueId::new(904), "ChildOriginBox".into());
            child
                .function_state
                .type_ctx
                .string_literals
                .insert(ValueId::new(905), "child literal".into());
            child
                .function_state
                .type_ctx
                .map_value_types
                .insert(ValueId::new(906), MirType::Bool);
            child
                .function_state
                .type_ctx
                .map_literal_value_types
                .insert((ValueId::new(907), "child-key".into()), MirType::Float);
            child
                .function_state
                .binding_ctx
                .insert("child_binding".into(), BindingId::new(901));
            child
                .function_state
                .resolved_binding_state
                .install(&resolved_product())
                .unwrap();
            child
                .function_state
                .compilation
                .register_record_local_value(ValueId::new(902), "ChildRecord".into(), Vec::new());
            child.record_value_origin_span(ValueId::new(908), Span::new(8, 1, 8, 9));
            child.record_value_origin_caller(ValueId::new(909), std::panic::Location::caller());
            Err("injected:child_entry".into())
        })
        .unwrap_err();

    assert!(error.contains("injected:child_entry"));
    assert_outer_state(&builder);
    assert!(builder
        .function_state
        .variable_ctx
        .lookup("child_value")
        .is_none());
    assert!(builder
        .function_state
        .type_ctx
        .value_types
        .get(&ValueId::new(900))
        .is_none());
    assert!(builder
        .function_state
        .type_ctx
        .value_kinds
        .get(&ValueId::new(903))
        .is_none());
    assert!(builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .get(&ValueId::new(904))
        .is_none());
    assert!(builder
        .function_state
        .type_ctx
        .string_literals
        .get(&ValueId::new(905))
        .is_none());
    assert!(builder
        .function_state
        .type_ctx
        .map_value_types
        .get(&ValueId::new(906))
        .is_none());
    assert!(builder
        .function_state
        .type_ctx
        .map_literal_value_types
        .get(&(ValueId::new(907), "child-key".into()))
        .is_none());
    assert!(builder
        .function_state
        .binding_ctx
        .lookup("child_binding")
        .is_none());
    assert!(builder
        .function_state
        .compilation
        .record_local_value(ValueId::new(902))
        .is_none());
    assert_eq!(
        builder.value_origin_span(ValueId::new(908)),
        Some(Span::new(8, 1, 8, 9)),
        "child metadata origin span intentionally remains until METAISO"
    );
    assert!(
        builder.value_origin_caller(ValueId::new(909)).is_some(),
        "child metadata origin caller intentionally remains until METAISO"
    );
}

#[test]
fn box_compilation_session_preserves_the_existing_partial_type_context_action() {
    let mut builder = seeded_builder();
    builder.comp_ctx.compilation_context = Some(BoxCompilationContext::new());

    let error = builder
        .with_function_lowering_session("Injected.box_context/0", Vec::new(), |child| {
            assert!(child.function_state.type_ctx.value_types.is_empty());
            assert!(child.function_state.type_ctx.value_kinds.is_empty());
            assert!(child.function_state.type_ctx.value_origin_newbox.is_empty());
            assert_eq!(
                child
                    .function_state
                    .type_ctx
                    .string_literals
                    .get(&ValueId::new(722)),
                Some(&"outer literal".into())
            );
            assert_eq!(
                child
                    .function_state
                    .type_ctx
                    .map_value_types
                    .get(&ValueId::new(723)),
                Some(&MirType::Integer)
            );
            assert_eq!(
                child
                    .function_state
                    .type_ctx
                    .map_literal_value_types
                    .get(&(ValueId::new(724), "outer-key".into())),
                Some(&MirType::String)
            );
            Err("injected:box_context".into())
        })
        .unwrap_err();

    assert!(error.contains("injected:box_context"));
    assert!(builder.function_state.type_ctx.value_types.is_empty());
    assert!(builder.function_state.type_ctx.value_kinds.is_empty());
    assert!(builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .is_empty());
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .string_literals
            .get(&ValueId::new(722)),
        Some(&"outer literal".into())
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_value_types
            .get(&ValueId::new(723)),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .map_literal_value_types
            .get(&(ValueId::new(724), "outer-key".into())),
        Some(&MirType::String)
    );
}

#[test]
fn panic_backstop_restores_without_publishing() {
    let mut builder = seeded_builder();
    let panic = catch_unwind(AssertUnwindSafe(|| {
        let _ = builder.with_function_lowering_session(
            "Injected.panic/0",
            Vec::new(),
            |_builder| -> Result<MirFunction, String> {
                panic!("injected panic");
            },
        );
    }));

    assert!(panic.is_err());
    assert_outer_state(&builder);
    assert!(builder
        .current_module
        .as_ref()
        .unwrap()
        .get_function("Injected.panic/0")
        .is_none());
}
