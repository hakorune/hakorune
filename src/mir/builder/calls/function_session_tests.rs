use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::MirBuilder;
use crate::mir::function::{MirFunction, MirModule};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::region::RegionId;
use crate::mir::{BasicBlockId, MirType, ValueId};

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

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.current_module = Some(MirModule::new("session-fixture".into()));
    builder.enter_function_for_test("outer".into());
    builder
        .variable_ctx
        .insert("outer_value".into(), ValueId::new(700));
    builder
        .type_ctx
        .value_types
        .insert(ValueId::new(700), MirType::Integer);
    builder.scope_ctx.push_lexical_scope();
    builder
        .scope_ctx
        .loop_header_stack
        .push(BasicBlockId::new(701));
    builder
        .scope_ctx
        .loop_exit_stack
        .push(BasicBlockId::new(702));
    builder
        .scope_ctx
        .if_merge_stack
        .push(BasicBlockId::new(703));
    builder
        .scope_ctx
        .debug_scope_stack
        .push("outer/debug".into());
    builder.scope_ctx.function_param_names.insert("arg".into());
    builder
        .scope_ctx
        .fastmem_region_stack
        .push(FastMemRegionId(704));
    builder.pending_phis.push((
        BasicBlockId::new(705),
        ValueId::new(706),
        "outer_phi".into(),
    ));
    builder.local_ssa_map.insert(
        (BasicBlockId::new(707), ValueId::new(708), 0),
        ValueId::new(709),
    );
    builder.schedule_mat_map.insert(
        (BasicBlockId::new(710), ValueId::new(711)),
        ValueId::new(712),
    );
    builder
        .pin_slot_names
        .insert(ValueId::new(713), "outer_pin".into());
    builder.return_defer_active = true;
    builder.return_defer_slot = Some(ValueId::new(714));
    builder.return_defer_target = Some(BasicBlockId::new(715));
    builder.return_deferred_emitted = true;
    builder.in_cleanup_block = true;
    builder.cleanup_allow_return = true;
    builder.cleanup_allow_throw = true;
    builder.suppress_pin_entry_copy_next = true;
    builder.in_unified_boxcall_fallback = true;
    builder.recursion_depth = 7;
    builder.comp_ctx.current_static_box = Some("OuterBox".into());
    builder.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    builder.comp_ctx.reserve_value_id(ValueId::new(716));
    builder.comp_ctx.fn_body_ast = Some(vec![literal(717)]);
    builder.metadata_ctx.set_current_span(Span::new(1, 2, 3, 4));
    builder.metadata_ctx.push_region(RegionId(718));
    builder
}

fn assert_outer_state(builder: &MirBuilder) {
    assert_eq!(
        builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|function| function.signature.name.as_str()),
        Some("outer")
    );
    assert!(builder.current_block.is_some());
    assert_eq!(
        builder.variable_ctx.lookup("outer_value"),
        Some(ValueId::new(700))
    );
    assert_eq!(
        builder.type_ctx.value_types.get(&ValueId::new(700)),
        Some(&MirType::Integer)
    );
    assert_eq!(builder.scope_ctx.lexical_scope_stack.len(), 1);
    assert_eq!(
        builder.scope_ctx.loop_header_stack,
        vec![BasicBlockId::new(701)]
    );
    assert_eq!(
        builder.scope_ctx.loop_exit_stack,
        vec![BasicBlockId::new(702)]
    );
    assert_eq!(
        builder.scope_ctx.if_merge_stack,
        vec![BasicBlockId::new(703)]
    );
    assert_eq!(builder.scope_ctx.debug_scope_stack, vec!["outer/debug"]);
    assert!(builder.scope_ctx.function_param_names.contains("arg"));
    assert_eq!(
        builder.scope_ctx.fastmem_region_stack,
        vec![FastMemRegionId(704)]
    );
    assert_eq!(builder.pending_phis.len(), 1);
    assert_eq!(builder.local_ssa_map.len(), 1);
    assert_eq!(builder.schedule_mat_map.len(), 1);
    assert_eq!(builder.pin_slot_names.len(), 1);
    assert!(builder.return_defer_active);
    assert_eq!(builder.return_defer_slot, Some(ValueId::new(714)));
    assert_eq!(builder.return_defer_target, Some(BasicBlockId::new(715)));
    assert!(builder.return_deferred_emitted);
    assert!(builder.in_cleanup_block);
    assert!(builder.cleanup_allow_return);
    assert!(builder.cleanup_allow_throw);
    assert!(builder.suppress_pin_entry_copy_next);
    assert!(builder.in_unified_boxcall_fallback);
    assert_eq!(builder.recursion_depth, 7);
    assert_eq!(
        builder.comp_ctx.current_static_box.as_deref(),
        Some("OuterBox")
    );
    assert!(builder.comp_ctx.current_slot_registry.is_some());
    assert!(builder.comp_ctx.is_reserved_value_id(ValueId::new(716)));
    assert_eq!(
        builder.comp_ctx.fn_body_ast.as_ref().map(|body| body.len()),
        Some(1)
    );
    assert_eq!(builder.metadata_ctx.current_span(), Span::new(1, 2, 3, 4));
    assert_eq!(
        builder.metadata_ctx.current_region_stack(),
        &[RegionId(718)]
    );
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
        builder.lower_function_body(body.clone())?;
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
