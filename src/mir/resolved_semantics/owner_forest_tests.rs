use hakorune_mir_core::BindingId;

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};

use super::ids::FunctionOwnerIssuerV1;
use super::owner_forest::{
    OwnerParentEdgeV1, SemanticOwnerForestDraftV1, SemanticOwnerForestVerificationErrorV1,
};
use super::tests::sample_verified_for_owner_forest;
use super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, OwnedExprSiteV1,
    ResolveFunctionErrorV1, ResolveOwnerForestErrorV1, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitSiteV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1, UpvarAccessKindV1,
};

fn function(body: Vec<ASTNode>) -> ASTNode {
    function_with_signature(body, &[], false)
}

fn function_with_receiver(body: Vec<ASTNode>, has_receiver: bool) -> ASTNode {
    function_with_signature(body, &[], has_receiver)
}

fn function_with_params(body: Vec<ASTNode>, params: &[&str]) -> ASTNode {
    function_with_signature(body, params, false)
}

fn function_with_signature(body: Vec<ASTNode>, params: &[&str], has_receiver: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "root".into(),
        params: params.iter().map(|name| (*name).to_owned()).collect(),
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

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn expr_site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

#[test]
fn seal_derives_roots_and_child_index_from_primary_topology() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let parent = issuer.issue().unwrap();
    let child = issuer.issue().unwrap();
    let parent_product = sample_verified_for_owner_forest(parent, BindingId::new(0));
    let parent_scope = parent_product.lowering_roots().body_pair().scope();
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
    let first_scope = first_product.lowering_roots().body_pair().scope();
    let second_scope = second_product.lowering_roots().body_pair().scope();
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
    assert_eq!(
        child_product.variable_ref(&use_site),
        Some(ResolvedLexicalRefV1::Local(parameter))
    );
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
fn resolver_seals_read_only_ancestor_use_as_structural_upvar() {
    let tree = function(vec![
        local("outer", int(1)),
        local("f", lambda(&[], vec![return_value(variable("outer"))])),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let definition = OwnedExprSiteV1::new(
        root,
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Initializer(0),
        ])),
    );
    let child = forest.child_at(&definition).unwrap();
    let use_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Value,
    ]));
    let ResolvedLexicalRefV1::Upvar(upvar) = forest
        .owner(child)
        .unwrap()
        .variable_ref(&use_site)
        .unwrap()
    else {
        panic!("expected structural Upvar");
    };
    assert_eq!(upvar.capturing_owner(), child);
    assert_eq!(upvar.source().owner(), root);
    assert_eq!(forest.upvars(), &[upvar]);
}

#[test]
fn resolver_seals_outer_parameter_read_as_structural_upvar() {
    let tree = function_with_params(
        vec![local(
            "f",
            lambda(&[], vec![return_value(variable("outer"))]),
        )],
        &["outer"],
    );
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let child = forest
        .child_at(&OwnedExprSiteV1::new(
            root,
            expr_site(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ]),
        ))
        .unwrap();
    let use_site = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Value,
    ]);
    let ResolvedLexicalRefV1::Upvar(upvar) = forest
        .owner(child)
        .unwrap()
        .variable_ref(&use_site)
        .unwrap()
    else {
        panic!("expected parameter Upvar");
    };
    let parameter = forest
        .owner(root)
        .unwrap()
        .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
        .unwrap();

    assert_eq!(upvar.source(), parameter);
    assert_eq!(forest.upvars(), &[upvar]);
}

#[test]
fn resolver_deduplicates_multiple_reads_and_rebinds_of_the_same_structural_upvar() {
    let tree = function(vec![
        local("outer", int(1)),
        local(
            "f",
            lambda(
                &[],
                vec![
                    assign("outer", int(2)),
                    return_value(add(variable("outer"), variable("outer"))),
                ],
            ),
        ),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let child = forest
        .child_at(&OwnedExprSiteV1::new(
            root,
            expr_site(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Initializer(0),
            ]),
        ))
        .unwrap();
    let child_product = forest.owner(child).unwrap();
    let lhs = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(1),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Lhs,
    ]);
    let rhs = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(1),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Rhs,
    ]);
    let ResolvedLexicalRefV1::Upvar(lhs_upvar) = child_product.variable_ref(&lhs).unwrap() else {
        panic!("expected lhs Upvar");
    };
    let ResolvedLexicalRefV1::Upvar(rhs_upvar) = child_product.variable_ref(&rhs).unwrap() else {
        panic!("expected rhs Upvar");
    };

    assert_eq!(lhs_upvar, rhs_upvar);
    assert_eq!(forest.upvars(), &[lhs_upvar]);
    assert_eq!(forest.upvar_observations().len(), 3);
    assert!(forest
        .upvar_observations()
        .iter()
        .any(|row| row.access() == UpvarAccessKindV1::Rebind));
}

#[test]
fn resolver_flattens_grandparent_rebind_to_the_original_binding_ref() {
    let tree = function(vec![
        local("outer", int(1)),
        local(
            "parent",
            lambda(
                &[],
                vec![local("child", lambda(&[], vec![assign("outer", int(2))]))],
            ),
        ),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let parent = forest
        .child_at(&OwnedExprSiteV1::new(
            root,
            expr_site(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Initializer(0),
            ]),
        ))
        .unwrap();
    let child = forest
        .child_at(&OwnedExprSiteV1::new(
            parent,
            expr_site(vec![
                SourcePathSegmentV1::LambdaBody(0),
                SourcePathSegmentV1::Initializer(0),
            ]),
        ))
        .unwrap();
    let target_site = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::Target,
    ]);
    let Some(ResolvedAssignmentTargetV1::UpvarRebind(upvar)) =
        forest.owner(child).unwrap().assignment_target(&target_site)
    else {
        panic!("expected grandparent Upvar rebind");
    };
    let source = forest
        .owner(root)
        .unwrap()
        .declaration_binding(&SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
            ])),
            ordinal: 0,
        })
        .unwrap();

    assert_eq!(upvar.capturing_owner(), child);
    assert_eq!(upvar.source(), source);
    assert_ne!(upvar.source().owner(), parent);
    assert_eq!(forest.upvars(), &[*upvar]);
    assert_eq!(
        forest.upvar_observations()[0].access(),
        UpvarAccessKindV1::Rebind
    );
}

#[test]
fn resolver_child_local_shadow_prevents_structural_upvar() {
    let tree = function(vec![
        local("outer", int(1)),
        local(
            "f",
            lambda(
                &[],
                vec![
                    local("outer", int(2)),
                    assign("outer", int(3)),
                    return_value(variable("outer")),
                ],
            ),
        ),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let root = forest.roots()[0];
    let child = forest
        .child_at(&OwnedExprSiteV1::new(
            root,
            expr_site(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Initializer(0),
            ]),
        ))
        .unwrap();
    let child_product = forest.owner(child).unwrap();
    let use_site = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(2),
        SourcePathSegmentV1::Value,
    ]);
    let local_binding = child_product
        .declaration_binding(&SourceBindingSiteV1::Local {
            statement: SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::LambdaBody(0),
            ])),
            ordinal: 0,
        })
        .unwrap();

    assert_eq!(
        child_product.variable_ref(&use_site),
        Some(ResolvedLexicalRefV1::Local(local_binding))
    );
    let target_site = expr_site(vec![
        SourcePathSegmentV1::LambdaBody(1),
        SourcePathSegmentV1::Target,
    ]);
    assert_eq!(
        child_product.assignment_target(&target_site),
        Some(&ResolvedAssignmentTargetV1::BindingRebind(local_binding))
    );
    assert!(forest.upvars().is_empty());
}

#[test]
fn resolver_seals_receiver_read_as_structural_upvar() {
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
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();

    assert_eq!(forest.upvars().len(), 1);
    assert_eq!(forest.upvars()[0].source().owner(), forest.roots()[0]);
}

#[test]
fn resolver_seals_simple_outer_rebind_as_one_upvar_observation() {
    let tree = function(vec![
        local("outer", int(1)),
        local("f", lambda(&[], vec![assign("outer", int(2))])),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let observation = &forest.upvar_observations()[0];

    assert_eq!(forest.upvars(), &[observation.upvar()]);
    assert_eq!(observation.access(), UpvarAccessKindV1::Rebind);
    assert_eq!(
        observation.site().site(),
        &expr_site(vec![
            SourcePathSegmentV1::LambdaBody(0),
            SourcePathSegmentV1::Target,
        ])
    );
}

#[test]
fn resolver_records_compound_outer_assignment_as_read_and_rebind_at_one_site() {
    let compound = ASTNode::CompoundAssignment {
        target: Box::new(variable("outer")),
        operator: BinaryOperator::Add,
        value: Box::new(int(1)),
        span: Span::unknown(),
    };
    let tree = function(vec![
        local("outer", int(1)),
        local("f", lambda(&[], vec![compound])),
    ]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let observations = forest.upvar_observations();

    assert_eq!(forest.upvars().len(), 1);
    assert_eq!(observations.len(), 2);
    assert_eq!(observations[0].site(), observations[1].site());
    assert_eq!(observations[0].upvar(), observations[1].upvar());
    assert_eq!(observations[0].access(), UpvarAccessKindV1::Read);
    assert_eq!(observations[1].access(), UpvarAccessKindV1::Rebind);
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
fn resolver_does_not_capture_a_later_ancestor_declaration() {
    let tree = function(vec![
        local("f", lambda(&[], vec![return_value(variable("later"))])),
        local("later", int(1)),
    ]);
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
fn normalized_forest_with_structural_upvars_is_independent_of_owner_issuer_brand() {
    let tree = function(vec![
        local("outer", int(1)),
        local(
            "f",
            lambda(
                &[],
                vec![
                    assign("outer", int(2)),
                    return_value(add(variable("outer"), variable("outer"))),
                ],
            ),
        ),
    ]);
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
    assert_eq!(first.upvars().len(), 1);
    assert_eq!(second.upvars().len(), 1);
    assert!(first
        .upvar_observations()
        .iter()
        .any(|row| row.access() == UpvarAccessKindV1::Rebind));
    assert_ne!(first.upvars(), second.upvars());
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
