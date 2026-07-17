use std::collections::BTreeSet;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    VerifiedQualifiedCallRouteFactsV1, VerifiedQualifiedReceiverLexicalDispositionsV1,
    VerifiedSourceMethodCallSiteV1, VerifiedSourceStaticCallTargetCatalogV1,
    VerifiedStaticImportAliasViewV1,
};
use crate::parser::NyashParser;

use super::super::{
    VerifiedCallableResultDispositionV1, VerifiedSameModuleCallableResultCatalogV1,
};

#[derive(Debug, Clone)]
pub(super) struct CallSiteSpecV1 {
    pub(super) caller_owner: &'static str,
    pub(super) caller_name: &'static str,
    pub(super) caller_arity: usize,
    pub(super) site: SourceExprSiteV1,
}

pub(super) fn seal(
    source: &str,
) -> (
    &'static VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableResultCatalogV1<'static, 'static>,
) {
    // These source-proof fixtures predate the target catalog and return both
    // catalogs from one helper. The production result intentionally borrows
    // exact target evidence, so the test-only owner is process-lifetime.
    let declarations = Box::leak(Box::new(declarations(source)));
    let targets = Box::leak(Box::new(qualified_targets(declarations, &[], &[])));
    let results = seal_with_targets(declarations, targets);
    (declarations, results)
}

pub(super) fn declarations(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = NyashParser::parse_from_string(source).expect("result-catalog fixture must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("declaration catalog must seal")
}

pub(super) fn seal_with_targets<'targets, 'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    targets: &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
) -> VerifiedSameModuleCallableResultCatalogV1<'targets, 'catalog> {
    VerifiedSameModuleCallableResultCatalogV1::verify(declarations, targets)
        .expect("result catalog with targets must seal")
}

pub(super) fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

pub(super) fn qualified_targets<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    aliases: &[(&str, &str)],
    specs: &[CallSiteSpecV1],
) -> VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
    let imports = VerifiedStaticImportAliasViewV1::seal(
        declarations,
        aliases
            .iter()
            .map(|(alias, owner)| ((*alias).to_string(), (*owner).to_string())),
    )
    .expect("import aliases must seal");
    let calls = exact_call_sites(declarations, specs);
    let caller_keys = calls
        .iter()
        .map(|call| call.caller().clone())
        .collect::<BTreeSet<_>>();
    let lexical = caller_keys
        .iter()
        .map(|caller| {
            let caller_calls = calls
                .iter()
                .filter(|call| call.caller() == caller)
                .collect::<Vec<_>>();
            VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&caller_calls)
                .expect("qualified receiver lexical facts must seal")
        })
        .collect::<Vec<_>>();
    let facts = calls
        .iter()
        .map(|call| {
            let lexical = lexical
                .iter()
                .find(|rows| rows.caller() == call.caller())
                .expect("caller-local lexical product");
            VerifiedQualifiedCallRouteFactsV1::verify(call, lexical, &imports)
                .expect("qualified route facts must seal")
        })
        .collect::<Vec<_>>();

    VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, facts)
        .expect("qualified source targets must seal")
}

pub(super) fn extend_current_owner_targets<'catalog>(
    targets: VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    specs: &[CallSiteSpecV1],
) -> VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
    let calls = exact_call_sites(declarations, specs);
    targets
        .extend_current_owner(calls.iter())
        .expect("current-owner source targets must seal")
}

fn exact_call_sites<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    specs: &[CallSiteSpecV1],
) -> Vec<VerifiedSourceMethodCallSiteV1<'catalog>> {
    specs
        .iter()
        .map(|spec| {
            let caller = key(
                declarations,
                spec.caller_owner,
                spec.caller_name,
                spec.caller_arity,
            );
            VerifiedSourceMethodCallSiteV1::verify(declarations, &caller, spec.site.clone())
                .expect("exact source method-call site must seal")
        })
        .collect()
}

pub(super) fn key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    name: &str,
    arity: usize,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner,
            name,
            arity,
        )
        .unwrap_or_else(|| panic!("missing static declaration {owner}.{name}/{arity}"))
        .key()
        .clone()
}

pub(super) fn disposition(
    source: &str,
    owner: &str,
    name: &str,
    arity: usize,
) -> VerifiedCallableResultDispositionV1 {
    let declarations = declarations(source);
    let targets = qualified_targets(&declarations, &[], &[]);
    let results = seal_with_targets(&declarations, &targets);
    results
        .disposition(&key(&declarations, owner, name, arity))
        .expect("result row")
        .clone()
}

pub(super) fn normalized(
    source: &str,
) -> Vec<(String, String, u32, VerifiedCallableResultDispositionV1)> {
    let declarations = declarations(source);
    let targets = qualified_targets(&declarations, &[], &[]);
    let results = seal_with_targets(&declarations, &targets);
    results
        .rows()
        .map(|(key, disposition)| {
            (
                key.owner().to_string(),
                key.name().to_string(),
                key.arity(),
                disposition.clone(),
            )
        })
        .collect()
}
