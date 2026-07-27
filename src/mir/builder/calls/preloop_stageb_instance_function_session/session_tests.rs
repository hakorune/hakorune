//! Focused F6-3 proofs for the unpublished Stage-B function session.

use crate::mir::builder::calls::preloop_nested_result_test_support::{
    actual_parser_stageb_ingress, with_actual_parser_stageb_ingress,
};
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::preloop_stageb_carrier::PreparedPreloopStageBFunctionIngressV1;
use crate::mir::{MirBuilder, MirInstruction, MirModule, MirType};
use crate::parser::NyashParser;

use super::session::{
    capture_preloop_stageb_instance_function_v1, CompletedPreloopStageBInstanceFunctionV1,
    PreparedPreloopStageBInstanceFunctionV1,
};
use super::session_rejection::{
    PreloopStageBInstanceFunctionStageV1, RejectedPreloopStageBInstanceFunctionSessionV1,
};

fn run_session(
    builder: &mut MirBuilder,
    ingress: PreparedPreloopStageBFunctionIngressV1,
) -> Result<CompletedPreloopStageBInstanceFunctionV1, RejectedPreloopStageBInstanceFunctionSessionV1>
{
    let prepared = PreparedPreloopStageBInstanceFunctionV1::prepare(ingress)
        .expect("exact instance source projection");
    let mut invocation =
        ModuleLoweringInvocationV1::with_collector(builder, ModuleDraftCollectorV1::default());
    invocation.with_module_port(|builder, module_port| {
        let pending = capture_preloop_stageb_instance_function_v1(
            builder,
            RawInvocationChildPortV1::new(module_port),
            prepared,
        )?;
        assert!(pending.parent_is_captured_for_test());
        Ok(pending.complete())
    })
}

#[test]
fn phase_a_indexed_actual_parser_completes_one_unpublished_stageb_function() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_parser_stageb_ingress(|mut builder, ingress| {
            let root = NyashParser::parse_from_string(
                &actual_parser_add_fixture::stageb_source_for_lowering(),
            )
            .expect("actual Stage-B source");
            builder.current_module = Some(MirModule::new("stageb-session".to_owned()));
            crate::mir::builder::declaration_indexer::index_declarations(&mut builder, &root);
            builder.enter_function_for_test("parent/0".to_owned());

            let completed = run_session(&mut builder, ingress)
                .unwrap_or_else(|rejected| panic!("{}", rejected.bounded_report()));

            assert_eq!(
                builder
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("parent restored")
                    .signature
                    .name,
                "parent/0"
            );
            assert_eq!(
                completed.draft().signature.name,
                "ParserBox.static_const_parse_add/2"
            );
            assert_eq!(
                completed.draft().signature.return_type,
                MirType::Unknown,
                "the unannotated function signature remains distinct from the Integer carrier"
            );
            assert_eq!(completed.draft().signature.params.len(), 3);
            let call_targets = completed
                .draft()
                .blocks
                .values()
                .flat_map(|block| block.instructions.iter())
                .filter_map(|instruction| match instruction {
                    MirInstruction::Call { callee, .. } => Some(format!("{callee:?}")),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let inner = call_targets
                .iter()
                .position(|target| target.contains("static_const_eval_pos"))
                .unwrap_or_else(|| panic!("inner physical Call: {call_targets:?}"));
            let outer = call_targets
                .iter()
                .position(|target| target.contains("ParserStringUtilsBox.skip_ws/2"))
                .expect("outer physical Call");
            assert!(inner < outer, "inner Call must precede outer Call");
            assert_ne!(
                completed.payload().schedule().carrier().inner_destination(),
                completed.payload().schedule().carrier().outer_destination()
            );
            assert_eq!(
                completed
                    .payload()
                    .schedule()
                    .carrier()
                    .assigned_destination(),
                completed.payload().schedule().carrier().outer_destination()
            );
            assert!(!builder
                .current_module
                .as_ref()
                .expect("candidate module")
                .functions
                .contains_key("ParserBox.static_const_parse_add/2"));
            let (_draft, payload) = completed.into_parts();
            payload.discard();
        });
    });
}

#[test]
fn suffix_failure_restores_parent_retains_carrier_then_fresh_session_succeeds() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        let (mut builder, first_ingress) = actual_parser_stageb_ingress();
        builder.current_module = Some(MirModule::new("stageb-session-reuse".to_owned()));
        builder.enter_function_for_test("parent/0".to_owned());

        let rejected = run_session(&mut builder, first_ingress)
            .expect_err("without Phase A indexing the exact suffix must fail");
        assert_eq!(
            rejected.stage(),
            PreloopStageBInstanceFunctionStageV1::BodySchedule,
            "{}",
            rejected.bounded_report()
        );
        assert!(rejected.retains_published_carrier_for_test());
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("parent restored after typed primary failure")
                .signature
                .name,
            "parent/0"
        );
        assert!(!builder
            .current_module
            .as_ref()
            .expect("candidate module")
            .functions
            .contains_key("ParserBox.static_const_parse_add/2"));
        rejected.discard();

        let (_, fresh_ingress) = actual_parser_stageb_ingress();
        let root = NyashParser::parse_from_string(
            &actual_parser_add_fixture::stageb_source_for_lowering(),
        )
        .expect("actual Stage-B source");
        crate::mir::builder::declaration_indexer::index_declarations(&mut builder, &root);
        let completed = run_session(&mut builder, fresh_ingress)
            .unwrap_or_else(|rejected| panic!("{}", rejected.bounded_report()));
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("parent restored after fresh success")
                .signature
                .name,
            "parent/0"
        );
        let (_draft, payload) = completed.into_parts();
        payload.discard();
    });
}
