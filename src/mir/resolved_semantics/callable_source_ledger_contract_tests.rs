use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v2, CORE_METHOD_MANIFEST_BRAND_V2,
};

use super::{
    CoreMethodInstanceTargetIssuerV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    ResolvedLoopPlacementV1, ResolverCoreMethodCallableContractIssuerV1,
    ResolverCoreMethodCallableContractRejectV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
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
fn resolver_callable_contract_co_seals_condition_length_and_generated_target() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(method_call(variable("text"), "length", vec![])),
            body: Vec::new(),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let (call_site, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
        .expect("generated length row");
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();

    let contract = ResolverCoreMethodCallableContractIssuerV1::issue(
        &view,
        call,
        &membership,
        ResolvedLoopPlacementV1::Condition,
        target,
    )
    .unwrap();
    assert_eq!(contract.owner(), owner);
    assert_eq!(contract.call_site(), call_site);
    assert_eq!(contract.arguments().len(), 0);
    assert_eq!(contract.loop_site(), &stmt(1));
    assert_eq!(contract.frame(), membership.frame());
    assert_eq!(contract.placement(), ResolvedLoopPlacementV1::Condition);
    assert_eq!(contract.target().row().arity(), 0);
}

#[test]
fn resolver_callable_contract_co_seals_body_substring_and_generated_target() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: vec![ASTNode::Return {
                value: Some(Box::new(method_call(
                    variable("text"),
                    "substring",
                    vec![literal(0), literal(1)],
                ))),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 2)
        .expect("generated substring row");
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();

    let contract = ResolverCoreMethodCallableContractIssuerV1::issue(
        &view,
        call,
        &membership,
        ResolvedLoopPlacementV1::Body,
        target,
    )
    .unwrap();
    assert_eq!(contract.placement(), ResolvedLoopPlacementV1::Body);
    assert_eq!(contract.arguments().len(), 2);
    assert_eq!(contract.target().row().arity(), 2);
}

#[test]
fn resolver_callable_contract_co_seals_condition_substring_and_generated_target() {
    // `StringSubstring/2` at `Condition` is inside the bounded arm
    // vocabulary: the same contract issuer seals the site's actual
    // condition placement.
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(method_call(
                variable("text"),
                "substring",
                vec![literal(0), literal(1)],
            )),
            body: Vec::new(),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 2)
        .expect("generated substring row");
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();

    let contract = ResolverCoreMethodCallableContractIssuerV1::issue(
        &view,
        call,
        &membership,
        ResolvedLoopPlacementV1::Condition,
        target,
    )
    .unwrap();
    assert_eq!(contract.placement(), ResolvedLoopPlacementV1::Condition);
    assert_eq!(contract.arguments().len(), 2);
    assert_eq!(contract.target().row().arity(), 2);
}

#[test]
fn resolver_callable_contract_rejects_body_length_placement() {
    // `StringLen/0` remains `Condition`-only: a Body-position `length` call
    // claimed at `Body` is a named TargetPlacementMismatch.
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: vec![ASTNode::Return {
                value: Some(Box::new(method_call(variable("text"), "length", vec![]))),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let view = forest.callable_source_ledger(forest.roots()[0]).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0).unwrap();
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();

    assert_eq!(
        ResolverCoreMethodCallableContractIssuerV1::issue(
            &view,
            call,
            &membership,
            ResolvedLoopPlacementV1::Body,
            target,
        )
        .unwrap_err(),
        ResolverCoreMethodCallableContractRejectV1::TargetPlacementMismatch {
            op: CoreMethodOp::StringLen,
            expected: ResolvedLoopPlacementV1::Condition,
            actual: ResolvedLoopPlacementV1::Body,
        }
    );
}

#[test]
fn resolver_callable_contract_rejects_non_lexical_receiver() {
    let tree = function(vec![ASTNode::Loop {
        condition: Box::new(method_call(literal(7), "length", vec![])),
        body: Vec::new(),
        span: Span::unknown(),
    }]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let view = forest.callable_source_ledger(forest.roots()[0]).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0).unwrap();
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(0)).unwrap();

    assert_eq!(
        ResolverCoreMethodCallableContractIssuerV1::issue(
            &view,
            call,
            &membership,
            ResolvedLoopPlacementV1::Condition,
            target,
        )
        .unwrap_err(),
        ResolverCoreMethodCallableContractRejectV1::UnsupportedReceiver
    );
}

#[test]
fn resolver_callable_contract_rejects_argument_arity_drift() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(method_call(variable("text"), "substring", vec![literal(0)])),
            body: Vec::new(),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let view = forest.callable_source_ledger(forest.roots()[0]).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 2).unwrap();
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();

    assert_eq!(
        ResolverCoreMethodCallableContractIssuerV1::issue(
            &view,
            call,
            &membership,
            ResolvedLoopPlacementV1::Condition,
            target,
        )
        .unwrap_err(),
        ResolverCoreMethodCallableContractRejectV1::TargetArityMismatch {
            expected: 2,
            actual: 1,
        }
    );
}

#[test]
fn resolver_callable_contract_rejects_call_outside_selected_loop_body() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: Vec::new(),
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(method_call(variable("text"), "length", vec![]))),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
        .expect("generated length row");
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();

    assert!(matches!(
        ResolverCoreMethodCallableContractIssuerV1::issue(
            &view,
            call,
            &membership,
            ResolvedLoopPlacementV1::Condition,
            target
        ),
        Err(ResolverCoreMethodCallableContractRejectV1::PlacementMismatch { actual: None, .. })
    ));
}

#[test]
fn resolver_callable_contract_rejects_target_placement_drift() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(method_call(variable("text"), "length", vec![])),
            body: Vec::new(),
            span: Span::unknown(),
        },
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let owner = forest.roots()[0];
    let view = forest.callable_source_ledger(owner).unwrap();
    let (_, call) = view.method_calls().next().expect("one method call");
    let membership = view.resolved_loop_source(&stmt(1)).unwrap();
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
        .expect("generated length row");
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();

    assert!(matches!(
        ResolverCoreMethodCallableContractIssuerV1::issue(
            &view,
            call,
            &membership,
            ResolvedLoopPlacementV1::Body,
            target
        ),
        Err(
            ResolverCoreMethodCallableContractRejectV1::TargetPlacementMismatch {
                expected: ResolvedLoopPlacementV1::Condition,
                actual: ResolvedLoopPlacementV1::Body,
                ..
            }
        )
    ));
}

#[test]
fn resolver_callable_contract_rejects_foreign_loop_membership() {
    let tree = function(vec![
        local("text", literal(7)),
        ASTNode::Loop {
            condition: Box::new(method_call(variable("text"), "length", vec![])),
            body: Vec::new(),
            span: Span::unknown(),
        },
    ]);
    let mut first_session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let first = first_session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let first_view = first.callable_source_ledger(first.roots()[0]).unwrap();
    let (_, call) = first_view.method_calls().next().expect("one method call");

    let mut second_session = FunctionSemanticResolverSessionV1::new(1).unwrap();
    let second = second_session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let second_view = second.callable_source_ledger(second.roots()[0]).unwrap();
    let foreign_membership = second_view.resolved_loop_source(&stmt(1)).unwrap();

    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
        .expect("generated length row");
    let mut target_issuer =
        CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap();
    let target = target_issuer.issue(row).unwrap();

    assert_eq!(
        ResolverCoreMethodCallableContractIssuerV1::issue(
            &first_view,
            call,
            &foreign_membership,
            ResolvedLoopPlacementV1::Condition,
            target
        )
        .unwrap_err(),
        ResolverCoreMethodCallableContractRejectV1::ForeignLoopMembership
    );
}
