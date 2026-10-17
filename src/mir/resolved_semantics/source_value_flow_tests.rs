//! Source control-flow boundaries, observed through the production resolver.
use super::source_value_flow::{
    SourceValueEdgeV1 as E, SourceValueJoinKindV1 as K, SourceValueOriginV1 as O, SourceValueRefV1,
    VerifiedSourceValueFlowV1,
};
use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1 as P, VerifiedResolvedFunctionV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use std::sync::Arc;

fn var() -> ASTNode {
    ASTNode::Variable {
        name: "x".into(),
        span: Span::unknown(),
    }
}
fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}
fn set(value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var()),
        value: Box::new(value),
        span: Span::unknown(),
    }
}
fn ret(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}
fn local(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Local {
        variables: vec!["x".into()],
        initial_values: vec![value.map(Box::new)],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}
fn branch(then_body: Vec<ASTNode>, else_body: Vec<ASTNode>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(var()),
        then_body,
        else_body: Some(else_body),
        span: Span::unknown(),
    }
}
fn repeat(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(var()),
        body,
        span: Span::unknown(),
    }
}
fn brk() -> ASTNode {
    ASTNode::Break {
        span: Span::unknown(),
    }
}
fn next() -> ASTNode {
    ASTNode::Continue {
        span: Span::unknown(),
    }
}
fn resolve(body: Vec<ASTNode>) -> Arc<VerifiedResolvedFunctionV1> {
    let tree = ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: vec!["x".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap()
}
fn site(path: &[P]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(path.to_vec()))
}
fn flow(product: &VerifiedResolvedFunctionV1) -> &VerifiedSourceValueFlowV1 {
    product.expression_source().value_flow().unwrap()
}
fn read(product: &VerifiedResolvedFunctionV1, path: &[P]) -> SourceValueRefV1 {
    let site = site(path);
    let Some(ResolvedLexicalRefV1::Local(binding)) = product.variable_ref(&site) else {
        panic!("not a local read: {path:?}");
    };
    flow(product).read(&site, binding).unwrap()
}
fn expression(product: &VerifiedResolvedFunctionV1, value: SourceValueRefV1, path: &[P]) {
    assert!(
        matches!(flow(product).origin(value), Some(O::Expression(original)) if original == &site(path))
    );
}
fn join(
    product: &VerifiedResolvedFunctionV1,
    value: SourceValueRefV1,
    expected: K,
) -> &[(E, SourceValueRefV1)] {
    let Some(O::Join { kind, incoming, .. }) = flow(product).origin(value) else {
        panic!("not a join: {:?}", flow(product).origin(value));
    };
    assert_eq!(*kind, expected);
    incoming
}

#[test]
fn source_value_flow_rebind_reads_old_value_then_publishes_rhs_and_checks_owner() {
    let product = resolve(vec![set(var()), ret(var())]);
    let first = read(&product, &[P::Body(0), P::Value]);
    assert!(matches!(flow(&product).origin(first), Some(O::Entry(_))));
    expression(
        &product,
        read(&product, &[P::Body(1), P::Value]),
        &[P::Body(0), P::Value],
    );
    assert_eq!(flow(&product).owner(), product.owner());
    let other = resolve(vec![ret(var())]);
    let Some(ResolvedLexicalRefV1::Local(foreign)) =
        other.variable_ref(&site(&[P::Body(0), P::Value]))
    else {
        panic!()
    };
    assert!(flow(&product)
        .read(&site(&[P::Body(0), P::Value]), foreign)
        .is_none());
    assert!(flow(&other).origin(first).is_none());
}

#[test]
fn source_value_flow_branch_excludes_return_and_keeps_uninitialized_path() {
    let product = resolve(vec![
        branch(
            vec![set(int(1)), ret(var()), set(int(99))],
            vec![set(int(2))],
        ),
        ret(var()),
    ]);
    expression(
        &product,
        read(&product, &[P::Body(1), P::Value]),
        &[P::Body(0), P::IfElse(0), P::Value],
    );
    // The dead assignment cannot re-enter the Normal join.
}

#[test]
fn source_value_flow_loop_keeps_zero_break_continue_and_nested_targets() {
    let product = resolve(vec![
        repeat(vec![
            repeat(vec![set(int(10)), brk()]),
            branch(
                vec![set(int(20)), next(), set(int(99))],
                vec![set(int(30)), brk()],
            ),
        ]),
        ret(var()),
    ]);
    let header = read(&product, &[P::Body(0), P::LoopCondition]);
    let head = join(&product, header, K::LoopHeader);
    assert_eq!(head.len(), 2); // entry + targeted continue, no unreachable backedge
    assert!(head.iter().any(|(edge, _)| *edge == E::LoopEntry));
    let (_, continued) = head
        .iter()
        .find(|(edge, _)| matches!(edge, E::Continue(_)))
        .unwrap();
    expression(
        &product,
        *continued,
        &[P::Body(0), P::LoopBody(1), P::IfThen(0), P::Value],
    );
    let after = join(
        &product,
        read(&product, &[P::Body(1), P::Value]),
        K::LoopAfter,
    );
    assert_eq!(after.len(), 2); // inner break belongs only to inner after
    assert!(after
        .iter()
        .any(|(edge, value)| *edge == E::ConditionFalse && *value == header));
    let (_, broken) = after
        .iter()
        .find(|(edge, _)| matches!(edge, E::Break(_)))
        .unwrap();
    expression(
        &product,
        *broken,
        &[P::Body(0), P::LoopBody(1), P::IfElse(0), P::Value],
    );
}

#[test]
fn source_value_flow_short_circuit_assignment_is_conditional_and_rhs_uses_old_value() {
    for operator in [BinaryOperator::And, BinaryOperator::Or] {
        let grouped = ASTNode::GroupedAssignmentExpr {
            lhs: "x".into(),
            rhs: Box::new(var()),
            span: Span::unknown(),
        };
        let product = resolve(vec![
            ASTNode::BinaryOp {
                operator,
                left: Box::new(var()),
                right: Box::new(grouped),
                span: Span::unknown(),
            },
            ret(var()),
        ]);
        let before = read(&product, &[P::Body(0), P::Lhs]);
        assert_eq!(before, read(&product, &[P::Body(0), P::Rhs, P::Value]));
        let incoming = join(
            &product,
            read(&product, &[P::Body(1), P::Value]),
            K::BranchNormal,
        );
        assert_eq!(incoming.len(), 2);
        assert!(incoming.iter().any(|(_, value)| *value == before));
        let (_, assigned) = incoming.iter().find(|(_, value)| *value != before).unwrap();
        expression(&product, *assigned, &[P::Body(0), P::Rhs, P::Value]);
    }
}

#[test]
fn source_value_flow_uninitialized_and_compound_result_stay_unknown() {
    let tree = ASTNode::FunctionDeclaration {
        name: "fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            local(None),
            branch(vec![set(int(1))], vec![]),
            var(),
            ASTNode::CompoundAssignment {
                target: Box::new(var()),
                operator: BinaryOperator::Add,
                value: Box::new(int(1)),
                span: Span::unknown(),
            },
            ret(var()),
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let incoming = join(&product, read(&product, &[P::Body(2)]), K::BranchNormal);
    assert!(incoming
        .iter()
        .any(|(_, value)| matches!(flow(&product).origin(*value), Some(O::Unknown))));
    assert!(matches!(
        flow(&product).origin(read(&product, &[P::Body(4), P::Value])),
        Some(O::Unknown)
    ));
    assert!(flow(&product).is_complete()); // complete relation, not a proven value type
}

#[test]
fn source_value_flow_async_observation_never_claims_completeness() {
    let product = resolve(vec![
        ASTNode::AwaitExpression {
            expression: Box::new(var()),
            span: Span::unknown(),
        },
        ret(var()),
    ]);
    assert!(!flow(&product).is_complete());
    let site = site(&[P::Body(1), P::Value]);
    let Some(ResolvedLexicalRefV1::Local(binding)) = product.variable_ref(&site) else {
        panic!()
    };
    assert!(flow(&product).read(&site, binding).is_none());
}
