use super::*;
use crate::mir::builder::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeDemandV1;
use crate::mir::builder::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationDemandV1;
use crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1;
use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1};
use std::collections::BTreeMap;

fn synthetic_non_empty_pair() -> (
    VerifiedScriptDirectStaticRecipeV1,
    VerifiedScriptDirectStaticResultPublicationOwnerV1,
) {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    let source_owner = issuer.issue().expect("source owner");
    let statement = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(9))
        .stmt();
    let call_site = SourcePathV1::from_node(statement.node()).expr();
    let receiver_site = SourcePathV1::from_node(call_site.node())
        .child(SourcePathSegmentV1::Receiver)
        .expr();
    let argument_site = SourcePathV1::from_node(call_site.node())
        .child(SourcePathSegmentV1::Argument(0))
        .expr();
    let target = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "Helpers", "run", 1,
    );
    let key = ScriptDirectStaticRecipeKeyV1::from_ordinal_for_test(4);
    let destination = ScriptDirectStaticRecipeDestinationV1::FinalSequence {
        statement: statement.clone(),
    };
    let representation = crate::mir::callable_result_representation::
        VerifiedCallableResultRepresentationV1::ExactI64;
    let recipe_demand = VerifiedScriptDirectStaticRecipeDemandV1::from_parts_for_test(
        key,
        source_owner,
        call_site.clone(),
        receiver_site.clone(),
        vec![argument_site.clone()].into_boxed_slice(),
        call_site.clone(),
        Box::new([]),
        destination,
        target.clone(),
        representation.clone(),
        Box::new([]),
    );
    let publication_demand =
        VerifiedScriptDirectStaticResultPublicationDemandV1::from_parts_for_test(
            source_owner,
            call_site.clone(),
            receiver_site,
            vec![argument_site].into_boxed_slice(),
            call_site.clone(),
            Box::new([]),
            ScriptSourceContinuationTerminalV1::Sequence(statement),
            target,
            representation,
            Box::new([]),
        );
    let recipe = VerifiedScriptDirectStaticRecipeV1::from_parts_for_test(
        source_owner,
        41,
        BTreeMap::from([(key, recipe_demand)]),
    );
    let publication_owner = VerifiedScriptDirectStaticResultPublicationOwnerV1::from_parts_for_test(
        source_owner,
        41,
        BTreeMap::from([(call_site, publication_demand)]),
    );
    (recipe, publication_owner)
}


#[test]
fn join_handoff_rejects_a_foreign_source_owner() {
    let (left_recipe, owner) = synthetic_non_empty_pair();
    let right_publication_owner =
        VerifiedScriptDirectStaticResultPublicationOwnerV1::from_parts_for_test(
            owner.source_owner(),
            42,
            owner
                .rows()
                .map(|(site, demand)| (site.clone(), demand.clone()))
                .collect(),
        );
    assert_eq!(
        VerifiedScriptDirectStaticJoinHandoffV1::issue(&left_recipe, &right_publication_owner),
        Err(ScriptDirectStaticJoinHandoffIssueV1::SourceIdentityMismatch)
    );
}

#[test]
fn non_empty_recipe_row_is_carried_by_recipe_key() {
    let (recipe, publication_owner) = synthetic_non_empty_pair();
    assert_eq!(recipe.len(), 1);
    let handoff = VerifiedScriptDirectStaticJoinHandoffV1::issue(&recipe, &publication_owner)
        .expect("non-empty join handoff");
    assert_eq!(handoff.len(), 1);
    let (key, row) = handoff.rows().next().expect("one handoff row");
    assert_eq!(*key, row.key());
    assert_eq!(
        row.destination(),
        recipe.demand(*key).unwrap().destination()
    );
    assert_eq!(row.call_site(), recipe.demand(*key).unwrap().call_site());
}
