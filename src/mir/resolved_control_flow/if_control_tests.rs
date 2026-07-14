use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::function_control::verify_function_completion_v1;
use super::if_control::{
    analyze_resolved_if_control_v1, IfControlCoverageUseErrorV1, ResolvedIfControlErrorV1,
    ResolvedIfElsePortV1, ResolvedIfFallthroughPortV1, VerifiedResolvedFunctionIfControlV1,
};
use super::source_coverage::CoveredSourceSiteV1;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
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

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn block_expr(prelude_stmts: Vec<ASTNode>, tail_expr: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr: Box::new(tail_expr),
        span: Span::unknown(),
    }
}

fn if_stmt(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "if_control_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn analyze(
    body: Vec<ASTNode>,
) -> Result<VerifiedResolvedFunctionIfControlV1, ResolvedIfControlErrorV1> {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(body)).unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    analyze_resolved_if_control_v1(input, &completion)
}

#[test]
fn no_if_seals_an_empty_function_product() {
    let product = analyze(vec![literal(1)]).unwrap();
    assert!(product.rows().is_empty());
    assert_eq!(product.coverage_partition_len(), 0);
}

#[test]
fn absent_and_explicit_empty_else_are_distinct_typed_ports() {
    let absent = analyze(vec![if_stmt(literal(1), vec![], None)]).unwrap();
    let explicit = analyze(vec![if_stmt(literal(1), vec![], Some(vec![]))]).unwrap();

    assert_eq!(
        absent.rows()[0].else_port(),
        ResolvedIfElsePortV1::ImplicitIdentity
    );
    assert!(absent.rows()[0].regions().else_pair().is_none());
    assert_eq!(
        explicit.rows()[0].else_port(),
        ResolvedIfElsePortV1::Explicit(ResolvedIfFallthroughPortV1::verified())
    );
    assert!(explicit.rows()[0].regions().else_pair().is_some());
    assert_eq!(
        explicit.rows()[0].coverage_preorder().len(),
        absent.rows()[0].coverage_preorder().len() + 1,
        "the explicit empty else owns its exact body marker"
    );
}

#[test]
fn nested_if_rows_use_exact_preorder_and_exclusive_coverage() {
    let nested = if_stmt(literal(2), vec![literal(3)], Some(vec![]));
    let product = analyze(vec![if_stmt(literal(1), vec![nested], None)]).unwrap();
    assert_eq!(product.rows().len(), 2);
    assert_eq!(
        product.rows()[0].site().node().segments(),
        &[SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(
        product.rows()[1].site().node().segments(),
        &[SourcePathSegmentV1::Body(0), SourcePathSegmentV1::IfThen(0),]
    );

    let outer = product.rows()[0].coverage_preorder();
    let inner = product.rows()[1].coverage_preorder();
    assert!(outer.iter().all(|site| !inner.contains(site)));
    assert_eq!(
        product.coverage_partition_len(),
        outer.len() + inner.len(),
        "each exact covered site belongs to one If row"
    );
    assert_eq!(
        product.if_control(product.rows()[1].site()),
        Some(&product.rows()[1])
    );
}

#[test]
fn condition_block_expr_closes_parent_coverage_around_nested_if_row() {
    let nested = if_stmt(literal(2), vec![], None);
    let condition = block_expr(vec![nested], literal(1));
    let product = analyze(vec![if_stmt(condition, vec![], None)]).unwrap();
    assert_eq!(product.rows().len(), 2);

    let outer = product.rows()[0].coverage_preorder();
    let nested_statement = CoveredSourceSiteV1::Statement {
        owner: product.owner(),
        site: product.rows()[1].site().clone(),
    };
    assert!(!outer.contains(&nested_statement));
    assert!(outer
        .iter()
        .any(|site| matches!(site, CoveredSourceSiteV1::Body(body)
        if body.root().segments().last() == Some(&SourcePathSegmentV1::BlockExprPreludeRoot))));
}

#[test]
fn each_row_co_seals_one_exact_outer_statement_range() {
    let product = analyze(vec![
        literal(0),
        if_stmt(literal(1), vec![], None),
        if_stmt(literal(2), vec![], Some(vec![])),
    ])
    .unwrap();
    assert_eq!(product.rows().len(), 2);
    assert_eq!(product.rows()[0].outer_range().start(), 1);
    assert_eq!(product.rows()[1].outer_range().start(), 2);
    assert_eq!(product.rows()[0].outer_range().count().get(), 1);
    assert_eq!(product.rows()[1].outer_range().count().get(), 1);
}

#[test]
fn coverage_use_requires_exact_owner_order_and_completion() {
    let product = analyze(vec![if_stmt(literal(1), vec![literal(2)], None)]).unwrap();
    let row = &product.rows()[0];
    let expected = row.coverage_preorder();

    let mut success = row.coverage_use();
    for site in expected {
        success.claim(site).unwrap();
    }
    assert_eq!(success.finish(), Ok(()));

    let mut missing = row.coverage_use();
    missing.claim(&expected[0]).unwrap();
    assert_eq!(missing.finish(), Err(IfControlCoverageUseErrorV1::Missing));

    let mut duplicate = row.coverage_use();
    duplicate.claim(&expected[0]).unwrap();
    assert_eq!(
        duplicate.claim(&expected[0]),
        Err(IfControlCoverageUseErrorV1::Duplicate)
    );

    let mut wrong_order = row.coverage_use();
    assert_eq!(
        wrong_order.claim(&expected[1]),
        Err(IfControlCoverageUseErrorV1::WrongOrder)
    );
}

#[test]
fn coverage_use_rejects_foreign_and_unexpected_sites() {
    let product = analyze(vec![
        if_stmt(literal(1), vec![], None),
        if_stmt(literal(2), vec![], None),
    ])
    .unwrap();
    let foreign = analyze(vec![if_stmt(literal(3), vec![], None)]).unwrap();
    let mut use_ledger = product.rows()[0].coverage_use();
    assert_eq!(
        use_ledger.claim(&foreign.rows()[0].coverage_preorder()[0]),
        Err(IfControlCoverageUseErrorV1::ForeignOwner)
    );
    assert_eq!(
        use_ledger.claim(&product.rows()[1].coverage_preorder()[0]),
        Err(IfControlCoverageUseErrorV1::Unexpected)
    );
}

#[test]
fn unsupported_loop_rejects_before_any_builder_connection() {
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(literal(1)),
        body: Vec::new(),
        span: Span::unknown(),
    };
    assert!(matches!(
        analyze(vec![if_stmt(literal(1), vec![loop_stmt], None)]),
        Err(ResolvedIfControlErrorV1::UnsupportedStatement(_))
    ));
}

#[test]
fn assignments_and_shadows_do_not_change_control_port_shape() {
    let plain = analyze(vec![if_stmt(literal(1), vec![literal(2)], None)]).unwrap();
    let effects = analyze(vec![
        local("x", literal(0)),
        if_stmt(
            literal(1),
            vec![assignment("x", literal(2)), local("x", literal(3))],
            None,
        ),
    ])
    .unwrap();
    assert_eq!(plain.rows()[0].then_port(), effects.rows()[0].then_port());
    assert_eq!(plain.rows()[0].else_port(), effects.rows()[0].else_port());
}
