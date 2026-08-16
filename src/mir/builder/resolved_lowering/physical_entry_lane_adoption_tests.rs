use crate::mir::builder::CompilationContext;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;

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
        let prepared =
            issue_common_v2_physical_function_entry_input(loan).expect("physical entry input");
        let skeleton =
            reserve_common_v2_physical_function_skeleton(prepared).expect("physical skeleton");
        let (retained_loan, detached, descriptors) = skeleton.into_parts();
        let input = retained_loan.callable().selected().source();
        let loop_site = input.function().only_loop_site().expect("one loop");
        let completion = verify_function_completion_v1(input).expect("completion");
        let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
            input,
            loop_site.node(),
        )
        .expect("loop-only control");
        let mut canonical =
            CanonicalSsaFunctionSessionV2::new(input, if_control, completion, 0).expect("session");
        let function_name = detached.signature.name.clone();

        let mut builder = MirBuilder::new();
        let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
        {
            let draft = outer.builder_view_mut_for_lowering();
            draft
                .function_state
                .resolved_binding_state
                .install(input.function())
                .expect("resolver authority");
            draft
                .install_prepared_physical_function_skeleton(detached)
                .expect("install physical skeleton");
            canonical
                .adopt_physical_entry_lanes(draft, &descriptors)
                .expect("adopt physical entry lanes");
        }
        assert_eq!(canonical.physical_entry_sidecar_row_count(), 2);
        assert_eq!(
            outer
                .builder_view()
                .function_state
                .current_function
                .as_ref()
                .expect("installed function")
                .params
                .len(),
            4
        );
        let rejected = {
            let draft = outer.builder_view_mut_for_lowering();
            canonical.adopt_physical_entry_lanes(draft, &descriptors)
        };
        assert!(
            rejected.is_err(),
            "a session may adopt entry lanes only once"
        );
        assert_eq!(canonical.physical_entry_sidecar_row_count(), 2);

        outer.discard_unpublished();
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
