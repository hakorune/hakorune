use super::*;
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, SourcePathSegmentV1,
    SourcePathV1, VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::mir::source_call_target::VerifiedWholeSourceStaticCallTargetInventoryV1;
use crate::parser::NyashParser;

fn script_program() -> ASTNode {
    NyashParser::parse_from_string("0").expect("literal Script source")
}

fn window() -> VerifiedScriptRootDemandWindowV1 {
    VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .expect("complete Script window")
}

#[test]
fn bundle_co_seals_complete_script_owner_and_empty_target_observation() {
    let script = Box::leak(Box::new(
        crate::mir::builder::PreparedNormalDefaultProgramRootV1::seal(script_program())
            .expect("Program source"),
    ));
    let declaration_root =
        NyashParser::parse_from_string("static box Helper { value() { return 7 } }")
            .expect("declaration fixture");
    let declarations = Box::leak(Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&declaration_root)
            .expect("declaration catalog"),
    ));
    let imports = Box::leak(Box::new(
        VerifiedStaticImportAliasViewV1::seal(declarations, std::iter::empty())
            .expect("empty import view"),
    ));
    let window = window();
    let target_inventory = VerifiedScriptDirectStaticCallTargetInventoryV1::issue(
        script.source_ast(),
        &window,
        declarations,
        imports,
    )
    .expect("Script target inventory");
    let mut resolver = FunctionSemanticResolverSessionV1::new(3).expect("resolver");
    let owner = match resolver
        .resolve_script(
            crate::mir::resolved_semantics::ScriptSyntaxViewV1::from_program(script.source_ast())
                .expect("Script view"),
            &window,
        )
        .expect("Script resolver")
    {
        ResolveScriptOutcomeV1::Complete(owner) => owner,
        ResolveScriptOutcomeV1::Deferred(_) => panic!("fixture must complete: deferred"),
    };
    let source = VerifiedScriptSemanticSourceV1::seal(script, owner, &window)
        .expect("Script semantic source");
    let whole = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, imports)
        .expect("whole target inventory");
    let targets = whole.into_targets();
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
        .expect("callee result catalog");
    let bundle = VerifiedScriptDirectStaticResultBundleV1::issue(
        &source,
        &window,
        &target_inventory,
        declarations,
        imports,
        &results,
    )
    .expect("complete Script bundle");

    assert_eq!(bundle.len(), 0);
    assert_eq!(bundle.source_owner(), source.forest().roots()[0]);
    assert_eq!(
        bundle.source_identity(),
        script.source_ast() as *const _ as usize
    );
    assert!(bundle.rows().next().is_none());
}

#[test]
fn bundle_rejects_a_target_inventory_from_a_foreign_program() {
    let script = Box::leak(Box::new(
        crate::mir::builder::PreparedNormalDefaultProgramRootV1::seal(script_program())
            .expect("Program source"),
    ));
    let declaration_root =
        NyashParser::parse_from_string("static box Helper { value() { return 7 } }")
            .expect("declaration fixture");
    let declarations = Box::leak(Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&declaration_root)
            .expect("declaration catalog"),
    ));
    let imports = Box::leak(Box::new(
        VerifiedStaticImportAliasViewV1::seal(declarations, std::iter::empty())
            .expect("empty import view"),
    ));
    let window = window();
    let foreign = script_program();
    let foreign_inventory = VerifiedScriptDirectStaticCallTargetInventoryV1::issue(
        &foreign,
        &window,
        declarations,
        imports,
    )
    .expect("foreign target inventory itself is valid");
    let mut resolver = FunctionSemanticResolverSessionV1::new(4).expect("resolver");
    let owner = match resolver
        .resolve_script(
            crate::mir::resolved_semantics::ScriptSyntaxViewV1::from_program(script.source_ast())
                .expect("Script view"),
            &window,
        )
        .expect("Script resolver")
    {
        ResolveScriptOutcomeV1::Complete(owner) => owner,
        ResolveScriptOutcomeV1::Deferred(_) => panic!("fixture must complete"),
    };
    let source = VerifiedScriptSemanticSourceV1::seal(script, owner, &window)
        .expect("Script semantic source");
    let whole = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, imports)
        .expect("whole target inventory");
    let targets = whole.into_targets();
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
        .expect("callee result catalog");
    assert_eq!(
        VerifiedScriptDirectStaticResultBundleV1::issue(
            &source,
            &window,
            &foreign_inventory,
            declarations,
            imports,
            &results,
        ),
        Err(ScriptDirectStaticResultBundleErrorV1::TargetInventoryBrandMismatch)
    );
}
