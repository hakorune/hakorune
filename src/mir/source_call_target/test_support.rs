use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::parser::NyashParser;

use super::*;

pub(super) fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("source-call target fixture must parse")
}

pub(super) fn catalog(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&parse(source))
        .expect("declaration catalog must seal")
}

pub(super) fn key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    namespace: SameModuleCallableNamespaceV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(namespace, owner, method, arity)
        .unwrap_or_else(|| panic!("missing declaration {owner}.{method}/{arity}"))
        .key()
        .clone()
}

pub(super) fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

pub(super) fn return_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

pub(super) fn exact_call<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: SourceExprSiteV1,
) -> VerifiedSourceMethodCallSiteV1<'catalog> {
    VerifiedSourceMethodCallSiteV1::verify(declarations, caller, call_site)
        .expect("exact MethodCall site must seal")
}

pub(super) fn empty_imports(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedStaticImportAliasViewV1<'_> {
    VerifiedStaticImportAliasViewV1::seal(declarations, []).expect("empty imports must seal")
}

pub(super) fn empty_targets(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedSourceStaticCallTargetCatalogV1<'_> {
    let imports = empty_imports(declarations);
    VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, [])
        .expect("empty target catalog must seal")
}

pub(super) fn seal_one_qualified<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    imports: &VerifiedStaticImportAliasViewV1<'catalog>,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: SourceExprSiteV1,
) -> VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
    let call = exact_call(declarations, caller, call_site);
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call])
        .expect("lexical facts must seal");
    let facts = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, imports)
        .expect("route facts must seal");
    VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(imports, [facts])
        .expect("qualified target must seal")
}

pub(super) fn qualified<'a>(
    targets: &'a VerifiedSourceStaticCallTargetCatalogV1<'_>,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1,
) -> &'a VerifiedQualifiedStaticCallTargetV1 {
    match targets.target(caller, call_site).expect("target row") {
        VerifiedSourceStaticCallTargetV1::QualifiedStatic(row) => row,
        VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(_) => {
            panic!("expected qualified target row")
        }
    }
}

pub(super) fn current_owner<'a>(
    targets: &'a VerifiedSourceStaticCallTargetCatalogV1<'_>,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1,
) -> &'a VerifiedCurrentOwnerStaticCallTargetV1 {
    match targets.target(caller, call_site).expect("target row") {
        VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(row) => row,
        VerifiedSourceStaticCallTargetV1::QualifiedStatic(_) => {
            panic!("expected current-owner target row")
        }
    }
}
