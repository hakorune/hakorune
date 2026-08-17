use crate::mir::builder::CompilationContext;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::loop_recipe_contract::LoopJoinBranchExitTargetV2;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::with_common_v2_physical_entry_session;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("physical return-read source");
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

fn session_fixture(
    resolver_seed: u32,
) -> (
    crate::mir::normal_callable_semantic_package::InstalledNormalCallableSemanticPackageV1,
    CompilationContext,
) {
    let mut resolver = FunctionSemanticResolverSessionV1::new(resolver_seed).expect("resolver");
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
fn return_read_physical_receipt_joins_read_segments_and_completion() {
    let (installed, context) = session_fixture(1020);
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
                    .expect("source local entry seed");
                drop(seed);
                let segments = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("source segment blocks");
                canonical
                    .with_return_read_physical_receipt(draft, &segments, |draft, receipt| {
                        assert_eq!(receipt.owner(), expected_owner);
                        assert_eq!(receipt.return_item().raw(), 9);
                        assert_eq!(receipt.logical_result().raw(), 11);
                        assert_eq!(receipt.then_block().raw(), 2);
                        assert_eq!(receipt.if_item().raw(), 8);
                        assert_eq!(receipt.if_block().raw(), 1);
                        assert_eq!(receipt.continuation().item.raw(), 11);
                        assert_eq!(receipt.exit_item().raw(), 10);
                        assert_eq!(
                            receipt.join_target(),
                            LoopJoinBranchExitTargetV2::FunctionExit
                        );
                        assert_eq!(receipt.terminal_block(), receipt.then_physical_block());
                        assert_ne!(
                            receipt.continuation_physical_block(),
                            receipt.if_physical_block()
                        );
                        assert!(draft
                            .function_state
                            .current_function
                            .as_ref()
                            .expect("unpublished function")
                            .get_block(receipt.then_physical_block())
                            .is_some());
                        Ok(())
                    })
                    .expect("physical Return-read receipt");
                let second =
                    canonical.with_return_read_physical_receipt(draft, &segments, |_, _| {
                        Ok::<(), String>(())
                    });
                assert!(matches!(
                    second,
                    Err(super::common_v2_session::ReturnReadPhysicalReceiptRejectV1::AlreadyIssued)
                ));
                Ok(())
            },
        )
        .expect("return-read receipt session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn return_read_physical_receipt_late_failure_discards_read_and_target() {
    let (installed, context) = session_fixture(1021);
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
                    .expect("source local entry seed");
                drop(seed);
                let segments = canonical
                    .allocate_v2_segment_blocks(draft)
                    .expect("source segment blocks");
                canonical
                    .with_return_read_physical_receipt(draft, &segments, |_, _| {
                        Err::<(), _>("late Return-read rejection".to_owned())
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(matches!(
            rejected,
            Err(message) if message.contains("late Return-read rejection")
        ));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
