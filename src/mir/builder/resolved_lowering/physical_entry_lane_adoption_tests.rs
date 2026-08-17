use crate::mir::builder::CompilationContext;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{Callee, EffectMask, MirInstruction};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::with_common_v2_physical_entry_session;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("physical entry source");
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

#[test]
fn emits_one_direct_length_call_and_i64_receipt_in_unpublished_session() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(1001).expect("resolver");
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
                let receipt =
                    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
                        canonical.emit_length_call_result(draft)
                    })
                    .expect("direct StringBox.length Call");
                assert_eq!(receipt.owner(), expected_owner);
                assert_eq!(receipt.stamp_owner(), expected_owner);
                assert_ne!(receipt.destination(), crate::mir::ValueId::INVALID);
                assert_eq!(
                    draft
                        .function_state
                        .type_ctx
                        .get_type(receipt.destination()),
                    Some(&crate::mir::MirType::Integer)
                );

                let instructions = draft.current_function_instructions();
                let calls: Vec<_> = instructions
                    .iter()
                    .filter_map(|instruction| match instruction {
                        MirInstruction::Call {
                            dst,
                            callee: Some(callee),
                            args,
                            effects,
                            ..
                        } => Some((*dst, callee, args, *effects)),
                        _ => None,
                    })
                    .collect();
                assert_eq!(calls.len(), 1);
                let (dst, callee, args, effects) = calls[0];
                assert_eq!(dst, Some(receipt.destination()));
                assert_eq!(effects, EffectMask::READ);
                assert!(args
                    .first()
                    .is_some_and(|value| *value != crate::mir::ValueId::INVALID));
                assert!(matches!(
                    callee,
                    Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        ..
                    } if box_name == "StringBox"
                        && method == "length"
                        && *receiver == receipt.receiver()
                ));
                assert!(matches!(
                    canonical.emit_length_call_result(draft),
                    Err(super::common_v2_session::LengthCallDirectEmitterRejectV1::AlreadyIssued)
                ));
                Ok(())
            },
        )
        .expect("caller-zero direct Call session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn direct_length_call_late_failure_discards_call_and_receipt() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(1002).expect("resolver");
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
                let receipt =
                    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
                        canonical.emit_length_call_result(draft)
                    })
                    .expect("direct StringBox.length Call");
                assert_ne!(receipt.destination(), crate::mir::ValueId::INVALID);
                assert!(draft
                    .current_function_instructions()
                    .iter()
                    .any(|instruction| matches!(instruction, MirInstruction::Call { .. })));
                Err::<(), _>("late direct-call rejection".to_owned())
            },
        );
        assert_eq!(rejected, Err("late direct-call rejection".to_owned()));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn adopts_exact_text_slot_once_and_retains_generation_sidecar() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(991).expect("resolver");
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
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let expected_owner = loan.callable().owner();
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let input = skeleton.into_session_input();
        with_common_v2_physical_entry_session(&mut builder, input, |canonical, _draft| {
            assert_eq!(canonical.physical_entry_sidecar_row_count(), 2);
            assert_eq!(canonical.owner(), expected_owner);
            assert_eq!(
                canonical
                    .physical_entry_stamp()
                    .expect("entry stamp")
                    .owner(),
                expected_owner
            );
            Ok(())
        })
        .expect("one consuming common-V2 physical entry session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn length_result_canary_is_same_cohort_and_one_shot() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(996).expect("resolver");
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
            |canonical, _draft| {
                let plan = canonical
                    .issue_length_call_target_plan()
                    .expect("same-cohort StringLen target plan");
                assert_eq!(plan.owner(), expected_owner);
                assert_eq!(plan.box_name(), "StringBox");
                assert_eq!(plan.method_name(), "length");
                let canary = canonical
                    .issue_length_call_materialization_canary()
                    .expect("same-cohort Length canary");
                assert_eq!(canary.owner(), expected_owner);
                assert_eq!(canary.stamp_owner(), expected_owner);
                assert_eq!(plan.item(), canary.call_item());
                assert_eq!(plan.block(), canary.condition_block());
                assert_eq!(plan.result(), canary.result());
                drop(canary);
                assert!(canonical.issue_length_call_target_plan().is_err());
                assert!(canonical
                    .issue_length_call_materialization_canary()
                    .is_err());
                Ok(())
            },
        )
        .expect("Length canary session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn condition_block_target_is_same_session_and_callback_scoped() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(997).expect("resolver");
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
                let expected_condition_block =
                    canonical.envelope().condition_producer().condition_block();
                let target = canonical
                    .with_condition_block_target(draft, |_, target| {
                        assert_eq!(target.owner(), expected_owner);
                        assert_eq!(target.logical_block(), expected_condition_block);
                        assert_eq!(target.stamp_owner(), expected_owner);
                        assert_ne!(target.physical_block().as_u32(), u32::MAX);
                        Ok(target.physical_block())
                    })
                    .expect("same-session condition target");
                assert!(target.as_u32() < u32::MAX);
                Ok(())
            },
        )
        .expect("condition target session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn length_receiver_operand_is_same_session_and_one_shot() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(999).expect("resolver");
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
                let segments = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("source segment blocks");
                let expected_binding = match canonical.envelope().condition_operands().rows()[1].kind()
                {
                    crate::mir::loop_recipe_contract::PreparedLoopV2ConditionOperandKindV1::LengthCall {
                        source,
                    } => source.receiver_binding().expect("local receiver"),
                    crate::mir::loop_recipe_contract::PreparedLoopV2ConditionOperandKindV1::ReadBinding {
                        ..
                    } => panic!("Length row must be a call"),
                };
                canonical
                    .with_length_receiver_operand(draft, &segments, |_, receiver| {
                        assert_eq!(receiver.owner(), expected_owner);
                        assert_eq!(receiver.binding(), expected_binding);
                        assert_eq!(receiver.stamp_owner(), expected_owner);
                        assert_ne!(receiver.physical_block().as_u32(), u32::MAX);
                        assert_ne!(receiver.physical_value(), crate::mir::ValueId::INVALID);
                        Ok(())
                    })
                    .expect("same-session Length receiver operand");
                let second = canonical.with_length_receiver_operand(draft, &segments, |_, _| {
                    Ok(())
                });
                assert!(matches!(
                    second,
                    Err(super::common_v2_session::LengthReceiverPhysicalOperandRejectV1::AlreadyIssued)
                ));
                Ok(())
            },
        )
        .expect("receiver operand session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn length_receiver_operand_late_failure_discards_unpublished_session() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(1000).expect("resolver");
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
                let segments = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("source segment blocks");
                canonical
                    .with_length_receiver_operand(draft, &segments, |_, _| {
                        Err::<(), _>("late receiver rejection".to_owned())
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(matches!(
            rejected,
            Err(message) if message.contains("late receiver rejection")
        ));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn condition_block_target_late_failure_discards_unpublished_session() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(998).expect("resolver");
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
                canonical
                    .with_condition_block_target(draft, |_, _target| {
                        Err::<(), _>("late condition target rejection".to_owned())
                    })
                    .map(|_| ())
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(
            matches!(rejected, Err(message) if message.contains("late condition target rejection"))
        );
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn late_callback_failure_discards_builder_and_physical_session() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(992).expect("resolver");
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
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let mut builder = MirBuilder::new();
        let input = skeleton.into_session_input();
        let rejected =
            with_common_v2_physical_entry_session(&mut builder, input, |canonical, _draft| {
                canonical
                    .issue_length_call_target_plan()
                    .expect("target plan before late rejection");
                canonical
                    .issue_length_call_materialization_canary()
                    .expect("Length canary before late rejection");
                Err::<(), _>("late canary rejection".to_owned())
            });
        assert_eq!(rejected, Err("late canary rejection".to_owned()));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn allocates_only_source_segment_blocks() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(993).expect("resolver");
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
                let receipt = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("segment blocks");
                assert_eq!(receipt.rows().len(), 3);
                assert!(receipt
                    .rows()
                    .windows(2)
                    .all(|rows| rows[0].physical_block() != rows[1].physical_block()));
                Ok(())
            },
        )
        .expect("segment allocation session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn segment_allocation_late_failure_discards_unpublished_blocks() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(994).expect("resolver");
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
                let receipt = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("segment blocks");
                assert_eq!(receipt.rows().len(), 3);
                let after = canonical
                    .allocate_v2_after_block(draft, &receipt)
                    .expect("After block");
                assert!(draft
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("unpublished function")
                    .get_block(after.physical_block())
                    .is_some());
                Err::<(), _>("late segment rejection".to_owned())
            },
        );
        assert_eq!(rejected, Err("late segment rejection".to_owned()));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn after_allocation_is_one_shot_and_unpublished() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(995).expect("resolver");
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
                let segment_receipt = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("segment blocks");
                let next = draft.core_ctx.peek_next_block();
                let view = canonical
                    .allocate_v2_after_block(draft, &segment_receipt)
                    .expect("one unpublished After block");
                assert_eq!(view.physical_block(), next);
                assert!(draft
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("unpublished function")
                    .get_block(view.physical_block())
                    .is_some());
                drop(view);

                let second = canonical.allocate_v2_after_block(draft, &segment_receipt);
                assert!(matches!(
                    second,
                    Err(super::common_v2_after_block_allocation::AfterBlockAllocationRejectV1::
                        AlreadyAllocated)
                ));
                Ok(())
            },
        )
        .expect("After allocation session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
