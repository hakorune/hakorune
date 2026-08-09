use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::{
    CallableSourceLedgerRejectV1, CallableSourceRowDispositionV1, CallableSourceRowFamilyV1,
    FunctionOwnerIssuerV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    OwnedExprSiteV1, ResolvedLoopRegionLookupErrorV1, SourceBindingSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "ledger_fixture".into(),
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

fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: method.into(),
        arguments,
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

fn node(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn stmt(index: u32) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(node(vec![SourcePathSegmentV1::Body(index)]))
}

#[test]
fn ledger_exposes_typed_rows_and_resolver_identity_without_a_copy() {
    let tree = function(vec![
        local("x", literal(1)),
        ASTNode::Assignment {
            target: Box::new(variable("x")),
            value: Box::new(literal(2)),
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();

    assert_eq!(view.owner(), owner);
    assert_eq!(
        view.source_kind(),
        super::SemanticOwnerSourceKindV1::DeclaredFunction
    );
    assert_eq!(view.declaration_sites().count(), 1);
    assert!(view.variable_refs().count() >= 1);
    assert_eq!(view.assignment_targets().count(), 1);
    assert!(view.resolved_exits().count() >= 1);
    assert!(view.source_site_inventory().contains_statement(&stmt(2)));
    assert_eq!(view.capture_demands().len(), 0);

    assert!(view.family_count(CallableSourceRowFamilyV1::Declaration) > 0);
    assert!(view.family_count(CallableSourceRowFamilyV1::AssignmentTarget) > 0);
    assert!(view.family_count(CallableSourceRowFamilyV1::LoopMembership) > 0);
    assert_eq!(CallableSourceRowFamilyV1::ALL.len(), 8);
    for family in CallableSourceRowFamilyV1::ALL {
        let disposition = view.family_disposition(family);
        assert_eq!(disposition.count(), view.family_count(family));
    }
    let declaration_sites = view.declaration_sites().collect::<Vec<_>>();
    assert_eq!(
        declaration_sites.len(),
        declaration_sites
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    );
    assert!(matches!(
        view.family_disposition(CallableSourceRowFamilyV1::DirectCall),
        CallableSourceRowDispositionV1::Empty
    ));
    assert!(matches!(
        view.family_disposition(CallableSourceRowFamilyV1::MethodCall),
        CallableSourceRowDispositionV1::Empty
    ));
    assert_eq!(
        view.family_disposition(CallableSourceRowFamilyV1::Declaration)
            .count(),
        1
    );
}

#[test]
fn ledger_seals_method_call_receiver_arguments_and_result_without_ast_borrows() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Return {
            value: Some(Box::new(method_call(
                variable("text"),
                "slice",
                vec![literal(1), literal(2)],
            ))),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let rows = view.method_calls().collect::<Vec<_>>();

    assert_eq!(rows.len(), 1);
    let (site, row) = rows[0];
    assert_eq!(row.owner(), owner);
    assert_eq!(row.site(), site);
    assert_eq!(row.result_site(), site);
    assert_eq!(row.selector(), "slice");
    assert_eq!(row.arity(), 2);
    assert_eq!(
        row.arguments()
            .iter()
            .map(|argument| argument.ordinal())
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert!(view
        .source_site_inventory()
        .contains_expression(row.receiver_site()));
    assert!(row.arguments().iter().all(|argument| view
        .source_site_inventory()
        .contains_expression(argument.site())));
    assert_eq!(view.family_count(CallableSourceRowFamilyV1::MethodCall), 1);
}

#[test]
fn ledger_loop_membership_is_issued_by_the_sealed_index() {
    let tree = function(vec![ASTNode::Loop {
        condition: Box::new(literal(1)),
        body: Vec::new(),
        span: Span::unknown(),
    }]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let loop_site = stmt(0);
    let membership = view.resolved_loop_source(&loop_site).unwrap();

    assert_eq!(membership.source().site(), &loop_site);
    assert_eq!(
        membership.source().function_origin(),
        view.function_origin()
    );
    assert!(membership.frame().matches(&membership.frame().clone()));

    let missing = stmt(9);
    assert_eq!(
        view.resolved_loop_source(&missing),
        Err(ResolvedLoopRegionLookupErrorV1::MissingExactBundle(missing))
    );
}

#[test]
fn ledger_only_loop_site_requires_exactly_one_resolver_member() {
    let one_loop = function(vec![ASTNode::Loop {
        condition: Box::new(literal(1)),
        body: Vec::new(),
        span: Span::unknown(),
    }]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&one_loop).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let membership = view.only_loop_site().unwrap();
    assert_eq!(membership.source().site(), &stmt(0));

    let no_loop = function(vec![local("x", literal(1))]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&no_loop).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    assert_eq!(
        view.only_loop_site(),
        Err(ResolvedLoopRegionLookupErrorV1::NoUniqueLoopSite { actual: 0 })
    );

    let two_loops = function(vec![
        ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: Vec::new(),
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: Vec::new(),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&two_loops).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    assert_eq!(
        view.only_loop_site(),
        Err(ResolvedLoopRegionLookupErrorV1::NoUniqueLoopSite { actual: 2 })
    );
}

#[test]
fn ledger_keeps_lambda_capture_at_the_existing_forest_boundary() {
    let tree = function(vec![
        local("outer", literal(1)),
        local(
            "child",
            ASTNode::Lambda {
                params: Vec::new(),
                body: vec![ASTNode::Return {
                    value: Some(Box::new(variable("outer"))),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
        ),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let definition = OwnedExprSiteV1::new(
        root,
        super::SourceExprSiteV1::from_node(node(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Initializer(0),
        ])),
    );
    let child = forest.child_at(&definition).unwrap();
    let view = forest.callable_source_ledger(child).unwrap();

    assert_eq!(view.capture_demands().len(), 1);
    assert_eq!(view.capture_demands()[0].source_binding().owner(), root);
}

#[test]
fn ledger_rejects_foreign_or_non_callable_owner_before_builder_effects() {
    let tree = function(vec![local("x", literal(1))]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let foreign = FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap();

    assert!(matches!(
        forest.callable_source_ledger(foreign),
        Err(CallableSourceLedgerRejectV1::MissingOwner(owner)) if owner == foreign
    ));
}
