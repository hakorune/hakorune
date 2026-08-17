use crate::mir::builder::CompilationContext;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{MirBuilder, MirInstruction, MirType};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::common_v2_session::S6CTextEqOperandIssuerRejectV1;
use super::with_common_v2_physical_entry_session;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("S6C operand source");
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
fn s6c_operand_issuer_emits_only_v6_v7_v8_in_one_body_segment() {
    let (installed, context) = installed_port(1301);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        let owner = loan.callable().owner();
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
                        let receipt = canonical
                            .with_s6c_text_eq_operands(draft, scope.receipt(), |draft, receipt| {
                                assert_eq!(receipt.owner(), owner);
                                assert_eq!(receipt.index_key().raw(), 6);
                                assert_eq!(receipt.end_key().raw(), 8);
                                assert_eq!(receipt.substring_result().raw(), 9);
                                assert_eq!(
                                    draft
                                        .function_state
                                        .type_ctx
                                        .get_type(receipt.index_value()),
                                    Some(&MirType::Integer)
                                );
                                assert_eq!(
                                    draft.function_state.type_ctx.get_type(receipt.one_value()),
                                    Some(&MirType::Integer)
                                );
                                assert_eq!(
                                    draft.function_state.type_ctx.get_type(receipt.end_value()),
                                    Some(&MirType::Integer)
                                );
                                assert!(draft.current_function_instructions().iter().any(
                                    |instruction| matches!(
                                        instruction,
                                        MirInstruction::Const {
                                            dst,
                                            value: crate::mir::ConstValue::Integer(1)
                                        } if *dst == receipt.one_value()
                                    )
                                ));
                                assert!(draft.current_function_instructions().iter().any(
                                    |instruction| matches!(
                                        instruction,
                                        MirInstruction::BinOp {
                                            dst, lhs, rhs, ..
                                        } if *dst == receipt.end_value()
                                            && *lhs == receipt.index_value()
                                            && *rhs == receipt.one_value()
                                    )
                                ));
                                Ok::<(), String>(())
                            })
                            .expect("S6C operand receipt");
                        Ok::<(), String>(())
                    })
                    .expect("shared body segment");
                Ok(())
            },
        )
        .expect("caller-zero S6C operand session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn s6c_operand_issuer_rejects_duplicate_before_second_effect() {
    let (installed, context) = installed_port(1302);
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
                        canonical
                            .with_s6c_text_eq_operands(draft, scope.receipt(), |_draft, receipt| {
                                drop(receipt);
                                Ok::<(), String>(())
                            })
                            .expect("first operand receipt");
                        let rejected = canonical.with_s6c_text_eq_operands(
                            draft,
                            scope.receipt(),
                            |_draft, _receipt| Ok::<(), String>(()),
                        );
                        assert!(matches!(
                            rejected,
                            Err(S6CTextEqOperandIssuerRejectV1::AlreadyIssued)
                        ));
                        Ok::<(), String>(())
                    })
                    .expect("shared body segment");
                Ok(())
            },
        )
        .expect("duplicate guard session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn s6c_operand_issuer_missing_seed_rejects_before_const_or_add() {
    let (installed, context) = installed_port(1303);
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
                let rejected = canonical
                    .with_shared_segment_scope(draft, |canonical, draft, scope| {
                        let rejected = canonical.with_s6c_text_eq_operands(
                            draft,
                            scope.receipt(),
                            |_draft, _receipt| Ok::<(), String>(()),
                        );
                        assert!(matches!(
                            rejected,
                            Err(S6CTextEqOperandIssuerRejectV1::Read(_))
                                | Err(S6CTextEqOperandIssuerRejectV1::OperandType(_))
                        ));
                        assert!(!draft.current_function_instructions().iter().any(
                            |instruction| matches!(
                                instruction,
                                MirInstruction::Const {
                                    value: crate::mir::ConstValue::Integer(1),
                                    ..
                                } | MirInstruction::BinOp { .. }
                            )
                        ));
                        Ok::<(), String>(())
                    })
                    .expect("pre-effect rejection is contained");
                Ok(())
            },
        )
        .expect("missing-seed guard session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn s6c_operand_issuer_late_callback_discards_unpublished_body_effects() {
    let (installed, context) = installed_port(1304);
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
                        canonical
                            .with_s6c_text_eq_operands(draft, scope.receipt(), |_draft, receipt| {
                                drop(receipt);
                                Ok::<(), String>(())
                            })
                            .expect("operand receipt");
                        Err::<(), _>("late S6C operand rejection".to_owned())
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(matches!(
            rejected,
            Err(message) if message.contains("late S6C operand rejection")
        ));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
