use std::num::NonZeroU32;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

use super::source_coverage::{
    verify_located_source_coverage_v1, CoveredSourceSiteV1, SourceCoverageVerificationErrorV1,
};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "coverage_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![local("x", literal(1)), literal(2)],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

#[test]
fn verified_coverage_preserves_owner_branded_preorder_and_outer_range() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();
    let suffix = view.body_suffix(body.clone(), 0).unwrap();
    let range = view
        .consumed_prefix(&suffix, NonZeroU32::new(1).unwrap())
        .unwrap();
    let statement = view.suffix_first_stmt(&suffix).unwrap();
    let expression = view
        .child_expr_from_stmt(&statement, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();

    let coverage = verify_located_source_coverage_v1(
        owner,
        range,
        vec![
            CoveredSourceSiteV1::statement(&statement),
            CoveredSourceSiteV1::expression(&expression),
            CoveredSourceSiteV1::body(&body),
        ],
    )
    .unwrap();

    assert_eq!(coverage.owner(), owner);
    assert_eq!(coverage.outer().start(), 0);
    assert_eq!(coverage.outer().count(), NonZeroU32::new(1).unwrap());
    assert_eq!(coverage.preorder().len(), 3);
}

#[test]
fn coverage_rejects_empty_and_duplicate_typed_sites() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();
    let suffix = view.body_suffix(body, 0).unwrap();
    let statement = view.suffix_first_stmt(&suffix).unwrap();

    let empty_range = view
        .consumed_prefix(&suffix, NonZeroU32::new(1).unwrap())
        .unwrap();
    assert_eq!(
        verify_located_source_coverage_v1(owner, empty_range, Vec::new()),
        Err(SourceCoverageVerificationErrorV1::EmptyPreorder)
    );

    let duplicate_range = view
        .consumed_prefix(&suffix, NonZeroU32::new(1).unwrap())
        .unwrap();
    let site = CoveredSourceSiteV1::statement(&statement);
    assert_eq!(
        verify_located_source_coverage_v1(owner, duplicate_range, vec![site.clone(), site],),
        Err(SourceCoverageVerificationErrorV1::DuplicateSite {
            first_index: 0,
            duplicate_index: 1,
        })
    );
}

#[test]
fn coverage_rejects_foreign_outer_and_each_foreign_site_family() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
    let owner = unit.forest().roots()[0];
    let view = unit.function_source_view(owner).unwrap();
    let body = view.root_body().unwrap();
    let suffix = view.body_suffix(body.clone(), 0).unwrap();
    let first = view.suffix_first_stmt(&suffix).unwrap();
    let range = view
        .consumed_prefix(&suffix, NonZeroU32::new(1).unwrap())
        .unwrap();
    let foreign_owner = FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap();

    assert_eq!(
        verify_located_source_coverage_v1(
            foreign_owner,
            range,
            vec![CoveredSourceSiteV1::statement(&first)],
        ),
        Err(SourceCoverageVerificationErrorV1::ForeignOuterOwner {
            expected: foreign_owner,
            actual: owner,
        })
    );

    let foreign_unit = VerifiedResolvedSourceUnitV1::resolve_function(function()).unwrap();
    let foreign_owner = foreign_unit.forest().roots()[0];
    let foreign_view = foreign_unit.function_source_view(foreign_owner).unwrap();
    let foreign_body = foreign_view.root_body().unwrap();
    let foreign_stmt = foreign_view.body_stmt(&foreign_body, 0).unwrap();
    let foreign_expr = foreign_view
        .child_expr_from_stmt(&foreign_stmt, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();

    for foreign_site in [
        CoveredSourceSiteV1::body(&foreign_body),
        CoveredSourceSiteV1::statement(&foreign_stmt),
        CoveredSourceSiteV1::expression(&foreign_expr),
    ] {
        let range = view
            .consumed_prefix(&suffix, NonZeroU32::new(1).unwrap())
            .unwrap();
        assert_eq!(
            verify_located_source_coverage_v1(owner, range, vec![foreign_site]),
            Err(SourceCoverageVerificationErrorV1::ForeignCoveredOwner {
                index: 0,
                expected: owner,
                actual: foreign_owner,
            })
        );
    }
}
