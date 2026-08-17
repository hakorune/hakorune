use crate::mir::builder::CompilationContext;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{MirBuilder, MirInstruction, MirType};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::common_v2_session::{
    ConditionBoolMaterializationRejectV1, ConditionBoolReturnReadRejectV1,
};
use super::with_common_v2_physical_entry_session;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("condition bool source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn installed_port(
    ordinal: u32,
) -> (
    crate::mir::normal_callable_semantic_package::InstalledNormalCallableSemanticPackageV1,
    CompilationContext,
) {
    let mut resolver = FunctionSemanticResolverSessionV1::new(ordinal).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source(include_str!(
            "../../../../apps/tests/scan_with_init_typed_ok_min.hako"
        )),
    )
    .expect("same-cohort package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    (installed, context)
}

#[test]
fn condition_bool_consumes_length_receipt_and_emits_one_less() {
    let (installed, context) = installed_port(1201);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let expected_owner = loan.callable().owner();
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                let seed = canonical
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                drop(seed);
                let length =
                    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
                        canonical.emit_length_call_result(draft)
                    })
                    .expect("length result");
                let condition = length
                    .consume_for_condition_bool(draft)
                    .expect("condition Bool");
                assert_eq!(condition.owner(), expected_owner);
                assert_eq!(
                    draft
                        .function_state
                        .type_ctx
                        .get_type(condition.destination()),
                    Some(&MirType::Bool)
                );
                assert!(draft
                    .current_function_instructions()
                    .iter()
                    .any(|instruction| matches!(
                        instruction,
                        MirInstruction::Compare { dst, op, lhs, rhs }
                            if *dst == condition.destination()
                                && *op == crate::mir::CompareOp::Lt
                                && *lhs == condition.left()
                                && *rhs == condition.right()
                    )));
                drop(condition);
                Ok(())
            },
        )
        .expect("caller-zero condition session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn condition_bool_rejects_before_compare_when_seed_is_missing() {
    let (installed, context) = installed_port(1202);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                let length =
                    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
                        canonical.emit_length_call_result(draft)
                    })
                    .expect("length result");
                let rejected = match length.consume_for_condition_bool(draft) {
                    Ok(_) => panic!("missing seed unexpectedly produced Bool"),
                    Err(error) => error,
                };
                assert!(matches!(
                    rejected,
                    ConditionBoolMaterializationRejectV1::LeftRead(_)
                ));
                assert!(!draft
                    .current_function_instructions()
                    .iter()
                    .any(|instruction| matches!(instruction, MirInstruction::Compare { .. })));
                Ok(())
            },
        )
        .expect("caller-zero missing-seed session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn condition_bool_late_failure_discards_compare_and_receipt() {
    let (installed, context) = installed_port(1203);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let rejected = with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                let seed = canonical
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                drop(seed);
                let length =
                    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
                        canonical.emit_length_call_result(draft)
                    })
                    .expect("length result");
                let condition = length
                    .consume_for_condition_bool(draft)
                    .expect("condition Bool");
                assert!(draft
                    .current_function_instructions()
                    .iter()
                    .any(|instruction| matches!(instruction, MirInstruction::Compare { .. })));
                drop(condition);
                Err::<(), _>("late condition rejection".to_owned())
            },
        );
        assert_eq!(rejected, Err("late condition rejection".to_owned()));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn shared_segment_scope_threads_length_into_condition_bool() {
    let (installed, context) = installed_port(1204);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                let seed = canonical
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                drop(seed);
                canonical
                    .with_shared_segment_scope(draft, |canonical, draft, scope| {
                        let length = crate::test_support::with_env_var(
                            "NYASH_MIR_UNIFIED_CALL",
                            "1",
                            || canonical.emit_length_call_result_from_scope(draft, &scope),
                        )
                        .map_err(|error| format!("{error:?}"))?;
                        let condition = length
                            .consume_for_condition_bool(draft)
                            .map_err(|error| format!("{error:?}"))?;
                        assert_eq!(condition.logical_result().raw(), 5);
                        assert_eq!(
                            draft
                                .function_state
                                .type_ctx
                                .get_type(condition.destination()),
                            Some(&MirType::Bool)
                        );
                        drop(condition);
                        drop(scope);
                        Ok(())
                    })
                    .map_err(|error| format!("{error:?}"))?;
                Ok(())
            },
        )
        .expect("shared segment scope session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn shared_segment_scope_rejects_second_allocation() {
    let (installed, context) = installed_port(1205);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                canonical
                    .with_shared_segment_scope(draft, |_canonical, _draft, _scope| {
                        Ok::<(), String>(())
                    })
                    .expect("first shared segment scope");
                let second = canonical
                    .with_shared_segment_scope(draft, |_canonical, _draft, _scope| {
                        Ok::<(), String>(())
                    });
                assert!(matches!(
                    second,
                    Err(super::common_v2_session::SharedSegmentScopeRejectV1::Allocation(
                        message
                    )) if message.contains("AlreadyIssued")
                ));
                Ok(())
            },
        )
        .expect("second-allocation guard session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn shared_segment_scope_rejects_return_read_condition_mismatch() {
    let (installed, context) = installed_port(1206);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let rejected = with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                let seed = canonical
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                drop(seed);
                canonical
                    .with_shared_segment_scope(draft, |canonical, draft, scope| {
                        let length = crate::test_support::with_env_var(
                            "NYASH_MIR_UNIFIED_CALL",
                            "1",
                            || canonical.emit_length_call_result_from_scope(draft, &scope),
                        )
                        .map_err(|error| format!("{error:?}"))?;
                        let condition = length
                            .consume_for_condition_bool(draft)
                            .map_err(|error| format!("{error:?}"))?;
                        let rejected = condition.with_return_read_physical_receipt(
                            draft,
                            scope,
                            |_draft, _receipt| Ok::<(), String>(()),
                        );
                        assert!(matches!(
                            rejected,
                            Err(ConditionBoolReturnReadRejectV1::ConditionLogicalMismatch)
                        ));
                        Ok(())
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        )
        .expect("shared Return-read mismatch guard session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn shared_segment_scope_late_callback_discards_everything() {
    let (installed, context) = installed_port(1207);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let rejected = with_common_v2_physical_entry_session(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft| {
                let seed = canonical
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                drop(seed);
                canonical
                    .with_shared_segment_scope(draft, |canonical, draft, scope| {
                        let length = crate::test_support::with_env_var(
                            "NYASH_MIR_UNIFIED_CALL",
                            "1",
                            || canonical.emit_length_call_result_from_scope(draft, &scope),
                        )
                        .map_err(|error| format!("{error:?}"))?;
                        let condition = length
                            .consume_for_condition_bool(draft)
                            .map_err(|error| format!("{error:?}"))?;
                        drop(condition);
                        Err::<(), _>("late shared-scope rejection".to_owned())
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(matches!(
            rejected,
            Err(message) if message.contains("late shared-scope rejection")
        ));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
