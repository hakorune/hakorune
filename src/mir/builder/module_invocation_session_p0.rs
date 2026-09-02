//! CUT0-I0-SESSION0 fixtures for the disconnected Builder transaction.

use std::collections::HashMap;

use super::emit_debug_policy::BuilderEmitDebugPolicySnapshotV1;
use super::module_invocation_session::{
    BuilderCommitReadinessErrorV1, BuilderCoreIdSeedV1, BuilderCoreSeedPolicyV1,
    BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use super::{
    BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirModule, MirType,
};
use hakorune_mir_builder::BoxCompilationContext;

fn advanced_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.repl_mode = true;
    builder.comp_ctx.quiet_internal_logs = true;
    builder
        .comp_ctx
        .using_import_boxes
        .insert("Alias".into(), "Imported".into());
    builder
        .comp_ctx
        .plugin_method_sigs
        .insert(("PluginBox".into(), "value/0".into()), MirType::Integer);
    builder.set_source_file_hint("session0.hako");
    builder.next_value_id();
    builder.next_value_id();
    builder.next_block_id();
    builder.allocate_binding_id().unwrap();
    builder.core_ctx.next_temp_slot();
    builder.debug_next_join_id();
    builder
}

fn core_cursor(builder: &MirBuilder) -> (u32, u32, u32, u32, u32) {
    (
        builder.core_ctx.peek_next_value().as_u32(),
        builder.core_ctx.peek_next_block().as_u32(),
        builder.core_ctx.next_binding_id,
        builder.core_ctx.temp_slot_counter,
        builder.core_ctx.debug_join_counter,
    )
}

fn fresh_session() -> ModuleBuilderInvocationSessionV1 {
    let live = MirBuilder::new();
    let config =
        BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
    ModuleBuilderInvocationSessionV1::open(&live, config)
}

fn assert_module_session_rejected(
    session: ModuleBuilderInvocationSessionV1,
    expected: BuilderCommitReadinessErrorV1,
) {
    let rejected = session
        .prepare_module_session()
        .expect_err("readiness failure must retain a rejected owner");
    assert_eq!(rejected.error(), &expected);
    let (_session, error) = rejected.into_parts();
    assert_eq!(error, expected);
}

fn debug_policy_updates(enabled: bool) -> [(&'static str, Option<&'static str>); 11] {
    let value = if enabled { Some("1") } else { Some("0") };
    [
        ("HAKO_JOINIR_DEBUG", value),
        ("NYASH_JOINIR_DEBUG", value),
        ("HAKO_JOINIR_STRICT", value),
        ("NYASH_JOINIR_STRICT", value),
        ("HAKO_JOINIR_PLANNER_REQUIRED", value),
        ("NYASH_LOCAL_SSA_TRACE", value),
        ("NYASH_BUILDER_TRACE_RECV", value),
        (
            "NYASH_BUILDER_DEBUG",
            if enabled { Some("1") } else { None },
        ),
        ("NYASH_STATIC_CALL_TRACE", value),
        ("NYASH_STATIC_METHOD_TRACE", value),
        ("NYASH_CALL_RESOLVE_TRACE", value),
    ]
}

fn debug_policy_bits(policy: BuilderEmitDebugPolicySnapshotV1) -> [bool; 8] {
    [
        policy.joinir_debug_enabled(),
        policy.strict_planner_required_debug_enabled(),
        policy.local_ssa_trace(),
        policy.trace_recv(),
        policy.builder_debug_enabled(),
        policy.static_call_trace(),
        policy.static_method_trace(),
        policy.call_resolve_trace(),
    ]
}

#[test]
fn module_session_readiness_success_is_non_clone_and_consuming() {
    let prepared = fresh_session()
        .prepare_module_session()
        .expect("fresh candidate is closed for finalization");
    assert_eq!(
        prepared.brand(),
        super::module_invocation_identity::ModuleInvocationBrandV1::legacy_test()
    );
    assert_eq!(
        prepared.family(),
        super::module_invocation_identity::ModuleInvocationFamilyV1::Raw
    );
    let (_brand, _family, _session) = prepared.into_parts();
}

#[test]
fn module_session_readiness_rejects_current_module() {
    let mut session = fresh_session();
    session.builder_mut().current_module = Some(MirModule::new("open".into()));
    assert_module_session_rejected(session, BuilderCommitReadinessErrorV1::CurrentModuleOpen);
}

#[test]
fn module_session_readiness_rejects_current_function() {
    let mut session = fresh_session();
    session.builder_mut().function_state.current_function = Some(MirFunction::new(
        FunctionSignature {
            name: "open/0".into(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    ));
    assert_module_session_rejected(session, BuilderCommitReadinessErrorV1::CurrentFunctionOpen);
}

#[test]
fn module_session_readiness_rejects_current_block() {
    let mut session = fresh_session();
    session.builder_mut().function_state.current_block = Some(BasicBlockId::new(3));
    assert_module_session_rejected(session, BuilderCommitReadinessErrorV1::CurrentBlockOpen);
}

#[test]
fn module_session_readiness_rejects_function_state() {
    let mut session = fresh_session();
    session
        .builder_mut()
        .function_state
        .variable_ctx
        .insert("stale".into(), super::ValueId::new(7));
    assert_module_session_rejected(session, BuilderCommitReadinessErrorV1::FunctionStateOpen);
}

#[test]
fn module_session_readiness_rejects_slot_registry() {
    let mut session = fresh_session();
    session.builder_mut().comp_ctx.current_slot_registry =
        Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());
    assert_module_session_rejected(session, BuilderCommitReadinessErrorV1::SlotRegistryOpen);
}

#[test]
fn module_session_readiness_rejects_compilation_context() {
    let mut session = fresh_session();
    session.builder_mut().comp_ctx.compilation_context = Some(BoxCompilationContext::new());
    assert_module_session_rejected(
        session,
        BuilderCommitReadinessErrorV1::CompilationContextOpen,
    );
}

#[test]
fn module_session_readiness_rejects_recursion_depth() {
    let mut session = fresh_session();
    session.builder_mut().recursion_depth = 1;
    assert_module_session_rejected(session, BuilderCommitReadinessErrorV1::RecursionDepthOpen);
}

#[test]
fn snapshot_installs_all_explicit_builder_inputs() {
    let live = advanced_builder();
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config.clone());
    let candidate = session.builder_mut();

    assert_eq!(config.repl_mode(), candidate.repl_mode);
    assert_eq!(
        config.quiet_internal_logs(),
        candidate.comp_ctx.quiet_internal_logs
    );
    assert_eq!(
        config.emit_debug_policy(),
        *candidate.comp_ctx.emit_debug_policy()
    );
    assert_eq!(
        config.using_import_boxes(),
        &candidate.comp_ctx.using_import_boxes
    );
    assert_eq!(
        config.plugin_method_sigs(),
        &candidate.comp_ctx.plugin_method_sigs
    );
    assert_eq!(
        config.source_file(),
        candidate.current_source_file().as_deref()
    );
    assert!(matches!(
        config.core_id_seed(),
        BuilderCoreIdSeedV1::ContinueLive(_)
    ));
}

#[test]
fn invocation_debug_policy_survives_ambient_flip() {
    let policy_a = crate::test_support::with_env_vars(&debug_policy_updates(true), || {
        let live = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_raw(&live, None);
        let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
        debug_policy_bits(*session.builder_mut().comp_ctx.emit_debug_policy())
    });
    let policy_b = crate::test_support::with_env_vars(&debug_policy_updates(false), || {
        let live = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_raw(&live, None);
        let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
        debug_policy_bits(*session.builder_mut().comp_ctx.emit_debug_policy())
    });

    crate::test_support::with_env_vars(&debug_policy_updates(false), || {
        assert_eq!(policy_a, [true, true, true, true, true, true, true, true]);
        assert_eq!(
            policy_b,
            [false, false, false, false, false, false, false, false]
        );
    });
}

#[test]
fn raw_public_snapshot_forces_empty_imports_without_mutating_live() {
    let live = advanced_builder();
    let before_imports = live.comp_ctx.using_import_boxes.clone();
    let config = BuilderInvocationConfigV1::snapshot_for_raw_with_imports(
        &live,
        Some("raw-public.hako"),
        HashMap::new(),
    );

    assert!(config.using_import_boxes().is_empty());
    assert_eq!(config.source_file(), Some("raw-public.hako"));
    assert_eq!(live.comp_ctx.using_import_boxes, before_imports);
}

#[test]
fn continue_live_and_fresh_seed_all_five_core_cursors() {
    let live = advanced_builder();
    let expected = core_cursor(&live);

    let continue_config = BuilderInvocationConfigV1::snapshot_with_policy(
        &live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut continue_session = ModuleBuilderInvocationSessionV1::open(&live, continue_config);
    assert_eq!(core_cursor(continue_session.builder_mut()), expected);

    let fresh_config =
        BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
    let mut fresh_session = ModuleBuilderInvocationSessionV1::open(&live, fresh_config);
    assert_eq!(core_cursor(fresh_session.builder_mut()), (0, 0, 0, 0, 0));
}

#[test]
fn dropping_failed_candidate_leaves_live_builder_unchanged() {
    let live = advanced_builder();
    let before = (
        live.repl_mode,
        live.comp_ctx.quiet_internal_logs,
        live.comp_ctx.using_import_boxes.clone(),
        live.comp_ctx.plugin_method_sigs.clone(),
        live.current_source_file(),
        core_cursor(&live),
    );
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
    session.builder_mut().repl_mode = false;
    session.builder_mut().comp_ctx.using_import_boxes.clear();
    session.builder_mut().next_value_id();
    drop(session);

    assert_eq!(
        (
            live.repl_mode,
            live.comp_ctx.quiet_internal_logs,
            live.comp_ctx.using_import_boxes.clone(),
            live.comp_ctx.plugin_method_sigs.clone(),
            live.current_source_file(),
            core_cursor(&live),
        ),
        before
    );
}

#[test]
fn commit_readiness_rejects_open_slot_state_before_external_commit() {
    let live = MirBuilder::new();
    let config =
        BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
    session.builder_mut().comp_ctx.current_slot_registry =
        Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());
    let error = match session.prepare_external_commit() {
        Ok(_) => panic!("open slot registry must block external commit"),
        Err(error) => error,
    };
    assert_eq!(error, BuilderCommitReadinessErrorV1::SlotRegistryOpen);
    assert_eq!(core_cursor(&live), (0, 0, 0, 0, 0));
}

#[test]
fn commit_readiness_rejects_function_owned_residue() {
    let live = MirBuilder::new();
    let config =
        BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
    session
        .builder_mut()
        .function_state
        .variable_ctx
        .insert("stale".into(), super::ValueId::new(7));
    let error = match session.prepare_external_commit() {
        Ok(_) => panic!("function-owned residue must block external commit"),
        Err(error) => error,
    };
    assert_eq!(error, BuilderCommitReadinessErrorV1::FunctionStateOpen);
}

#[test]
fn prepared_commit_moves_candidate_once_and_reuse_is_fresh() {
    let mut live = MirBuilder::new();
    let config =
        BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
    session.builder_mut().repl_mode = true;
    let prepared = session.prepare_external_commit().unwrap();
    prepared.commit(&mut live);
    assert!(live.repl_mode);

    let fresh = MirBuilder::new();
    assert_eq!(core_cursor(&fresh), (0, 0, 0, 0, 0));
    assert!(live.current_module.is_none());
}
