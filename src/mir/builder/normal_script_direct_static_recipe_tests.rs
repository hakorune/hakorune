use super::*;
use crate::ast::ASTNode;
use crate::mir::builder::normal_script_direct_static_result_bundle::VerifiedScriptDirectStaticResultBundleV1;
use crate::mir::builder::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::source_call_target::{
    VerifiedScriptDirectStaticCallTargetInventoryV1, VerifiedStaticImportAliasViewV1,
    VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use crate::parser::NyashParser;

fn complete_empty_owner() -> (
    &'static ASTNode,
    VerifiedScriptRootDemandWindowV1,
    VerifiedScriptDirectStaticResultPublicationOwnerV1,
) {
    let root = Box::leak(Box::new(
        crate::mir::builder::PreparedNormalDefaultProgramRootV1::seal(
            NyashParser::parse_from_string("0").expect("Script fixture"),
        )
        .expect("prepared source"),
    ));
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .expect("complete Script window");
    let declarations = Box::leak(Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(
            &NyashParser::parse_from_string("static box Helper { value() { return 7 } }")
                .expect("declaration fixture"),
        )
        .expect("declaration catalog"),
    ));
    let imports = Box::leak(Box::new(
        VerifiedStaticImportAliasViewV1::seal(declarations, std::iter::empty()).expect("imports"),
    ));
    let target_inventory = VerifiedScriptDirectStaticCallTargetInventoryV1::issue(
        root.source_ast(),
        &window,
        declarations,
        imports,
    )
    .expect("target inventory");
    let mut resolver = FunctionSemanticResolverSessionV1::new(707).expect("resolver");
    let owner = match resolver
        .resolve_script(
            ScriptSyntaxViewV1::from_program(root.source_ast()).expect("Script view"),
            &window,
        )
        .expect("Script resolution")
    {
        ResolveScriptOutcomeV1::Complete(owner) => owner,
        ResolveScriptOutcomeV1::Deferred => panic!("fixture must complete"),
    };
    let source =
        VerifiedScriptSemanticSourceV1::seal(root, owner, &window).expect("semantic source");
    let whole = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, imports)
        .expect("whole target inventory");
    let targets = whole.into_targets();
    let results = crate::mir::callable_result_representation::
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
        .expect("result catalog");
    let bundle = VerifiedScriptDirectStaticResultBundleV1::issue(
        &source,
        &window,
        &target_inventory,
        declarations,
        imports,
        &results,
    )
    .expect("result bundle");
    let owner = VerifiedScriptDirectStaticResultPublicationOwnerV1::issue(
        &source,
        &bundle,
        source.continuation(),
    )
    .expect("publication owner");
    (root.source_ast(), window, owner)
}

#[test]
fn complete_empty_owner_emits_a_valid_empty_recipe() {
    let (source, window, owner) = complete_empty_owner();
    let recipe = VerifiedScriptDirectStaticRecipeV1::issue(&owner, &window)
        .expect("empty Complete Script recipe");
    assert_eq!(recipe.len(), 0);
    assert_eq!(recipe.source_owner(), owner.source_owner());
    assert_eq!(recipe.source_identity(), source as *const _ as usize);
    assert!(recipe.rows().next().is_none());
    assert!(recipe.demand(ScriptDirectStaticRecipeKeyV1(0)).is_none());
}
