use crate::mir::builder::CompilationContext;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::loop_recipe_contract::LoopJoinBranchArmTransferRefV2;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::with_common_v2_physical_entry_session;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("physical target source");
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

fn continuation_item(
    canonical: &super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
) -> crate::mir::loop_recipe_contract::LoopJoinNextItemV1 {
    let branch = canonical.envelope().control().transfer().branches()[0];
    match branch.else_arm {
        LoopJoinBranchArmTransferRefV2::Fallthrough { continuation, .. } => continuation,
        LoopJoinBranchArmTransferRefV2::Exit(_) => panic!("S6C fallthrough arm missing"),
    }
}

#[test]
fn continuation_target_placement_is_callback_scoped_and_one_shot() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(1010).expect("resolver");
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
                let continuation = continuation_item(canonical);
                let expected_split = segments
                    .rows()
                    .iter()
                    .find(|row| row.logical_block() == continuation.block)
                    .expect("continuation source row")
                    .split_ordinal();
                let target = canonical
                    .with_if_continuation_target(draft, &segments, |draft, target| {
                        assert_eq!(target.owner(), expected_owner);
                        assert_eq!(target.if_item().raw(), 8);
                        assert_eq!(target.continuation(), continuation);
                        assert_eq!(target.source_block(), continuation.block);
                        assert_eq!(target.source_split_ordinal(), expected_split);
                        assert_eq!(target.stamp_owner(), expected_owner);
                        assert!(draft
                            .function_state
                            .current_function
                            .as_ref()
                            .expect("unpublished function")
                            .get_block(target.physical_block())
                            .is_some());
                        assert!(segments
                            .rows()
                            .iter()
                            .all(|row| row.physical_block() != target.physical_block()));
                        Ok(target.physical_block())
                    })
                    .expect("one continuation target");
                assert_ne!(target.as_u32(), u32::MAX);

                let second = canonical.with_if_continuation_target(draft, &segments, |_, _| Ok(()));
                assert!(matches!(
                    second,
                    Err(super::common_v2_if_continuation_target::
                        IfContinuationPhysicalTargetRejectV1::AlreadyIssued)
                ));
                Ok(())
            },
        )
        .expect("continuation target session");
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn continuation_target_late_failure_discards_unpublished_block() {
    let mut resolver = FunctionSemanticResolverSessionV1::new(1011).expect("resolver");
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
                    .with_if_continuation_target(draft, &segments, |_, _| {
                        Err::<(), _>("late continuation target rejection".to_owned())
                    })
                    .map_err(|error| format!("{error:?}"))
            },
        );
        assert!(matches!(
            rejected,
            Err(message) if message.contains("late continuation target rejection")
        ));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
