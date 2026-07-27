//! F5-A proof for the exact outer source/physical carrier owner.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::preloop_stageb_carrier::PreparedPreloopStageBFunctionBodyRecipeV1;
use crate::mir::{Callee, MirInstruction, MirModule, MirType, ValueId};

use super::preloop_located_outer_completion::{
    complete_preloop_located_outer_request_v1, CompletedPreloopLocatedOuterRequestV1,
};
use super::preloop_nested_result_test_support::with_actual_parser_stageb_ingress;
use super::preloop_outer_carrier_transaction::{
    complete_preloop_outer_carrier_call_v1, PreloopOuterCarrierCorrespondenceErrorV1,
    PreloopOuterCarrierCorrespondenceStageV1,
};

fn with_actual_outer_physical<R>(
    f: impl for<'site, 'view, 'catalog> FnOnce(
        &mut crate::mir::builder::MirBuilder,
        CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>,
        PreparedPreloopStageBFunctionBodyRecipeV1,
    ) -> R,
) -> R {
    with_actual_parser_stageb_ingress(|mut builder, ingress| {
        builder.current_module = Some(MirModule::new("preloop-outer-carrier".to_owned()));
        ingress
            .with_prepared_located_argument(|prepared, recipe| {
                builder
                    .lower_instance_method_prefix_for_test(
                        "ParserBox",
                        actual_parser_add_fixture::method_declaration_for_lowering(),
                        3,
                        |builder, suffix| {
                            assert!(matches!(suffix.first(), Some(ASTNode::Assignment { .. })));
                            let physical = complete_preloop_located_outer_request_v1(
                                builder,
                                RawLegacyChildLoweringPortV1,
                                prepared,
                            )
                            .expect("exact located outer physical completion");
                            Ok((ValueId::new(0), f(builder, physical, recipe)))
                        },
                    )
                    .expect("configured candidate fixture")
            })
            .expect("actual Parser function ingress")
    })
}

#[test]
fn actual_parser_recipe_seals_the_exact_outer_physical_carrier() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_outer_physical(|builder, physical, recipe| {
            let completed = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("recipe/source/physical correspondence");

            assert!(completed.result_is_integer());
            assert_eq!(completed.assignment_target(), "pos");
            assert_ne!(
                completed.inner_destination(),
                completed.outer_destination()
            );
            assert!(builder.current_function_instructions().iter().any(
                |instruction| matches!(
                    instruction,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Global(symbol)),
                        ..
                    } if symbol == "ParserStringUtilsBox.skip_ws/2"
                        && *dst == completed.outer_destination()
                )
            ));
            assert_ne!(
                builder
                    .function_state
                    .type_ctx
                    .get_type(completed.outer_destination()),
                Some(&MirType::Integer)
            );
            completed.discard();
        });
    });
}

#[test]
fn recipe_selected_index_drift_retains_the_complete_outer_owner() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_outer_physical(|_, physical, mut recipe| {
            recipe.replace_selected_argument_index_for_test(0);
            let rejected = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect_err("recipe/source selected-index drift");
            assert_eq!(
                rejected.stage(),
                PreloopOuterCarrierCorrespondenceStageV1::SelectedArgument
            );
            assert_eq!(
                rejected.cause(),
                PreloopOuterCarrierCorrespondenceErrorV1::SelectedArgumentMismatch
            );
            assert!(rejected.bounded_report().contains("SelectedArgumentMismatch"));
            rejected.discard();
        });
    });
}
