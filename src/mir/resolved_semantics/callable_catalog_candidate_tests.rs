use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};

use super::*;

fn function(name: &str, parameter_count: usize) -> ASTNode {
    let params = (0..parameter_count)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    ASTNode::FunctionDeclaration {
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: name.clone(),
                declared_type_name: Some("i64".to_string()),
            })
            .collect(),
        name: name.to_string(),
        params,
        return_type_name: Some("i64".to_string()),
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn source(statements: Vec<ASTNode>) -> VerifiedCallableCatalogSourceUnitV1 {
    VerifiedCallableCatalogSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements,
        span: Span::unknown(),
    })
    .unwrap()
}

#[test]
fn seals_owner_free_candidates_in_source_order() {
    let unit = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source(vec![
        function("first", 1),
        function("second", 2),
    ]))
    .unwrap();

    assert_eq!(unit.candidate_count(), 2);
    for (index, expected) in [(0, ("first", 1)), (1, ("second", 2))] {
        let site = unit.source().declaration_sites()[index];
        let candidate = unit.candidate(site).unwrap();
        assert_eq!(candidate.source_key().name(), expected.0);
        assert_eq!(candidate.source_key().arity(), expected.1);
        let expected_symbol = format!("{}/{}", expected.0, expected.1);
        assert_eq!(candidate.symbol().as_mir_name(), expected_symbol);
        assert_eq!(unit.source_site_for_key(candidate.source_key()), Some(site));
        assert_eq!(unit.source_site_for_symbol(candidate.symbol()), Some(site));
    }
}

#[test]
fn allows_same_name_with_different_arity() {
    let unit = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source(vec![
        function("f", 1),
        function("f", 2),
    ]))
    .unwrap();

    assert_eq!(unit.candidate_count(), 2);
}

#[test]
fn rejects_duplicate_exact_key_with_both_declaration_sites() {
    let error = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source(vec![
        function("f", 1),
        function("f", 1),
    ]))
    .unwrap_err();

    let CallableCatalogCandidateSealErrorV1::DuplicateSourceKey {
        key,
        first_site,
        second_site,
    } = error
    else {
        panic!("expected duplicate source key")
    };
    assert_eq!(key.name(), "f");
    assert_eq!(key.arity(), 1);
    assert_eq!(first_site.statement_index(), 0);
    assert_eq!(second_site.statement_index(), 1);
}

#[test]
fn reports_profile_failure_at_the_exact_source_site() {
    let mut invalid = function("main", 1);
    let ASTNode::FunctionDeclaration { is_static, .. } = &mut invalid else {
        unreachable!()
    };
    *is_static = false;
    let error = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source(vec![
        function("ok", 1),
        invalid,
    ]))
    .unwrap_err();

    let CallableCatalogCandidateSealErrorV1::HeaderOutsideExactI64Profile { site, reason } = error
    else {
        panic!("expected exact profile failure")
    };
    assert_eq!(site.statement_index(), 1);
    assert_eq!(reason, CallableIndexSealErrorV1::StaticRequired);
}
