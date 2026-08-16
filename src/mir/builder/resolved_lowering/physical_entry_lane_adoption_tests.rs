use crate::mir::builder::CompilationContext;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
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
            Ok(())
        })
        .expect("one consuming common-V2 physical entry session");
        assert!(builder.function_state.current_function.is_none());
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
            with_common_v2_physical_entry_session(&mut builder, input, |_canonical, _draft| {
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
