use std::sync::Arc;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::loop_recipe_contract::{
    LoopConditionV2, LoopNodeSourceBindingV1, LoopNodeV2, LoopRecipeBlockV2, LoopRecipeSourceBindingV1,
    LoopRecipeSourceOwnerV1, LoopRecipeV2, LoopRecipeVerifierV2, LoopSourcePathStepV1, LoopSourcePathV1,
};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SemanticOwnerSourceKindV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

use super::{bind_resolved_loop_root_v1, LoopRootSourceBindingRejectV1};

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn loop_stmt(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(int(1)),
        body,
        span: Span::unknown(),
    }
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

fn stmt(segments: Vec<SourcePathSegmentV1>) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn resolve_function(tree: &ASTNode) -> Arc<VerifiedResolvedFunctionV1> {
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_ast(tree).unwrap())
        .unwrap()
}

fn bind(
    product: &VerifiedResolvedFunctionV1,
    site: SourceStmtSiteV1,
) -> Result<LoopRecipeSourceBindingV1, LoopRootSourceBindingRejectV1> {
    let recipe = verified_root_recipe_v2();
    bind_resolved_loop_root_v1(product.resolved_loop_source(&site).unwrap())
        .map(|source| source.into_root_claim_v2(&recipe))
}

fn expected(steps: Vec<LoopSourcePathStepV1>) -> LoopRecipeSourceBindingV1 {
    let recipe = verified_root_recipe_v2();
    LoopRecipeSourceBindingV1::new(
        LoopRecipeSourceOwnerV1::function_body(0, 0),
        vec![LoopNodeSourceBindingV1::new(
            recipe.root_loop(),
            LoopSourcePathV1::new(steps),
        )],
    )
}

fn verified_root_recipe_v2() -> crate::mir::loop_recipe_contract::VerifiedLoopRecipeV2 {
    LoopRecipeVerifierV2::verify(LoopRecipeV2 {
        read_only_inputs: Vec::new(),
        root_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
        loops: vec![LoopNodeV2 {
            key: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV2::Always,
            body: crate::mir::loop_recipe_contract::LoopBlockKeyV1::new(0),
        }],
        blocks: vec![LoopRecipeBlockV2 {
            key: crate::mir::loop_recipe_contract::LoopBlockKeyV1::new(0),
            owner_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
            items: vec![],
        }],
        items: vec![],
        bindings: vec![],
        values: vec![],
        inputs: vec![],
        carriers: vec![],
        exits: vec![],
    })
    .expect("minimal V2 root verifies")
}

#[test]
fn direct_function_loop_has_one_exact_body_item() {
    let tree = function(vec![loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);

    assert_eq!(
        bind(&product, stmt(vec![SourcePathSegmentV1::Body(0)])).unwrap(),
        expected(vec![LoopSourcePathStepV1::BodyItem { index: 0 }])
    );
}

#[test]
fn resolved_root_adapter_issues_v2_root_without_manual_coordinates() {
    let tree = function(vec![loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);
    let source = bind_resolved_loop_root_v1(
        product
            .resolved_loop_source(&stmt(vec![SourcePathSegmentV1::Body(0)]))
            .unwrap(),
    )
    .unwrap();
    let claim = source.into_root_claim_v2(&verified_root_recipe_v2());
    assert_eq!(
        claim,
        LoopRecipeSourceBindingV1::new(
            LoopRecipeSourceOwnerV1::function_body(0, 0),
            vec![LoopNodeSourceBindingV1::new(
                crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
                LoopSourcePathV1::new(vec![LoopSourcePathStepV1::BodyItem { index: 0 }]),
            )],
        )
    );
}

#[test]
fn scope_loop_preserves_scope_body_root_and_item_identity() {
    let tree = function(vec![ASTNode::ScopeBox {
        body: vec![loop_stmt(Vec::new())],
        span: Span::unknown(),
    }]);
    let product = resolve_function(&tree);

    assert_eq!(
        bind(
            &product,
            stmt(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::ScopeBody(0),
            ]),
        )
        .unwrap(),
        expected(vec![
            LoopSourcePathStepV1::BodyItem { index: 0 },
            LoopSourcePathStepV1::ScopeBodyItem { index: 0 },
        ])
    );
}

#[test]
fn nested_loop_preserves_loop_body_root_and_item_identity() {
    let tree = function(vec![loop_stmt(vec![loop_stmt(Vec::new())])]);
    let product = resolve_function(&tree);

    assert_eq!(
        bind(
            &product,
            stmt(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::LoopBody(0),
            ]),
        )
        .unwrap(),
        expected(vec![
            LoopSourcePathStepV1::BodyItem { index: 0 },
            LoopSourcePathStepV1::LoopBodyItem { index: 0 },
        ])
    );
}

#[test]
fn multiple_scopes_under_a_loop_preserve_every_container_boundary() {
    let tree = function(vec![loop_stmt(vec![ASTNode::ScopeBox {
        body: vec![ASTNode::ScopeBox {
            body: vec![loop_stmt(Vec::new())],
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }])]);
    let product = resolve_function(&tree);

    assert_eq!(
        bind(
            &product,
            stmt(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::LoopBody(0),
                SourcePathSegmentV1::ScopeBody(0),
                SourcePathSegmentV1::ScopeBody(0),
            ]),
        )
        .unwrap(),
        expected(vec![
            LoopSourcePathStepV1::BodyItem { index: 0 },
            LoopSourcePathStepV1::LoopBodyItem { index: 0 },
            LoopSourcePathStepV1::ScopeBodyItem { index: 0 },
            LoopSourcePathStepV1::ScopeBodyItem { index: 0 },
        ])
    );
}

#[test]
fn supported_owner_with_if_then_ancestor_preserves_branch_path() {
    let tree = function(vec![ASTNode::If {
        condition: Box::new(int(1)),
        then_body: vec![loop_stmt(Vec::new())],
        else_body: None,
        span: Span::unknown(),
    }]);
    let product = resolve_function(&tree);
    let site = stmt(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::IfThen(0),
    ]);

    assert_eq!(
        bind_resolved_loop_root_v1(product.resolved_loop_source(&site).unwrap())
            .unwrap()
            .into_root_claim_v2(&verified_root_recipe_v2()),
        expected(vec![
            LoopSourcePathStepV1::BodyItem { index: 0 },
            LoopSourcePathStepV1::IfThenItem { index: 0 },
        ])
    );
}

#[test]
fn lambda_owner_is_typed_unsupported_before_path_projection() {
    let tree = ASTNode::Lambda {
        params: Vec::new(),
        body: vec![loop_stmt(Vec::new())],
        span: Span::unknown(),
    };
    let product = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(FunctionSyntaxViewV1::from_lambda_ast(&tree).unwrap())
        .unwrap();
    let site = stmt(vec![SourcePathSegmentV1::LambdaBody(0)]);

    assert_eq!(
        bind_resolved_loop_root_v1(product.resolved_loop_source(&site).unwrap()),
        Err(LoopRootSourceBindingRejectV1::UnsupportedOwnerRoot(
            SemanticOwnerSourceKindV1::Lambda,
        ))
    );
}
