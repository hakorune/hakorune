//! Caller-zero Direct Accum producer for the portable Loop recipe contract.
//!
//! This module consumes one already-selected semantic demand. It does not
//! inspect syntax, select routes, or allocate physical identities.

use crate::mir::loop_structural_facts::{
    DirectAccumFactsPayloadRejectV1, DirectAccumStructuralShapeV1, LoopRootSourceBindingRejectV1,
    VerifiedSelectedLoopRecipeDemandV1,
};

use super::error::LoopRecipeRejectReasonV1;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
};
use super::join_sig::{LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, VerifiedLoopJoinSigV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopOperationV1, LoopRecipeArtifactV1,
    LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1, LoopRecipeItemRowV1,
    LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeV1, LoopRecipeValueV1, LoopValueClassV1,
};
use super::verify::{LoopRecipeVerifierV1, VerifiedLoopRecipeV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectAccumRecipeProducerRejectV1 {
    FactsPayload(DirectAccumFactsPayloadRejectV1),
    ConflictingBindingRoles,
    InductionRoleMismatch,
    AccumulatorRoleMismatch,
    SourceBinding(LoopRootSourceBindingRejectV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedDirectAccumRecipeProductV1 {
    recipe: VerifiedLoopRecipeV1,
    join_sig: VerifiedLoopJoinSigV1,
}

impl VerifiedDirectAccumRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &VerifiedLoopRecipeV1 {
        &self.recipe
    }

    pub(crate) fn join_sig(&self) -> &VerifiedLoopJoinSigV1 {
        &self.join_sig
    }

    pub(crate) fn into_parts(self) -> (VerifiedLoopRecipeV1, VerifiedLoopJoinSigV1) {
        (self.recipe, self.join_sig)
    }
}

pub(crate) fn produce_direct_accum_recipe_v1(
    demand: VerifiedSelectedLoopRecipeDemandV1,
) -> Result<VerifiedDirectAccumRecipeProductV1, DirectAccumRecipeProducerRejectV1> {
    let (_winner, facts, source) = demand.into_parts();
    let shape = facts
        .into_direct_accum_v1()
        .map_err(DirectAccumRecipeProducerRejectV1::FactsPayload)?;
    validate_roles(&shape)?;
    let source_root = crate::mir::loop_structural_facts::bind_resolved_loop_root_v1(source)
        .map_err(DirectAccumRecipeProducerRejectV1::SourceBinding)?;
    let recipe = direct_accum_recipe(&shape);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(DirectAccumRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::DirectAccumV1),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact)
        .map_err(DirectAccumRecipeProducerRejectV1::Recipe)?;
    let verified_recipe = verified_artifact.into_recipe();
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(DirectAccumRecipeProducerRejectV1::JoinSig)?;
    Ok(VerifiedDirectAccumRecipeProductV1 {
        recipe: verified_recipe,
        join_sig,
    })
}

fn validate_roles(
    shape: &DirectAccumStructuralShapeV1,
) -> Result<(), DirectAccumRecipeProducerRejectV1> {
    if shape.induction == shape.accumulator {
        return Err(DirectAccumRecipeProducerRejectV1::ConflictingBindingRoles);
    }
    if shape.condition_binding != shape.induction || shape.step.binding != shape.induction {
        return Err(DirectAccumRecipeProducerRejectV1::InductionRoleMismatch);
    }
    if shape.update.binding != shape.accumulator {
        return Err(DirectAccumRecipeProducerRejectV1::AccumulatorRoleMismatch);
    }
    Ok(())
}

fn direct_accum_recipe(shape: &DirectAccumStructuralShapeV1) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);
    let induction = LoopBindingKeyV1::new(0);
    let accumulator = LoopBindingKeyV1::new(1);
    let values = (0..=10)
        .map(|raw| LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(raw),
            class: if raw == 4 {
                LoopValueClassV1::Bool
            } else {
                LoopValueClassV1::I64
            },
        })
        .collect();
    let item = |key, item| LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(key),
        item,
    };
    let mut items = vec![
        item(
            0,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(3),
                    value: shape.condition_bound,
                },
            },
        ),
        item(
            1,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding: induction,
                    result: LoopValueKeyV1::new(2),
                },
            },
        ),
        item(
            2,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::CompareI64 {
                    op: LoopCompareI64OpV1::Less,
                    left: LoopValueKeyV1::new(2),
                    right: LoopValueKeyV1::new(3),
                    result: LoopValueKeyV1::new(4),
                },
            },
        ),
    ];
    items.extend(body_update_items(3, accumulator, shape.update.delta));
    items.extend(body_update_items(7, induction, shape.step.delta));

    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![super::schema::LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: condition_block,
                value: LoopValueKeyV1::new(4),
            },
            body: body_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: vec![
                    LoopItemKeyV1::new(0),
                    LoopItemKeyV1::new(1),
                    LoopItemKeyV1::new(2),
                ],
            },
            LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: (3..=10).map(LoopItemKeyV1::new).collect(),
            },
        ],
        items,
        bindings: vec![
            LoopRecipeBindingV1 {
                key: induction,
                label: "induction".into(),
                class: LoopValueClassV1::I64,
            },
            LoopRecipeBindingV1 {
                key: accumulator,
                label: "accumulator".into(),
                class: LoopValueClassV1::I64,
            },
        ],
        values,
        inputs: vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)],
        carriers: vec![
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(0),
                owner_loop: loop_key,
                binding: induction,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(0),
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(1),
                owner_loop: loop_key,
                binding: accumulator,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(1),
            },
        ],
        exits: Vec::new(),
    }
}

fn body_update_items(
    start: u32,
    binding: LoopBindingKeyV1,
    delta: i64,
) -> [LoopRecipeItemRowV1; 4] {
    let read = if binding == LoopBindingKeyV1::new(1) {
        5
    } else {
        8
    };
    let constant = if binding == LoopBindingKeyV1::new(1) {
        6
    } else {
        9
    };
    let result = if binding == LoopBindingKeyV1::new(1) {
        7
    } else {
        10
    };
    [
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding,
                    result: LoopValueKeyV1::new(read),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start + 1),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(constant),
                    value: delta,
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start + 2),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::BinaryI64 {
                    op: LoopBinaryI64OpV1::Add,
                    left: LoopValueKeyV1::new(read),
                    right: LoopValueKeyV1::new(constant),
                    result: LoopValueKeyV1::new(result),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start + 3),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::WriteBinding {
                    binding,
                    value: LoopValueKeyV1::new(result),
                },
            },
        },
    ]
}
