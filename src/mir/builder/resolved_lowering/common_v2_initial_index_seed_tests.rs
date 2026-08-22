use crate::mir::builder::CompilationContext;
use crate::mir::compiler::common_v2_physical_function_entry_input::issue_common_v2_physical_function_entry_input;
use crate::mir::compiler::common_v2_physical_function_skeleton::reserve_common_v2_physical_function_skeleton;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{ConstValue, MirBuilder, MirInstruction, MirType};
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::common_v2_session::InitialIndexSeedMaterializationRejectV1;
use super::with_common_v2_physical_entry_session;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("initial seed source");
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
fn initial_index_seed_emits_one_entry_const_and_exact_declaration() {
    let (installed, context) = installed_port(1101);
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
                let receipt = canonical
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                assert_eq!(receipt.owner(), owner);
                assert_eq!(receipt.binding().owner(), owner);
                assert_eq!(receipt.carrier_entry().raw(), 2);
                assert_eq!(
                    draft.function_state.type_ctx.get_type(receipt.value()),
                    Some(&MirType::Integer)
                );
                assert!(draft
                    .current_function_instructions()
                    .iter()
                    .any(|instruction| matches!(
                        instruction,
                        MirInstruction::Const {
                            dst,
                            value: ConstValue::Integer(0)
                        } if *dst == receipt.value()
                    )));
                drop(receipt);
                assert!(matches!(
                    canonical.emit_initial_index_seed(draft),
                    Err(InitialIndexSeedMaterializationRejectV1::AlreadyIssued)
                ));
                Ok(())
            },
        )
        .expect("caller-zero seed session");
        assert!(builder.function_state.current_function.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn initial_index_seed_rejects_missing_function_before_effect() {
    let (installed, context) = installed_port(1102);
    let mut port = installed.begin_lowering(&context).expect("same catalog");

    port.with_s6c_common_v2_pre_session(|loan| {
        crate::mir::compiler::common_v2_session_admission::with_loop_v2_canonical_session_admission(
            &loan,
            |admission| {
                crate::mir::builder::with_common_v2_canonical_session(admission, |canonical| {
                    let mut builder = MirBuilder::new();
                    assert!(matches!(
                        canonical.emit_initial_index_seed(&mut builder),
                        Err(InitialIndexSeedMaterializationRejectV1::MissingFunction)
                    ));
                })
                .expect("canonical session open");
            },
        )
        .expect("callback-scoped admission");
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}

#[test]
fn initial_index_seed_late_failure_discards_const_and_declaration() {
    let (installed, context) = installed_port(1103);
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
                    .emit_initial_index_seed(draft)
                    .expect("initial index seed");
                assert!(!draft.current_function_instructions().is_empty());
                drop(receipt);
                Err::<(), _>("late seed rejection".to_owned())
            },
        );
        assert_eq!(rejected, Err("late seed rejection".to_owned()));
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
    })
    .expect("one installed S6C callback");
    port.complete().expect("selected child coverage");
}
