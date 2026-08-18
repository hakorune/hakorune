use crate::mir::builder::CompilationContext;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::pinned_text_residence_lifecycle::PreparedPinnedTextResidenceLifecycleV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{MirBuilder, MirInstruction};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    with_common_v2_physical_entry_session_with_s6c_loan,
    with_common_v2_s6c_physical_entry_draft_seal,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("cursor CFG source");
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

fn pinned_text_count(builder: &MirBuilder) -> usize {
    builder
        .current_function_instructions()
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::PinnedTextOp { .. }))
        .count()
}

fn source_result(
    result: Result<(), String>,
) -> Result<(), crate::mir::loop_recipe_contract::S6CScalarScanSourceRejectV1> {
    result.map_err(|_| {
        crate::mir::loop_recipe_contract::S6CScalarScanSourceRejectV1::CompletionRelation
    })
}

#[test]
fn lifecycle_entry_boundary_places_seed_on_execution_successor() {
    let (installed, context) = installed_port(1511);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        with_common_v2_physical_entry_session_with_s6c_loan(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft, _loan| {
                let execution_entry = canonical
                    .issue_physical_entry_execution_boundary(draft)
                    .map_err(|error| format!("{error:?}"))?;
                assert!(canonical
                    .issue_physical_entry_execution_boundary(draft)
                    .is_err());
                let trap_entry = canonical
                    .create_unpublished_block(draft)
                    .map_err(|error| format!("{error:?}"))?;
                let plans = PinnedTextAccessPlanTableV1::new(17);
                let frame_contract =
                    PinnedTextBackendFrameContractV1::from_test(canonical.owner(), 17, 1);
                let carrier = PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
                    canonical.owner(),
                    &plans,
                    frame_contract.borrow(),
                    execution_entry,
                    trap_entry,
                )
                .map_err(|error| format!("{error:?}"))?;
                let _finish = canonical
                    .emit_pinned_text_residence_enter(draft, carrier)
                    .map_err(|error| format!("{error:?}"))?;
                canonical
                    .select_block(draft, execution_entry)
                    .map_err(|error| format!("{error:?}"))?;
                let seed = canonical
                    .emit_initial_index_seed(draft)
                    .map_err(|error| format!("{error:?}"))?;
                assert_eq!(seed.physical_block(), execution_entry);

                let function = draft
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("unpublished function");
                assert!(matches!(
                    function
                        .get_block(function.entry_block)
                        .expect("function entry")
                        .terminator,
                    Some(MirInstruction::PinnedTextResidenceEnter {
                        normal_landing,
                        ..
                    }) if normal_landing == execution_entry
                ));
                Ok(())
            },
        )
        .map_err(|error| format!("{error:?}"))
    })
    .expect("one E0/E1 lifecycle boundary")
    .expect("lifecycle entry boundary");
    port.complete().expect("selected child coverage");
}

#[test]
fn cursor_cfg_consumes_typed_condition_and_same_cohort_source() {
    let (installed, context) = installed_port(1501);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        with_common_v2_physical_entry_session_with_s6c_loan(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft, loan| {
                loan.callable()
                    .with_completion(|completion| {
                        loan.callable()
                            .with_scalar_scan_source(|source| {
                                let result = (|| {
                                    let seed = canonical
                                        .emit_initial_index_seed(draft)
                                        .map_err(|error| format!("{error:?}"))?;
                                    drop(seed);
                                    canonical
                                        .with_shared_segment_scope(
                                            draft,
                                            |canonical, draft, scope| {
                                                let cursor = canonical
                                                    .consume_s6c_cursor_cfg(
                                                        draft, &scope, source, completion,
                                                    )
                                                    .map_err(|error| format!("{error:?}"))?;
                                                assert_ne!(
                                                    cursor.text_equal_value(),
                                                    cursor.width_value()
                                                );
                                                assert_ne!(
                                                    cursor.text_equal_value(),
                                                    cursor.loop_condition()
                                                );
                                                assert_eq!(pinned_text_count(draft), 3);
                                                drop(cursor);
                                                drop(scope);
                                                Ok(())
                                            },
                                        )
                                        .map_err(|error| format!("{error:?}"))
                                })();
                                source_result(result)
                            })
                            .map_err(|error| format!("{error:?}"))
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        )
        .expect("typed cursor CFG handoff");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn cursor_cfg_late_failure_discards_typed_handoff() {
    let (installed, context) = installed_port(1502);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let rejected = with_common_v2_physical_entry_session_with_s6c_loan(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft, loan| {
                loan.callable()
                    .with_completion(|completion| {
                        loan.callable()
                            .with_scalar_scan_source(|source| {
                                source_result((|| {
                                    let seed = canonical
                                        .emit_initial_index_seed(draft)
                                        .map_err(|error| format!("{error:?}"))?;
                                    drop(seed);
                                    canonical
                                        .with_shared_segment_scope(
                                            draft,
                                            |canonical, draft, scope| {
                                                let cursor = canonical
                                                    .consume_s6c_cursor_cfg(
                                                        draft, &scope, source, completion,
                                                    )
                                                    .map_err(|error| format!("{error:?}"))?;
                                                assert_ne!(
                                                    cursor.text_equal_value(),
                                                    cursor.loop_condition()
                                                );
                                                assert_eq!(pinned_text_count(draft), 3);
                                                drop(cursor);
                                                drop(scope);
                                                Err::<(), _>("late cursor CFG rejection".to_owned())
                                            },
                                        )
                                        .map_err(|error| format!("{error:?}"))
                                })())
                            })
                            .map_err(|error| format!("{error:?}"))
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(matches!(
            rejected,
            Err(error) if error.contains("CompletionRelation")
        ));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn draftseal_ingress_consumes_same_outer_transaction() {
    let (installed, context) = installed_port(1503);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let function = with_common_v2_s6c_physical_entry_draft_seal(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft, _physical_effects, loan| {
                loan.callable()
                    .with_completion(|completion| {
                        loan.callable()
                            .with_scalar_scan_source(|source| {
                                let result = (|| -> Result<(), String> {
                                    let seed = canonical
                                        .emit_initial_index_seed(draft)
                                        .map_err(|error| format!("{error:?}"))?;
                                    drop(seed);
                                    canonical
                                        .with_shared_segment_scope(
                                            draft,
                                            |canonical, draft, scope| {
                                                let cursor = canonical
                                                    .consume_s6c_cursor_cfg(
                                                        draft, &scope, source, completion,
                                                    )
                                                    .map_err(|error| format!("{error:?}"))?;
                                                assert_eq!(
                                                    draft.function_state.current_block,
                                                    Some(cursor.after_block())
                                                );
                                                assert_eq!(pinned_text_count(draft), 3);
                                                drop(cursor);
                                                drop(scope);
                                                Ok(())
                                            },
                                        )
                                        .map_err(|error| format!("{error:?}"))
                                })();
                                source_result(result)
                            })
                            .map_err(|error| format!("{error:?}"))
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        )
        .expect("caller-zero DraftSeal ingress");
        assert!(!function.blocks.is_empty());
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn draftseal_ingress_discards_outer_on_tail_callback_failure() {
    let (installed, context) = installed_port(1504);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let rejected = with_common_v2_s6c_physical_entry_draft_seal(
            &mut builder,
            skeleton.into_session_input(),
            |canonical, draft, _physical_effects, loan| {
                loan.callable()
                    .with_completion(|completion| {
                        loan.callable()
                            .with_scalar_scan_source(|source| {
                                let result = (|| -> Result<(), String> {
                                    let seed = canonical
                                        .emit_initial_index_seed(draft)
                                        .map_err(|error| format!("{error:?}"))?;
                                    drop(seed);
                                    canonical
                                        .with_shared_segment_scope(
                                            draft,
                                            |canonical, draft, scope| {
                                                let cursor = canonical
                                                    .consume_s6c_cursor_cfg(
                                                        draft, &scope, source, completion,
                                                    )
                                                    .map_err(|error| format!("{error:?}"))?;
                                                drop(cursor);
                                                drop(scope);
                                                Err::<(), _>("tail callback rejection".to_owned())
                                            },
                                        )
                                        .map_err(|error| format!("{error:?}"))
                                })();
                                source_result(result)
                            })
                            .map_err(|error| format!("{error:?}"))
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(rejected.is_err());
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
