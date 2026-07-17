use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::parser::NyashParser;

use super::super::{
    VerifiedCallableResultDispositionV1, VerifiedSameModuleCallableResultCatalogV1,
};

pub(super) fn seal(
    source: &str,
) -> (
    VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableResultCatalogV1,
) {
    let root = NyashParser::parse_from_string(source).expect("result-catalog fixture must parse");
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("declaration catalog must seal");
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(&declarations)
        .expect("result catalog must seal");
    (declarations, results)
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
    let (declarations, results) = seal(source);
    results
        .disposition(&key(&declarations, owner, name, arity))
        .expect("result row")
        .clone()
}

pub(super) fn normalized(
    source: &str,
) -> Vec<(String, String, u32, VerifiedCallableResultDispositionV1)> {
    let (_, results) = seal(source);
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
