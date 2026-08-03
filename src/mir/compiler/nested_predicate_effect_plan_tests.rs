use super::nested_predicate_effect_plan::{
    issue_nested_binding_execution_claims_v1, NestedBindingEffectEntryV1,
    NestedBindingEffectRoleV1, NestedPrefixBindingRoleV1,
};
use super::nested_predicate_producer::produce_nested_predicate_recipe_v1;
use super::nested_predicate_producer_tests::nested_function;
use super::nested_predicate_projection::issue_nested_predicate_source_projection_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

fn claims() -> super::nested_predicate_effect_plan::VerifiedNestedBindingExecutionClaimsV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested function resolves");
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("root body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let projection = issue_nested_predicate_source_projection_v1(input, &root)
        .expect("nested source projection");
    let product = produce_nested_predicate_recipe_v1(projection).expect("nested recipe product");
    issue_nested_binding_execution_claims_v1(input.function(), product.source_handoff())
        .expect("nested execution claims")
}

#[test]
fn resolver_issued_prefix_seals_initialized_and_uninitialized_roles() {
    let claims = claims();
    let prefix = claims.prefix();
    assert_eq!(prefix.initialized().len(), 2);
    assert_eq!(
        prefix.initialized()[0].role(),
        NestedPrefixBindingRoleV1::RootInductionI
    );
    assert_eq!(
        prefix.initialized()[1].role(),
        NestedPrefixBindingRoleV1::AncestorAccumulatorSum
    );
    assert_eq!(prefix.initialized()[0].initial(), 0);
    assert_eq!(prefix.initialized()[1].initial(), 0);
    assert_eq!(
        prefix.uninitialized().role(),
        NestedPrefixBindingRoleV1::ChildRecurrenceJ
    );
    assert_eq!(
        prefix.uninitialized().lexical_scope(),
        prefix.root_loop_pair().scope()
    );
    assert_ne!(
        prefix.uninitialized().lexical_scope(),
        prefix.child_loop_pair().scope()
    );
}

#[test]
fn effect_plan_preserves_fixed_order_and_first_assignment_is_not_read() {
    let claims = claims();
    let plan = claims.effect_plan();
    let roles = plan
        .entries()
        .iter()
        .map(NestedBindingEffectEntryV1::role)
        .collect::<Vec<_>>();
    assert_eq!(roles.as_slice(), NestedBindingEffectRoleV1::ALL.as_slice());
    assert!(matches!(
        plan.entry(NestedBindingEffectRoleV1::ChildInitializeWriteJ),
        NestedBindingEffectEntryV1::FirstAssignment(claim) if claim.value() == 0
    ));
    assert!(matches!(
        plan.entry(NestedBindingEffectRoleV1::ChildPredicateReadJ),
        NestedBindingEffectEntryV1::Read(_)
    ));
    assert_eq!(
        plan.retirement().scope(),
        claims.prefix().uninitialized().lexical_scope()
    );
    assert_eq!(
        plan.retirement().region(),
        claims.prefix().root_loop_pair().region()
    );
}

#[test]
fn effect_plan_uses_one_owner_and_root_frame() {
    let claims = claims();
    assert_eq!(claims.prefix().owner(), claims.effect_plan().owner());
    assert_eq!(
        claims.prefix().frame_key(),
        claims.effect_plan().frame_key()
    );
}

#[test]
fn effect_claims_reject_a_foreign_resolved_function_owner() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested function resolves");
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("root body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let projection = issue_nested_predicate_source_projection_v1(input, &root)
        .expect("nested source projection");
    let product = produce_nested_predicate_recipe_v1(projection).expect("nested recipe product");

    let foreign_unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("foreign nested function resolves");
    let foreign_input = foreign_unit
        .root_function_input()
        .expect("foreign root function input");
    assert_eq!(
        issue_nested_binding_execution_claims_v1(foreign_input.function(), product.source_handoff()),
        Err(
            super::nested_predicate_effect_plan::NestedBindingExecutionClaimsRejectV1::OwnerMismatch
        )
    );
}
