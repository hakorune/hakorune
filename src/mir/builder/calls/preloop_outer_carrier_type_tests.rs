use hakorune_mir_builder::lowering_facts::TypeFactDecisionErrorV1;

use crate::mir::builder::calls::preloop_outer_carrier_assignment::{
    complete_preloop_carrier_assignment_v1, CompletedPreloopCarrierAssignmentV1,
};
use crate::mir::{MirBuilder, MirType, ValueId};

use super::preloop_outer_carrier_transaction::complete_preloop_outer_carrier_call_v1;
use super::preloop_outer_carrier_transaction_tests::with_actual_outer_physical;
use super::preloop_outer_carrier_type::{
    publish_preloop_outer_carrier_integer_v1, PreloopOuterCarrierIntegerPublicationDispositionV1,
};

fn with_actual_assignment<R>(
    f: impl for<'site, 'view, 'catalog> FnOnce(
        &mut MirBuilder,
        CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
        ValueId,
    ) -> R,
) -> R {
    with_actual_outer_physical(|builder, physical, recipe| {
        let inner_destination = physical.inner_destination();
        let carrier =
            complete_preloop_outer_carrier_call_v1(physical, recipe).expect("exact outer carrier");
        let assignment = complete_preloop_carrier_assignment_v1(builder, carrier)
            .expect("exact assignment correspondence");
        f(builder, assignment, inner_destination)
    })
}

#[test]
fn missing_outer_fact_publishes_integer_without_touching_inner_destination() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_assignment(|builder, assignment, inner_destination| {
            let outer_destination = assignment.outer_destination();
            builder
                .function_state
                .type_ctx
                .value_types
                .remove(&outer_destination);
            builder
                .function_state
                .type_ctx
                .set_type(inner_destination, MirType::Bool);

            let completed = publish_preloop_outer_carrier_integer_v1(
                assignment,
                &mut builder.function_state.type_ctx,
            )
            .expect("missing fact publishes");

            assert_eq!(completed.destination(), outer_destination);
            assert_eq!(
                completed.disposition(),
                PreloopOuterCarrierIntegerPublicationDispositionV1::Published
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(outer_destination),
                Some(&MirType::Integer)
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(inner_destination),
                Some(&MirType::Bool)
            );
            completed.discard();
        });
    });
}

#[test]
fn unknown_publishes_and_existing_integer_is_idempotent() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_assignment(|builder, assignment, _| {
            let destination = assignment.outer_destination();
            builder
                .function_state
                .type_ctx
                .set_type(destination, MirType::Unknown);

            let completed = publish_preloop_outer_carrier_integer_v1(
                assignment,
                &mut builder.function_state.type_ctx,
            )
            .expect("Unknown is a non-fact");
            assert_eq!(
                completed.disposition(),
                PreloopOuterCarrierIntegerPublicationDispositionV1::Published
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Integer)
            );
            completed.discard();
        });

        with_actual_assignment(|builder, assignment, _| {
            let destination = assignment.outer_destination();
            builder
                .function_state
                .type_ctx
                .set_type(destination, MirType::Integer);

            let completed = publish_preloop_outer_carrier_integer_v1(
                assignment,
                &mut builder.function_state.type_ctx,
            )
            .expect("matching fact is idempotent");
            assert_eq!(
                completed.disposition(),
                PreloopOuterCarrierIntegerPublicationDispositionV1::Idempotent
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Integer)
            );
            completed.discard();
        });
    });
}

#[test]
fn concrete_conflict_preserves_fact_and_fresh_fixture_succeeds() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_actual_assignment(|builder, assignment, _| {
            let destination = assignment.outer_destination();
            builder
                .function_state
                .type_ctx
                .set_type(destination, MirType::Bool);

            let rejected = publish_preloop_outer_carrier_integer_v1(
                assignment,
                &mut builder.function_state.type_ctx,
            )
            .expect_err("concrete conflict");
            assert_eq!(rejected.destination(), destination);
            assert_eq!(
                rejected.cause(),
                &TypeFactDecisionErrorV1::ConcreteFactConflict {
                    existing: MirType::Bool,
                    proposed: MirType::Integer,
                }
            );
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Bool)
            );
            assert!(rejected.bounded_report().contains("concrete_fact_conflict"));
            rejected.discard();
        });

        with_actual_assignment(|builder, assignment, _| {
            let destination = assignment.outer_destination();
            let completed = publish_preloop_outer_carrier_integer_v1(
                assignment,
                &mut builder.function_state.type_ctx,
            )
            .expect("fresh fixture succeeds");
            assert_eq!(
                builder.function_state.type_ctx.get_type(destination),
                Some(&MirType::Integer)
            );
            completed.discard();
        });
    });
}
