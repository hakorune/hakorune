use hakorune_mir_core::BindingId;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::ids::FunctionOwnerIssuerV1;
use super::owner_forest::{
    OwnerParentEdgeV1, SemanticOwnerForestDraftV1, SemanticOwnerForestVerificationErrorV1,
};
use super::tests::sample_verified_for_owner_forest;
use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, OwnedExprSiteV1,
    ResolveFunctionErrorV1, ResolveOwnerForestErrorV1, ResolvedControlTransferV1,
    ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};

fn function(body: Vec<ASTNode>) -> ASTNode {
    function_with_receiver(body, false)
}

fn function_with_receiver(body: Vec<ASTNode>, has_receiver: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "root".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: !has_receiver,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
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

fn local(name: &str, initializer: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(initializer))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn lambda(params: &[&str], body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Lambda {
        params: params.iter().map(|name| (*name).to_owned()).collect(),
        body,
        span: Span::unknown(),
    }
}

fn return_value(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

#[test]
fn seal_derives_roots_and_child_index_from_primary_topology() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let parent = issuer.issue().unwrap();
    let child = issuer.issue().unwrap();
    let parent_product = sample_verified_for_owner_forest(parent, BindingId::new(0));
    let parent_scope = parent_product.function_scope();
    let child_product = sample_verified_for_owner_forest(child, BindingId::new(0));
    let definition_site = OwnedExprSiteV1::new(
        parent,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ])),
    );
    let edge = OwnerParentEdgeV1::new(parent, definition_site.clone(), parent_scope);
    let mut draft = SemanticOwnerForestDraftV1::new();
    draft.insert_owner(parent, parent_product).unwrap();
    draft.insert_owner(child, child_product).unwrap();
    draft.insert_parent(child, edge).unwrap();
    let forest = draft.seal().unwrap();

    assert_eq!(forest.roots(), &[parent]);
    assert_eq!(forest.child_at(&definition_site), Some(child));
    assert_eq!(forest.parent(child).unwrap().parent_owner(), parent);
    assert_eq!(forest.owner_count(), 2);
}

#[test]
fn seal_rejects_parent_cycle() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let first = issuer.issue().unwrap();
    let second = issuer.issue().unwrap();
    let first_product = sample_verified_for_owner_forest(first, BindingId::new(0));
    let second_product = sample_verified_for_owner_forest(second, BindingId::new(0));
    let first_scope = first_product.function_scope();
    let second_scope = second_product.function_scope();
    let site = |owner| {
        OwnedExprSiteV1::new(
            owner,
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
            ])),
        )
    };
    let mut draft = SemanticOwnerForestDraftV1::new();
    draft.insert_owner(first, first_product).unwrap();
    draft.insert_owner(second, second_product).unwrap();
    draft
        .insert_parent(
            first,
            OwnerParentEdgeV1::new(second, site(second), second_scope),
        )
        .unwrap();
    draft
        .insert_parent(
            second,
            OwnerParentEdgeV1::new(first, site(first), first_scope),
        )
        .unwrap();
    let result = draft.seal();

    assert!(matches!(
        result,
        Err(SemanticOwnerForestVerificationErrorV1::ParentCycle(_))
    ));
}

#[test]
fn resolver_seals_one_noncapturing_lambda_as_a_child_owner() {
    let tree = function(vec![local(
        "f",
        lambda(&["x"], vec![return_value(variable("x"))]),
    )]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();

    let root = forest.roots()[0];
    let definition_site = OwnedExprSiteV1::new(
        root,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ])),
    );
    let child = forest.child_at(&definition_site).unwrap();
    let child_product = forest.owner(child).unwrap();
    let parameter = child_product
        .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
        .unwrap();
    let use_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Value,
    ]));
    let return_site = ResolvedExitSiteV1::Statement(SourceStmtSiteV1::from_node(
        SourceNodeSiteV1::from_segments(vec![SourcePathSegmentV1::LambdaBody(0)]),
    ));

    assert_eq!(forest.owner_count(), 2);
    assert_eq!(forest.parent(child).unwrap().parent_owner(), root);
    assert_eq!(child_product.variable_binding(&use_site), Some(parameter));
    assert_eq!(
        child_product
            .resolved_exit(&return_site)
            .unwrap()
            .transfer(),
        ResolvedControlTransferV1::Return {
            target_function: child_product.function_region(),
        }
    );
}

#[test]
fn resolver_reports_ancestor_use_as_exact_unsupported_capture() {
    let tree = function(vec![
        local("outer", int(1)),
        local("f", lambda(&[], vec![return_value(variable("outer"))])),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let error = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap_err();

    let ResolveOwnerForestErrorV1::UnsupportedCapture { use_site, source } = error else {
        panic!("expected capture boundary");
    };
    assert_ne!(use_site.owner(), source.owner());
    assert_eq!(
        use_site.site().node().segments(),
        &[
            SourcePathSegmentV1::LambdaBody(0),
            SourcePathSegmentV1::Value,
        ]
    );
}

#[test]
fn resolver_reports_receiver_capture_at_the_same_boundary() {
    let tree = function_with_receiver(
        vec![local(
            "f",
            lambda(
                &[],
                vec![return_value(ASTNode::Me {
                    span: Span::unknown(),
                })],
            ),
        )],
        true,
    );
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let error = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap_err();

    assert!(matches!(
        error,
        ResolveOwnerForestErrorV1::UnsupportedCapture { .. }
    ));
}

#[test]
fn resolver_does_not_backpatch_lambda_initializer_self_reference() {
    let tree = function(vec![local(
        "f",
        lambda(&[], vec![return_value(variable("f"))]),
    )]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let error = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap_err();

    assert!(matches!(
        error,
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::Syntax(
            super::shadow::ShadowResolveErrorV0::UnresolvedName { .. }
        ))
    ));
}

#[test]
fn normalized_forest_is_independent_of_owner_issuer_brand() {
    let tree = function(vec![local(
        "f",
        lambda(&["x"], vec![return_value(variable("x"))]),
    )]);
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let first = FunctionSemanticResolverSessionV1::new(9)
        .unwrap()
        .resolve_forest(view)
        .unwrap();
    let second = FunctionSemanticResolverSessionV1::new(9)
        .unwrap()
        .resolve_forest(view)
        .unwrap();

    assert_ne!(first.roots(), second.roots());
    assert_eq!(first.normalized_graph(), second.normalized_graph());
}

#[test]
fn seal_rejects_a_second_root_in_of0() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let first = issuer.issue().unwrap();
    let second = issuer.issue().unwrap();
    let mut draft = SemanticOwnerForestDraftV1::new();
    draft
        .insert_owner(
            first,
            sample_verified_for_owner_forest(first, BindingId::new(0)),
        )
        .unwrap();
    draft
        .insert_owner(
            second,
            sample_verified_for_owner_forest(second, BindingId::new(0)),
        )
        .unwrap();

    assert!(matches!(
        draft.seal(),
        Err(SemanticOwnerForestVerificationErrorV1::MultipleRoots)
    ));
}

#[test]
fn seal_rejects_mixed_compilation_owner_brands() {
    let first = FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap();
    let second = FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap();
    let mut draft = SemanticOwnerForestDraftV1::new();
    draft
        .insert_owner(
            first,
            sample_verified_for_owner_forest(first, BindingId::new(0)),
        )
        .unwrap();
    draft
        .insert_owner(
            second,
            sample_verified_for_owner_forest(second, BindingId::new(0)),
        )
        .unwrap();

    assert!(matches!(
        draft.seal(),
        Err(SemanticOwnerForestVerificationErrorV1::MixedCompilation(owner)) if owner == second
    ));
}

#[test]
fn seal_rejects_parent_scope_that_is_not_the_definition_scope() {
    let parent_tree = function(vec![ASTNode::ScopeBox {
        body: vec![local("x", int(1))],
        span: Span::unknown(),
    }]);
    let child_tree = function(Vec::new());
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let parent_product = std::sync::Arc::try_unwrap(
        session
            .resolve(FunctionSyntaxViewV1::from_ast(&parent_tree).unwrap())
            .unwrap(),
    )
    .unwrap();
    let child_product = std::sync::Arc::try_unwrap(
        session
            .resolve(FunctionSyntaxViewV1::from_ast(&child_tree).unwrap())
            .unwrap(),
    )
    .unwrap();
    let parent = parent_product.owner();
    let child = child_product.owner();
    let wrong_scope = parent_product.function_scope();
    let definition_site = OwnedExprSiteV1::new(
        parent,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::ScopeBody(0),
            SourcePathSegmentV1::Initializer(0),
        ])),
    );
    let mut draft = SemanticOwnerForestDraftV1::new();
    draft.insert_owner(parent, parent_product).unwrap();
    draft.insert_owner(child, child_product).unwrap();
    draft
        .insert_parent(
            child,
            OwnerParentEdgeV1::new(parent, definition_site, wrong_scope),
        )
        .unwrap();

    assert!(matches!(
        draft.seal(),
        Err(SemanticOwnerForestVerificationErrorV1::ParentScopeMismatch(owner)) if owner == child
    ));
}
