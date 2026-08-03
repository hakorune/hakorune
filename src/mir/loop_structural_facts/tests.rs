use std::sync::Arc;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_recipe_contract::{
    verify_artifact_for_test, LoopNodeSourceBindingV1, LoopRecipeArtifactV1,
    LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1, LoopRecipeSourceOwnerV1, LoopRecipeV1,
    LoopRecipeVerifierV1, LoopSourcePathStepV1, LoopSourcePathV1, VerifiedLoopRecipeV1,
};
use crate::mir::loop_route_policy::issue_policy_winner_for_test_with_frame;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SemanticOwnerSourceKindV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

use super::{
    bind_resolved_loop_root_v1, issue_selected_loop_recipe_demand_v1,
    verified_loop_structural_facts_for_test_with_frame, LoopRootSourceBindingRejectV1,
    DirectAccumSingletonObservationRejectV1, SelectedLoopDemandRejectV1,
};

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
    let recipe = verified_root_recipe();
    bind_resolved_loop_root_v1(product.resolved_loop_source(&site).unwrap())
        .map(|source| source.into_root_claim(&recipe))
}

fn expected(steps: Vec<LoopSourcePathStepV1>) -> LoopRecipeSourceBindingV1 {
    let recipe = verified_root_recipe();
    LoopRecipeSourceBindingV1::new(
        LoopRecipeSourceOwnerV1::function_body(0, 0),
        vec![LoopNodeSourceBindingV1::new(
            recipe.root_loop(),
            LoopSourcePathV1::new(steps),
        )],
    )
}

fn root_recipe() -> LoopRecipeV1 {
    serde_json::from_str(
        r#"{
            "root_loop": 0,
            "loops": [{"key":0,"parent":null,"condition":{"kind":"always"},"body":0}],
            "blocks": [{"key":0,"owner_loop":0,"items":[0]}],
            "items": [{"key":0,"item":{"kind":"exit","exit":0}}],
            "bindings": [],
            "values": [],
            "inputs": [],
            "carriers": [],
            "exits": [{"key":0,"owner_loop":0,"kind":{"kind":"break","target_loop":0}}]
        }"#,
    )
    .expect("minimal semantic recipe JSON")
}

fn verified_root_recipe() -> VerifiedLoopRecipeV1 {
    LoopRecipeVerifierV1::verify(root_recipe()).expect("minimal semantic recipe verifies")
}

#[test]
fn selected_demand_consumes_matching_policy_facts_and_source_once() {
    let tree = function(vec![loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);
    let site = stmt(vec![SourcePathSegmentV1::Body(0)]);
    let source = product.resolved_loop_source(&site).unwrap();
    let frame_key = source.frame_key();
    let facts = verified_loop_structural_facts_for_test_with_frame(
        crate::mir::resolved_semantics::FunctionOriginV1::new(0, 0),
        SemanticOwnerSourceKindV1::DeclaredFunction,
        site.clone(),
        frame_key.clone(),
    );

    let demand = issue_selected_loop_recipe_demand_v1(
        issue_policy_winner_for_test_with_frame(4, &frame_key),
        facts,
        source,
    )
    .expect("matching selected demand must seal");
    let (winner, _facts, source) = demand.into_parts();
    assert_eq!(winner.into_raw_cursor(), 4);
    let (origin, owner, resolved_site) = source.into_parts();
    assert_eq!(origin.compilation_unit_ordinal(), 0);
    assert_eq!(origin.function_ordinal(), 0);
    assert_eq!(owner, SemanticOwnerSourceKindV1::DeclaredFunction);
    assert_eq!(resolved_site, site);
}

#[test]
fn selected_demand_rejects_facts_source_identity_mismatch() {
    let tree = function(vec![loop_stmt(Vec::new()), loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);
    let source_site = stmt(vec![SourcePathSegmentV1::Body(0)]);
    let facts_site = stmt(vec![SourcePathSegmentV1::Body(1)]);
    let source = product.resolved_loop_source(&source_site).unwrap();
    let frame_key = source.frame_key();
    let facts = verified_loop_structural_facts_for_test_with_frame(
        crate::mir::resolved_semantics::FunctionOriginV1::new(0, 0),
        SemanticOwnerSourceKindV1::DeclaredFunction,
        facts_site,
        frame_key.clone(),
    );

    assert_eq!(
        issue_selected_loop_recipe_demand_v1(
            issue_policy_winner_for_test_with_frame(4, &frame_key),
            facts,
            source,
        ),
        Err(SelectedLoopDemandRejectV1::FactsSourceIdentityMismatch)
    );
}

#[test]
fn selected_demand_rejects_foreign_execution_frame() {
    let tree = function(vec![loop_stmt(Vec::new()), loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);
    let site = stmt(vec![SourcePathSegmentV1::Body(1)]);
    let source = product.resolved_loop_source(&site).unwrap();
    let foreign_frame = crate::mir::resolved_semantics::loop_execution_frame_key_for_test();
    let facts = verified_loop_structural_facts_for_test_with_frame(
        crate::mir::resolved_semantics::FunctionOriginV1::new(0, 0),
        SemanticOwnerSourceKindV1::DeclaredFunction,
        site,
        foreign_frame.clone(),
    );

    assert_eq!(
        issue_selected_loop_recipe_demand_v1(
            issue_policy_winner_for_test_with_frame(4, &foreign_frame),
            facts,
            source,
        ),
        Err(SelectedLoopDemandRejectV1::ExecutionFrameMismatch)
    );
}

#[test]
fn singleton_observation_rejects_identity_only_facts() {
    let tree = function(vec![loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);
    let site = stmt(vec![SourcePathSegmentV1::Body(0)]);
    let source = product.resolved_loop_source(&site).unwrap();
    let frame_key = source.frame_key();
    let facts = verified_loop_structural_facts_for_test_with_frame(
        crate::mir::resolved_semantics::FunctionOriginV1::new(0, 0),
        SemanticOwnerSourceKindV1::DeclaredFunction,
        site,
        frame_key,
    );

    assert_eq!(
        facts.into_direct_accum_singleton_observation_v1(source),
        Err(DirectAccumSingletonObservationRejectV1::NotDirectAccum)
    );
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
fn resolved_adapter_to_structurally_verified_artifact_is_end_to_end_green() {
    let tree = function(vec![loop_stmt(Vec::new())]);
    let product = resolve_function(&tree);
    let recipe = verified_root_recipe();
    assert_eq!(recipe.root_loop().raw(), 0);

    let local_source = bind_resolved_loop_root_v1(
        product
            .resolved_loop_source(&stmt(vec![SourcePathSegmentV1::Body(0)]))
            .unwrap(),
    )
    .unwrap();
    let source_binding = local_source.into_root_claim(&recipe);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1 {
            producer_route: LoopRouteId::LoopSimpleWhile,
        },
        source_binding,
        root_recipe(),
    );

    verify_artifact_for_test(artifact).expect("source-bound artifact verifies");
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
fn supported_owner_with_unportable_if_ancestor_is_typed_reject() {
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
        bind_resolved_loop_root_v1(product.resolved_loop_source(&site).unwrap()),
        Err(LoopRootSourceBindingRejectV1::UnsupportedAncestor {
            depth: 1,
            segment: SourcePathSegmentV1::IfThen(0),
        })
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
