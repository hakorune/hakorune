//! CUT0-I0-SESSION0 fixtures for the disconnected Builder transaction.

use super::module_invocation_session::{
    BuilderCoreIdSeedV1, BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1,
    BuilderCommitReadinessErrorV1, ModuleBuilderInvocationSessionV1,
};
use super::{MirBuilder, MirType};

fn advanced_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.repl_mode = true;
    builder.comp_ctx.quiet_internal_logs = true;
    builder
        .comp_ctx
        .using_import_boxes
        .insert("Alias".into(), "Imported".into());
    builder.comp_ctx.plugin_method_sigs.insert(
        ("PluginBox".into(), "value/0".into()),
        MirType::Integer,
    );
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
    assert_eq!(config.quiet_internal_logs(), candidate.comp_ctx.quiet_internal_logs);
    assert_eq!(config.using_import_boxes(), &candidate.comp_ctx.using_import_boxes);
    assert_eq!(config.plugin_method_sigs(), &candidate.comp_ctx.plugin_method_sigs);
    assert_eq!(config.source_file(), candidate.current_source_file().as_deref());
    assert!(matches!(
        config.core_id_seed(),
        BuilderCoreIdSeedV1::ContinueLive(_)
    ));
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
    let mut live = advanced_builder();
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
    let mut live = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &live,
        BuilderCoreSeedPolicyV1::Fresh,
    );
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
    session.builder_mut().comp_ctx.current_slot_registry = Some(
        crate::mir::region::function_slot_registry::FunctionSlotRegistry::new(),
    );
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
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &live,
        BuilderCoreSeedPolicyV1::Fresh,
    );
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
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        &live,
        BuilderCoreSeedPolicyV1::Fresh,
    );
    let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
    session.builder_mut().repl_mode = true;
    let prepared = session.prepare_external_commit().unwrap();
    prepared.commit(&mut live);
    assert!(live.repl_mode);

    let fresh = MirBuilder::new();
    assert_eq!(core_cursor(&fresh), (0, 0, 0, 0, 0));
    assert!(live.current_module.is_none());
}
