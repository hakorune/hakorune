//! Caller-zero LoopCond Recipe producer.
//!
//! This profile consumes the sealed policy demand, emits the existing
//! portable Recipe envelope, verifies it, and delegates logical
//! elaboration to the one shared JoinSig owner. It does not inspect
//! syntax or allocate physical IDs.

use crate::mir::compiler::loop_cond_break_continue_typed_map::LoopCondTypedCompareV1;
use crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1;
use crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyDemandV1;

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
pub(crate) enum LoopCondBreakContinueRecipeProducerRejectV1 {
    PolicyFrameMismatch,
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedLoopCondBreakContinueRecipeProductV1 {
    policy_receipt: crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyReceiptV1,
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
}

impl VerifiedLoopCondBreakContinueRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn policy_receipt(
        &self,
    ) -> &crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyReceiptV1 {
        &self.policy_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyReceiptV1,
        VerifiedLoopRecipeV1,
        VerifiedLoopJoinSigV1,
    ) {
        (self.policy_receipt, self.recipe, self.join_sig)
    }
}

pub(crate) fn produce_loop_cond_break_continue_recipe_v1(
    demand: VerifiedLoopCondBreakContinuePolicyDemandV1,
) -> Result<
    VerifiedLoopCondBreakContinueRecipeProductV1,
    LoopCondBreakContinueRecipeProducerRejectV1,
> {
    let (policy_receipt, map) = demand.into_parts();
    let (source_root, projection, _carrier, loop_condition, branch_condition, _frame_key) =
        map.into_parts();
    if !policy_receipt
        .frame_key()
        .matches(projection.root_frame_key())
    {
        return Err(LoopCondBreakContinueRecipeProducerRejectV1::PolicyFrameMismatch);
    }
    let recipe = loop_cond_break_continue_recipe(&loop_condition, &branch_condition);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::LoopCondBreakContinueV1),
        source_binding,
        recipe,
    );
    let verified_recipe = verify_source_bound_recipe_v1(artifact)
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Recipe)?;
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::JoinSig)?;
    Ok(VerifiedLoopCondBreakContinueRecipeProductV1 {
        policy_receipt,
        recipe: verified_recipe,
        join_sig,
    })
}

fn compare_op(operator: SyntaxBinaryOperatorV1) -> LoopCompareI64OpV1 {
    match operator {
        SyntaxBinaryOperatorV1::Less => LoopCompareI64OpV1::Less,
        SyntaxBinaryOperatorV1::LessEqual => LoopCompareI64OpV1::LessEqual,
        SyntaxBinaryOperatorV1::Equal => LoopCompareI64OpV1::Equal,
        // The typed map seals only the compare vocabulary; any other
        // operator cannot reach this producer.
        _ => unreachable!("typed LoopCond compare operator outside the sealed vocabulary"),
    }
}

pub(super) fn loop_cond_break_continue_recipe(
    loop_condition: &LoopCondTypedCompareV1,
    branch_condition: &LoopCondTypedCompareV1,
) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let binding = LoopBindingKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);
    let then_block = LoopBlockKeyV1::new(2);
    let else_block = LoopBlockKeyV1::new(3);
    let input = LoopValueKeyV1::new(0);
    let loop_read = LoopValueKeyV1::new(1);
    let loop_bound = LoopValueKeyV1::new(2);
    let loop_predicate = LoopValueKeyV1::new(3);
    let branch_read = LoopValueKeyV1::new(4);
    let branch_bound = LoopValueKeyV1::new(5);
    let branch_predicate = LoopValueKeyV1::new(6);
    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: condition_block,
                value: loop_predicate,
            },
            body: body_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: (0..=2).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: (3..=6).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV1 {
                key: then_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(7)],
            },
            LoopRecipeBlockV1 {
                key: else_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(8)],
            },
        ],
        items: vec![
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ReadBinding {
                        binding,
                        result: loop_read,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ConstI64 {
                        result: loop_bound,
                        value: loop_condition.bound,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(2),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::CompareI64 {
                        op: compare_op(loop_condition.operator),
                        left: loop_read,
                        right: loop_bound,
                        result: loop_predicate,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(3),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ReadBinding {
                        binding,
                        result: branch_read,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(4),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ConstI64 {
                        result: branch_bound,
                        value: branch_condition.bound,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(5),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::CompareI64 {
                        op: compare_op(branch_condition.operator),
                        left: branch_read,
                        right: branch_bound,
                        result: branch_predicate,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(6),
                item: LoopRecipeItemV1::If {
                    condition: branch_predicate,
                    then_block,
                    else_block: Some(else_block),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(7),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(8),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(1),
                },
            },
        ],
        bindings: vec![LoopRecipeBindingV1 {
            key: binding,
            label: "loop_cond_carrier".into(),
            class: LoopValueClassV1::I64,
        }],
        values: vec![
            LoopRecipeValueV1 {
                key: input,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: loop_read,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: loop_bound,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: loop_predicate,
                class: LoopValueClassV1::Bool,
            },
            LoopRecipeValueV1 {
                key: branch_read,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: branch_bound,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: branch_predicate,
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
