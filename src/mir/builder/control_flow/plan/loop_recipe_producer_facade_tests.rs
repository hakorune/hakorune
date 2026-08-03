//! Caller-zero semantic producer facade proof.
//!
//! This is deliberately a test-only migration seam.  It consumes one already
//! selected owned recipe plus an opaque diagnostic receipt, then performs the
//! single verifier -> logical JoinSig terminal transition.  It does not own
//! route policy, AST/source lookup, Builder/PHI state, or publication.

#![cfg(test)]

use crate::mir::loop_recipe_contract::{
    LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopRecipeArtifactV1,
    LoopRecipeNormalizerV1, LoopRecipeRejectReasonV1, LoopRecipeV1,
    LoopRecipeVerifierV1, VerifiedLoopJoinSigV1, VerifiedLoopRecipeV1,
};
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;

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
        let join_sig = LoopJoinSigElaboratorV1::elaborate(&recipe)
            .map_err(ProducerRejectV1::JoinSig)?;
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
fn facade_reports_join_sig_reject_without_retry() {
    let artifact: LoopRecipeArtifactV1 = serde_json::from_str(super::GOLDEN).expect("nested golden");
    let mut recipe = artifact.recipe().clone();
    recipe.blocks[1]
        .items
        .push(crate::mir::loop_recipe_contract::LoopItemKeyV1::new(10));
    recipe.items.push(crate::mir::loop_recipe_contract::LoopRecipeItemRowV1 {
        key: crate::mir::loop_recipe_contract::LoopItemKeyV1::new(10),
        item: crate::mir::loop_recipe_contract::LoopRecipeItemV1::Operation {
            operation: crate::mir::loop_recipe_contract::LoopOperationV1::ConstI64 {
                result: crate::mir::loop_recipe_contract::LoopValueKeyV1::new(7),
                value: 0,
            },
        },
    });
    recipe.values.push(crate::mir::loop_recipe_contract::LoopRecipeValueV1 {
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
