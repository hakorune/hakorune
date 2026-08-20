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

fn source_program() -> (ASTNode, VerifiedScriptRootDemandWindowV1) {
    let root = NyashParser::parse_from_string("0").expect("Script fixture");
    let ASTNode::Program { statements, .. } = &root else {
        panic!("fixture must be Program");
    };
    let entries = statements
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let site = SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(index as u32))
                .stmt();
            let semantic =
                ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore);
            VerifiedScriptRootDemandEntryV1::new(
                site,
                semantic,
                ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
            )
        })
        .collect();
    let window = VerifiedScriptRootDemandWindowV1::seal(entries, statements.len())
        .expect("complete Script window");
    (root, window)
}

fn complete_bundle(
    root: &'static ASTNode,
    window: &VerifiedScriptRootDemandWindowV1,
) -> (
    VerifiedScriptSemanticSourceV1<'static>,
    VerifiedScriptDirectStaticResultBundleV1,
) {
    let prepared = Box::leak(Box::new(
        crate::mir::builder::PreparedNormalDefaultProgramRootV1::seal(root.clone())
            .expect("prepared source"),
    ));
    let source_ast = prepared.source_ast();
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
        source_ast,
        window,
        declarations,
        imports,
    )
    .expect("target inventory");
    let mut resolver = FunctionSemanticResolverSessionV1::new(1701).expect("resolver");
    let owner = match resolver
        .resolve_script(
            ScriptSyntaxViewV1::from_program(source_ast).expect("Script view"),
            window,
        )
        .expect("Script resolution")
    {
        ResolveScriptOutcomeV1::Complete(owner) => owner,
        ResolveScriptOutcomeV1::Deferred => panic!("fixture must complete"),
    };
    let source =
        VerifiedScriptSemanticSourceV1::seal(prepared, owner, window).expect("semantic source");
    let whole = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, imports)
        .expect("whole target inventory");
    let targets = whole.into_targets();
    let results = crate::mir::callable_result_representation::
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
        .expect("result catalog");
    let bundle = VerifiedScriptDirectStaticResultBundleV1::issue(
        &source,
        window,
        &target_inventory,
        declarations,
        imports,
        &results,
    )
    .expect("result bundle");
    (source, bundle)
}

#[test]
fn owner_accepts_a_complete_script_source_bundle() {
    let (root, window) = source_program();
    let root = Box::leak(Box::new(root));
    let (source, bundle) = complete_bundle(root, &window);
    assert_eq!(bundle.len(), 0);
    let owner = VerifiedScriptDirectStaticResultPublicationOwnerV1::issue(
        &source,
        &bundle,
        source.continuation(),
    )
    .expect("publication owner");
    assert_eq!(owner.len(), 0);
    assert!(owner.rows().next().is_none());
    assert_eq!(owner.source_owner(), source.continuation().owner());
    assert_eq!(
        owner.source_identity(),
        source.source() as *const _ as usize
    );
}

#[test]
fn owner_rejects_a_bundle_from_a_foreign_source() {
    let (left_root, left_window) = source_program();
    let (right_root, right_window) = source_program();
    let left_root = Box::leak(Box::new(left_root));
    let right_root = Box::leak(Box::new(right_root));
    let (left_source, _) = complete_bundle(left_root, &left_window);
    let (right_source, right_bundle) = complete_bundle(right_root, &right_window);
    assert_eq!(
        VerifiedScriptDirectStaticResultPublicationOwnerV1::issue(
            &left_source,
            &right_bundle,
            left_source.continuation(),
        ),
        Err(ScriptDirectStaticResultPublicationOwnerIssueV1::BundleSourceMismatch)
    );
    assert_ne!(
        left_source.source() as *const _,
        right_source.source() as *const _
    );
}
