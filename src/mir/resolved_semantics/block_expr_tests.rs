use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::expression_source::ResolvedExpressionSourceInventoryV1;
use super::ids::{FunctionOwnerIdV1, FunctionOwnerIssuerV1, RegionId, ScopeId};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    RegionKindV1, RegionOriginV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::resolver::{FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1};
use super::shadow::ShadowResolveErrorV0;
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};
use super::verifier::{source_region_contains_site_v1, ResolvedFunctionVerificationErrorV1};
use super::{
    FunctionSyntaxViewV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
};

fn owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn node(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn blockexpr_root() -> SourceNodeSiteV1 {
    node(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::BlockExprPreludeRoot,
    ])
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
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

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn local(name: &str, initial: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(initial))],
        declared_type_names: vec![None],
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

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn lambda(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Lambda {
        params: Vec::new(),
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

fn expr(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(node(segments))
}

fn stmt(segments: Vec<SourcePathSegmentV1>) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(node(segments))
}

fn resolve(tree: &ASTNode) -> std::sync::Arc<super::VerifiedResolvedFunctionV1> {
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(tree).unwrap())
        .unwrap()
}

fn blockexpr_data(owner: FunctionOwnerIdV1) -> ResolvedFunctionDataV1 {
    let function_origin = FunctionOriginV1::new(0, 0);
    let function_scope = ScopeId::new(owner, 0);
    let body_scope = ScopeId::new(owner, 1000);
    let blockexpr_scope = ScopeId::new(owner, 1);
    let function_region = RegionId::new(owner, 0);
    let body_region = RegionId::new(owner, 1000);
    let blockexpr_region = RegionId::new(owner, 1);
    let body_origin = node(vec![SourcePathSegmentV1::FunctionBody]);
    let blockexpr_origin = blockexpr_root();

    ResolvedFunctionDataV1 {
        brand_call_relations: BTreeMap::new(),
        explicit_extern_calls: BTreeMap::new(),
        owner,
        function_origin,
        root_profile: super::SemanticOwnerRootProfileV1::DeclaredFunction {
            receiver_policy: super::ReceiverPolicyV1::Absent,
        },
        function_scope,
        function_region,
        bindings: BTreeMap::new(),
        scopes: BTreeMap::from([
            (
                function_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::Function,
                    None,
                    function_region,
                    Vec::new(),
                    ScopeOriginV1::Function(function_origin),
                ),
            ),
            (
                body_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::LexicalBlock,
                    Some(function_scope),
                    body_region,
                    Vec::new(),
                    ScopeOriginV1::Source(body_origin.clone()),
                ),
            ),
            (
                blockexpr_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::BlockExpr,
                    Some(body_scope),
                    blockexpr_region,
                    Vec::new(),
                    ScopeOriginV1::Source(blockexpr_origin.clone()),
                ),
            ),
        ]),
        regions: BTreeMap::from([
            (
                function_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Function,
                    None,
                    Some(function_scope),
                    RegionOriginV1::Function(function_origin),
                ),
            ),
            (
                body_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Sequence,
                    Some(function_region),
                    Some(body_scope),
                    RegionOriginV1::Source(body_origin),
                ),
            ),
            (
                blockexpr_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::BlockExpr,
                    Some(body_region),
                    Some(blockexpr_scope),
                    RegionOriginV1::Source(blockexpr_origin),
                ),
            ),
        ]),
        declarations: BTreeMap::new(),
        variable_uses: BTreeMap::new(),
        assignment_targets: BTreeMap::new(),
        direct_call_targets: BTreeMap::new(),
        method_calls: BTreeMap::new(),
        expression_source: ResolvedExpressionSourceInventoryV1::default(),
        resolved_exits: BTreeMap::new(),
    }
}

fn seal(
    data: ResolvedFunctionDataV1,
) -> Result<super::VerifiedResolvedFunctionV1, ResolvedFunctionVerificationErrorV1> {
    ResolvedFunctionDraftV1 { data }.seal()
}

#[test]
fn sealed_blockexpr_pair_has_exact_kinds_origin_and_normalized_parity() {
    let first = seal(blockexpr_data(owner())).unwrap();
    let second = seal(blockexpr_data(owner())).unwrap();
    let scope = first.scope(ScopeId::new(first.owner(), 1)).unwrap();
    let region = first.region(RegionId::new(first.owner(), 1)).unwrap();

    assert_eq!(scope.kind(), ScopeKindV1::BlockExpr);
    assert_eq!(region.kind(), RegionKindV1::BlockExpr);
    assert_eq!(scope.owner_region(), RegionId::new(first.owner(), 1));
    assert_eq!(region.lexical_scope(), Some(ScopeId::new(first.owner(), 1)));
    assert_eq!(first.normalized_graph(), second.normalized_graph());
}

#[test]
fn seal_rejects_blockexpr_scope_with_non_blockexpr_region() {
    let owner = owner();
    let mut data = blockexpr_data(owner);
    data.regions.insert(
        RegionId::new(owner, 1),
        ResolvedRegionRecordV1::new(
            RegionKindV1::LexicalScope,
            Some(RegionId::new(owner, 1000)),
            Some(ScopeId::new(owner, 1)),
            RegionOriginV1::Source(blockexpr_root()),
        ),
    );

    assert!(matches!(
        seal(data),
        Err(ResolvedFunctionVerificationErrorV1::BlockExprScopeContractMismatch(_))
    ));
}

#[test]
fn seal_rejects_blockexpr_region_with_non_blockexpr_scope() {
    let owner = owner();
    let mut data = blockexpr_data(owner);
    data.scopes.insert(
        ScopeId::new(owner, 1),
        ResolvedScopeRecordV1::new(
            ScopeKindV1::LexicalBlock,
            Some(ScopeId::new(owner, 1000)),
            RegionId::new(owner, 1),
            Vec::new(),
            ScopeOriginV1::Source(blockexpr_root()),
        ),
    );

    assert!(matches!(
        seal(data),
        Err(ResolvedFunctionVerificationErrorV1::BlockExprRegionContractMismatch(_))
    ));
}

#[test]
fn seal_rejects_blockexpr_pair_with_different_exact_origins() {
    let owner = owner();
    let mut data = blockexpr_data(owner);
    data.regions.insert(
        RegionId::new(owner, 1),
        ResolvedRegionRecordV1::new(
            RegionKindV1::BlockExpr,
            Some(RegionId::new(owner, 1000)),
            Some(ScopeId::new(owner, 1)),
            RegionOriginV1::Source(node(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::BlockExprPreludeRoot,
            ])),
        ),
    );

    assert!(matches!(
        seal(data),
        Err(ResolvedFunctionVerificationErrorV1::BlockExprScopeContractMismatch(_))
    ));
}

#[test]
fn blockexpr_region_contains_only_its_prelude_and_tail_descendants() {
    let origin = RegionOriginV1::Source(blockexpr_root());
    let contained = [
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprPrelude(0),
        ]),
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprPrelude(0),
            SourcePathSegmentV1::Value,
        ]),
        node(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprTail,
        ]),
    ];
    for site in contained {
        assert!(source_region_contains_site_v1(
            super::SemanticOwnerRootProfileV1::DeclaredFunction {
                receiver_policy: super::ReceiverPolicyV1::Absent,
            },
            RegionKindV1::BlockExpr,
            &origin,
            &site
        ));
    }
    for site in [
        node(vec![SourcePathSegmentV1::Body(1)]),
        node(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::BlockExprTail,
        ]),
        blockexpr_root(),
    ] {
        assert!(!source_region_contains_site_v1(
            super::SemanticOwnerRootProfileV1::DeclaredFunction {
                receiver_policy: super::ReceiverPolicyV1::Absent,
            },
            RegionKindV1::BlockExpr,
            &origin,
            &site
        ));
    }
}

#[test]
fn resolver_accepts_empty_blockexpr_as_one_exact_scope_region_pair() {
    let product = resolve(&function(vec![block_expr(Vec::new(), int(1))]));
    assert_eq!(product.scope_count(), 3);
    assert_eq!(product.region_count(), 3);
    assert_eq!(
        product
            .scope(ScopeId::new(product.owner(), 2))
            .unwrap()
            .kind(),
        ScopeKindV1::BlockExpr
    );
    assert_eq!(
        product
            .region(RegionId::new(product.owner(), 2))
            .unwrap()
            .kind(),
        RegionKindV1::BlockExpr
    );
}

#[test]
fn prelude_local_is_visible_to_tail_but_does_not_leak() {
    let accepted = resolve(&function(vec![block_expr(
        vec![local("x", int(1))],
        var("x"),
    )]));
    let declaration = accepted
        .declaration_binding(&SourceBindingSiteV1::Local {
            statement: stmt(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::BlockExprPrelude(0),
            ]),
            ordinal: 0,
        })
        .unwrap();
    assert_eq!(
        accepted.variable_ref(&expr(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprTail,
        ])),
        Some(ResolvedLexicalRefV1::Local(declaration))
    );

    let error = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(
            FunctionSyntaxViewV1::from_ast(&function(vec![
                block_expr(vec![local("x", int(1))], int(0)),
                var("x"),
            ]))
            .unwrap(),
        )
        .unwrap_err();
    assert!(
        matches!(error, ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::UnresolvedName { name, .. }) if &*name == "x")
    );
}

#[test]
fn shadow_initializer_and_following_use_restore_the_outer_binding() {
    let product = resolve(&function(vec![
        local("x", int(1)),
        block_expr(vec![local("x", var("x"))], var("x")),
        var("x"),
    ]));
    let outer = product
        .declaration_binding(&SourceBindingSiteV1::Local {
            statement: stmt(vec![SourcePathSegmentV1::Body(0)]),
            ordinal: 0,
        })
        .unwrap();
    let inner = product
        .declaration_binding(&SourceBindingSiteV1::Local {
            statement: stmt(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::BlockExprPrelude(0),
            ]),
            ordinal: 0,
        })
        .unwrap();
    let reference = |segments| product.variable_ref(&expr(segments));
    assert_eq!(
        reference(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::BlockExprPrelude(0),
            SourcePathSegmentV1::Initializer(0)
        ]),
        Some(ResolvedLexicalRefV1::Local(outer))
    );
    assert_eq!(
        reference(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::BlockExprTail
        ]),
        Some(ResolvedLexicalRefV1::Local(inner))
    );
    assert_eq!(
        reference(vec![SourcePathSegmentV1::Body(2)]),
        Some(ResolvedLexicalRefV1::Local(outer))
    );
}

#[test]
fn blockexpr_assignment_rebinds_the_existing_outer_binding() {
    let product = resolve(&function(vec![
        local("x", int(1)),
        block_expr(vec![assign("x", int(2))], var("x")),
    ]));
    let outer = product
        .declaration_binding(&SourceBindingSiteV1::Local {
            statement: stmt(vec![SourcePathSegmentV1::Body(0)]),
            ordinal: 0,
        })
        .unwrap();
    assert_eq!(
        product.assignment_target(&expr(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::BlockExprPrelude(0),
            SourcePathSegmentV1::Target,
        ])),
        Some(&ResolvedAssignmentTargetV1::BindingRebind(outer))
    );
}

#[test]
fn condition_blockexpr_locals_end_before_if_branch_and_loop_body() {
    let if_tree = function(vec![ASTNode::If {
        condition: Box::new(block_expr(vec![local("x", int(1))], int(1))),
        then_body: vec![var("x")],
        else_body: None,
        span: Span::unknown(),
    }]);
    let loop_tree = function(vec![ASTNode::Loop {
        condition: Box::new(block_expr(vec![local("x", int(1))], int(1))),
        body: vec![var("x")],
        span: Span::unknown(),
    }]);
    for tree in [if_tree, loop_tree] {
        let error = FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
            .unwrap_err();
        assert!(
            matches!(error, ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::UnresolvedName { name, .. }) if &*name == "x")
        );
    }
}

#[test]
fn nested_blockexprs_get_independent_lexical_pairs() {
    let product = resolve(&function(vec![block_expr(
        vec![local("outer", int(1))],
        block_expr(vec![local("inner", var("outer"))], var("inner")),
    )]));
    assert_eq!(product.scope_count(), 4);
    assert_eq!(product.region_count(), 4);
    let outer = product.scope(ScopeId::new(product.owner(), 2)).unwrap();
    let inner = product.scope(ScopeId::new(product.owner(), 3)).unwrap();
    assert_eq!(outer.kind(), ScopeKindV1::BlockExpr);
    assert_eq!(inner.kind(), ScopeKindV1::BlockExpr);
    assert_eq!(inner.parent(), Some(ScopeId::new(product.owner(), 2)));
}

#[test]
fn same_scope_redeclaration_inside_blockexpr_is_typed_rejection() {
    let tree = function(vec![block_expr(
        vec![local("x", int(1)), local("x", int(2))],
        int(0),
    )]);
    let error = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap_err();
    assert!(
        matches!(error, ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::SameScopeRedeclaration { name }) if &*name == "x")
    );
}

#[test]
fn blockexpr_rejects_non_local_exit_with_exact_container_site() {
    let prelude = function(vec![block_expr(vec![return_value(int(1))], int(0))]);
    let prelude_error = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&prelude).unwrap())
        .unwrap_err();
    assert_eq!(
        prelude_error,
        ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::BlockExprNonLocalExit {
            site: ResolvedExitSiteV1::Statement(stmt(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::BlockExprPrelude(0),
            ])),
        })
    );

    let tail = function(vec![block_expr(Vec::new(), return_value(int(1)))]);
    let tail_error = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(&tail).unwrap())
        .unwrap_err();
    assert_eq!(
        tail_error,
        ResolveFunctionErrorV1::Syntax(ShadowResolveErrorV0::BlockExprNonLocalExit {
            site: ResolvedExitSiteV1::Expression(expr(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::BlockExprTail,
            ])),
        })
    );
}

#[test]
fn loop_local_exits_inside_blockexpr_are_accepted_but_outer_loop_exit_is_not() {
    let loop_node = ASTNode::Loop {
        condition: Box::new(int(1)),
        body: vec![
            ASTNode::Continue {
                span: Span::unknown(),
            },
            ASTNode::Break {
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };
    resolve(&function(vec![block_expr(vec![loop_node], int(0))]));

    let escaping = function(vec![ASTNode::Loop {
        condition: Box::new(int(1)),
        body: vec![block_expr(
            vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            int(0),
        )],
        span: Span::unknown(),
    }]);
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve(FunctionSyntaxViewV1::from_ast(&escaping).unwrap()),
        Err(ResolveFunctionErrorV1::Syntax(
            ShadowResolveErrorV0::BlockExprNonLocalExit { .. }
        ))
    ));
}

#[test]
fn lambda_children_observe_blockexpr_declaration_order_and_parent_scope() {
    let tree = function(vec![block_expr(
        vec![
            local("early", int(1)),
            local("first", lambda(vec![return_value(var("early"))])),
            local("late", int(2)),
        ],
        lambda(vec![return_value(var("late"))]),
    )]);
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let comparison = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    assert_eq!(forest.normalized_graph(), comparison.normalized_graph());
    let root = forest.roots()[0];
    let definition = |segments| OwnedExprSiteV1::new(root, expr(segments));
    let first = forest
        .child_at(&definition(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprPrelude(1),
            SourcePathSegmentV1::Initializer(0),
        ]))
        .unwrap();
    let tail = forest
        .child_at(&definition(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::BlockExprTail,
        ]))
        .unwrap();
    let block_scope = forest.parent(first).unwrap().parent_scope();
    assert_eq!(forest.parent(tail).unwrap().parent_scope(), block_scope);
    assert_eq!(
        forest
            .owner(root)
            .unwrap()
            .scope(block_scope)
            .unwrap()
            .kind(),
        ScopeKindV1::BlockExpr
    );
    assert!(matches!(
        forest.owner(first).unwrap().variable_ref(&expr(vec![
            SourcePathSegmentV1::LambdaBody(0),
            SourcePathSegmentV1::Value
        ])),
        Some(ResolvedLexicalRefV1::Upvar(_))
    ));
    assert!(matches!(
        forest.owner(tail).unwrap().variable_ref(&expr(vec![
            SourcePathSegmentV1::LambdaBody(0),
            SourcePathSegmentV1::Value
        ])),
        Some(ResolvedLexicalRefV1::Upvar(_))
    ));
}

#[test]
fn blockexpr_prelude_lambda_cannot_observe_a_later_declaration() {
    let tree = function(vec![block_expr(
        vec![
            local("first", lambda(vec![return_value(var("late"))])),
            local("late", int(1)),
        ],
        int(0),
    )]);
    let error = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap_err();
    assert!(matches!(
        error,
        super::ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::Syntax(
            ShadowResolveErrorV0::UnresolvedName { name, .. }
        )) if &*name == "late"
    ));
}
