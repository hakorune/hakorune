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
use super::preloop_outer_carrier_assignment::{
    complete_preloop_carrier_assignment_v1, seal_preloop_carrier_assignment_v1,
    PreloopCarrierAssignmentErrorV1, PreloopCarrierAssignmentStageV1,
};
use super::preloop_outer_carrier_transaction::{
    complete_preloop_outer_carrier_call_v1, PreloopOuterCarrierCorrespondenceErrorV1,
    PreloopOuterCarrierCorrespondenceStageV1,
};
use crate::mir::builder::stmts::build_variable_assignment_with_completion_v1;

pub(super) fn with_actual_outer_physical<R>(
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
            assert_ne!(completed.inner_destination(), completed.outer_destination());
            assert!(builder
                .current_function_instructions()
                .iter()
                .any(|instruction| matches!(
                    instruction,
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Global(symbol)),
                        ..
                    } if symbol == "ParserStringUtilsBox.skip_ws/2"
                        && *dst == completed.outer_destination()
                )));
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
            assert!(rejected
                .bounded_report()
                .contains("SelectedArgumentMismatch"));
            let rejected = rejected.into_owned_rejection_v1();
            assert_eq!(
                rejected.stage(),
                PreloopOuterCarrierCorrespondenceStageV1::SelectedArgument
            );
            assert_eq!(
                rejected.cause(),
                PreloopOuterCarrierCorrespondenceErrorV1::SelectedArgumentMismatch
            );
            rejected.discard();
        });
    });
}

#[test]
fn actual_parser_assignment_seals_the_exact_outer_carrier() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_outer_physical(|builder, physical, recipe| {
            let carrier = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("exact outer carrier");
            let completed = complete_preloop_carrier_assignment_v1(builder, carrier)
                .expect("exact assignment correspondence");

            assert_eq!(completed.target(), "pos");
            assert_eq!(
                completed.assigned_destination(),
                completed.outer_destination()
            );
            assert_ne!(
                builder
                    .function_state
                    .type_ctx
                    .get_type(completed.assigned_destination()),
                Some(&MirType::Integer),
                "F5-B must not publish the outer Integer fact"
            );
            completed.discard();
        });
    });
}

#[test]
fn assignment_correspondence_drift_retains_both_complete_owners() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_outer_physical(|builder, physical, recipe| {
            let carrier = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("exact outer carrier");
            let outer = carrier.outer_destination();
            let mut assignment =
                build_variable_assignment_with_completion_v1(builder, "pos".to_owned(), outer)
                    .expect("existing assignment authority");
            assignment.replace_target_for_test("other");
            let rejected = seal_preloop_carrier_assignment_v1(carrier, assignment)
                .expect_err("source target drift");
            assert_eq!(rejected.stage(), PreloopCarrierAssignmentStageV1::Target);
            assert_eq!(
                rejected.cause(),
                PreloopCarrierAssignmentErrorV1::TargetMismatch
            );
            assert!(rejected.bounded_report().contains("TargetMismatch"));
            let rejected = rejected.into_owned_rejection_v1();
            assert_eq!(rejected.stage(), PreloopCarrierAssignmentStageV1::Target);
            assert_eq!(
                rejected.cause(),
                PreloopCarrierAssignmentErrorV1::TargetMismatch
            );
            rejected.discard();
        });

        with_actual_outer_physical(|builder, physical, recipe| {
            let carrier = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("exact outer carrier");
            let outer = carrier.outer_destination();
            let mut assignment =
                build_variable_assignment_with_completion_v1(builder, "pos".to_owned(), outer)
                    .expect("existing assignment authority");
            assignment.replace_rhs_for_test(ValueId::new(outer.as_u32() + 100));
            let rejected = seal_preloop_carrier_assignment_v1(carrier, assignment)
                .expect_err("assignment RHS drift");
            assert_eq!(rejected.stage(), PreloopCarrierAssignmentStageV1::Rhs);
            assert_eq!(
                rejected.cause(),
                PreloopCarrierAssignmentErrorV1::RhsMismatch
            );
            rejected.discard();
        });

        with_actual_outer_physical(|builder, physical, recipe| {
            let carrier = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("exact outer carrier");
            let outer = carrier.outer_destination();
            let mut assignment =
                build_variable_assignment_with_completion_v1(builder, "pos".to_owned(), outer)
                    .expect("existing assignment authority");
            assignment.replace_assigned_for_test(ValueId::new(outer.as_u32() + 200));
            let rejected = seal_preloop_carrier_assignment_v1(carrier, assignment)
                .expect_err("returned carrier drift");
            assert_eq!(
                rejected.stage(),
                PreloopCarrierAssignmentStageV1::ReturnedCarrier
            );
            assert_eq!(
                rejected.cause(),
                PreloopCarrierAssignmentErrorV1::ReturnedCarrierMismatch
            );
            rejected.discard();
        });
    });
}

#[test]
fn assignment_failure_retains_the_carrier_and_fresh_fixture_succeeds() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_outer_physical(|builder, physical, recipe| {
            let carrier = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("exact outer carrier");
            let destination = carrier.outer_destination();
            let existing = builder
                .function_state
                .type_ctx
                .get_type(destination)
                .cloned();
            builder
                .function_state
                .variable_ctx
                .variable_map
                .remove("pos");
            let rejected = complete_preloop_carrier_assignment_v1(builder, carrier)
                .expect_err("missing declared carrier");
            assert_eq!(
                rejected.stage(),
                PreloopCarrierAssignmentStageV1::Assignment
            );
            assert_eq!(
                rejected.cause(),
                PreloopCarrierAssignmentErrorV1::AssignmentFailed
            );
            assert!(rejected.bounded_report().contains("target=pos"));
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                existing.as_ref(),
                "assignment failure must not publish an outer carrier fact"
            );
            rejected.discard();
        });

        with_actual_outer_physical(|builder, physical, recipe| {
            let carrier = complete_preloop_outer_carrier_call_v1(physical, recipe)
                .expect("fresh exact outer carrier");
            complete_preloop_carrier_assignment_v1(builder, carrier)
                .expect("fresh assignment succeeds")
                .discard();
        });
    });
}
