//! Caller-zero semantic producer facade proof.
//!
//! This is deliberately a test-only migration seam.  It consumes one already
//! selected owned recipe plus an opaque diagnostic receipt, then performs the
//! single verifier -> logical JoinSig terminal transition.  It does not own
//! route policy, AST/source lookup, Builder/PHI state, or publication.

#![cfg(test)]

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_recipe_contract::{
    LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopRecipeArtifactV1,
    LoopRecipeNormalizerV1, LoopRecipeProducerIdV1, LoopRecipeProvenanceV1,
    LoopRecipeRejectReasonV1, LoopRecipeV1, LoopRecipeVerifierV1, VerifiedLoopJoinSigV1,
    VerifiedLoopRecipeV1,
};
use crate::mir::loop_structural_facts::bind_resolved_loop_source_forest_v1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};

#[derive(Debug)]
struct ProducerReceiptV1 {
    route: LoopRouteId,
}

#[derive(Debug)]
struct VerifiedLoopRecipeDemandV1 {
    recipe: LoopRecipeV1,
    receipt: ProducerReceiptV1,
}

impl VerifiedLoopRecipeDemandV1 {
    fn new(recipe: LoopRecipeV1, route: LoopRouteId) -> Self {
        Self {
            recipe,
            receipt: ProducerReceiptV1 { route },
        }
    }
}

#[derive(Debug)]
enum ProducerRejectV1 {
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
}

#[derive(Debug)]
struct VerifiedLoopRecipeProductV1 {
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
    receipt: ProducerReceiptV1,
}

impl VerifiedLoopRecipeProductV1 {
    fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    fn diagnostic_route(&self) -> LoopRouteId {
        self.receipt.route
    }
}

struct VerifiedLoopRecipeProducerFacadeV1;

impl VerifiedLoopRecipeProducerFacadeV1 {
    fn consume(
        demand: VerifiedLoopRecipeDemandV1,
    ) -> Result<VerifiedLoopRecipeProductV1, ProducerRejectV1> {
        let VerifiedLoopRecipeDemandV1 { recipe, receipt } = demand;
        let recipe = LoopRecipeVerifierV1::verify(recipe).map_err(ProducerRejectV1::Recipe)?;
        let join_sig =
            LoopJoinSigElaboratorV1::elaborate(&recipe).map_err(ProducerRejectV1::JoinSig)?;
        Ok(VerifiedLoopRecipeProductV1 {
            recipe,
            join_sig,
            receipt,
        })
    }
}

fn recipe_from(json: &str) -> LoopRecipeV1 {
    let artifact: LoopRecipeArtifactV1 = serde_json::from_str(json).expect("recipe golden");
    artifact.recipe().clone()
}

fn nested_source_fixture() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "nested_always_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            },
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                body: vec![ASTNode::Loop {
                    condition: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    body: Vec::new(),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

#[test]
fn facade_accepts_direct_and_nested_always_golden() {
    let direct = VerifiedLoopRecipeProducerFacadeV1::consume(VerifiedLoopRecipeDemandV1::new(
        recipe_from(super::DIRECT_GOLDEN),
        LoopRouteId::AccumConstLoop,
    ))
    .expect("direct semantic product");
    assert_eq!(direct.recipe().root_loop().raw(), 0);
    assert!(!direct.join_sig().as_sig().loops.is_empty());

    let nested = VerifiedLoopRecipeProducerFacadeV1::consume(VerifiedLoopRecipeDemandV1::new(
        recipe_from(super::GOLDEN),
        LoopRouteId::AccumConstLoop,
    ))
    .expect("nested Always semantic product");
    assert_eq!(nested.join_sig().as_sig().loops.len(), 2);
}

#[test]
fn facade_semantics_ignore_diagnostic_route_receipt() {
    let recipe = recipe_from(super::DIRECT_GOLDEN);
    let left = VerifiedLoopRecipeProducerFacadeV1::consume(VerifiedLoopRecipeDemandV1::new(
        recipe.clone(),
        LoopRouteId::AccumConstLoop,
    ))
    .expect("left product");
    let right = VerifiedLoopRecipeProducerFacadeV1::consume(VerifiedLoopRecipeDemandV1::new(
        recipe,
        LoopRouteId::GenericLoopV1,
    ))
    .expect("right product");
    assert_eq!(left.diagnostic_route(), LoopRouteId::AccumConstLoop);
    assert_eq!(right.diagnostic_route(), LoopRouteId::GenericLoopV1);
    let left_json = LoopRecipeNormalizerV1::normalize_semantic(left.recipe()).expect("left json");
    let right_json =
        LoopRecipeNormalizerV1::normalize_semantic(right.recipe()).expect("right json");
    assert_eq!(left_json, right_json);
    assert_eq!(left.join_sig().as_sig(), right.join_sig().as_sig());
}

#[test]
fn nested_always_witness_binds_source_without_production_caller() {
    let source = nested_source_fixture();
    let resolved = FunctionSemanticResolverSessionV1::new(0)
        .expect("resolver session")
        .resolve(FunctionSyntaxViewV1::from_ast(&source).expect("source view"))
        .expect("resolved fixture");
    let root = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(2),
    ]));
    let projection = bind_resolved_loop_source_forest_v1(
        resolved
            .resolved_loop_source_forest(&root)
            .expect("resolved nested source forest"),
    )
    .expect("D1 source projection");

    let raw_recipe = recipe_from(super::GOLDEN);
    let product = VerifiedLoopRecipeProducerFacadeV1::consume(VerifiedLoopRecipeDemandV1::new(
        raw_recipe.clone(),
        LoopRouteId::NestedLoopMinimal,
    ))
    .expect("nested Always semantic product");
    assert_eq!(product.diagnostic_route(), LoopRouteId::NestedLoopMinimal);
    assert_eq!(product.join_sig().as_sig().loops.len(), 2);

    let source_binding = projection
        .into_source_binding(product.recipe())
        .expect("source/recipe correspondence");
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::NestedPredicateV1),
        source_binding,
        raw_recipe.clone(),
    );
    crate::mir::loop_recipe_contract::verify_artifact_for_test(artifact)
        .expect("source-bound nested artifact");

    let rebound = LoopRecipeVerifierV1::verify(raw_recipe).expect("rebound recipe");
    let normalized_product =
        LoopRecipeNormalizerV1::normalize_semantic(product.recipe()).expect("product semantics");
    let normalized_rebound =
        LoopRecipeNormalizerV1::normalize_semantic(&rebound).expect("rebound semantics");
    assert_eq!(normalized_product, normalized_rebound);
    let rebound_sig = LoopJoinSigElaboratorV1::elaborate(&rebound).expect("rebound JoinSig");
    assert_eq!(product.join_sig().as_sig(), rebound_sig.as_sig());
}

#[test]
fn facade_reports_join_sig_reject_without_retry() {
    let artifact: LoopRecipeArtifactV1 =
        serde_json::from_str(super::GOLDEN).expect("nested golden");
    let mut recipe = artifact.recipe().clone();
    recipe.blocks[1]
        .items
        .push(crate::mir::loop_recipe_contract::LoopItemKeyV1::new(10));
    recipe
        .items
        .push(crate::mir::loop_recipe_contract::LoopRecipeItemRowV1 {
            key: crate::mir::loop_recipe_contract::LoopItemKeyV1::new(10),
            item: crate::mir::loop_recipe_contract::LoopRecipeItemV1::Operation {
                operation: crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                    result: crate::mir::loop_recipe_contract::LoopValueKeyV1::new(7),
                    value: 0,
                },
            },
        });
    recipe
        .values
        .push(crate::mir::loop_recipe_contract::LoopRecipeValueV1 {
            key: crate::mir::loop_recipe_contract::LoopValueKeyV1::new(7),
            class: crate::mir::loop_recipe_contract::LoopValueClassV1::I64,
        });
    let result = VerifiedLoopRecipeProducerFacadeV1::consume(VerifiedLoopRecipeDemandV1::new(
        recipe,
        LoopRouteId::AccumConstLoop,
    ));
    assert!(matches!(
        result,
        Err(ProducerRejectV1::JoinSig(
            LoopJoinSigRejectReasonV1::UnreachableItem { .. }
        ))
    ));
}
