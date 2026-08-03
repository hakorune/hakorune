#![cfg(test)]

use super::loop_accum_physicalizer::*;
use super::loop_physical_input::*;
use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::module_invocation_session::{
    BuilderCommitReadinessErrorV1, BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1,
    ModuleBuilderInvocationSessionV1,
};
use crate::mir::loop_recipe_contract::{
    direct_accum_product_for_test, VerifiedLoopPhysicalInputV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
use crate::mir::{BasicBlockId, BindingId, MirBuilder, ValueId};

fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .expect("issuer")
        .issue()
        .expect("owner")
}

fn roles() -> VerifiedLoopPhysicalRolePlanV1 {
    VerifiedLoopPhysicalRolePlanV1::try_new(vec![
        (LoopPhysicalRoleV1::Preheader, BasicBlockId::new(0)),
        (LoopPhysicalRoleV1::Header, BasicBlockId::new(1)),
        (LoopPhysicalRoleV1::Body, BasicBlockId::new(2)),
        (LoopPhysicalRoleV1::Step, BasicBlockId::new(3)),
        (LoopPhysicalRoleV1::After, BasicBlockId::new(4)),
    ])
    .expect("standard5 roles")
}

fn seed_direct_accum(
    builder: &mut MirBuilder,
    name: &str,
) -> (
    VerifiedLoopBindingProjectionV1,
    VerifiedLoopInputProjectionV1,
) {
    builder.enter_function_for_test(name.to_owned());
    let initial_i = loop_operation::emit_const_i64(builder, 0).expect("initial i");
    let initial_sum = loop_operation::emit_const_i64(builder, 0).expect("initial sum");
    let owner = owner();
    let bindings = VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![
            (
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0)),
            ),
            (
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(1),
                crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(1)),
            ),
        ],
    )
    .expect("binding projection");
    let inputs = VerifiedLoopInputProjectionV1::try_new(
        BasicBlockId::new(0),
        vec![
            (
                crate::mir::loop_recipe_contract::LoopValueKeyV1::new(0),
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                initial_i,
            ),
            (
                crate::mir::loop_recipe_contract::LoopValueKeyV1::new(1),
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(1),
                initial_sum,
            ),
        ],
    )
    .expect("input projection");
    (bindings, inputs)
}

fn candidate(live: &MirBuilder) -> ModuleBuilderInvocationSessionV1 {
    let config = BuilderInvocationConfigV1::snapshot_with_policy(
        live,
        BuilderCoreSeedPolicyV1::ContinueLive,
    );
    ModuleBuilderInvocationSessionV1::open(live, config)
}

#[test]
fn direct_accum_physicalizer_emits_through_existing_owners() {
    let mut builder = MirBuilder::new();
    let (bindings, inputs) = seed_direct_accum(&mut builder, "direct_accum_physicalizer/0");
    let receipt = physicalize_direct_accum_v1(
        &mut builder,
        VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
        bindings,
        inputs,
        roles(),
    )
    .expect("physicalize");
    assert_eq!(receipt.result, LoopResultDispositionV1::Unit);
    assert_eq!(receipt.final_values.len(), 2);
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("function")
            .blocks
            .len(),
        5
    );
}

#[test]
fn candidate_rejection_discards_physicalized_loop_before_fresh_reuse() {
    let live = MirBuilder::new();
    let before = live.loop_candidate_test_fingerprint();

    let mut first = candidate(&live);
    let (bindings, inputs) = seed_direct_accum(first.builder_mut(), "candidate/first");
    let first_receipt = physicalize_direct_accum_v1(
        first.builder_mut(),
        VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
        bindings,
        inputs,
        roles(),
    )
    .expect("first candidate physicalization");
    let first_error = first.prepare_external_commit().unwrap_err();
    assert_eq!(
        first_error,
        BuilderCommitReadinessErrorV1::CurrentFunctionOpen
    );
    assert_eq!(live.loop_candidate_test_fingerprint(), before);

    let mut second = candidate(&live);
    let (bindings, inputs) = seed_direct_accum(second.builder_mut(), "candidate/second");
    let second_receipt = physicalize_direct_accum_v1(
        second.builder_mut(),
        VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
        bindings,
        inputs,
        roles(),
    )
    .expect("fresh candidate physicalization");
    assert_eq!(first_receipt, second_receipt);
    let second_error = second.prepare_external_commit().unwrap_err();
    assert_eq!(
        second_error,
        BuilderCommitReadinessErrorV1::CurrentFunctionOpen
    );
    assert_eq!(live.loop_candidate_test_fingerprint(), before);
}

#[test]
fn candidate_abort_after_late_physicalizer_failure_allows_fresh_reuse() {
    let live = MirBuilder::new();
    let before = live.loop_candidate_test_fingerprint();
    let mut first = candidate(&live);
    let (bindings, inputs) = seed_direct_accum(first.builder_mut(), "candidate/failure");
    let error = physicalize_direct_accum_v1_with_test_failure(
        first.builder_mut(),
        VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
        bindings,
        inputs,
        roles(),
        DirectAccumPhysicalizerTestFailurePointV1::AfterHeaderCondition,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        LoopPhysicalizeErrorV1::PhiAbort(message) if message.contains("InjectedTestFailure")
    ));
    drop(first);
    assert_eq!(live.loop_candidate_test_fingerprint(), before);

    let mut second = candidate(&live);
    let (bindings, inputs) = seed_direct_accum(second.builder_mut(), "candidate/reuse");
    physicalize_direct_accum_v1(
        second.builder_mut(),
        VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
        bindings,
        inputs,
        roles(),
    )
    .expect("fresh candidate after late failure");
    assert_eq!(live.loop_candidate_test_fingerprint(), before);
}

#[test]
fn missing_preheader_input_rejects_before_block_creation() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("direct_accum_physicalizer/reject".to_owned());
    let owner = owner();
    let bindings = VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![(
            crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
            crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0)),
        )],
    )
    .expect("binding projection");
    let inputs = VerifiedLoopInputProjectionV1::try_new(
        BasicBlockId::new(0),
        vec![(
            crate::mir::loop_recipe_contract::LoopValueKeyV1::new(0),
            crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
            ValueId::new(99),
        )],
    )
    .expect("input projection");
    let error = physicalize_direct_accum_v1(
        &mut builder,
        VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
        bindings,
        inputs,
        roles(),
    )
    .unwrap_err();
    assert!(matches!(error, LoopPhysicalizeErrorV1::RecipeShape(_)));
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("function")
            .blocks
            .len(),
        1
    );
}
