//! Caller-zero LoopTrue Recipe producer.
//!
//! This profile consumes the sealed policy demand, emits the existing portable
//! Recipe envelope, verifies it, and delegates logical elaboration to the one
//! shared JoinSig owner. It does not inspect syntax or allocate physical IDs.

use crate::mir::compiler::loop_true_break_continue_projection::VerifiedLoopTrueBreakContinueSourceShapeV1;
use crate::mir::loop_route_policy::{
    VerifiedLoopTrueBreakContinuePolicyDemandV1, VerifiedLoopTrueBreakContinuePolicyReceiptV1,
};

use super::error::LoopRecipeRejectReasonV1;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::join_sig::{LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, VerifiedLoopJoinSigV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopCompareI64OpV1, LoopConditionV1, LoopExitKindV1, LoopNodeV1, LoopOperationV1,
    LoopRecipeArtifactV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeExitV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeV1,
    LoopRecipeValueV1, LoopValueClassV1,
};
use super::verify::{verify_source_bound_recipe_v1, LoopRecipeVerifierV1, VerifiedLoopRecipeV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopTrueBreakContinueRecipeProducerRejectV1 {
    PolicyFrameMismatch,
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedLoopTrueBreakContinueRecipeProductV1 {
    policy_receipt: VerifiedLoopTrueBreakContinuePolicyReceiptV1,
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
}

impl VerifiedLoopTrueBreakContinueRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn policy_receipt(&self) -> &VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
        &self.policy_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopTrueBreakContinuePolicyReceiptV1,
        VerifiedLoopRecipeV1,
        VerifiedLoopJoinSigV1,
    ) {
        (self.policy_receipt, self.recipe, self.join_sig)
    }
}

pub(crate) fn produce_loop_true_break_continue_recipe_v1(
    demand: VerifiedLoopTrueBreakContinuePolicyDemandV1,
) -> Result<VerifiedLoopTrueBreakContinueRecipeProductV1, LoopTrueBreakContinueRecipeProducerRejectV1>
{
    let (policy_receipt, projection) = demand.into_parts();
    if !policy_receipt
        .frame_key()
        .matches(projection.root_frame_key())
    {
        return Err(LoopTrueBreakContinueRecipeProducerRejectV1::PolicyFrameMismatch);
    }
    let (source_root, shape, _frame_key) = projection.into_parts();
    let recipe = loop_true_break_continue_recipe(&shape);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1),
        source_binding,
        recipe,
    );
    let verified_recipe = verify_source_bound_recipe_v1(artifact)
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Recipe)?;
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::JoinSig)?;
    Ok(VerifiedLoopTrueBreakContinueRecipeProductV1 {
        policy_receipt,
        recipe: verified_recipe,
        join_sig,
    })
}

fn loop_true_break_continue_recipe(
    shape: &VerifiedLoopTrueBreakContinueSourceShapeV1,
) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let binding = LoopBindingKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let then_block = LoopBlockKeyV1::new(1);
    let else_block = LoopBlockKeyV1::new(2);
    let input = LoopValueKeyV1::new(0);
    let read = LoopValueKeyV1::new(1);
    let bound = LoopValueKeyV1::new(2);
    let condition = LoopValueKeyV1::new(3);
    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Always,
            body: condition_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: (0..=3).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV1 {
                key: then_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(4)],
            },
            LoopRecipeBlockV1 {
                key: else_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(5)],
            },
        ],
        items: vec![
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ReadBinding {
                        binding,
                        result: read,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ConstI64 {
                        result: bound,
                        value: shape.branch_condition_bound,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(2),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::CompareI64 {
                        op: LoopCompareI64OpV1::Equal,
                        left: read,
                        right: bound,
                        result: condition,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(3),
                item: LoopRecipeItemV1::If {
                    condition,
                    then_block,
                    else_block: Some(else_block),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(4),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(5),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(1),
                },
            },
        ],
        bindings: vec![LoopRecipeBindingV1 {
            key: binding,
            label: "loop_true_branch_condition".into(),
            class: LoopValueClassV1::I64,
        }],
        values: vec![
            LoopRecipeValueV1 {
                key: input,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: read,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: bound,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: condition,
                class: LoopValueClassV1::Bool,
            },
        ],
        inputs: vec![input],
        carriers: vec![LoopRecipeCarrierV1 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: loop_key,
            binding,
            class: LoopValueClassV1::I64,
            entry_value: input,
        }],
        exits: vec![
            LoopRecipeExitV1 {
                key: LoopExitKeyV1::new(0),
                owner_loop: loop_key,
                kind: LoopExitKindV1::Break {
                    target_loop: loop_key,
                },
            },
            LoopRecipeExitV1 {
                key: LoopExitKeyV1::new(1),
                owner_loop: loop_key,
                kind: LoopExitKindV1::Continue {
                    target_loop: loop_key,
                },
            },
        ],
    }
}
