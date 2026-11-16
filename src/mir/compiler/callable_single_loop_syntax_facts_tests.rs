use super::*;
use crate::ast::{BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};

impl VerifiedSourceSyntaxFactsV1 {
    pub(crate) fn replace_tail_value_site_for_test(mut self, site: SourceExprSiteV1) -> Self {
        self.tail.value_site = site;
        self
    }
}

pub(crate) fn foreign_expression_site_for_test() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(99),
        SourcePathSegmentV1::Value,
    ]))
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn method(object: ASTNode, name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: name.into(),
        arguments,
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

fn function(extra_root_statement: Option<ASTNode>, condition_rhs: ASTNode) -> ASTNode {
    let mut body = vec![
        ASTNode::Local {
            variables: vec!["value".into()],
            initial_values: vec![Some(Box::new(method(
                variable("helper"),
                "to_i64",
                vec![variable("n")],
            )))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["i".into()],
            initial_values: vec![Some(Box::new(integer(0)))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(variable("i")),
                right: Box::new(condition_rhs),
                span: Span::unknown(),
            }),
            body: vec![assignment(
                "i",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                },
            )],
            span: Span::unknown(),
        },
    ];
    if let Some(statement) = extra_root_statement {
        body.push(statement);
    }
    body.push(ASTNode::Return {
        value: Some(Box::new(variable("value"))),
        span: Span::unknown(),
    });
    ASTNode::FunctionDeclaration {
        name: "int_to_str".into(),
        params: vec!["n".into(), "helper".into()],
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

pub(crate) fn unit(
    extra_root_statement: Option<ASTNode>,
    condition_rhs: ASTNode,
) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(function(extra_root_statement, condition_rhs))
        .expect("syntax facts fixture resolves")
}

fn no_loop_unit() -> VerifiedResolvedSourceUnitV1 {
    let mut root = function(None, integer(1));
    if let ASTNode::FunctionDeclaration { body, .. } = &mut root {
        body.remove(2);
    }
    VerifiedResolvedSourceUnitV1::resolve_function(root).expect("no-loop fixture resolves")
}

pub(crate) fn input_loop_and_context(
    unit: &VerifiedResolvedSourceUnitV1,
) -> (
    ResolvedFunctionLoweringInputV1<'_>,
    LocatedStmtV1<'_>,
    VerifiedCallableLoopMembershipV1,
) {
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("function body");
    let loop_stmt = input.source().body_stmt(&body, 2).expect("loop statement");
    let context = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger")
        .resolved_loop_source(loop_stmt.site())
        .expect("loop context");
    (input, loop_stmt, context)
}

#[test]
fn issues_exact_nine_rows_plus_prefix_boundary() {
    let unit = unit(None, integer(1));
    let (input, loop_stmt, context) = input_loop_and_context(&unit);
    let facts = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("syntax facts");

    assert_eq!(facts.initial().shape(), &SourceLiteralShapeV1::Integer(0));
    assert_eq!(facts.condition().operator(), SyntaxBinaryOperatorV1::Less);
    assert_eq!(
        facts.condition().rhs_shape(),
        &SourceLiteralShapeV1::Integer(1)
    );
    assert_eq!(facts.step().operator(), SyntaxBinaryOperatorV1::Add);
    assert_eq!(facts.step().rhs_shape(), &SourceLiteralShapeV1::Integer(1));
    assert_eq!(facts.prefix().call().argument_count(), 1);
    assert!(matches!(
        facts.tail().value_shape(),
        SourceExprShapeV1::Variable
    ));
    let scope_region = facts.loop_context().scope_region();
    assert_eq!(scope_region.scope().owner(), facts.owner());
    assert_eq!(scope_region.region().owner(), facts.owner());
}

#[test]
fn ledger_issuer_uses_exact_loop_membership_and_source_view() {
    let unit = unit(None, integer(1));
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");
    let facts = issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger)
        .expect("ledger-backed syntax facts");

    assert_eq!(
        facts.loop_site(),
        ledger.only_loop_site().unwrap().source().site()
    );
}

#[test]
fn ledger_issuer_rejects_foreign_compilation_brand() {
    let first = unit(None, integer(1));
    let second = unit(None, integer(1));
    let input = first.root_function_input().expect("first input");
    let other_input = second.root_function_input().expect("second input");
    let ledger = other_input
        .forest()
        .callable_source_ledger(other_input.owner())
        .expect("foreign ledger");

    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::ForeignOwner)
    );
}

#[test]
fn ledger_issuer_rejects_multiple_loop_sites_before_source_navigation() {
    let extra_loop = ASTNode::Loop {
        condition: Box::new(integer(1)),
        body: Vec::new(),
        span: Span::unknown(),
    };
    let unit = unit(Some(extra_loop), integer(1));
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");

    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::LoopCardinality)
    );
}

#[test]
fn ledger_issuer_rejects_zero_loop_sites_before_source_navigation() {
    let unit = no_loop_unit();
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");

    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::LoopCardinality)
    );
}

#[test]
fn ledger_issuer_rejects_duplicate_initial_carrier() {
    let mut root = function(None, integer(1));
    if let ASTNode::FunctionDeclaration { body, .. } = &mut root {
        body.insert(
            2,
            ASTNode::Local {
                variables: vec!["duplicate".into()],
                initial_values: vec![Some(Box::new(integer(2)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
        );
    }
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root)
        .expect("duplicate-carrier fixture resolves");
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");

    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::DuplicateInitialCarrier)
    );
}

#[test]
fn ledger_issuer_rejects_nested_loop_profile() {
    let mut root = function(None, integer(1));
    if let ASTNode::FunctionDeclaration { body, .. } = &mut root {
        if let ASTNode::Loop {
            body: loop_body, ..
        } = &mut body[2]
        {
            loop_body.push(ASTNode::Loop {
                condition: Box::new(integer(1)),
                body: Vec::new(),
                span: Span::unknown(),
            });
        }
    }
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(root).expect("nested-loop fixture resolves");
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");

    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::LoopCardinality)
    );
}

#[test]
fn ledger_issuer_rejects_opaque_prefix_shape() {
    let mut root = function(None, integer(1));
    if let ASTNode::FunctionDeclaration { body, .. } = &mut root {
        if let ASTNode::Local { initial_values, .. } = &mut body[0] {
            initial_values[0] = Some(Box::new(variable("helper")));
        }
    }
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root)
        .expect("opaque-prefix fixture resolves");
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");

    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::PrefixBoundaryShape)
    );
}

#[test]
fn failed_source_issuer_does_not_poison_a_fresh_request() {
    let bad = unit(None, variable("n"));
    let bad_input = bad.root_function_input().expect("bad input");
    let bad_ledger = bad_input
        .forest()
        .callable_source_ledger(bad_input.owner())
        .expect("bad ledger");
    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(bad_input, &bad_ledger),
        Err(CallableSyntaxFactsRejectV1::ConditionRhsNotLiteral)
    );

    let good = unit(None, integer(1));
    let good_input = good.root_function_input().expect("good input");
    let good_ledger = good_input
        .forest()
        .callable_source_ledger(good_input.owner())
        .expect("good ledger");
    issue_callable_single_loop_syntax_facts_from_ledger_v1(good_input, &good_ledger)
        .expect("fresh request remains usable");
}

#[test]
fn product_survives_source_unit_drop() {
    let facts = {
        let unit = unit(None, integer(1));
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context).expect("syntax facts")
    };
    assert_eq!(facts.condition().operator(), SyntaxBinaryOperatorV1::Less);
    assert_eq!(facts.prefix().call().argument_count(), 1);
}

#[test]
fn loop_membership_parts_retain_scope_region_brand() {
    let unit = unit(None, integer(1));
    let (input, _, context) = input_loop_and_context(&unit);
    let (_, _, scope_region) = context.into_parts();
    assert_eq!(scope_region.scope().owner(), input.owner());
    assert_eq!(scope_region.region().owner(), input.owner());
}

#[test]
fn rejects_foreign_loop_context() {
    let first = unit(None, integer(1));
    let second = unit(None, integer(1));
    let (input, loop_stmt, _) = input_loop_and_context(&first);
    let (_, _, foreign_context) = input_loop_and_context(&second);
    assert_eq!(
        issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, foreign_context),
        Err(CallableSyntaxFactsRejectV1::LoopContextMismatch)
    );
}

#[test]
fn rejects_unknown_root_statement_instead_of_skipping_it() {
    let unit = unit(Some(assignment("helper", variable("helper"))), integer(1));
    let input = unit.root_function_input().expect("root function input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("callable ledger");
    assert_eq!(
        issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger),
        Err(CallableSyntaxFactsRejectV1::UnexpectedBodyStatement)
    );
}

#[test]
fn rejects_non_literal_condition_rhs() {
    let unit = unit(None, variable("n"));
    let (input, loop_stmt, context) = input_loop_and_context(&unit);
    assert_eq!(
        issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context),
        Err(CallableSyntaxFactsRejectV1::ConditionRhsNotLiteral)
    );
}
