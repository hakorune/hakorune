//! Source control-flow boundaries, observed through the production resolver.
use super::source_value_flow::{
    SourceValueEdgeV1 as E, SourceValueJoinKindV1 as K, SourceValueOriginV1 as O, SourceValueRefV1,
    VerifiedSourceValueFlowV1,
};
use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1, SourcePathSegmentV1 as P, VerifiedResolvedFunctionV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span, UnaryOperator};
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

#[test]
fn moved_local_provenance_uses_the_original_operand_read() {
    let product = resolve(vec![
        ASTNode::Local {
            variables: vec!["y".into()],
            initial_values: vec![Some(Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Move,
                operand: Box::new(var()),
                span: Span::unknown(),
            }))],
            declared_type_names: vec![None],
            span: Span::unknown(),
        },
        ret(ASTNode::Variable { name: "y".into(), span: Span::unknown() }),
    ]);
    let value = read(&product, &[P::Body(1), P::Value]);
    let original = read(&product, &[P::Body(0), P::Initializer(0), P::Operand]);
    let terminal = flow(&product).alias_terminal_with_read_site(value, |site| {
        let read = product.expression_source().unary(site)
            .filter(|row| row.operator() == super::ResolvedUnaryOperatorV1::Move)
            .map(|row| row.operand()).unwrap_or(site);
        product.variable_ref(read).map(|reference| (read.clone(), reference))
    }, |_| false);
    assert_eq!(terminal, Some(original));
    assert!(matches!(flow(&product).origin(original), Some(O::Entry(_))));
}
fn join(
    product: &VerifiedResolvedFunctionV1,
    value: SourceValueRefV1,
    expected: K,
) -> &[(E, SourceValueRefV1)] {
    assert!(flow(product).alias_origin(value, |site| product.variable_ref(site)).is_none(),
        "an alias query cannot select one incoming predecessor");
    assert_eq!(flow(product).alias_terminal_with_expression_boundary(value,
        |site| product.variable_ref(site), |_| false), Some(value), "passive terminal is the join itself");
    let Some(O::Join { binding, site, kind, incoming }) = flow(product).origin(value) else {
        panic!("not a join: {:?}", flow(product).origin(value));
    };
    assert_eq!(*kind, expected);
    let original = flow(product).loop_join(&SourceStmtSiteV1::from_node(site.node().clone()), *binding, expected);
    if expected == K::BranchNormal { assert!(original.is_none(), "Loop query cannot borrow a branch join"); }
    else {
        let (queried, all_incoming) = original.expect("unique exact original Loop join");
        assert_eq!(queried, value);
        assert!(std::ptr::eq(all_incoming, incoming.as_ref()), "query borrows the whole original census");
    }
    incoming
}

#[test]
fn source_value_flow_rebind_reads_old_value_then_publishes_rhs_and_checks_owner() {
    let product = resolve(vec![set(var()), ret(var())]);
    let first = read(&product, &[P::Body(0), P::Value]);
    assert!(matches!(flow(&product).origin(first), Some(O::Entry(_))));
    assert_eq!(flow(&product).alias_origin(read(&product, &[P::Body(1), P::Value]),
        |site| product.variable_ref(site)), Some(first));
    let assigned = read(&product, &[P::Body(1), P::Value]);
    assert_eq!(flow(&product).alias_origin_with_expression_boundary(assigned,
        |site| product.variable_ref(site), |original| original == &site(&[P::Body(0), P::Value])), Some(assigned));
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
    assert!(flow(&other).alias_origin(first, |site| other.variable_ref(site)).is_none());
    assert!(flow(&other).alias_origin_with_expression_boundary(first,
        |site| other.variable_ref(site), |_| true).is_none());
    let literal = resolve(vec![local(Some(int(7))), ret(var())]);
    let value = read(&literal, &[P::Body(1), P::Value]);
    assert_eq!(flow(&literal).alias_origin(value, |site| literal.variable_ref(site)), Some(value));
    assert!(flow(&literal).alias_entry_origin(value, |site| literal.variable_ref(site)).is_none());
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
    assert!(flow(&product).alias_origin_with_expression_boundary(header,
        |site| product.variable_ref(site), |_| true).is_none(), "an expression boundary cannot choose a Loop predecessor");
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

#[test]
fn source_value_flow_normalizes_only_complete_readonly_loop_equations() {
    // The original traversal census includes targeted Continue/Break, nested
    // loops and iteration-local shadow bindings. No literal/type guess is used.
    for body in [
        vec![var()],
        vec![branch(vec![next()], vec![brk()])],
        vec![repeat(vec![var(), brk()]), var()],
        vec![local(Some(int(7))), var(), brk()],
    ] {
        let product = resolve(vec![repeat(body), ret(var())]);
        let header = read(&product, &[P::Body(0), P::LoopCondition]);
        let after = read(&product, &[P::Body(1), P::Value]);
        assert_eq!(header, after, "complete readonly Header/After retain the same original entry");
        assert!(matches!(flow(&product).origin(header), Some(O::Entry(_))));
        assert!(flow(&product).alias_entry_origin(header, |site| product.variable_ref(site)).is_some());
        let Some(O::Entry(binding)) = flow(&product).origin(header) else { panic!() };
        let loop_site = SourceStmtSiteV1::from_node(site(&[P::Body(0)]).node().clone());
        let (original_header, _) = flow(&product).loop_join(&loop_site, *binding, K::LoopHeader).unwrap();
        let (original_after, _) = flow(&product).loop_join(&loop_site, *binding, K::LoopAfter).unwrap();
        assert_ne!(original_header, header, "passive original Header is not permission to replace its readonly entry");
        assert_ne!(original_after, after);
        assert!(flow(&product).alias_origin(original_header, |site| product.variable_ref(site)).is_none());
        let wrong_site = SourceStmtSiteV1::from_node(site(&[P::Body(99)]).node().clone());
        assert!(flow(&product).loop_join(&wrong_site, *binding, K::LoopHeader).is_none());

    }
    for body in [
        vec![set(int(1))],
        vec![branch(vec![set(int(1)), next()], vec![brk()])],
        vec![repeat(vec![set(int(1)), brk()])],
        vec![set(var())], // Alias evaluation is a new source definition.
    ] {
        let product = resolve(vec![repeat(body), ret(var())]);
        join(&product, read(&product, &[P::Body(0), P::LoopCondition]), K::LoopHeader);
        join(&product, read(&product, &[P::Body(1), P::Value]), K::LoopAfter);
    }
    // Port invariance is not an assertion that every body path is write-free.
    // A Return path does not contribute to a Header or After predecessor.
    let returning = resolve(vec![repeat(vec![set(int(9)), ret(var())]), ret(var())]);
    assert!(matches!(flow(&returning).origin(read(&returning, &[P::Body(1), P::Value])), Some(O::Entry(_))));
    expression(&returning, read(&returning, &[P::Body(0), P::LoopBody(1), P::Value]),
        &[P::Body(0), P::LoopBody(0), P::Value]);
    let shadow = resolve(vec![repeat(vec![local(Some(int(7))), var(), brk()]), ret(var())]);
    let inner = read(&shadow, &[P::Body(0), P::LoopBody(1)]);
    expression(&shadow, inner, &[P::Body(0), P::LoopBody(0), P::Initializer(0)]);
    assert!(flow(&shadow).alias_entry_origin(inner, |site| shadow.variable_ref(site)).is_none());
}
