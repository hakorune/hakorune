use crate::mir::dynamic_invocation_contract::DynamicInvocationInputHomeV1;
use crate::mir::dynamic_operator_contract::{
    DynamicOperatorFaultV1, DynamicOperatorNormalResultV1,
};
use crate::mir::loop_recipe_contract::{
    issue_sole_root_carrier_join_closure_v2, LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
};

use super::super::super::{
    issue_dynamic_full_loop_semantic_program_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_operator_carrier_lifecycle_program_v1,
};
use super::{
    issuer::{require_backedge, require_invocation_argument, require_write_binding},
    DynamicOperatorCarrierLifecycleProgramRejectV1,
};
use super::{DynamicOperatorCarrierDestinationRefV1, DynamicOperatorCarrierPublicationV1};
use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
    issue_dynamic_full_loop_source_recipe_envelope_v2, tests::fixture,
};

#[test]
fn exact_program_issues_two_non_splittable_operator_lifecycle_rows() {
    let fixture = fixture(true);
    let envelope =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("exact source/Recipe envelope");
    let semantic = issue_dynamic_full_loop_semantic_program_v2(envelope).expect("semantic program");
    let invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(semantic)
        .expect("invocation lifecycle");
    let program = issue_dynamic_operator_carrier_lifecycle_program_v1(invocation)
        .expect("operator lifecycle");

    let rows = program.operator_lifecycle().rows().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);

    assert_eq!(rows[0].producer(), LoopItemKeyV1::new(5));
    assert_eq!(
        rows[0].operands(),
        [LoopValueKeyV1::new(7), LoopValueKeyV1::new(8)]
    );
    assert_eq!(rows[0].result(), LoopValueKeyV1::new(9));
    assert_eq!(
        rows[0].publication(),
        DynamicOperatorCarrierPublicationV1::OnNormalResultPublication
    );
    assert_eq!(
        rows[0].contract().normal_result(),
        DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier
    );
    assert_eq!(
        rows[0].contract().fault(),
        DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
    );
    assert!(matches!(
        rows[0].destination(),
        DynamicOperatorCarrierDestinationRefV1::EndAfterInvocationNormalOrFaultOutcome {
            invocation,
            argument_ordinal: 1,
            input_contract: DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation,
        } if invocation == LoopItemKeyV1::new(6)
    ));

    assert_eq!(rows[1].producer(), LoopItemKeyV1::new(15));
    assert_eq!(
        rows[1].operands(),
        [LoopValueKeyV1::new(15), LoopValueKeyV1::new(16)]
    );
    assert_eq!(rows[1].result(), LoopValueKeyV1::new(17));
    assert!(matches!(
        rows[1].destination(),
        DynamicOperatorCarrierDestinationRefV1::ForwardToBindingAtRebindCommit {
            write,
            binding,
            backedge_loop,
            ..
        } if write == LoopItemKeyV1::new(16)
            && binding == LoopBindingKeyV1::new(0)
            && backedge_loop == LoopNodeKeyV1::new(0)
    ));

    assert_eq!(program.invocation_lifecycle().rows().len(), 2);
    assert_eq!(program.after().binding(), LoopBindingKeyV1::new(0));
}

#[test]
fn operator_lifecycle_surface_has_one_input_and_no_effect_owner() {
    let semantic = include_str!("../mod.rs");
    let issuer = include_str!("issuer.rs");
    let model = include_str!("model.rs");

    assert!(semantic
        .contains("issue_dynamic_operator_carrier_lifecycle_program_v1(\n    invocation_program:"));
    assert!(!semantic.contains("<'env, 'decl>"));
    for forbidden in [
        "into_parts",
        "MirBuilder",
        "BasicBlockId",
        "ValueId",
        "FunctionOwnerIdV1",
        "Completion",
        "ReleaseStrong",
        "retry",
        "fallback",
        "EndOld",
        "InstallNew",
    ] {
        assert!(!issuer.contains(forbidden), "issuer contains {forbidden}");
        assert!(!model.contains(forbidden), "model contains {forbidden}");
    }
    assert!(
        model.contains("#[derive(Debug)]\npub(super) struct DynamicOperatorCarrierLifecycleRowV1")
    );
    assert!(model.contains(
        "#[derive(Debug)]\npub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) struct VerifiedDynamicOperatorCarrierLifecycleCatalogV1"
    ));
}

#[test]
fn exact_recipe_relations_reject_wrong_consumer_ordinal_binding_value_and_backedge() {
    let fixture = fixture(true);
    let verified_recipe = fixture.candidate.artifact().recipe();
    let recipe = verified_recipe.as_recipe();

    assert_eq!(
        require_invocation_argument(recipe, LoopItemKeyV1::new(6), 0, LoopValueKeyV1::new(9),),
        Err(DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation)
    );
    assert_eq!(
        require_invocation_argument(recipe, LoopItemKeyV1::new(7), 1, LoopValueKeyV1::new(9),),
        Err(DynamicOperatorCarrierLifecycleProgramRejectV1::InvocationRelation)
    );
    assert_eq!(
        require_write_binding(
            recipe,
            LoopItemKeyV1::new(16),
            LoopBindingKeyV1::new(1),
            LoopValueKeyV1::new(17),
        ),
        Err(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation)
    );
    assert_eq!(
        require_write_binding(
            recipe,
            LoopItemKeyV1::new(16),
            LoopBindingKeyV1::new(0),
            LoopValueKeyV1::new(9),
        ),
        Err(DynamicOperatorCarrierLifecycleProgramRejectV1::RecipeRelation)
    );

    let closure = issue_sole_root_carrier_join_closure_v2(verified_recipe)
        .expect("exact single-carrier closure");
    assert_eq!(
        require_backedge(
            closure.join_sig(),
            LoopNodeKeyV1::new(0),
            LoopBindingKeyV1::new(0),
            LoopValueKeyV1::new(9),
        ),
        Err(DynamicOperatorCarrierLifecycleProgramRejectV1::BackedgeRelation)
    );
}
