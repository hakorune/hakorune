use super::*;

use crate::ast::ASTNode;
use crate::mir::builder::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeDemandV1;
use crate::mir::builder::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use crate::mir::builder::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationDemandV1;
use crate::mir::builder::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1;
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIssuerV1, FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1,
    ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1,
    ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::source_call_target::{
    VerifiedScriptDirectStaticCallLookupV1, VerifiedScriptDirectStaticCallTargetInventoryV1,
    VerifiedStaticImportAliasViewV1,
    VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use crate::parser::NyashParser;
use std::collections::BTreeMap;

fn recipe_and_owner(
    root: ASTNode,
    window: VerifiedScriptRootDemandWindowV1,
) -> (
    VerifiedScriptDirectStaticRecipeV1,
    VerifiedScriptDirectStaticResultPublicationOwnerV1,
) {
    let prepared = Box::leak(Box::new(
        crate::mir::builder::PreparedNormalDefaultProgramRootV1::seal(root)
            .expect("prepared source"),
    ));
    let declaration_root =
        NyashParser::parse_from_string("static box Helpers { run(x) { return x } }")
            .expect("declaration fixture");
    let declarations = Box::leak(Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&declaration_root)
            .expect("declaration catalog"),
    ));
    let imports = Box::leak(Box::new(
        VerifiedStaticImportAliasViewV1::seal(declarations, std::iter::empty()).expect("imports"),
    ));
    let target_inventory = VerifiedScriptDirectStaticCallTargetInventoryV1::issue(
        prepared.source_ast(),
        &window,
        declarations,
        imports,
    )
    .expect("target inventory");
    let mut resolver = FunctionSemanticResolverSessionV1::new(4701).expect("resolver");
    let owner = match resolver
        .resolve_script(
            ScriptSyntaxViewV1::from_program(prepared.source_ast()).expect("Script view"),
            &window,
        )
        .expect("Script resolution")
    {
        ResolveScriptOutcomeV1::Complete(owner) => owner,
        ResolveScriptOutcomeV1::Deferred(_) => panic!("fixture must complete"),
    };
    let source =
        VerifiedScriptSemanticSourceV1::seal(prepared, owner, &window).expect("semantic source");
    let whole = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, imports)
        .expect("whole target inventory");
    let targets = whole.into_targets();
    let results = crate::mir::callable_result_representation::
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
        .expect("result catalog");
    let lookup = VerifiedScriptDirectStaticCallLookupV1::from_test_inventory(
        &target_inventory,
        &results,
    );
    let bundle = VerifiedScriptDirectStaticResultBundleV1::issue(
        &source,
        &window,
        lookup,
    )
    .expect("result bundle");
    let publication_owner = VerifiedScriptDirectStaticResultPublicationOwnerV1::issue(
        &source,
        &bundle,
        source.continuation(),
    )
    .expect("publication owner");
    let recipe = VerifiedScriptDirectStaticRecipeV1::issue(&publication_owner, &window)
        .expect("empty recipe");
    (recipe, publication_owner)
}

fn empty_recipe_and_owner() -> (
    VerifiedScriptDirectStaticRecipeV1,
    VerifiedScriptDirectStaticResultPublicationOwnerV1,
) {
    let root = NyashParser::parse_from_string("0").expect("Script fixture");
    let site = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(0))
        .stmt();
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            site,
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .expect("complete Script window");
    recipe_and_owner(root, window)
}

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
fn empty_recipe_emits_empty_join_handoff() {
    let (recipe, publication_owner) = empty_recipe_and_owner();
    let handoff = VerifiedScriptDirectStaticJoinHandoffV1::issue(&recipe, &publication_owner)
        .expect("empty join handoff");
    assert_eq!(handoff.len(), 0);
    assert_eq!(handoff.source_owner(), recipe.source_owner());
    assert_eq!(handoff.source_identity(), recipe.source_identity());
    assert!(handoff.rows().next().is_none());
}

#[test]
fn join_handoff_rejects_a_foreign_source_owner() {
    let (left_recipe, _) = empty_recipe_and_owner();
    let (_, right_publication_owner) = empty_recipe_and_owner();
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
